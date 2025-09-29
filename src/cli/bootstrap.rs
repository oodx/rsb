// CLI Bootstrap and Script Awareness

use crate::prelude::*;
use crate::global::get_all_vars;

/// Store CLI arguments in global store with 1-based indexing (bash convention)
/// Sets: cli_argc, cli_args (semicolon-joined), cli_arg_1, cli_arg_2, etc.
///
/// Note: Acquires global lock multiple times for atomic operations.
/// Each set_var/unset_var call locks and unlocks the global mutex.
pub fn cli_to_global(args: &[String]) {
    // Clear any existing cli_arg_* variables first to prevent stale data
    // This is important when cli_to_global is called multiple times
    // Note: get_all_vars() acquires a lock, clones the HashMap, then releases
    let all_vars = get_all_vars();
    for key in all_vars.keys() {
        if key.starts_with("cli_arg_") {
            unset_var(key); // Each unset_var acquires and releases lock
        }
    }

    // Store argument count (excluding program name)
    let argc = args.len().saturating_sub(1);
    set_var("cli_argc", argc.to_string());

    // Store program name separately (argv[0])
    if let Some(prog) = args.first() {
        set_var("cli_argv_0", prog);
        set_var("cli_prog", prog);
    }

    // Store individual arguments with 1-based indexing (bash convention)
    // Skip first element (program name) for positional args
    for (i, arg) in args.iter().skip(1).enumerate() {
        let key = format!("cli_arg_{}", i + 1);  // 1-based
        set_var(&key, arg);
    }

    // Store all args (excluding program) as semicolon-joined string
    if args.len() > 1 {
        let joined = args[1..].join(";");
        set_var("cli_args", &joined);
    } else {
        set_var("cli_args", "");
    }

    // Store raw argv for completeness (all args including program)
    let all_joined = args.join(";");
    set_var("cli_argv", &all_joined);
}

/// CLI bootstrap sequences host bootstrap and prepares CLI context.
/// Now includes storing CLI args in global by default.
pub fn cli_bootstrap(args: &[String]) {
    // Leverage host bootstrap (env, XDG/RSB, dirs, modes, script, args)
    crate::hosts::bootstrap(args);

    // Store CLI args in global store (new in v0.7.0+)
    cli_to_global(args);

    // CLI‑specific extensions could go here (help registry, interactive checks, etc.)
}

/// Bootstrap using std::env::args().
pub fn cli_bootstrap_from_env() {
    let args: Vec<String> = std::env::args().collect();
    cli_bootstrap(&args);
}

// FUNCTIONS TO MOVE FROM context.rs (CLI-specific parts):

// setup_script_awareness(args: &[String]) -> ()
// - Extract script information from command line args
// - Set SCRIPT_NAME, SCRIPT_PATH, SCRIPT_DIR, PWD variables

// rsb_bootstrap(args: &[String]) -> ()
// - SPLIT: CLI parts here, environment parts in global::bootstrap
// - This function becomes: cli_bootstrap() + global::env_bootstrap()

// CLI BOOTSTRAP OPERATIONS:

// cli_bootstrap(args: &[String]) -> ()
// - Complete CLI bootstrap including environment
// - Calls: global::env_bootstrap() + setup_script_awareness(args)
// - Example: cli_bootstrap(&env::args().collect());

// setup_script_awareness(args) -> ()
// - Extract and set script-related variables:
//   - SCRIPT_NAME: filename only (e.g., "backup.sh")
//   - SCRIPT_PATH: full path (e.g., "/usr/local/bin/backup.sh")
//   - SCRIPT_DIR: containing directory (e.g., "/usr/local/bin")
//   - PWD: current working directory

// SCRIPT INFORMATION:

// get_script_name() -> String
// - Get current script name from global context
// - Example: get_script_name() -> "backup"

// get_script_path() -> String
// - Get full script path from global context
// - Example: get_script_path() -> "/usr/local/bin/backup"

// get_script_dir() -> String
// - Get script directory from global context
// - Example: get_script_dir() -> "/usr/local/bin"

// BOOTSTRAP UTILITIES:

// is_cli_bootstrapped() -> bool
// - Check if CLI bootstrap has been completed
// - Example: if !is_cli_bootstrapped() { cli_bootstrap(&args); }

// bootstrap_from_env() -> ()
// - Bootstrap using std::env::args() automatically
// - Example: bootstrap_from_env(); // Uses env::args().collect()

// minimal_cli_bootstrap() -> ()
// - Minimal CLI bootstrap (no directory creation, just script awareness)
// - Example: minimal_cli_bootstrap();

// INTEGRATION WITH GLOBAL:

// cli_bootstrap(args) internally calls:
// 1. global::env_bootstrap()     // Environment, XDG paths, modes
// 2. setup_script_awareness(args) // Script-specific CLI info
// 3. init_help_system()          // CLI help system

// This provides the bridge pattern:
// - global::env_bootstrap() - for libraries that just want environment
// - cli::bootstrap() - for CLI apps that want full bootstrap

// COMMAND LINE INTEGRATION:

// parse_bootstrap_args(args: &[String]) -> (Vec<String>, BootstrapConfig)
// - Separate bootstrap config from application args
// - Example: --rsb-config, --rsb-debug flags for bootstrap behavior

// apply_cli_config(config: BootstrapConfig) -> ()
// - Apply CLI-specific bootstrap configuration
// - Example: debug mode, quiet mode, custom config paths

// CLI CONTEXT SETUP:

// setup_cli_context(args: &[String]) -> ()
// - Set CLI-specific context variables
// - Example: ARGC, ARGV_0, ARGV_1, etc.

// detect_interactive_mode() -> ()
// - Detect if running in interactive terminal
// - Sets INTERACTIVE_MODE based on isatty check

// setup_signal_handlers() -> ()
// - Set up signal handlers for CLI applications
// - Handle SIGINT, SIGTERM gracefully
