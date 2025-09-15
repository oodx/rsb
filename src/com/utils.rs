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
    if s.is_empty() { return false; }
    match s.to_ascii_lowercase().as_str() {
        "true" | "yes" | "on" => return true,
        "false" | "no" | "off" => return false,
        "1" => return true,
        "0" => return false,
        _ => {}
    }
    if let Ok(n) = s.parse::<i64>() { return n != 0; }
    false
}

#[inline]
pub fn is_false_val<S: AsRef<str>>(v: S) -> bool { !is_true_val(v) }

/// Read a key from Global and interpret as boolean (Rust-native semantics).
#[inline]
pub fn is_true(key: &str) -> bool { is_true_val(crate::global::get_var(key)) }

#[inline]
pub fn is_false(key: &str) -> bool { !is_true(key) }

// Generic conversions
pub trait ToRSBBool {
    fn rsb_is_true(&self) -> bool;
}

impl ToRSBBool for bool { fn rsb_is_true(&self) -> bool { *self } }
impl ToRSBBool for i32 { fn rsb_is_true(&self) -> bool { *self != 0 } }
impl ToRSBBool for i64 { fn rsb_is_true(&self) -> bool { *self != 0 } }
impl ToRSBBool for isize { fn rsb_is_true(&self) -> bool { *self != 0 } }
impl ToRSBBool for usize { fn rsb_is_true(&self) -> bool { *self != 0 } }
impl ToRSBBool for &str { fn rsb_is_true(&self) -> bool { is_true_val(*self) } }
impl ToRSBBool for String { fn rsb_is_true(&self) -> bool { is_true_val(self.as_str()) } }

#[inline]
pub fn is_true_any<T: ToRSBBool + ?Sized>(v: &T) -> bool { v.rsb_is_true() }
#[inline]
pub fn is_false_any<T: ToRSBBool + ?Sized>(v: &T) -> bool { !v.rsb_is_true() }

// Exit code modeling and bridges
use std::process::ExitCode;

/// Canonical RSB exit kinds mapped onto process exit codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCodeKind {
    Success,           // 0
    Failure,           // 1
    AnnotatedFailure,  // 2
}

impl ExitCodeKind {
    #[inline]
    pub fn code(self) -> u8 {
        match self {
            ExitCodeKind::Success => 0,
            ExitCodeKind::Failure => 1,
            ExitCodeKind::AnnotatedFailure => 2,
        }
    }

    #[inline]
    pub fn as_exit(self) -> ExitCode { ExitCode::from(self.code()) }
}

/// Trait that converts values into process `ExitCode`.
pub trait AsExit { fn as_exit(self) -> ExitCode; }

impl AsExit for bool {
    #[inline]
    fn as_exit(self) -> ExitCode { if self { ExitCode::SUCCESS } else { ExitCode::from(1) } }
}

impl AsExit for ExitCodeKind {
    #[inline]
    fn as_exit(self) -> ExitCode { self.as_exit() }
}

impl AsExit for i32 { #[inline] fn as_exit(self) -> ExitCode { ExitCode::from((self as i16).clamp(0, u8::MAX as i16) as u8) } }
impl AsExit for u8 { #[inline] fn as_exit(self) -> ExitCode { ExitCode::from(self) } }

// Helpers to classify exit codes from integer values
#[inline]
pub fn is_success(code: i32) -> bool { code == 0 }
#[inline]
pub fn is_fail(code: i32) -> bool { code == 1 }
#[inline]
pub fn is_other_fail(code: i32) -> bool { code != 0 && code != 1 }
