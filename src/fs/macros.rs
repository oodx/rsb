// --- File System & Temp Macros (module-owned) ---

#[macro_export]
macro_rules! chmod {
    ($path:expr, $mode:expr) => { $crate::fs::chmod($path, $mode).ok() };
}

#[macro_export]
macro_rules! backup {
    ($path:expr, $suffix:expr) => { $crate::fs::backup_file($path, $suffix).ok() };
}

#[macro_export]
macro_rules! tmp {
    () => { $crate::fs::create_temp_file_path("random") };
    ($type:ident) => { $crate::fs::create_temp_file_path(stringify!($type)) };
}

#[macro_export]
macro_rules! cap_stream { ($stream:expr) => { $crate::fs::capture_stream_to_temp_file(&mut $stream) }; }
#[macro_export]
macro_rules! subst { ($stream:expr) => { $crate::cap_stream!($stream) }; }

// --- Dictionary Macros ---
#[macro_export]
macro_rules! dict {
    ($path:expr) => { $crate::fs::load_dict_from_file($path) };
}

// --- Advanced Sed Macros ---
#[macro_export]
macro_rules! sed_lines { ($content:expr, $start:expr, $end:expr) => { $crate::streams::Stream::from_string($content).sed_lines($start, $end).to_string() }; }
#[macro_export]
macro_rules! sed_around { ($content:expr, $pattern:expr, $context:expr) => { $crate::streams::Stream::from_string($content).sed_around($pattern, $context).to_string() }; }

#[macro_export]
macro_rules! sed_insert {
    ($content:expr, $sentinel:expr, $source:expr) => {{
        match $crate::streams::Stream::from_string($source).sed_insert($content, $sentinel) {
            Ok(stream) => stream.to_string(),
            Err(e) => { $crate::error!("sed_insert failed: {}", e); std::process::exit(1); }
        }
    }};
}

#[macro_export]
macro_rules! sed_template { ($content:expr, $sentinel:expr, $source:expr) => { $crate::streams::Stream::from_string($source).sed_template($content, $sentinel).to_string() }; }

#[macro_export]
macro_rules! sed_replace {
    ($source:expr, $from:expr, $to:expr) => { $source.replace($from, $to) };
    ($source:expr, $from:expr, $to:expr, all) => { $source.replace($from, $to) };
}

// --- File-based Sed Macros ---
#[macro_export]
macro_rules! sed_lines_file { ($path:expr, $start:expr, $end:expr) => { $crate::fs::sed_lines_file($path, $start, $end) }; }
#[macro_export]
macro_rules! sed_around_file { ($path:expr, $pattern:expr, $context:expr) => { $crate::fs::sed_around_file($path, $pattern, $context) }; }

#[macro_export]
macro_rules! sed_insert_file {
    ($path:expr, $content:expr, $sentinel:expr) => {{
        match $crate::fs::sed_insert_file($path, $content, $sentinel) {
            Ok(_) => {},
            Err(e) => { $crate::error!("sed_insert_file failed: {}", e); std::process::exit(1); }
        }
    }};
}

#[macro_export]
macro_rules! sed_template_file { ($path:expr, $content:expr, $sentinel:expr) => { $crate::fs::sed_template_file($path, $content, $sentinel) }; }

// --- Archive Macros ---
#[macro_export]
macro_rules! tar {
    (create: $archive:expr, $($path:expr),+) => {{
        let paths = vec![$($path),+];
        match $crate::os::create_tar($archive, &paths) {
            result if result.status == 0 => { $crate::okay!("Created tar archive: {}", $archive); },
            result => { $crate::error!("Failed to create tar: {}", result.error); std::process::exit(result.status); }
        }
    }};
    (extract: $archive:expr) => {{
        match $crate::os::extract_tar($archive, None) {
            result if result.status == 0 => { $crate::okay!("Extracted tar archive: {}", $archive); },
            result => { $crate::error!("Failed to extract tar: {}", result.error); std::process::exit(result.status); }
        }
    }};
    (extract: $archive:expr, to: $dest:expr) => {{
        match $crate::os::extract_tar($archive, Some($dest)) {
            result if result.status == 0 => { $crate::okay!("Extracted tar archive to: {}", $dest); },
            result => { $crate::error!("Failed to extract tar: {}", result.error); std::process::exit(result.status); }
        }
    }};
    (list: $archive:expr) => {{
        match $crate::os::list_tar($archive) {
            result if result.status == 0 => result.output,
            result => { $crate::error!("Failed to list tar: {}", result.error); std::process::exit(result.status); }
        }
    }};
}

#[macro_export]
macro_rules! tar_gz {
    (create: $archive:expr, $($path:expr),+) => {{
        let paths = vec![$($path),+];
        match $crate::os::create_tar_gz($archive, &paths) {
            result if result.status == 0 => { $crate::okay!("Created tar.gz archive: {}", $archive); },
            result => { $crate::error!("Failed to create tar.gz: {}", result.error); std::process::exit(result.status); }
        }
    }};
}

#[macro_export]
macro_rules! zip {
    (create: $archive:expr, $($path:expr),+) => {{
        let paths = vec![$($path),+];
        match $crate::os::create_zip($archive, &paths) {
            result if result.status == 0 => { $crate::okay!("Created zip archive: {}", $archive); },
            result => { $crate::error!("Failed to create zip: {}", result.error); std::process::exit(result.status); }
        }
    }};
    (extract: $archive:expr) => {{
        match $crate::os::extract_zip($archive, None) {
            result if result.status == 0 => { $crate::okay!("Extracted zip archive: {}", $archive); },
            result => { $crate::error!("Failed to extract zip: {}", result.error); std::process::exit(result.status); }
        }
    }};
    (extract: $archive:expr, to: $dest:expr) => {{
        match $crate::os::extract_zip($archive, Some($dest)) {
            result if result.status == 0 => { $crate::okay!("Extracted zip archive to: {}", $dest); },
            result => { $crate::error!("Failed to extract zip: {}", result.error); std::process::exit(result.status); }
        }
    }};
    (list: $archive:expr) => {{
        match $crate::os::list_zip($archive) {
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
macro_rules! path_canon { ($path:expr) => {{ $crate::fs::path_canon($path).unwrap_or_default() }}; }

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
macro_rules! meta_keys { ($path:expr, into: $name:expr) => {{ $crate::fs::parse_meta_keys($path, $name); }}; }
