// REBEL boolean convention (Unix/POSIX aligned):
// - 0 = true (success)
// - 1 = false (failure)

// Primary numeric constants
pub const TRUE: i32 = 0;
pub const FALSE: i32 = 1;

// Additive string forms (for global string store)
pub const TRUE_STR: &str = "0";
pub const FALSE_STR: &str = "1";

#[inline]
pub fn bool_to_i32_rsb(b: bool) -> i32 { if b { TRUE } else { FALSE } }

#[inline]
pub fn i32_to_bool_rsb(code: i32) -> bool { code == TRUE }

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

#[inline]
pub fn is_false_val<S: AsRef<str>>(v: S) -> bool { !is_true_val(v) }

/// Read a key from Global and interpret as REBEL boolean.
#[inline]
pub fn is_true(key: &str) -> bool { is_true_val(crate::global::get_var(key)) }

#[inline]
pub fn is_false(key: &str) -> bool { !is_true(key) }

// Generic conversions
pub trait ToRSBBool {
    fn rsb_is_true(&self) -> bool;
}

impl ToRSBBool for bool {
    fn rsb_is_true(&self) -> bool { *self }
}
impl ToRSBBool for i32 { fn rsb_is_true(&self) -> bool { *self == TRUE } }
impl ToRSBBool for i64 { fn rsb_is_true(&self) -> bool { *self == (TRUE as i64) } }
impl ToRSBBool for isize { fn rsb_is_true(&self) -> bool { *self == (TRUE as isize) } }
impl ToRSBBool for usize { fn rsb_is_true(&self) -> bool { *self == (TRUE as usize) } }
impl ToRSBBool for &str { fn rsb_is_true(&self) -> bool { is_true_val(*self) } }
impl ToRSBBool for String { fn rsb_is_true(&self) -> bool { is_true_val(self.as_str()) } }

#[inline]
pub fn is_true_any<T: ToRSBBool + ?Sized>(v: &T) -> bool { v.rsb_is_true() }
#[inline]
pub fn is_false_any<T: ToRSBBool + ?Sized>(v: &T) -> bool { !v.rsb_is_true() }

