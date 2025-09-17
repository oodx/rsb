// --- Loop Macros ---
// Namespaced re-exports for selective imports
pub use crate::{
    case, export, file_in, for_in, load_config, require_command, require_dir, require_file,
    require_var, src, test, validate,
};
#[macro_export]
macro_rules! for_in {
    ($var:ident in $array_name:expr => $body:block) => {
        for item in $crate::utils::get_array($array_name) {
            $crate::global::set_var(stringify!($var), &item);
            $body
        }
    };
    ($index:ident, $var:ident in $array_name:expr => $body:block) => {
        for (i, item) in $crate::utils::get_array($array_name).iter().enumerate() {
            $crate::global::set_var(stringify!($index), &i.to_string());
            $crate::global::set_var(stringify!($var), item);
            $body
        }
    };
}

// --- Logic and Control Flow Macros ---
#[macro_export]
macro_rules! test {
    (-e $path:expr) => {
        $crate::fs::is_entity($path)
    };
    (-f $path:expr) => {
        $crate::fs::is_file($path)
    };
    (-d $path:expr) => {
        $crate::fs::is_dir($path)
    };
    (-L $path:expr) => {
        $crate::fs::is_link($path)
    };
    (-r $path:expr) => {
        $crate::fs::is_readable($path)
    };
    (-w $path:expr) => {
        $crate::fs::is_writable($path)
    };
    (-x $path:expr) => {
        $crate::fs::is_executable($path)
    };
    (-s $path:expr) => {
        $crate::fs::is_nonempty_file($path)
    };
    (-n $str:expr) => {
        !$str.is_empty()
    };
    (-z $str:expr) => {
        $str.is_empty()
    };
    ($a:expr, ==, $b:expr) => {
        $crate::string::str_equals($a, $b)
    };
    ($a:expr, !=, $b:expr) => {
        !$crate::string::str_equals($a, $b)
    };
    ($a:expr, =~, $b:expr) => {
        $crate::string::str_matches($a, $b)
    };
    ($a:expr, <, $b:expr) => {
        $a < $b
    };
    ($a:expr, >, $b:expr) => {
        $a > $b
    };
    ($a:expr, -eq, $b:expr) => {
        $crate::utils::num_eq($a, $b)
    };
    ($a:expr, -ne, $b:expr) => {
        !$crate::utils::num_eq($a, $b)
    };
    ($a:expr, -lt, $b:expr) => {
        $crate::utils::num_lt($a, $b)
    };
    ($a:expr, -le, $b:expr) => {
        $crate::utils::num_lt($a, $b) || $crate::utils::num_eq($a, $b)
    };
    ($a:expr, -gt, $b:expr) => {
        $crate::utils::num_gt($a, $b)
    };
    ($a:expr, -ge, $b:expr) => {
        $crate::utils::num_gt($a, $b) || $crate::utils::num_eq($a, $b)
    };
}

#[macro_export]
macro_rules! case {
    ($value:expr, { $($pattern:expr => $body:block),* $(, _ => $default:block)? }) => {
        {
            let val_to_match = $value;
            let mut matched = false;
            $(
                if !matched && $crate::string::str_matches(val_to_match, $pattern) {
                    matched = true;
                    $body
                }
            )*
            $(
                if !matched { $default }
            )?
        }
    };
}

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

// --- Config Macros ---
#[macro_export]
macro_rules! export {
    () => {
        $crate::global::export_vars(&$crate::global::get_var("RSB_EXPORT"));
    };
    ($path:expr) => {
        $crate::global::export_vars($path);
    };
}

#[macro_export]
macro_rules! src {
    ($($path:expr),+) => { $crate::load_config!($($path),+); };
}

#[macro_export]
macro_rules! load_config {
    ($($path:expr),+) => { $( $crate::global::load_config_file($path); )+ };
}

// --- Validation Macros ---
#[macro_export]
macro_rules! validate {
    ($condition:expr, $($arg:tt)*) => {
        if !$condition {
            $crate::error!("Validation failed: {}", format!($($arg)*));

            // Detect if running in test environment
            let is_test = std::env::var("CARGO_TEST").is_ok() || std::thread::current().name().map_or(false, |name| name.contains("test"));
            if !is_test {
                std::process::exit(1);
            } else {
                panic!("Validation failed: {}", format!($($arg)*));
            }
        }
    };
    ($condition:expr, exit_code: $code:expr, $($arg:tt)*) => {
        if !$condition {
            $crate::error!("Validation failed: {}", format!($($arg)*));

            // Detect if running in test environment
            let is_test = std::env::var("CARGO_TEST").is_ok() || std::thread::current().name().map_or(false, |name| name.contains("test"));
            if !is_test {
                std::process::exit($code);
            } else {
                panic!("Validation failed: {}", format!($($arg)*));
            }
        }
    };
}

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

#[macro_export]
macro_rules! require_command {
    ($cmd:expr) => {
        $crate::validate!($crate::os::is_command($cmd), "Command not found: {}", $cmd);
    };
}

#[macro_export]
macro_rules! require_var {
    ($var:expr) => {
        $crate::validate!(
            $crate::global::has_var($var),
            "Required variable not set: {}",
            $var
        );
    };
}

// --- Variable Management ---
// Intentionally no set_var!/get_var! macros.
// Policy: prefer function calls `rsb::global::{set_var, get_var}` to avoid macro leakage.
