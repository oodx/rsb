use crate::global::expand_vars;
use std::path::Path;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref TEMP_FILES_TO_CLEAN: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

// --- File I/O Functions ---

pub fn read_file(path: &str) -> String {
    let expanded_path = expand_vars(path);
    match std::fs::read_to_string(&expanded_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("rsb-error: Failed to read file '{}': {}", expanded_path, e);
            std::process::exit(1);
        }
    }
}

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

pub fn mkdir_p(path: &str) -> bool { std::fs::create_dir_all(expand_vars(path)).is_ok() }

pub fn rm(path: &str) -> bool {
    let expanded_path = expand_vars(path);
    if Path::new(&expanded_path).is_dir() { std::fs::remove_dir(&expanded_path).is_ok() } else { std::fs::remove_file(&expanded_path).is_ok() }
}

pub fn rm_rf(path: &str) -> bool {
    let expanded_path = expand_vars(path);
    if Path::new(&expanded_path).is_dir() { std::fs::remove_dir_all(&expanded_path).is_ok() } else { std::fs::remove_file(&expanded_path).is_ok() }
}

pub fn cp(src: &str, dest: &str) -> bool {
    let src_exp = expand_vars(src);
    let dest_exp = expand_vars(dest);
    std::fs::copy(&src_exp, &dest_exp).is_ok()
}

pub fn cp_r(src: &str, dest: &str) -> bool {
    use crate::os::is_command;
    let src_exp = expand_vars(src);
    let dest_exp = expand_vars(dest);

    if is_command("cp") {
        let status = std::process::Command::new("cp").arg("-r").arg(&src_exp).arg(&dest_exp).status();
        status.map(|s| s.success()).unwrap_or(false)
    } else {
        if !is_dir(&src_exp) { return false; }
        if !mkdir_p(&dest_exp) { return false; }
        for entry in std::fs::read_dir(&src_exp).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            let dest_path = Path::new(&dest_exp).join(entry.file_name());
            if entry_path.is_dir() {
                if !cp_r(entry_path.to_str().unwrap(), dest_path.to_str().unwrap()) { return false; }
            } else if let Err(_) = std::fs::copy(&entry_path, &dest_path) {
                return false;
            }
        }
        true
    }
}

pub fn mv(src: &str, dest: &str) -> bool { std::fs::rename(expand_vars(src), expand_vars(dest)).is_ok() }

pub fn touch(path: &str) -> bool {
    use std::fs::OpenOptions;
    let expanded_path = expand_vars(path);
    if Path::new(&expanded_path).exists() {
        let file = OpenOptions::new().append(true).open(&expanded_path);
        if let Ok(file) = file { return file.set_len(file.metadata().unwrap().len()).is_ok(); }
        false
    } else {
        std::fs::File::create(&expanded_path).is_ok()
    }
}

// --- Meta Parsing ---

pub fn extract_meta_from_file(path: &str) -> std::collections::HashMap<String, String> {
    let content = read_file(path);
    let mut meta = std::collections::HashMap::new();
    let re = regex::Regex::new(r"^\s*#\s*([^:]+?)\s*:\s*(.+?)\s*$").unwrap();
    for line in content.lines() {
        if let Some(caps) = re.captures(line) {
            let key = caps.get(1).map_or("", |m| m.as_str()).trim().to_string();
            let value = caps.get(2).map_or("", |m| m.as_str()).trim().to_string();
            if !key.is_empty() { meta.insert(key, value); }
        }
    }
    meta
}

// --- Advanced File Utilities ---

pub fn backup_file(path: &str, suffix: &str) -> Result<String, std::io::Error> {
    let expanded_path = expand_vars(path);
    let backup_path = format!("{}{}", expanded_path, suffix);
    std::fs::copy(&expanded_path, &backup_path)?;
    Ok(backup_path)
}

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
pub fn chmod(path: &str, mode: &str) -> Result<(), std::io::Error> {
    let mode_octal = u32::from_str_radix(mode, 8).unwrap_or(0o644);
    let perms = std::fs::Permissions::from_mode(mode_octal);
    std::fs::set_permissions(expand_vars(path), perms)
}

#[cfg(not(unix))]
pub fn chmod(_path: &str, _mode: &str) -> Result<(), std::io::Error> { Ok(()) }

// --- Path Utilities ---

pub fn path_canon(path: &str) -> Result<String, std::io::Error> {
    std::fs::canonicalize(expand_vars(path)).map(|p| p.to_string_lossy().to_string())
}

pub fn path_split(path: &str) -> std::collections::HashMap<String, String> {
    let p = std::path::Path::new(path);
    let mut map = std::collections::HashMap::new();
    map.insert("path".to_string(), path.to_string());
    if let Some(parent) = p.parent().and_then(|s| s.to_str()) { map.insert("parent".to_string(), parent.to_string()); }
    if let Some(file_name) = p.file_name().and_then(|s| s.to_str()) { map.insert("file_name".to_string(), file_name.to_string()); }
    if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) { map.insert("file_stem".to_string(), stem.to_string()); }
    if let Some(extension) = p.extension().and_then(|s| s.to_str()) { map.insert("extension".to_string(), extension.to_string()); }
    map
}

pub fn parse_meta_keys(path: &str, into: &str) {
    let content = read_file(path);
    for line in content.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('#') { break; }
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

pub fn is_file(path: &str) -> bool { Path::new(&expand_vars(path)).is_file() }
pub fn is_dir(path: &str) -> bool { Path::new(&expand_vars(path)).is_dir() }
pub fn is_entity(path: &str) -> bool { Path::new(&expand_vars(path)).exists() }
pub fn is_link(path: &str) -> bool { Path::new(&expand_vars(path)).is_symlink() }

pub fn is_readable(path: &str) -> bool {
    std::fs::metadata(&expand_vars(path)).map(|m| !m.permissions().readonly()).unwrap_or(false)
}
pub fn is_writable(path: &str) -> bool {
    std::fs::metadata(&expand_vars(path)).map(|m| !m.permissions().readonly()).unwrap_or(false)
}

#[cfg(unix)]
pub fn is_executable(path: &str) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(&expand_vars(path)).map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false)
}
#[cfg(not(unix))]
pub fn is_executable(path: &str) -> bool { is_file(path) }

pub fn is_nonempty_file(path: &str) -> bool {
    std::fs::metadata(&expand_vars(path)).map(|m| m.is_file() && m.len() > 0).unwrap_or(false)
}

pub fn load_dict_from_file(path: &str) -> Vec<String> {
    let content = read_file(path);
    content.split_whitespace().map(|s| s.to_string()).collect()
}

// --- Counters (wc-like) ---
/// Count lines in a string (splits on '\n').
pub fn count_lines_str(s: &str) -> usize { s.lines().count() }

/// Count words in a string (whitespace-delimited).
pub fn count_words_str(s: &str) -> usize { s.split_whitespace().count() }

/// Count Unicode scalar characters in a string.
pub fn count_chars_str(s: &str) -> usize { s.chars().count() }

/// Return wc-like triple for a string: (lines, words, chars).
pub fn wc_tuple_str(s: &str) -> (usize, usize, usize) {
    (count_lines_str(s), count_words_str(s), count_chars_str(s))
}

/// wc as a single space-delimited string: "lines words chars".
pub fn wc_string(s: &str) -> String {
    let (l, w, c) = wc_tuple_str(s);
    format!("{} {} {}", l, w, c)
}

/// File-backed counters
pub fn count_lines_file(path: &str) -> usize {
    let content = read_file(path);
    count_lines_str(&content)
}

pub fn count_words_file(path: &str) -> usize {
    let content = read_file(path);
    count_words_str(&content)
}

pub fn count_chars_file(path: &str) -> usize {
    let content = read_file(path);
    count_chars_str(&content)
}

pub fn wc_tuple_file(path: &str) -> (usize, usize, usize) {
    let content = read_file(path);
    wc_tuple_str(&content)
}

pub fn wc_file_string(path: &str) -> String {
    let (l, w, c) = wc_tuple_file(path);
    format!("{} {} {}", l, w, c)
}

// --- Streaming counters for large files ---
pub fn count_lines_file_stream(path: &str) -> usize {
    use std::io::{BufRead, BufReader};
    let f = std::fs::File::open(expand_vars(path)).unwrap();
    let reader = BufReader::new(f);
    reader.lines().count()
}

pub fn wc_tuple_file_stream(path: &str) -> (usize, usize, usize) {
    use std::io::{BufRead, BufReader};
    let f = std::fs::File::open(expand_vars(path)).unwrap();
    let reader = BufReader::new(f);
    let mut lines = 0usize;
    let mut words = 0usize;
    let mut chars = 0usize;
    for line in reader.lines() {
        if let Ok(l) = line {
            lines += 1;
            words += l.split_whitespace().count();
            chars += l.chars().count();
        }
    }
    (lines, words, chars)
}

pub fn wc_file_string_stream(path: &str) -> String {
    let (l, w, c) = wc_tuple_file_stream(path);
    format!("{} {} {}", l, w, c)
}
pub fn create_temp_file_path(name_type: &str) -> String {
    let tmp_dir = crate::global::get_var("XDG_TMP");
    let _ = std::fs::create_dir_all(&tmp_dir);
    let filename = match name_type {
        "pid" => format!("{}.tmp", std::process::id()),
        "timestamp" => format!("{}.tmp", chrono::Utc::now().timestamp_millis()),
        "random" | _ => format!("{}.tmp", crate::gx::string::get_rand_alnum(8)),
    };
    let mut path = std::path::PathBuf::from(tmp_dir);
    path.push(filename);
    path.to_string_lossy().to_string()
}

pub fn capture_stream_to_temp_file(stream: &mut crate::streams::Stream) -> String {
    let path = create_temp_file_path("random");
    stream.clone().to_file(&path);
    TEMP_FILES_TO_CLEAN.lock().unwrap().push(path.clone());
    path
}

pub fn cleanup_temp_files() {
    if let Ok(files) = TEMP_FILES_TO_CLEAN.lock() {
        for file in files.iter() { let _ = std::fs::remove_file(file); }
    }
}

// File-based sed helpers now live under parse::sed_file
