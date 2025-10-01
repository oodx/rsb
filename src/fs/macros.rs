// --- File System & Temp Macros (module-owned) ---

#[macro_export]
macro_rules! chmod {
    ($path:expr, $mode:expr) => {
        $crate::fs::chmod($path, $mode).ok()
    };
}

#[macro_export]
macro_rules! backup {
    ($path:expr, $suffix:expr) => {
        $crate::fs::backup_file($path, $suffix).ok()
    };
}

#[macro_export]
macro_rules! tmp {
    () => {
        $crate::fs::create_temp_file_path("random")
    };
    ($type:ident) => {
        $crate::fs::create_temp_file_path(stringify!($type))
    };
}

#[macro_export]
macro_rules! cap_stream {
    ($stream:expr) => {
        $crate::fs::capture_stream_to_temp_file(&mut $stream)
    };
}
#[macro_export]
macro_rules! subst {
    ($stream:expr) => {
        $crate::cap_stream!($stream)
    };
}

// --- Dictionary Macros ---
#[macro_export]
macro_rules! dict {
    ($path:expr) => {
        $crate::fs::load_dict_from_file($path)
    };
}

// --- WC-like counters (string and file) ---
#[macro_export]
macro_rules! wc {
    ($content:expr) => {
        $crate::fs::wc_string($content)
    };
}

#[macro_export]
macro_rules! wc_lines {
    ($content:expr) => {
        $crate::fs::count_lines_str($content)
    };
}

#[macro_export]
macro_rules! wc_words {
    ($content:expr) => {
        $crate::fs::count_words_str($content)
    };
}

#[macro_export]
macro_rules! wc_chars {
    ($content:expr) => {
        $crate::fs::count_chars_str($content)
    };
}

#[macro_export]
macro_rules! wc_file {
    ($path:expr) => {
        $crate::fs::wc_file_string($path)
    };
}

#[macro_export]
macro_rules! wc_lines_file {
    ($path:expr) => {
        $crate::fs::count_lines_file($path)
    };
}

#[macro_export]
macro_rules! wc_words_file {
    ($path:expr) => {
        $crate::fs::count_words_file($path)
    };
}

#[macro_export]
macro_rules! wc_chars_file {
    ($path:expr) => {
        $crate::fs::count_chars_file($path)
    };
}

// File-based sed macros moved to parse::macros as an adapter over fs

// --- Archive Macros ---
#[macro_export]
macro_rules! tar {
    (create: $archive:expr, $($path:expr),+) => {{
        let paths = vec![$($path),+];
        match $crate::bash::create_tar($archive, &paths) {
            result if result.status == 0 => { $crate::okay!("Created tar archive: {}", $archive); },
            result => { $crate::error!("Failed to create tar: {}", result.error); std::process::exit(result.status); }
        }
    }};
    (extract: $archive:expr) => {{
        match $crate::bash::extract_tar($archive, None) {
            result if result.status == 0 => { $crate::okay!("Extracted tar archive: {}", $archive); },
            result => { $crate::error!("Failed to extract tar: {}", result.error); std::process::exit(result.status); }
        }
    }};
    (extract: $archive:expr, to: $dest:expr) => {{
        match $crate::bash::extract_tar($archive, Some($dest)) {
            result if result.status == 0 => { $crate::okay!("Extracted tar archive to: {}", $dest); },
            result => { $crate::error!("Failed to extract tar: {}", result.error); std::process::exit(result.status); }
        }
    }};
    (list: $archive:expr) => {{
        match $crate::bash::list_tar($archive) {
            result if result.status == 0 => result.output,
            result => { $crate::error!("Failed to list tar: {}", result.error); std::process::exit(result.status); }
        }
    }};
}

#[macro_export]
macro_rules! tar_gz {
    (create: $archive:expr, $($path:expr),+) => {{
        let paths = vec![$($path),+];
        match $crate::bash::create_tar_gz($archive, &paths) {
            result if result.status == 0 => { $crate::okay!("Created tar.gz archive: {}", $archive); },
            result => { $crate::error!("Failed to create tar.gz: {}", result.error); std::process::exit(result.status); }
        }
    }};
}

#[macro_export]
macro_rules! zip {
    (create: $archive:expr, $($path:expr),+) => {{
        let paths = vec![$($path),+];
        match $crate::bash::create_zip($archive, &paths) {
            result if result.status == 0 => { $crate::okay!("Created zip archive: {}", $archive); },
            result => { $crate::error!("Failed to create zip: {}", result.error); std::process::exit(result.status); }
        }
    }};
    (extract: $archive:expr) => {{
        match $crate::bash::extract_zip($archive, None) {
            result if result.status == 0 => { $crate::okay!("Extracted zip archive: {}", $archive); },
            result => { $crate::error!("Failed to extract zip: {}", result.error); std::process::exit(result.status); }
        }
    }};
    (extract: $archive:expr, to: $dest:expr) => {{
        match $crate::bash::extract_zip($archive, Some($dest)) {
            result if result.status == 0 => { $crate::okay!("Extracted zip archive to: {}", $dest); },
            result => { $crate::error!("Failed to extract zip: {}", result.error); std::process::exit(result.status); }
        }
    }};
    (list: $archive:expr) => {{
        match $crate::bash::list_zip($archive) {
            result if result.status == 0 => result.output,
            result => { $crate::error!("Failed to list zip: {}", result.error); std::process::exit(result.status); }
        }
    }};
}

// Simple pack/unpack macros
#[macro_export]
macro_rules! pack {
    ($archive:expr, $($path:expr),+) => {{
        let archive_path = $archive;
        if archive_path.ends_with(".tar.gz") || archive_path.ends_with(".tgz") {
            $crate::tar_gz!(create: archive_path, $($path),+);
        } else if archive_path.ends_with(".tar") {
            $crate::tar!(create: archive_path, $($path),+);
        } else if archive_path.ends_with(".zip") {
            $crate::zip!(create: archive_path, $($path),+);
        } else {
            $crate::error!("Unsupported archive format: {}", archive_path);
            std::process::exit(1);
        }
    }};
}

#[macro_export]
macro_rules! unpack {
    ($archive:expr) => {{
        let archive_path = $archive;
        if archive_path.ends_with(".tar.gz") || archive_path.ends_with(".tgz") || archive_path.ends_with(".tar") {
            $crate::tar!(extract: archive_path);
        } else if archive_path.ends_with(".zip") {
            $crate::zip!(extract: archive_path);
        } else {
            $crate::error!("Unsupported archive format: {}", archive_path);
            std::process::exit(1);
        }
    }};
    ($archive:expr, to: $dest:expr) => {{
        let archive_path = $archive;
        if archive_path.ends_with(".tar.gz") || archive_path.ends_with(".tgz") || archive_path.ends_with(".tar") {
            $crate::tar!(extract: archive_path, to: $dest);
        } else if archive_path.ends_with(".zip") {
            $crate::zip!(extract: archive_path, to: $dest);
        } else {
            $crate::error!("Unsupported archive format: {}", archive_path);
            std::process::exit(1);
        }
    }};
}

// --- Path utility macros ---
#[macro_export]
macro_rules! path_canon {
    ($path:expr) => {{
        $crate::fs::path_canon($path).unwrap_or_default()
    }};
}

#[macro_export]
macro_rules! path_split {
    ($path:expr, into: $name:expr) => {{
        let parts = $crate::fs::path_split($path);
        for (k, v) in parts.into_iter() {
            $crate::global::set_var(&format!("{}_{}", $name, k), &v);
        }
    }};
}

// --- Metadata parsing macro ---
#[macro_export]
macro_rules! meta_keys {
    ($path:expr, into: $name:expr) => {{
        $crate::fs::parse_meta_keys($path, $name);
    }};
}

// --- Filesystem Validation Macros ---
#[macro_export]
macro_rules! require_file {
    ($path:expr) => {
        $crate::validate!($crate::fs::is_file($path), "File does not exist: {}", $path);
    };
}

#[macro_export]
macro_rules! require_dir {
    ($path:expr) => {
        $crate::validate!(
            $crate::fs::is_dir($path),
            "Directory does not exist: {}",
            $path
        );
    };
}

// --- Filesystem Iteration Macro ---
#[macro_export]
macro_rules! file_in {
    ($file_var:ident in $dir:expr => $body:block) => {
        if let Ok(entries) = std::fs::read_dir($crate::global::expand_vars($dir)) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(path_str) = entry.path().to_str() {
                        $crate::global::set_var(stringify!($file_var), path_str);
                        $body
                    }
                }
            }
        }
    };
    ($file_var:ident, $content_var:ident in $dir:expr => $body:block) => {
        if let Ok(entries) = std::fs::read_dir($crate::global::expand_vars($dir)) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(path_str) = entry.path().to_str() {
                        if entry.path().is_file() {
                            $crate::global::set_var(stringify!($file_var), path_str);
                            let content = $crate::fs::read_file(path_str);
                            $crate::global::set_var(stringify!($content_var), &content);
                            $body
                        }
                    }
                }
            }
        }
    };
}
