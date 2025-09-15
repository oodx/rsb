//! String-related error types and logging helpers.

use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub enum StringError {
    // General pattern/regex issues
    InvalidPattern { pattern: String, reason: &'static str },
    RegexCompile { pattern: String },
    RegexReplace { pattern: String },

    // Guards
    SizeLimitExceeded { limit: usize, length: usize },
    IndexOutOfBounds { index: isize, len: usize },

    // Misc
    Utf8,
    Other(&'static str),
}

impl Display for StringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            StringError::SizeLimitExceeded { limit, length } => {
                write!(f, "Input {} bytes exceeds limit {} bytes", length, limit)
            }
            StringError::InvalidPattern { pattern, reason } => {
                write!(f, "Invalid pattern: '{}' ({})", pattern, reason)
            }
            StringError::RegexCompile { pattern } => {
                write!(f, "Regex compilation failed for pattern: '{}'", pattern)
            }
            StringError::RegexReplace { pattern } => {
                write!(f, "Regex replacement failed for pattern: '{}'", pattern)
            }
            StringError::IndexOutOfBounds { index, len } => {
                write!(f, "Index out of bounds: index={}, len={}", index, len)
            }
            StringError::Utf8 => write!(f, "Invalid UTF-8 sequence"),
            StringError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

/// Standardized fail-fast for string helpers (RS fail-immediately policy).
/// Until the trap system is finalized, any string helper failure is treated as fatal.
/// Prints a fatal message and exits the process with status 1.
pub fn log_string_error(op: &str, err: &StringError) -> ! {
    // Fail-fast without requiring visual feature flags
    crate::utils::stderrx("fatal", &format!("[string::{}] {}", op, err));
    std::process::exit(1);
}
