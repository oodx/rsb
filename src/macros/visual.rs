// --- Visual Macros ---
// These macros depend on the visual system (colors/glyphs) and are optional.

#[macro_export]
macro_rules! colored {
    // Single argument form: avoid format! so {color} tags are not treated as fmt placeholders
    ($s:expr) => {{
        let s = $s.to_string();
        $crate::utils::expand_colors_unified(&s)
    }};
    // Variadic format form: supports formatting with additional args
    ($fmt:expr, $($arg:tt)*) => {{
        let s = format!($fmt, $($arg)*);
        $crate::utils::expand_colors_unified(&s)
    }};
}

#[macro_export]
macro_rules! info { ($($arg:tt)*) => { $crate::utils::stderrx("info", &format!($($arg)*)); }; }
#[macro_export]
macro_rules! okay { ($($arg:tt)*) => { $crate::utils::stderrx("okay", &format!($($arg)*)); }; }
#[macro_export]
macro_rules! warn { ($($arg:tt)*) => { $crate::utils::stderrx("warn", &format!($($arg)*)); }; }
#[macro_export]
macro_rules! error { ($($arg:tt)*) => { $crate::utils::stderrx("error", &format!($($arg)*)); }; }
#[macro_export]
macro_rules! fatal { ($($arg:tt)*) => { $crate::utils::stderrx("fatal", &format!($($arg)*)); }; }
#[macro_export]
macro_rules! debug { ($($arg:tt)*) => { $crate::utils::stderrx("debug", &format!($($arg)*)); }; }
#[macro_export]
macro_rules! trace { ($($arg:tt)*) => { $crate::utils::stderrx("trace", &format!($($arg)*)); }; }

// --- Prompt Macros (feature: prompts) ---
// Thin macros that delegate to visual::prompts functions

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! confirm {
    ($msg:expr) => { $crate::visual::prompts::confirm($msg) };
}

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! confirm_default {
    ($msg:expr, $default:expr) => { $crate::visual::prompts::confirm_default($msg, $default) };
}

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! ask {
    ($msg:expr) => { $crate::visual::prompts::ask($msg, None) };
    ($msg:expr, $default:expr) => { $crate::visual::prompts::ask($msg, Some($default)) };
}

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! select {
    ($msg:expr, $options:expr) => { $crate::visual::prompts::select($msg, $options, None) };
    ($msg:expr, $options:expr, $default_idx:expr) => { $crate::visual::prompts::select($msg, $options, Some($default_idx)) };
}

// --- General Prompt Macros ---
// Multi-purpose prompt! macro for different prompt types

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! prompt {
    // prompt!("confirm", "Continue?") -> bool
    ("confirm", $msg:expr) => { $crate::visual::prompts::confirm($msg) };

    // prompt!("ask", "Name?") -> String
    ("ask", $msg:expr) => { $crate::visual::prompts::ask($msg, None) };

    // prompt!("ask", "Name?", "default") -> String
    ("ask", $msg:expr, $default:expr) => { $crate::visual::prompts::ask($msg, Some($default)) };

    // prompt!("select", "Pick", &["a", "b"]) -> String
    ("select", $msg:expr, $options:expr) => { $crate::visual::prompts::select($msg, $options, None) };

    // prompt!("select", "Pick", &["a", "b"], 1) -> String
    ("select", $msg:expr, $options:expr, $default_idx:expr) => { $crate::visual::prompts::select($msg, $options, Some($default_idx)) };
}

// --- Timeout-Enhanced Prompt Macros ---
// Enhanced versions with timeout support and global context integration

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! confirm_timeout {
    // confirm_timeout!("Continue?") -> bool (uses context timeout, defaults to false on timeout)
    ($msg:expr) => { $crate::visual::utils::confirm_with_timeout($msg, None, false) };

    // confirm_timeout!("Continue?", 10) -> bool (10 second timeout)
    ($msg:expr, $timeout:expr) => { $crate::visual::utils::confirm_with_timeout($msg, Some($timeout), false) };

    // confirm_timeout!("Continue?", 10, true) -> bool (10 second timeout, true on timeout)
    ($msg:expr, $timeout:expr, $default:expr) => { $crate::visual::utils::confirm_with_timeout($msg, Some($timeout), $default) };
}

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! ask_timeout {
    // ask_timeout!("Name?") -> String (uses context timeout, empty string on timeout)
    ($msg:expr) => { $crate::visual::utils::ask_with_timeout($msg, None, None) };

    // ask_timeout!("Name?", "default") -> String (with default value)
    ($msg:expr, $default:expr) => { $crate::visual::utils::ask_with_timeout($msg, Some($default), None) };

    // ask_timeout!("Name?", "default", 15) -> String (15 second timeout)
    ($msg:expr, $default:expr, $timeout:expr) => { $crate::visual::utils::ask_with_timeout($msg, Some($default), Some($timeout)) };
}

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! select_timeout {
    // select_timeout!("Pick", &["a", "b"]) -> String (uses context timeout, first option on timeout)
    ($msg:expr, $options:expr) => { $crate::visual::utils::select_with_timeout($msg, $options, None, None) };

    // select_timeout!("Pick", &["a", "b"], 1) -> String (default index 1)
    ($msg:expr, $options:expr, $default_idx:expr) => { $crate::visual::utils::select_with_timeout($msg, $options, Some($default_idx), None) };

    // select_timeout!("Pick", &["a", "b"], 1, 20) -> String (20 second timeout)
    ($msg:expr, $options:expr, $default_idx:expr, $timeout:expr) => { $crate::visual::utils::select_with_timeout($msg, $options, Some($default_idx), Some($timeout)) };
}

// --- Enhanced General Prompt with Timeout ---

#[cfg(feature = "prompts")]
#[macro_export]
macro_rules! prompt_timeout {
    // prompt_timeout!("confirm", "Continue?") -> bool
    ("confirm", $msg:expr) => { $crate::visual::utils::confirm_with_timeout($msg, None, false) };

    // prompt_timeout!("confirm", "Continue?", 10) -> bool
    ("confirm", $msg:expr, $timeout:expr) => { $crate::visual::utils::confirm_with_timeout($msg, Some($timeout), false) };

    // prompt_timeout!("ask", "Name?") -> String
    ("ask", $msg:expr) => { $crate::visual::utils::ask_with_timeout($msg, None, None) };

    // prompt_timeout!("ask", "Name?", "default") -> String
    ("ask", $msg:expr, $default:expr) => { $crate::visual::utils::ask_with_timeout($msg, Some($default), None) };

    // prompt_timeout!("ask", "Name?", "default", 10) -> String
    ("ask", $msg:expr, $default:expr, $timeout:expr) => { $crate::visual::utils::ask_with_timeout($msg, Some($default), Some($timeout)) };

    // prompt_timeout!("select", "Pick", &["a", "b"]) -> String
    ("select", $msg:expr, $options:expr) => { $crate::visual::utils::select_with_timeout($msg, $options, None, None) };

    // prompt_timeout!("select", "Pick", &["a", "b"], 1, 15) -> String
    ("select", $msg:expr, $options:expr, $default_idx:expr, $timeout:expr) => { $crate::visual::utils::select_with_timeout($msg, $options, Some($default_idx), Some($timeout)) };
}
