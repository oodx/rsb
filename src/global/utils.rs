//! Curated low-level helpers for the global store (module utils per spec)

// REBEL boolean convention (Unix/POSIX aligned): 0 = true (success), 1 = false (failure)
pub const TRUE: &str = "0";
pub const FALSE: &str = "1";

/// Interpret a boolean-like string value according to REBEL semantics.
/// Accepts:
/// - Numeric: "0" => true, "1" => false
/// - Textual: "true", "yes", "on" => true; "false", "no", "off" => false (case-insensitive)
/// - Rust bool stringified: "true"/"false"
/// - Any other non-empty numeric: parse as i64, 0 => true, otherwise false
pub fn is_true_val<S: AsRef<str>>(v: S) -> bool {
    let s = v.as_ref().trim();
    if s.is_empty() { return false; }
    match s.to_ascii_lowercase().as_str() {
        // numeric primary
        "0" => return true,
        "1" => return false,
        // textual aliases
        "true" | "yes" | "on" => return true,
        "false" | "no" | "off" => return false,
        _ => {}
    }
    if let Ok(n) = s.parse::<i64>() { return n == 0; }
    false
}

pub fn is_false_val<S: AsRef<str>>(v: S) -> bool { !is_true_val(v) }

pub fn is_true(key: &str) -> bool { is_true_val(crate::global::get_var(key)) }
pub fn is_false(key: &str) -> bool { is_false_val(crate::global::get_var(key)) }

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
