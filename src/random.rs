use rand::Rng;

// ============================================================================
// BACKWARD COMPATIBILITY RE-EXPORTS
// ============================================================================
// These functions have been moved to the gx package but are re-exported here
// for backward compatibility. Consider using rsb::gx directly in new code.

/// Generates a random alphanumeric string of a given length.
///
/// **Deprecated**: This function has moved to `rsb::gx::string::get_rand_alnum`.
/// This re-export is provided for backward compatibility.
#[deprecated(since = "0.2.19", note = "Use rsb::gx::string::get_rand_alnum instead")]
pub fn get_rand_alnum(n: usize) -> String {
    crate::gx::string::get_rand_alnum(n)
}

/// Generates a random alphabetic string of a given length.
///
/// **Deprecated**: This function has moved to `rsb::gx::string::get_rand_alpha`.
/// This re-export is provided for backward compatibility.
#[deprecated(since = "0.2.19", note = "Use rsb::gx::string::get_rand_alpha instead")]
pub fn get_rand_alpha(n: usize) -> String {
    crate::gx::string::get_rand_alpha(n)
}

/// Generates a random hexadecimal string of a given length.
///
/// **Deprecated**: This function has moved to `rsb::gx::string::get_rand_hex`.
/// This re-export is provided for backward compatibility.
#[deprecated(since = "0.2.19", note = "Use rsb::gx::string::get_rand_hex instead")]
pub fn get_rand_hex(n: usize) -> String {
    crate::gx::string::get_rand_hex(n)
}

/// Generates a random string of printable, non-whitespace characters of a given length.
///
/// **Deprecated**: This function has moved to `rsb::gx::string::get_rand_string`.
/// This re-export is provided for backward compatibility.
#[deprecated(
    since = "0.2.19",
    note = "Use rsb::gx::string::get_rand_string instead"
)]
pub fn get_rand_string(n: usize) -> String {
    crate::gx::string::get_rand_string(n)
}

/// Generates a new v4 UUID.
///
/// **Deprecated**: This function has moved to `rsb::gx::id::get_rand_uuid`.
/// This re-export is provided for backward compatibility.
#[deprecated(since = "0.2.19", note = "Use rsb::gx::id::get_rand_uuid instead")]
pub fn get_rand_uuid() -> String {
    crate::gx::id::get_rand_uuid()
}

/// Selects a random word from a slice of strings.
///
/// **Deprecated**: This function has moved to `rsb::gx::collection::get_rand_from_slice`.
/// This re-export is provided for backward compatibility.
#[deprecated(
    since = "0.2.19",
    note = "Use rsb::gx::collection::get_rand_from_slice instead"
)]
pub fn get_rand_from_slice(words: &[String]) -> Option<String> {
    crate::gx::collection::get_rand_from_slice(words)
}

/// Returns a random integer in [min, max] inclusive as usize.
pub fn rand_range_usize(min: usize, max: usize) -> usize {
    let mut rng = rand::rng();
    if min >= max {
        return min;
    }
    rng.random_range(min..=max)
}

// TODO: support jynx/boxy stderr colors
