//! Interactive Prompt Functions - Core TTY-aware prompt implementations
//!
//! This module contains the actual prompt function implementations that handle
//! TTY detection, user input, and integration with global context flags.

use std::io::{self, Write};

use crate::global::{expand_vars, get_var, is_true};
use crate::utils::expand_colors_unified;

fn stdin_is_tty() -> bool {
    // Use libc directly to avoid adding new deps.
    unsafe { libc::isatty(libc::STDIN_FILENO) == 1 }
}

fn render_prompt(prefix: &str, message: &str, suffix: &str) -> String {
    // Minimal, color-friendly prefix using simple colors only
    // Examples: "{yellow}?{reset}", "{green}›{reset}"
    let s = format!("{} {}{} ", prefix, message, suffix);
    expand_colors_unified(&s)
}

/// Ask the user to confirm (yes/no). Returns `default` when quiet or non‑TTY.
pub fn confirm_default(message: &str, default: bool) -> bool {
    if is_true("opt_yes") {
        return true;
    }
    if is_true("opt_quiet") {
        return default;
    }
    if !stdin_is_tty() {
        return default;
    }

    let suffix = match default {
        true => " [Y/n]",
        false => " [y/N]",
    };
    let prompt = render_prompt("{yellow}?{reset}", message, suffix);

    loop {
        print!("{}", prompt);
        io::stdout().flush().ok();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return default;
        }

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            "" => return default,
            _ => continue,
        }
    }
}

/// Ask the user to confirm (yes/no) with no default (Enter required).
/// Falls back to `false` in quiet/non‑TTY scenarios.
pub fn confirm(message: &str) -> bool {
    if is_true("opt_yes") {
        return true;
    }
    if is_true("opt_quiet") {
        return false;
    }
    if !stdin_is_tty() {
        return false;
    }

    let prompt = render_prompt("{yellow}?{reset}", message, " [y/n]");
    loop {
        print!("{}", prompt);
        io::stdout().flush().ok();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return false;
        }
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => continue,
        }
    }
}

/// Ask the user for a line of input. Returns default/empty in quiet or non‑TTY.
pub fn ask(message: &str, default: Option<&str>) -> String {
    if is_true("opt_quiet") || !stdin_is_tty() {
        return default.unwrap_or("").to_string();
    }
    let suffix = default.map(|d| format!(" [{}]", d)).unwrap_or_default();
    let prompt = render_prompt("{green}›{reset}", message, &suffix);

    print!("{}", prompt);
    io::stdout().flush().ok();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return default.unwrap_or("").to_string();
    }
    let trimmed = input.trim();
    if trimmed.is_empty() {
        default.unwrap_or("").to_string()
    } else {
        trimmed.to_string()
    }
}

/// Present a simple selection list. Returns selected item or default/first.
pub fn select(message: &str, options: &[&str], default_index: Option<usize>) -> String {
    if options.is_empty() {
        return String::new();
    }

    let idx_default = default_index
        .unwrap_or(0)
        .min(options.len().saturating_sub(1));

    if is_true("opt_quiet") || !stdin_is_tty() {
        return options[idx_default].to_string();
    }

    // Show inline options for brevity: "[1] one  [2] two  [3] three"
    let opts_inline = options
        .iter()
        .enumerate()
        .map(|(i, o)| {
            if i == idx_default {
                format!("{{yellow}}[{}]{}{{reset}}", i + 1, o)
            } else {
                format!("[{}]{}", i + 1, o)
            }
        })
        .collect::<Vec<_>>()
        .join("  ");

    let suffix = format!("  {}", opts_inline);
    let prompt = render_prompt("{cyan}●{reset}", message, &suffix);

    loop {
        print!("{} ", prompt);
        io::stdout().flush().ok();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return options[idx_default].to_string();
        }
        let t = input.trim();
        if t.is_empty() {
            return options[idx_default].to_string();
        }
        if let Ok(n) = t.parse::<usize>() {
            if n >= 1 && n <= options.len() {
                return options[n - 1].to_string();
            }
        }
        // Accept exact text match too
        if let Some(pos) = options.iter().position(|o| o.eq_ignore_ascii_case(t)) {
            return options[pos].to_string();
        }
    }
}

/// Helper: fetch default for prompts from context by key, fallback to provided value.
pub fn default_from(key: &str, fallback: &str) -> String {
    let v = get_var(key);
    if v.is_empty() {
        fallback.to_string()
    } else {
        v
    }
}

/// Simple prompt for user input with optional default (from utils.rs migration)
pub fn prompt_user(message: &str, default: Option<&str>) -> String {
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

/// Simple yes/no confirmation with optional default (from utils.rs migration)
pub fn confirm_action(message: &str, default: Option<bool>) -> bool {
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
