//! String generation constants
//!
//! Shared constants used by string generation functions.

/// Printable ASCII characters excluding whitespace
pub const PRINTABLE_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

/// Hexadecimal characters (lowercase)
pub const HEX_CHARS: &str = "0123456789abcdef";

/// Alphabetic characters (mixed case)
pub const ALPHA_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
