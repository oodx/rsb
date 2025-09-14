// General CLI Building Utilities

// GENERAL CLI HELPERS:

// parse_command_line(args: &[String]) -> (String, Vec<String>)
// - Split args into command and remaining arguments
// - Example: parse_command_line(&["app", "backup", "--full"]) -> ("backup", ["--full"])

// normalize_command(cmd: &str) -> String
// - Normalize command name (lowercase, trim, etc.)
// - Example: normalize_command("  BACKUP  ") -> "backup"

// split_flags_and_args(args: &[String]) -> (Vec<String>, Vec<String>)
// - Separate flags from positional arguments
// - Example: split_flags_and_args(&["--verbose", "file.txt", "-f"]) -> (["--verbose", "-f"], ["file.txt"])

// is_flag(arg: &str) -> bool
// - Check if argument is a flag (starts with - or --)
// - Example: is_flag("--help") -> true, is_flag("file.txt") -> false

// COMMAND VALIDATION:

// validate_command(cmd: &str, valid_commands: &[&str]) -> Result<String, CommandError>
// - Validate command against list of valid commands
// - Example: validate_command("backup", &["backup", "restore"]) -> Ok("backup")

// suggest_similar_command(cmd: &str, valid_commands: &[&str]) -> Option<String>
// - Suggest similar command for typos using edit distance
// - Example: suggest_similar_command("bakup", &["backup", "restore"]) -> Some("backup")

// command_exists(cmd: &str) -> bool
// - Check if command is registered in function registry
// - Example: command_exists("backup") -> true

// ARGUMENT PROCESSING:

// expand_args(args: &[String]) -> Vec<String>
// - Expand variables in arguments using global context
// - Example: expand_args(&["$HOME/file.txt"]) -> ["/home/user/file.txt"]

// process_config_args(args: &[String]) -> Vec<String>
// - Process --config arguments and load configuration files
// - Example: process_config_args(&["--config", "app.conf", "backup"]) loads config and returns ["backup"]

// merge_args_with_config(args: &[String]) -> Vec<String>
// - Merge command line args with configuration file settings
// - Example: CLI args override config file values

// CLI STATE MANAGEMENT:

// get_command_context() -> HashMap<String, String>
// - Get context variables relevant to current command
// - Example: SCRIPT_NAME, SCRIPT_PATH, current command, etc.

// set_cli_mode(mode: CliMode) -> ()
// - Set CLI interaction mode (interactive, batch, quiet)
// - Example: set_cli_mode(CliMode::Interactive)

// is_interactive() -> bool
// - Check if CLI is running in interactive mode
// - Example: if is_interactive() { prompt_user(); }

// CLI FORMATTING:

// format_command_list(commands: &[(String, String)]) -> String
// - Format command list for help display
// - Example: "  backup      Create system backup"

// align_help_text(items: &[(String, String)], padding: usize) -> String
// - Align help text with consistent column spacing
// - Example: align help text for commands and descriptions