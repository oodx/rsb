//! Common RSB constants and helpers (COM)
//! Centralizes REBEL boolean semantics and conversions.

// REBEL boolean convention (Unix/POSIX aligned):
// - 0 = true (success)
// - 1 = false (failure)

pub const TRUE_I32: i32 = 0;
pub const FALSE_I32: i32 = 1;

pub const TRUE_STR: &str = "0";
pub const FALSE_STR: &str = "1";

#[inline]
pub fn bool_to_i32_rsb(b: bool) -> i32 { if b { TRUE_I32 } else { FALSE_I32 } }

#[inline]
pub fn i32_to_bool_rsb(code: i32) -> bool { code == TRUE_I32 }

/// Interpret a boolean-like value according to REBEL semantics.
/// Accepts numeric 0/1, textual true/false/yes/no/on/off (case-insensitive), or parses other integers.
#[inline]
pub fn is_true_val<S: AsRef<str>>(v: S) -> bool {
    let s = v.as_ref().trim();
    if s.is_empty() { return false; }
    match s.to_ascii_lowercase().as_str() {
        "0" => return true,
        "1" => return false,
        "true" | "yes" | "on" => return true,
        "false" | "no" | "off" => return false,
        _ => {}
    }
    if let Ok(n) = s.parse::<i64>() { return n == 0; }
    false
}

