// Help System and Function Registry Display

// FUNCTIONS TO MOVE FROM context.rs:

// show_help() -> ()
// - Display complete help with script name, commands, built-ins
// - Uses SCRIPT_NAME from global context, formats with colors

// show_functions() -> ()
// - Display all registered functions
// - Gets data from global function registry

// show_call_stack() -> ()
// - Display current call stack with timing
// - Gets data from global call stack

// HELP SYSTEM OPERATIONS:

// show_help() -> ()
// - Complete help display with:
//   - Script name and usage
//   - Registered commands
//   - Built-in commands (help, inspect, stack)
// - Example output: "USAGE: backup <command> [options]"

// show_usage() -> ()
// - Show just usage line without full help
// - Example: "USAGE: backup <command> [options]"

// show_commands() -> ()
// - Show just registered commands without built-ins
// - Example: "  backup      Create system backup"

// show_functions() -> ()
// - Alias for show_commands() (from function registry)
// - Shows all registered function descriptions

// HELP FORMATTING:

// format_help_header() -> String
// - Format script name and usage header with colors
// - Example: "{bold}{blue}backup{reset}\n\nUSAGE:\n  backup <command> [options]"

// format_command_section(title: &str, commands: &[(String, String)]) -> String
// - Format section of commands with title
// - Example: "{bold}COMMANDS:{reset}\n  backup    Create backup"

// format_command_entry(name: &str, description: &str, width: usize) -> String
// - Format single command entry with alignment
// - Example: "  {cyan}backup{reset}         Create system backup"

// BUILT-IN COMMANDS:

// show_built_in_commands() -> ()
// - Display built-in RSB commands
// - help, inspect, stack, etc.

// handle_built_in_command(command: &str) -> bool
// - Handle built-in commands, return true if handled
// - Example: handle_built_in_command("help") -> true (shows help)

// is_built_in_command(command: &str) -> bool
// - Check if command is built-in
// - Example: is_built_in_command("help") -> true

// HELP CUSTOMIZATION:

// set_help_header(header: &str) -> ()
// - Customize help header text
// - Example: set_help_header("My Custom Tool v1.0")

// add_help_section(title: &str, content: &str) -> ()
// - Add custom section to help output
// - Example: add_help_section("EXAMPLES", "  backup --full /home")

// set_usage_template(template: &str) -> ()
// - Set custom usage template
// - Example: set_usage_template("{script} [OPTIONS] <COMMAND>")

// INTERACTIVE HELP:

// interactive_help() -> ()
// - Interactive help system with command search
// - Example: Type command name to get specific help

// search_commands(query: &str) -> Vec<(String, String)>
// - Search commands by name or description
// - Example: search_commands("back") -> [("backup", "Create backup")]

// suggest_command(input: &str) -> Option<String>
// - Suggest similar command for typos
// - Example: suggest_command("bakup") -> Some("backup")

// HELP INTEGRATION:

// auto_help_on_error() -> ()
// - Automatically show help when command fails
// - Example: Unknown command shows help

// context_sensitive_help(command: &str) -> ()
// - Show help specific to command context
// - Example: context_sensitive_help("backup") shows backup options

// help_with_examples(command: &str) -> ()
// - Show help with usage examples for specific command
// - Example: help_with_examples("backup") shows backup examples