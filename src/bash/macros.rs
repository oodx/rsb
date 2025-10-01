//! Bash-related macros re-export (curl, tar, zip, pack)
//!
//! This module re-exports existing crate-level macros that are bash-oriented
//! so they can be discovered via `rsb::bash::macros::*`.

// Recreate curl!/get! in bash namespace, crate-level via #[macro_export]

#[macro_export]
macro_rules! curl {
    ($url:expr) => {{
        match $crate::bash::curl_get($url) {
            result if result.status == 0 => result.output,
            result => {
                $crate::error!("curl failed: {}", result.error);
                std::process::exit(result.status);
            }
        }
    }};
    ($url:expr, options: $opts:expr) => {{
        match $crate::bash::curl_get_with_options($url, $opts) {
            result if result.status == 0 => result.output,
            result => {
                $crate::error!("curl failed: {}", result.error);
                std::process::exit(result.status);
            }
        }
    }};
    (post: $url:expr, data: $data:expr) => {{
        match $crate::bash::curl_post($url, $data) {
            result if result.status == 0 => result.output,
            result => {
                $crate::error!("curl POST failed: {}", result.error);
                std::process::exit(result.status);
            }
        }
    }};
}

#[macro_export]
macro_rules! get {
    ($url:expr) => { $crate::curl!($url) };
    ($url:expr, options: $opts:expr) => { $crate::curl!($url, options: $opts) };
}

// === Bash-style Control Flow Macros ===

/// Bash-style test conditional ([ test ] equivalent)
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
        $crate::math::comparison::num_eq($a, $b)
    };
    ($a:expr, -ne, $b:expr) => {
        !$crate::math::comparison::num_eq($a, $b)
    };
    ($a:expr, -lt, $b:expr) => {
        $crate::math::comparison::num_lt($a, $b)
    };
    ($a:expr, -le, $b:expr) => {
        $crate::math::comparison::num_lt($a, $b) || $crate::math::comparison::num_eq($a, $b)
    };
    ($a:expr, -gt, $b:expr) => {
        $crate::math::comparison::num_gt($a, $b)
    };
    ($a:expr, -ge, $b:expr) => {
        $crate::math::comparison::num_gt($a, $b) || $crate::math::comparison::num_eq($a, $b)
    };
}

/// Bash-style case statement (pattern matching with regex)
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

/// Bash-style for loop over arrays
#[macro_export]
macro_rules! for_in {
    ($var:ident in $array_name:expr => $body:block) => {
        for item in $crate::global::array::get_array($array_name) {
            $crate::global::set_var(stringify!($var), &item);
            $body
        }
    };
    ($index:ident, $var:ident in $array_name:expr => $body:block) => {
        for (i, item) in $crate::global::array::get_array($array_name).iter().enumerate() {
            $crate::global::set_var(stringify!($index), &i.to_string());
            $crate::global::set_var(stringify!($var), item);
            $body
        }
    };
}

// Archive macros are already defined at crate level in fs_data; optionally re-exported here
pub use crate::{pack, tar, tar_gz, unpack, zip};
