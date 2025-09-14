// --- Conversion / Text Macros ---
// Contains numeric/text conversions not covered by the dedicated strings package.
#[macro_export]
macro_rules! to_number {
    ($text:expr) => {
        {
            $text.trim().parse::<i32>().unwrap_or(0)
        }
    };
    ($text:expr, default: $default:expr) => {
        {
            $text.trim().parse::<i32>().unwrap_or($default)
        }
    };
}

// param! macro moved to src/param/macros.rs

// String macros moved to src/string/macros.rs (module-owned)
