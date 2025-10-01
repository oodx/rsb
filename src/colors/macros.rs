//! Color macros for the colors feature.
//!
//! Provides the `colored!` macro for expanding color tags to ANSI sequences.

/// Expand color tags to ANSI sequences using the unified expander.
#[macro_export]
macro_rules! colored {
    ($s:expr) => {{
        let s = $s.to_string();
        $crate::utils::expand_colors_unified(&s)
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        let s = format!($fmt, $($arg)*);
        $crate::utils::expand_colors_unified(&s)
    }};
}
