// Rust-native boolean convention:
// - Booleans are true/false
// - Text representation in the global store uses "true"/"false"

// Primary boolean constants
pub const TRUE: bool = true;
pub const FALSE: bool = false;

// String forms (for global string store)
pub const TRUE_STR: &str = "true";
pub const FALSE_STR: &str = "false";

/// Interpret a boolean-like value according to Rust-native semantics.
/// Accepts textual true/false/yes/no/on/off (case-insensitive) and numeric 1/0
/// for compatibility (non-zero -> true, zero -> false).
#[inline]
pub fn is_true_val<S: AsRef<str>>(v: S) -> bool {
    let s = v.as_ref().trim();
    if s.is_empty() {
        return false;
    }
    match s.to_ascii_lowercase().as_str() {
        "true" | "yes" | "on" | "enabled" | "pass" | "success" => return true,
        "false" | "no" | "off" | "disabled" | "fail" | "error" => return false,
        "1" => return true,
        "0" => return false,
        _ => {}
    }
    if let Ok(n) = s.parse::<i64>() {
        return n != 0;
    }
    false
}

#[inline]
pub fn is_false_val<S: AsRef<str>>(v: S) -> bool {
    !is_true_val(v)
}

/// Read a key from Global and interpret as boolean (Rust-native semantics).
#[inline]
pub fn is_true(key: &str) -> bool {
    is_true_val(crate::global::get_var(key))
}

#[inline]
pub fn is_false(key: &str) -> bool {
    !is_true(key)
}

// Generic conversions to boolean
pub trait ToBool {
    fn to_bool(&self) -> bool;
}

#[deprecated(
    note = "Converting bool to bool is an identity operation. Did you really mean to cast bool to bool?"
)]
fn bool_to_bool_identity_warning() -> bool {
    true // This function exists only to trigger a deprecation warning
}

impl ToBool for bool {
    fn to_bool(&self) -> bool {
        // Trigger compile-time warning by calling the deprecated function (but don't use its value)
        let _ = bool_to_bool_identity_warning();
        *self
    }
}
impl ToBool for i32 {
    fn to_bool(&self) -> bool {
        *self != 0
    }
}
impl ToBool for &str {
    fn to_bool(&self) -> bool {
        is_true_val(*self)
    }
}
impl ToBool for String {
    fn to_bool(&self) -> bool {
        is_true_val(self.as_str())
    }
}

#[inline]
pub fn is_true_any<T: ToBool + ?Sized>(v: &T) -> bool {
    v.to_bool()
}
#[inline]
pub fn is_false_any<T: ToBool + ?Sized>(v: &T) -> bool {
    !v.to_bool()
}
