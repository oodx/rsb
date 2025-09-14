//! Curated low-level string utilities.
//!
//! This module re-exports stable helper functions for consumers who prefer
//! an explicit `string::utils` namespace rather than pulling from the entire
//! `string` module surface.
//!
//! Note: Prefer using `rsb::string` for general usage. Use `string::utils`
//! when you want to target low-level helpers explicitly.

pub use super::helpers::*;
pub use super::case::*;
pub use super::error::*;

/// Optional static registry for debugging which helpers are ASCII-SAFE vs UNICODE-SAFE.
/// This is hand-maintained and informational only.
pub mod safety_registry {
    pub fn ascii_safe() -> &'static [&'static str] {
        &[
            "string::to_snake_case",
            "string::to_kebab_case",
            "string::to_dot_case",
            "string::to_space_case",
            "string::to_camel_case",
        ]
    }

    pub fn unicode_safe() -> &'static [&'static str] {
        &[
            "string::str_sub",
            "string::str_prefix",
            "string::str_suffix",
            "string::str_replace",
            "string::str_upper",
            "string::str_lower",
        ]
    }
}

// --- ASCII Filtering Utilities (ASCII-SAFE) ---

/// Remove all non-ASCII characters from the input string.
pub fn filter_ascii_strip(s: &str) -> String {
    s.chars().filter(|ch| ch.is_ascii()).collect()
}

/// Replace each non-ASCII character with the provided marker (e.g., "#INV#").
pub fn filter_ascii_sanitize(s: &str, marker: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if ch.is_ascii() {
            out.push(ch);
        } else {
            out.push_str(marker);
        }
    }
    out
}

/// Convenience: sanitize using the default invalid marker used in streaming tools.
pub const ASCII_INVALID_MARKER: &str = "#INV#";

pub fn filter_ascii_sanitize_default(s: &str) -> String {
    filter_ascii_sanitize(s, ASCII_INVALID_MARKER)
}

// --- Shell/URL helpers ---

/// Safely single-quote a string for POSIX shells.
/// Wraps the string in single quotes and escapes any embedded single quotes
/// using the standard `'
/// '"'"'` sequence.
pub fn shell_single_quote(s: &str) -> String {
    let escaped = s.replace("'", r"'\''");
    format!("'{}'", escaped)
}
