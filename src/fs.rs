// src/fs.rs
use crate::global::expand_vars;
use std::path::Path;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref TEMP_FILES_TO_CLEAN: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

// This module will contain file system operations like read_file,
// write_file, mkdir_p, rm_rf, etc.

// --- File I/O Functions ---

/// Reads the entire contents of a file into a string.
/// Exits the process with an error if the file cannot be read.
pub fn read_file(path: &str) -> String {
    let expanded_path = expand_vars(path);
    match std::fs::read_to_string(&expanded_path) {
        Ok(content) => content,
        Err(e) => {
            // Using the error! macro is not possible here due to module dependencies.
            // A direct eprintln is the most robust solution.
            eprintln!("rsb-error: Failed to read file '{}': {}", expanded_path, e);
            std::process::exit(1);
        }
    }
}

/// Writes a string to a file, creating the file if it does not exist.
/// Exits the process with an error if the file cannot be written.
pub fn write_file(path: &str, content: &str) {
    let expanded_path = expand_vars(path);
    if let Some(parent) = Path::new(&expanded_path).parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("rsb-error: Failed to create directory '{}': {}", parent.display(), e);
            std::process::exit(1);
        }
    }
    if let Err(e) = std::fs::write(&expanded_path, content) {
        eprintln!("rsb-error: Failed to write to file '{}': {}", expanded_path, e);
        std::process::exit(1);
    }
}

/// Appends a string to a file, creating the file if it does not exist.
/// Exits the process with an error if the file cannot be written to.
pub fn append_file(path: &str, content: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;

    let expanded_path = expand_vars(path);
    if let Some(parent) = Path::new(&expanded_path).parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("rsb-error: Failed to create directory '{}': {}", parent.display(), e);
            std::process::exit(1);
        }
    }

    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&expanded_path)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("rsb-error: Failed to open file '{}' for appending: {}", expanded_path, e);
            std::process::exit(1);
        }
    };

    if let Err(e) = writeln!(file, "{}", content) {
        eprintln!("rsb-error: Failed to append to file '{}': {}", expanded_path, e);
        std::process::exit(1);
    }
}


// --- File System Manipulation Functions ---

/// Creates a directory, including any necessary parent directories.
pub fn mkdir_p(path: &str) -> bool {
    std::fs::create_dir_all(expand_vars(path)).is_ok()
}

/// Removes a file or an empty directory.
pub fn rm(path: &str) -> bool {
    let expanded_path = expand_vars(path);
    if Path::new(&expanded_path).is_dir() {
        std::fs::remove_dir(&expanded_path).is_ok()
    } else {
        std::fs::remove_file(&expanded_path).is_ok()
    }
}

/// Removes a file or a directory recursively.
pub fn rm_rf(path: &str) -> bool {
    let expanded_path = expand_vars(path);
    if Path::new(&expanded_path).is_dir() {
        std::fs::remove_dir_all(&expanded_path).is_ok()
    } else {
        std::fs::remove_file(&expanded_path).is_ok()
    }
}

/// Copies a file.
pub fn cp(src: &str, dest: &str) -> bool {
    let src_exp = expand_vars(src);
    let dest_exp = expand_vars(dest);
    std::fs::copy(&src_exp, &dest_exp).is_ok()
}

/// Copies a directory recursively. Uses the system's `cp -r` if available.
pub fn cp_r(src: &str, dest: &str) -> bool {
    use crate::os::is_command;
    let src_exp = expand_vars(src);
    let dest_exp = expand_vars(dest);

    if is_command("cp") {
        let status = std::process::Command::new("cp")
            .arg("-r")
            .arg(&src_exp)
            .arg(&dest_exp)
            .status();
        status.map(|s| s.success()).unwrap_or(false)
    } else {
        // Fallback to a basic recursive copy.
        // This is a simplified implementation.
        if !is_dir(&src_exp) {
            return false;
        }
        if !mkdir_p(&dest_exp) {
            return false;
        }
        for entry in std::fs::read_dir(&src_exp).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            let dest_path = Path::new(&dest_exp).join(entry.file_name());
            if entry_path.is_dir() {
                if !cp_r(entry_path.to_str().unwrap(), dest_path.to_str().unwrap()) {
                    return false;
                }
            } else if let Err(_) = std::fs::copy(&entry_path, &dest_path) {
                return false;
            }
        }
        true
    }
}

/// Moves or renames a file or directory.
pub fn mv(src: &str, dest: &str) -> bool {
    std::fs::rename(expand_vars(src), expand_vars(dest)).is_ok()
}

/// Creates a file if it does not exist, or updates its modification time if it does.
pub fn touch(path: &str) -> bool {
    use std::fs::OpenOptions;
    let expanded_path = expand_vars(path);
    if Path::new(&expanded_path).exists() {
        let file = OpenOptions::new().append(true).open(&expanded_path);
        if let Ok(file) = file {
            // This is a bit of a hack to update mtime on some systems.
            // A more robust solution might use `filetime` crate.
            return file.set_len(file.metadata().unwrap().len()).is_ok();
        }
        false
    } else {
        std::fs::File::create(&expanded_path).is_ok()
    }
}


// --- Meta Parsing ---

/// Extracts key-value pairs from comments in a file.
/// Looks for patterns like `# key : value`
pub fn extract_meta_from_file(path: &str) -> std::collections::HashMap<String, String> {
    let content = read_file(path);
    let mut meta = std::collections::HashMap::new();
    // Regex to capture key and value from comments like `#  key  :  value  `
    let re = regex::Regex::new(r"^\s*#\s*([^:]+?)\s*:\s*(.+?)\s*$").unwrap();

    for line in content.lines() {
        if let Some(caps) = re.captures(line) {
            let key = caps.get(1).map_or("", |m| m.as_str()).trim().to_string();
            let value = caps.get(2).map_or("", |m| m.as_str()).trim().to_string();
            if !key.is_empty() {
                meta.insert(key, value);
            }
        }
    }
    meta
}


// --- Advanced File Utilities ---

/// Backs up a file by copying it with a given suffix.
pub fn backup_file(path: &str, suffix: &str) -> Result<String, std::io::Error> {
    let expanded_path = expand_vars(path);
    let backup_path = format!("{}{}", expanded_path, suffix);
    std::fs::copy(&expanded_path, &backup_path)?;
    Ok(backup_path)
}

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Changes the permissions of a file (Unix only).
#[cfg(unix)]
pub fn chmod(path: &str, mode: &str) -> Result<(), std::io::Error> {
    let mode_octal = u32::from_str_radix(mode, 8).unwrap_or(0o644);
    let perms = std::fs::Permissions::from_mode(mode_octal);
    std::fs::set_permissions(expand_vars(path), perms)
}

#[cfg(not(unix))]
pub fn chmod(path: &str, mode: &str) -> Result<(), std::io::Error> {
    // No-op on non-unix systems
    Ok(())
}


// --- Path Utilities ---

/// Canonicalizes a path, returning its absolute form.
pub fn path_canon(path: &str) -> Result<String, std::io::Error> {
    std::fs::canonicalize(expand_vars(path))
        .map(|p| p.to_string_lossy().to_string())
}

/// Splits a path into its components.
pub fn path_split(path: &str) -> std::collections::HashMap<String, String> {
    let p = std::path::Path::new(path);
    let mut map = std::collections::HashMap::new();
    map.insert("path".to_string(), path.to_string());
    if let Some(parent) = p.parent().and_then(|s| s.to_str()) {
        map.insert("parent".to_string(), parent.to_string());
    }
    if let Some(file_name) = p.file_name().and_then(|s| s.to_str()) {
        map.insert("file_name".to_string(), file_name.to_string());
    }
    if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
        map.insert("file_stem".to_string(), stem.to_string());
    }
    if let Some(extension) = p.extension().and_then(|s| s.to_str()) {
        map.insert("extension".to_string(), extension.to_string());
    }
    map
}

/// Parses metadata keys from a file's leading comment lines and populates context variables.
/// Supported formats: lines starting with '#' and containing `key: value` or `key : value`.
pub fn parse_meta_keys(path: &str, into: &str) {
    let content = read_file(path);
    for line in content.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('#') {
            // Stop at first non-comment line to mimic header parsing.
            break;
        }
        let rest = trimmed.trim_start_matches('#').trim();
        if rest.is_empty() { continue; }
        if let Some((k, v)) = rest.split_once(':') {
            let key = k.trim();
            let value = v.trim();
            if !key.is_empty() {
                crate::global::set_var(&format!("{}_{}", into, key), value);
            }
        }
    }
}


// --- File System Test Functions ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_split() {
        let parts = path_split("/home/user/file.txt");
        assert_eq!(parts.get("parent").unwrap(), "/home/user");
        assert_eq!(parts.get("file_name").unwrap(), "file.txt");
        assert_eq!(parts.get("file_stem").unwrap(), "file");
        assert_eq!(parts.get("extension").unwrap(), "txt");
    }
}


/// Checks if a path exists and is a regular file.
pub fn is_file(path: &str) -> bool {
    Path::new(&expand_vars(path)).is_file()
}

/// Checks if a path exists and is a directory.
pub fn is_dir(path: &str) -> bool {
    Path::new(&expand_vars(path)).is_dir()
}

/// Checks if a path exists (can be a file, directory, or symlink).
pub fn is_entity(path: &str) -> bool {
    Path::new(&expand_vars(path)).exists()
}

/// Checks if a path exists and is a symbolic link.
pub fn is_link(path: &str) -> bool {
    Path::new(&expand_vars(path)).is_symlink()
}

/// Checks if a file is readable.
pub fn is_readable(path: &str) -> bool {
    std::fs::metadata(&expand_vars(path))
        .map(|m| !m.permissions().readonly())
        .unwrap_or(false)
}

/// Checks if a file is writable.
pub fn is_writable(path: &str) -> bool {
    std::fs::metadata(&expand_vars(path))
        .map(|m| !m.permissions().readonly())
        .unwrap_or(false)
}

/// Checks if a file is executable.
#[cfg(unix)]
pub fn is_executable(path: &str) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(&expand_vars(path))
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
pub fn is_executable(path: &str) -> bool {
    // A basic check for non-Unix systems, might not be fully accurate.
    // For Windows, we could check for .exe, .bat, etc.
    is_file(path)
}

/// Checks if a file exists and is not empty.
pub fn is_nonempty_file(path: &str) -> bool {
    std::fs::metadata(&expand_vars(path))
        .map(|m| m.is_file() && m.len() > 0)
        .unwrap_or(false)
}

/// Reads a file and splits it into a vector of words by whitespace.
pub fn load_dict_from_file(path: &str) -> Vec<String> {
    let content = read_file(path);
    content.split_whitespace().map(|s| s.to_string()).collect()
}

/// Generates a path for a new temporary file in the RSB temp directory.
pub fn create_temp_file_path(name_type: &str) -> String {
    let tmp_dir = crate::global::get_var("XDG_TMP");
    let _ = std::fs::create_dir_all(&tmp_dir); // Ensure the directory exists

    let filename = match name_type {
        "pid" => format!("{}.tmp", std::process::id()),
        "timestamp" => format!("{}.tmp", chrono::Utc::now().timestamp_millis()),
        "random" | _ => format!("{}.tmp", crate::random::get_rand_alnum(8)),
    };

    let mut path = std::path::PathBuf::from(tmp_dir);
    path.push(filename);
    path.to_string_lossy().to_string()
}

/// Captures a stream to a temporary file and returns the path.
/// The file is registered for cleanup on script exit.
pub fn capture_stream_to_temp_file(stream: &mut crate::streams::Stream) -> String {
    let path = create_temp_file_path("random");
    stream.clone().to_file(&path);
    TEMP_FILES_TO_CLEAN.lock().unwrap().push(path.clone());
    path
}

/// Cleans up all temporary files created during the script's execution.
/// This is intended to be called from an EXIT trap.
pub fn cleanup_temp_files() {
    if let Ok(files) = TEMP_FILES_TO_CLEAN.lock() {
        for file in files.iter() {
            let _ = std::fs::remove_file(file);
        }
    }
}

/// Applies sed_lines operation directly to a file, returns the result as a string.
pub fn sed_lines_file(path: &str, start_line: usize, end_line: usize) -> String {
    use crate::streams::Stream;
    let content = read_file(path);
    Stream::from_string(&content).sed_lines(start_line, end_line).to_string()
}

/// Applies sed_around operation directly to a file, returns the result as a string.
pub fn sed_around_file(path: &str, pattern: &str, context_lines: usize) -> String {
    use crate::streams::Stream;
    let content = read_file(path);
    Stream::from_string(&content).sed_around(pattern, context_lines).to_string()
}

/// Applies sed_insert operation directly to a file, modifying the file in place.
pub fn sed_insert_file(path: &str, content: &str, sentinel: &str) -> Result<(), String> {
    use crate::streams::Stream;
    let file_content = read_file(path);
    let result_stream = Stream::from_string(&file_content).sed_insert(content, sentinel)?;
    write_file(path, &result_stream.to_string());
    Ok(())
}

/// Applies sed_template operation directly to a file, modifying the file in place.
pub fn sed_template_file(path: &str, content: &str, sentinel: &str) {
    use crate::streams::Stream;
    let file_content = read_file(path);
    let result = Stream::from_string(&file_content).sed_template(content, sentinel).to_string();
    write_file(path, &result);
}
