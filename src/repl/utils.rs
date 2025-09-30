//! Curated low-level helpers for REPL functionality
//!
//! Public utilities that users may explicitly opt into:
//! - Global storage for REPL arguments
//! - Argument extraction and formatting

use crate::cli::Args;
use crate::global::set_var;

/// Store REPL command arguments in global storage
///
/// Stores arguments with 0-indexed pattern:
/// - `repl_arg_0` = command
/// - `repl_arg_1` = first argument
/// - `repl_arg_N` = nth argument
/// - `repl_argc` = total argument count
/// - `repl_args` = semicolon-separated argument string
///
/// # Arguments
/// * `args` - Parsed command arguments (uses all() to get everything including command)
///
/// # Example
/// ```rust,ignore
/// let args = Args::from_line("status --verbose");
/// store_repl_args_global(&args);
/// // Global now has: repl_arg_0="status", repl_arg_1="--verbose", repl_argc="2"
/// ```
pub fn store_repl_args_global(args: &Args) {
    // For REPL, we want ALL arguments including the command (not just remaining)
    // Use all() which includes everything
    let all_args = args.all();
    let argc = all_args.len();

    // Store count
    set_var("repl_argc", &argc.to_string());

    // Store each argument (0-indexed)
    for (i, arg) in all_args.iter().enumerate() {
        set_var(&format!("repl_arg_{}", i), arg);
    }

    // Store all args as semicolon-separated string
    set_var("repl_args", &all_args.join(";"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::global::get_var;

    #[test]
    fn test_store_repl_args_global() {
        let args = Args::from_strs(&["status", "--verbose", "config"]);
        store_repl_args_global(&args);

        assert_eq!(get_var("repl_argc"), "3");
        assert_eq!(get_var("repl_arg_0"), "status");
        assert_eq!(get_var("repl_arg_1"), "--verbose");
        assert_eq!(get_var("repl_arg_2"), "config");
        assert_eq!(get_var("repl_args"), "status;--verbose;config");
    }
}