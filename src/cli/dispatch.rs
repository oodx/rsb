//! Command Dispatch and Routing - ported from macros to function implementation.
//!
//! Provides dispatch functions that work with dispatch! and pre_dispatch! macros
//! for command routing and execution following MODULE_SPEC pattern.

use crate::cli::Args;
use crate::global;

/// Type alias for command handler functions that take Args and return exit code.
pub type CommandHandler = fn(Args) -> i32;

/// Internal helper for dispatch macro - handles all dispatch logic.
///
/// This function contains all the business logic that was previously in the macro:
/// - Command extraction and argument processing
/// - Handler registration for introspection
/// - Built-in command handling (help, inspect, stack)
/// - User command routing and execution with call stack management
/// - Process exit with appropriate codes
pub fn execute_dispatch<F>(args: &Args, handler_lookup: F)
where
    F: Fn(&str) -> Option<CommandHandler>,
{
    let command = args.get_or(1, "help");
    let mut cmd_args = args.clone();
    cmd_args.has_pop(&command);

    // Handle built-in commands first
    match command.as_str() {
        "help" | "--help" | "-h" => {
            global::show_help();
            std::process::exit(0);
        }
        "inspect" => {
            global::show_functions();
            std::process::exit(0);
        }
        "stack" => {
            global::show_call_stack();
            std::process::exit(0);
        }
        _ => {
            // Try to find user command handler
            if let Some(handler) = handler_lookup(&command) {
                global::push_call(&command, cmd_args.all());
                let result = handler(cmd_args);
                global::pop_call();
                std::process::exit(result);
            } else {
                handle_unknown_command(&command, &handler_lookup);
            }
        }
    }
}

/// Internal helper for pre_dispatch macro - handles all pre-dispatch logic.
///
/// This is the test-friendly version that returns a boolean instead of calling
/// process::exit, enabling testing of command dispatch logic.
pub fn execute_pre_dispatch<F>(args: &Args, handler_lookup: F) -> bool
where
    F: Fn(&str) -> Option<CommandHandler>,
{
    let command = args.get_or(1, "");
    let is_test = std::env::var("CARGO_TEST").is_ok()
        || std::thread::current()
            .name()
            .map_or(false, |n| n.contains("test"));

    if let Some(handler) = handler_lookup(&command) {
        let mut cmd_args = args.clone();
        cmd_args.has_pop(&command);
        global::push_call(&command, cmd_args.all());
        let result = handler(cmd_args);
        global::pop_call();

        if is_test {
            true
        } else {
            std::process::exit(result);
        }
    } else {
        false
    }
}

/// Register command handlers for introspection (used by macros).
///
/// This is called by the dispatch macros to register all available commands
/// for the built-in "inspect" command functionality.
pub fn register_handlers(handlers: &[(&str, CommandHandler)]) {
    for (cmd, _handler) in handlers {
        // Store a clean, human-readable entry without leaking pointer debug strings.
        // Description can be enriched by higher-level helpers later.
        global::register_function(cmd, "");
    }
}

/// Enhanced error handling for unknown commands with helpful suggestions.
fn handle_unknown_command<F>(command: &str, handler_lookup: &F)
where
    F: Fn(&str) -> Option<CommandHandler>,
{
    eprintln!("Error: Unknown command '{}'", command);

    // Try to find similar commands (simple suggestion algorithm)
    let suggestions = find_command_suggestions(command, handler_lookup);

    if !suggestions.is_empty() {
        eprintln!();
        eprintln!("Did you mean one of these?");
        for suggestion in suggestions {
            eprintln!("  {}", suggestion);
        }
        eprintln!();
    }

    eprintln!("Use 'help' to see all available commands.");
    eprintln!("Use 'inspect' to see registered command handlers.");

    std::process::exit(1);
}

/// Find command suggestions using simple string distance algorithm.
fn find_command_suggestions<F>(target: &str, handler_lookup: &F) -> Vec<String>
where
    F: Fn(&str) -> Option<CommandHandler>,
{
    // Common commands to check for suggestions
    let common_commands = [
        "help", "version", "status", "init", "build", "test", "run", "start", "stop", "create",
        "delete", "list", "show", "get", "set", "config", "install", "update",
    ];

    let mut suggestions = Vec::new();

    // Check if any common commands are available and similar
    for cmd in &common_commands {
        if handler_lookup(cmd).is_some() && is_similar(target, cmd) {
            suggestions.push(cmd.to_string());
        }
    }

    // Limit suggestions to avoid clutter
    suggestions.truncate(3);
    suggestions
}

/// Simple similarity check for command suggestions.
fn is_similar(a: &str, b: &str) -> bool {
    if a.is_empty() || b.is_empty() {
        return false;
    }

    // Check for common prefixes
    if a.len() >= 2 && b.len() >= 2 {
        if a[..2] == b[..2] {
            return true;
        }
    }

    // Check for one character difference (simple edit distance)
    if (a.len() as i32 - b.len() as i32).abs() <= 1 {
        let distance = edit_distance(a, b);
        distance <= 2
    } else {
        false
    }
}

/// Simple edit distance calculation for command suggestions.
fn edit_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    // Initialize first row and column
    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 0..=b_len {
        matrix[0][j] = j;
    }

    // Fill the matrix
    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i - 1][j] + 1, // deletion
                    matrix[i][j - 1] + 1, // insertion
                ),
                matrix[i - 1][j - 1] + cost, // substitution
            );
        }
    }

    matrix[a_len][b_len]
}
