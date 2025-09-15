//! Curated low-level helpers for the global store (module utils per spec)

// String forms (for global store convenience)
pub use crate::com::{TRUE_STR, FALSE_STR};

/// Interpret a boolean-like string value according to REBEL semantics.
/// Accepts:
/// - Numeric: "0" => true, "1" => false
/// - Textual: "true", "yes", "on" => true; "false", "no", "off" => false (case-insensitive)
/// - Rust bool stringified: "true"/"false"
/// - Any other non-empty numeric: parse as i64, 0 => true, otherwise false
pub use crate::com::{is_true_val, is_false_val, is_true, is_false};

pub fn is_token_stream(value: &str) -> bool {
    if value.is_empty() { return false; }
    let has_comma = value.contains(',');
    let has_semicolon = value.contains(';');
    if !has_comma && !has_semicolon {
        return value.contains('=') && !value.starts_with('=') && !value.ends_with('=');
    }
    let delimiter = if has_comma { ',' } else { ';' };
    value.split(delimiter).all(|pair| {
        let t = pair.trim();
        t.contains('=') && !t.starts_with('=') && !t.ends_with('=')
    })
}
