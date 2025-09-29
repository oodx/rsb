//! Flag command handling - pre-dispatch flag processing
//!
//! Provides handlers for standard flags like --help and --version that execute
//! before normal command dispatch. Follows RSB idiom where flags are top-level
//! with smart re-routing for topic-specific help.

use crate::cli::Args;
use crate::global;

/// Handle --help / -h flag with topic-aware re-routing
///
/// Behavior:
/// - `prog --help` → generic help (calls cmd_help or show_help)
/// - `prog <topic> --help` → routes to help dispatcher if exists: `prog help <topic>`
/// - Falls back to generic help if help dispatcher or topic doesn't exist
fn rsb_handle_help_flag(args: &Args) -> Option<i32> {
    // Check if --help or -h is present
    if !args.has("--help") && !args.has("-h") {
        return None;
    }

    let first_arg = args.get(1);

    // Case 1: prog --help (flag is first argument, no topic)
    if first_arg == "--help" || first_arg == "-h" {
        // Try cmd_help handler
        if global::has_var("fn_cmd_help") {
            // Handler exists, construct minimal args
            let _help_args = Args::new(&[args.get(0)]);
            // Call via global registry would require function pointer storage
            // For now, fall through to show_help
            // TODO: Store function pointers in registry for dynamic dispatch
        }
        global::show_help();
        return Some(0);
    }

    // Case 2: prog <topic> --help (topic before flag)
    if !first_arg.is_empty() && !first_arg.starts_with('-') {
        // Check if help command is registered
        let functions = global::list_functions();
        let help_exists = functions.iter().any(|(name, _)| name == "help");

        if help_exists {
            // Help command exists - suggest using it for topic-specific help
            // Note: We can't call it dynamically here (would need help router system)
            // For now, show hint and fall through to generic help
            eprintln!("Tip: For detailed help on '{}', try: {} help {}",
                     first_arg, args.get(0), first_arg);
        }
    }

    // Fallback: generic help
    global::show_help();
    Some(0)
}

/// Handle --version / -v flag with smart defaults
///
/// Behavior:
/// - Tries to call cmd_version() handler if it exists
/// - Falls back to default version info extracted from Cargo.toml
/// - Format matches boxy style:
///   [ASCII ART if RSB_LOGO_ART exists]
///   Version: X.Y.Z | License: LICENSE
///   Copyright © YEAR AUTHOR
fn rsb_handle_version_flag(args: &Args) -> Option<i32> {
    // Check if --version or -v is present
    if !args.has("--version") && !args.has("-v") {
        return None;
    }

    // Try cmd_version handler first
    if global::has_var("fn_cmd_version") {
        // Handler exists
        // TODO: Dynamic dispatch through function registry
        // For now, fall through to default
    }

    // Default version output
    show_default_version();
    Some(0)
}

/// Show default version info extracted from package metadata
///
/// Format matches boxy style output. Sources (in priority order):
/// 1. RSB_LOGO_ART global var for ASCII banner
/// 2. CARGO_PKG_* env vars for version/license
/// 3. inf_* from TOML snooping (author, copyright, build_info)
/// 4. RSB_COPYRIGHT and RSB_BUILD_INFO globals as fallbacks
fn show_default_version() {
    // Check for ASCII art banner
    if global::has_var("RSB_LOGO_ART") {
        let logo = global::get_var("RSB_LOGO_ART");
        if !logo.is_empty() {
            println!("{}", logo);
        }
    }

    // Get package info from Cargo build-time env vars
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
    let license = std::env::var("CARGO_PKG_LICENSE").unwrap_or_else(|_| "Unknown".to_string());

    // Basic version line (always shown)
    println!("Version: {} | License: {}", version, license);

    // Copyright line - check inf metadata from TOML snooping first
    let copyright = if global::has_var("inf_copyright") {
        global::get_var("inf_copyright")
    } else if global::has_var("RSB_COPYRIGHT") {
        global::get_var("RSB_COPYRIGHT")
    } else {
        String::new()
    };

    if !copyright.is_empty() {
        println!("{}", copyright);
    }

    // Build info - check inf metadata from TOML snooping first
    let build_info = if global::has_var("inf_build_info") {
        global::get_var("inf_build_info")
    } else if global::has_var("RSB_BUILD_INFO") {
        global::get_var("RSB_BUILD_INFO")
    } else {
        String::new()
    };

    if !build_info.is_empty() {
        println!("\n{}", build_info);
    }
}

/// Check and handle flag commands before dispatch
///
/// This function is called by dispatch! macro to handle flags like
/// --help and --version before routing to regular command handlers.
///
/// Returns Some(exit_code) if a flag was handled, None otherwise.
pub fn check_flag_commands(args: &Args) -> Option<i32> {
    // Check help flag first (more common)
    if let Some(code) = rsb_handle_help_flag(args) {
        return Some(code);
    }

    // Check version flag
    if let Some(code) = rsb_handle_version_flag(args) {
        return Some(code);
    }

    // No flags matched
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_flag_detection() {
        let args = Args::new(&["prog", "--help"]);
        assert!(rsb_handle_help_flag(&args).is_some());

        let args = Args::new(&["prog", "-h"]);
        assert!(rsb_handle_help_flag(&args).is_some());

        let args = Args::new(&["prog", "build"]);
        assert!(rsb_handle_help_flag(&args).is_none());
    }

    #[test]
    fn test_version_flag_detection() {
        let args = Args::new(&["prog", "--version"]);
        assert!(rsb_handle_version_flag(&args).is_some());

        let args = Args::new(&["prog", "-v"]);
        assert!(rsb_handle_version_flag(&args).is_some());

        let args = Args::new(&["prog", "build"]);
        assert!(rsb_handle_version_flag(&args).is_none());
    }

    #[test]
    fn test_topic_help_detection() {
        let args = Args::new(&["prog", "build", "--help"]);
        let result = rsb_handle_help_flag(&args);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_check_flag_commands_help() {
        let args = Args::new(&["prog", "--help"]);
        assert_eq!(check_flag_commands(&args), Some(0));
    }

    #[test]
    fn test_check_flag_commands_version() {
        let args = Args::new(&["prog", "--version"]);
        assert_eq!(check_flag_commands(&args), Some(0));
    }

    #[test]
    fn test_check_flag_commands_none() {
        let args = Args::new(&["prog", "build"]);
        assert_eq!(check_flag_commands(&args), None);
    }
}