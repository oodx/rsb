//! Curated low-level helpers for the global store (module utils per spec)

pub fn is_true(key: &str) -> bool { crate::global::get_var(key) == "1" }
pub fn is_false(key: &str) -> bool { crate::global::get_var(key) == "0" }

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
