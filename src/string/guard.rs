//! Guard helpers for string operations (size and index checks).

use super::error::StringError;

/// Ensure input size is within limit.
pub fn guard_size(s: &str, limit: usize) -> Result<(), StringError> {
    let length = s.as_bytes().len();
    if length > limit {
        Err(StringError::SizeLimitExceeded { limit, length })
    } else {
        Ok(())
    }
}

/// Normalize an index (supports negative) against a given length and ensure bounds.
/// Returns the normalized non-negative index.
pub fn guard_index(len: usize, idx: isize) -> Result<usize, StringError> {
    let n = len as isize;
    let i = if idx < 0 { n + idx } else { idx };
    if i < 0 || i > n {
        return Err(StringError::IndexOutOfBounds { index: idx, len });
    }
    Ok(i as usize)
}
