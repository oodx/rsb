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

#[deprecated(since = "0.2.19", note = "use utils::stderrx instead")]
pub fn glyph_stderr(level: &str, message: &str) {
    stderrx(level, message)
}

// --- String & Name Helpers ---
// Moved to crate::string::{is_name, str_equals, str_matches}
pub use crate::string::is_name;
fn to_f64(s: &str) -> Option<f64> {
    s.parse::<f64>().ok()
}
pub fn num_eq(a: &str, b: &str) -> bool {
    match (to_f64(a), to_f64(b)) {
        (Some(na), Some(nb)) => (na - nb).abs() < f64::EPSILON,
        _ => false,
    }
}

pub fn num_lt(a: &str, b: &str) -> bool {
    match (to_f64(a), to_f64(b)) {
        (Some(na), Some(nb)) => na < nb,
        _ => false,
    }
}

pub fn num_gt(a: &str, b: &str) -> bool {
    match (to_f64(a), to_f64(b)) {
        (Some(na), Some(nb)) => na > nb,
        _ => false,
    }
}

// --- Array Helpers ---

pub fn array_push(key: &str, item: &str) {
    let mut items = get_array(key);
    items.push(item.to_string());
    let item_strs: Vec<&str> = items.iter().map(|s| s.as_str()).collect();
    set_array(key, &item_strs);
}

pub fn array_get(key: &str, index: usize) -> String {
    get_array(key).get(index).cloned().unwrap_or_default()
}

pub fn array_length(key: &str) -> usize {
    get_array(key).len()
}

pub fn array_contains(key: &str, item: &str) -> bool {
    get_array(key).contains(&item.to_string())
}

// The spec also calls for `get_array`. I will implement that here too.
pub fn get_array(key: &str) -> Vec<String> {
    use crate::global::get_var;
    let length_key = format!("{}_LENGTH", key);
    if !crate::global::has_var(&length_key) {
        let value = get_var(key);
        if value.is_empty() {
            return Vec::new();
        }
        return value.split_whitespace().map(|s| s.to_string()).collect();
    }
    let length: usize = get_var(&length_key).parse().unwrap_or(0);
    let mut items = Vec::new();
    for i in 0..length {
        let item_key = format!("{}_{}", key, i);
        if crate::global::has_var(&item_key) {
            items.push(get_var(&item_key));
        }
    }
    items
}

pub fn set_array(key: &str, items: &[&str]) {
    use crate::global::set_var;
    set_var(&format!("{}_LENGTH", key), &items.len().to_string());
    for (i, item) in items.iter().enumerate() {
        set_var(&format!("{}_{}", key, i), *item);
    }
    set_var(key, &items.join(" "));
}

// --- User Interaction ---

pub fn prompt_user(message: &str, default: Option<&str>) -> String {
    use std::io::{self, Write};

    let default_text = if let Some(def) = default {
        format!(" [{}]", def)
    } else {
        String::new()
    };

    print!("{}{}: ", expand_vars(message), default_text);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let trimmed = input.trim();
    if trimmed.is_empty() && default.is_some() {
        default.unwrap().to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn confirm_action(message: &str, default: Option<bool>) -> bool {
    use crate::global::is_true;
    use std::io::{self, Write};

    if is_true("opt_yes") {
        return true;
    }

    let default_text = match default {
        Some(true) => " [Y/n]",
        Some(false) => " [y/N]",
        None => " [y/n]",
    };

    loop {
        print!("{}{}: ", expand_vars(message), default_text);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            "" => {
                if let Some(def) = default {
                    return def;
                }
            }
            _ => continue,
        }
    }
}

// --- Boolean helpers (temporary location) ---
// These mirror context integer-boolean semantics and will move into the
// dedicated logic (lx) package later.
pub fn is_true(key: &str) -> bool {
    crate::global::is_true(key)
}
pub fn is_false(key: &str) -> bool {
    crate::global::is_false(key)
}

// note: tests moved adjacent to specific modules
