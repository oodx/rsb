use crate::global::{expand_vars, has_var};

// This module will contain miscellaneous utilities, such as the
// StringExt trait, array operations, and user interaction functions.

// --- Output Helpers ---

pub fn should_print_level(level: &str) -> bool {
    if has_var("QUIET_MODE") && !["error", "fatal"].contains(&level) {
        return false;
    }

    match level {
        "trace" | "think" => has_var("TRACE_MODE"),
        "debug" => has_var("DEBUG_MODE") || has_var("TRACE_MODE"),
        "info" | "warn" | "okay" => {
            has_var("DEBUG_MODE") || has_var("DEV_MODE") || has_var("TRACE_MODE")
        }
        "error" | "fatal" => true,
        _ => true,
    }
}

/// Unified color expansion used by the colored! macro.
/// If the visual feature is enabled, delegate to the new registry-based expander
/// which supports extended tags (e.g., {bg:red}). Otherwise, return unchanged.
pub fn expand_colors_unified(text: &str) -> String {
    #[cfg(feature = "colors-core")]
    {
        return crate::colors::colored(text);
    }
    #[cfg(not(feature = "colors-core"))]
    {
        // Strip inline color tags like {bold}, {cyan}, {reset}, etc. when visuals are disabled.
        // This keeps output clean and readable in plain mode.
        let mut out = String::with_capacity(text.len());
        let mut chars = text.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '{' {
                // skip until matching '}' if it looks like a tag
                let mut buf = String::new();
                while let Some(&nc) = chars.peek() {
                    chars.next();
                    if nc == '}' {
                        break;
                    }
                    buf.push(nc);
                }
                // omit the tag (do not push '{buf}')
                continue;
            } else {
                out.push(c);
            }
        }
        return out;
    }
}

pub fn stderrx(level: &str, message: &str) {
    if !should_print_level(level) {
        return;
    }
    let glyph = {
        #[cfg(feature = "glyphs")]
        {
            if crate::visual::glyphs::glyphs_enabled() {
                // Map levels to explicit glyph names (non-emoji)
                let key = match level {
                    "info" => "info",
                    "okay" => "pass",
                    "warn" => "delta",
                    "error" | "fatal" => "fail",
                    "debug" => "bolt",
                    "trace" => "dots",
                    _ => "bullet",
                };
                let g = crate::visual::glyphs::glyph(key);
                if !g.is_empty() {
                    g.to_string()
                } else {
                    "•".to_string()
                }
            } else {
                "•".to_string()
            }
        }
        #[cfg(not(feature = "glyphs"))]
        {
            "•".to_string()
        }
    };

    // Decide color name for level via visual registry if available, otherwise fallback mapping
    let color_name = {
        #[cfg(feature = "colors-core")]
        {
            if !crate::colors::color(level).is_empty() {
                level
            } else {
                match level {
                    "info" => "cyan",
                    "okay" => "green",
                    "warn" => "yellow",
                    "error" | "fatal" => "red",
                    "debug" => "grey",
                    "trace" => "magenta",
                    _ => "reset",
                }
            }
        }
        #[cfg(not(feature = "colors-core"))]
        {
            match level {
                "info" => "cyan",
                "okay" => "green",
                "warn" => "yellow",
                "error" | "fatal" => "red",
                "debug" => "grey",
                "trace" => "magenta",
                _ => "reset",
            }
        }
    };

    // Construct a format string with placeholders
    let format_string = format!("{{{}}}{} {}", color_name, glyph, message);

    // Expand variables and then colors (mutexes are now released)
    let expanded_vars = expand_vars(&format_string);
    eprintln!("{}", expand_colors_unified(&expanded_vars));
}

// Note: Math comparison functions (num_eq, num_lt, num_gt) moved to math::comparison module
// Note: Array operations (array_push, array_get, etc.) moved to global::array module
// Note: User prompts (prompt_user, confirm_action) moved to visual::prompts::interactive module
// Note: is_name already available in crate::string module
// Note: is_true/is_false already available in crate::global module

// note: tests moved adjacent to specific modules
