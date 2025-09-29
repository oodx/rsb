//! REPL-specific macros for argument access and dispatch
//!
//! Provides macros for accessing REPL command arguments from global storage:
//! - `repl_arg!($n)` - Get REPL argument by position (0-indexed)
//! - `repl_argc!()` - Get total REPL argument count
//! - `repl_args!()` - Get all REPL arguments as semicolon-separated string
//! - `repl_argv!()` - Get all REPL arguments as Vec<String>
//!
//! Also provides the main REPL dispatcher macro:
//! - `repl_dispatch!` - Interactive REPL loop with command routing

/// Get REPL argument by position (0-indexed)
///
/// # Example
/// ```rust,ignore
/// let cmd = repl_arg!(0);  // Get command
/// let arg1 = repl_arg!(1); // Get first argument
/// ```
#[macro_export]
macro_rules! repl_arg {
    ($n:expr) => {{
        $crate::global::get_var(&format!("repl_arg_{}", $n))
    }};
}

/// Get total count of REPL arguments
///
/// # Example
/// ```rust,ignore
/// let count = repl_argc!();
/// ```
#[macro_export]
macro_rules! repl_argc {
    () => {{
        let argc_str = $crate::global::get_var("repl_argc");
        if argc_str.is_empty() {
            0
        } else {
            argc_str.parse::<usize>().unwrap_or(0)
        }
    }};
}

/// Get all REPL arguments as semicolon-separated string
///
/// # Example
/// ```rust,ignore
/// let args = repl_args!(); // "cmd;arg1;arg2"
/// ```
#[macro_export]
macro_rules! repl_args {
    () => {{
        $crate::global::get_var("repl_args")
    }};
}

/// Get all REPL arguments as Vec<String>
///
/// # Example
/// ```rust,ignore
/// let argv = repl_argv!(); // vec!["cmd", "arg1", "arg2"]
/// ```
#[macro_export]
macro_rules! repl_argv {
    () => {{
        let args_str = $crate::global::get_var("repl_args");
        if args_str.is_empty() {
            Vec::new()
        } else {
            args_str
                .split(';')
                .map(String::from)
                .collect::<Vec<String>>()
        }
    }};
}