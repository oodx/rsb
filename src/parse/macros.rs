// --- Parse (string/stream) Macros (module-owned) ---

// String/stream sed-like helpers (content-based). File-based variants remain
// under fs::macros to keep responsibilities clear.

#[macro_export]
macro_rules! sed_lines {
    ($content:expr, $start:expr, $end:expr) => {
        $crate::streams::Stream::from_string($content)
            .sed_lines($start, $end)
            .to_string()
    };
}

#[macro_export]
macro_rules! sed_around {
    ($content:expr, $pattern:expr, $context:expr) => {
        $crate::streams::Stream::from_string($content)
            .sed_around($pattern, $context)
            .to_string()
    };
}

#[macro_export]
macro_rules! sed_insert {
    ($content:expr, $sentinel:expr, $source:expr) => {{
        match $crate::streams::Stream::from_string($source).sed_insert($content, $sentinel) {
            Ok(stream) => stream.to_string(),
            Err(e) => {
                $crate::error!("sed_insert failed: {}", e);
                std::process::exit(1);
            }
        }
    }};
}

#[macro_export]
macro_rules! sed_template {
    ($content:expr, $sentinel:expr, $source:expr) => {
        $crate::streams::Stream::from_string($source)
            .sed_template($content, $sentinel)
            .to_string()
    };
}

#[macro_export]
macro_rules! sed_replace {
    ($source:expr, $from:expr, $to:expr) => {
        $source.replace($from, $to)
    };
    ($source:expr, $from:expr, $to:expr, all) => {
        $source.replace($from, $to)
    };
}

// --- File read helper bounded by FIRST/LAST ---
#[macro_export]
macro_rules! sed_read {
    ($path:expr, FIRST, LAST) => {
        $crate::parse::sed_read($path, None, None)
    };
    ($path:expr, FIRST, $end:expr) => {
        $crate::parse::sed_read($path, None, Some($end as usize))
    };
    ($path:expr, $start:expr, LAST) => {
        $crate::parse::sed_read($path, Some($start as usize), None)
    };
    ($path:expr, $start:expr, $end:expr) => {
        $crate::parse::sed_read($path, Some($start as usize), Some($end as usize))
    };
}

// --- File-based Sed Macros (adapter; conceptually optional if fs is enabled) ---
#[macro_export]
macro_rules! sed_lines_file {
    ($path:expr, $start:expr, $end:expr) => {
        $crate::parse::sed_lines_file($path, $start, $end)
    };
}

#[macro_export]
macro_rules! sed_around_file {
    ($path:expr, $pattern:expr, $context:expr) => {
        $crate::parse::sed_around_file($path, $pattern, $context)
    };
}

#[macro_export]
macro_rules! sed_insert_file {
    ($path:expr, $content:expr, $sentinel:expr) => {{
        match $crate::parse::sed_insert_file($path, $content, $sentinel) {
            Ok(_) => {}
            Err(e) => {
                $crate::error!("sed_insert_file failed: {}", e);
                std::process::exit(1);
            }
        }
    }};
}

#[macro_export]
macro_rules! sed_template_file {
    ($path:expr, $content:expr, $sentinel:expr) => {
        $crate::parse::sed_template_file($path, $content, $sentinel)
    };
}

// WC-like counters moved to fs::macros
