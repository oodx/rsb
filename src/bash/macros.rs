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

// Archive macros are already defined at crate level in fs_data; optionally re-exported here
pub use crate::{pack, tar, tar_gz, unpack, zip};
