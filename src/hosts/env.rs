//! Environment Variable Discovery and Import
//!
//! Env-only bootstrap and helpers used by CLI/bootstrap layers.
//! Scope: load env → write to global, and provide simple accessors.

/// Environment-only bootstrap (no CLI dependencies).
/// Imports all environment variables into the global store and applies standard modes.
pub fn env_bootstrap() {
    import_environment();
    setup_standard_modes();
}

/// Import all environment variables into the global store.
pub fn import_environment() {
    for (key, value) in std::env::vars() {
        crate::global::set_var(&key, &value);
    }
}

/// Apply standard mode flags from environment into global (integer booleans).
/// Sets: DEBUG_MODE, DEV_MODE, QUIET_MODE, TRACE_MODE to "1" when present.
pub fn setup_standard_modes() {
    if std::env::var("DEBUG").is_ok() { crate::global::set_var("DEBUG_MODE", "1"); }
    if std::env::var("DEV").is_ok() { crate::global::set_var("DEV_MODE", "1"); }
    if std::env::var("QUIET").is_ok() { crate::global::set_var("QUIET_MODE", "1"); }
    if std::env::var("TRACE").is_ok() { crate::global::set_var("TRACE_MODE", "1"); }
}

/// Get an environment variable directly (bypasses global).
pub fn get_env_var(key: &str) -> Option<String> { std::env::var(key).ok() }

/// Set an environment variable and mirror it in the global store.
pub fn set_env_var(key: &str, value: &str) {
    std::env::set_var(key, value);
    crate::global::set_var(key, value);
}

/// Check if an environment variable exists.
pub fn has_env_var(key: &str) -> bool { std::env::var(key).is_ok() }

/// Sync all environment variables into the global store (alias to import_environment).
pub fn env_to_global() { import_environment(); }

/// Sync all global variables back into environment variables.
pub fn global_to_env() {
    for (k, v) in crate::global::get_all_vars().iter() {
        std::env::set_var(k, v);
    }
}

// expand_env_vars(text: &str) -> String
// - Expand environment variables in text (uses env, not global context)
// - Example: expand_env_vars("$HOME/docs") -> "/home/user/docs"

// ENVIRONMENT UTILITIES:

// list_env_vars() -> Vec<(String, String)>
// - Get all environment variables as key-value pairs
// - Example: let all_env = list_env_vars();

// filter_env_vars(prefix: &str) -> HashMap<String, String>
// - Get environment variables starting with prefix
// - Example: filter_env_vars("RUST_") -> RUST_LOG, RUST_BACKTRACE, etc.

// backup_environment() -> HashMap<String, String>
// - Create backup of current environment
// - Example: let env_backup = backup_environment();

// restore_environment(backup: &HashMap<String, String>) -> ()
// - Restore environment from backup
// - Example: restore_environment(&env_backup);

// ENVIRONMENT DETECTION:

// detect_shell() -> Option<String>
// - Detect current shell from environment
// - Example: detect_shell() -> Some("bash")

// is_interactive_session() -> bool
// - Check if running in interactive session
// - Example: if is_interactive_session() { show_welcome(); }

// detect_terminal() -> Option<String>
// - Detect terminal type from environment
// - Example: detect_terminal() -> Some("xterm-256color")

// get_user_info() -> UserInfo
// - Get user information from environment
// - USER, HOME, SHELL, etc.

// PATH HELPERS:

// get_path_dirs() -> Vec<String>
// - Get PATH environment variable as directory list
// - Example: get_path_dirs() -> vec!["/usr/bin", "/bin", "/usr/local/bin"]

// find_in_path(command: &str) -> Option<String>
// - Find command in PATH directories
// - Example: find_in_path("git") -> Some("/usr/bin/git")

// add_to_path(directory: &str) -> ()
// - Add directory to PATH environment variable
// - Example: add_to_path("/opt/myapp/bin");

// ENVIRONMENT VALIDATION:

// validate_required_env(vars: &[&str]) -> Result<(), MissingEnvError>
// - Ensure required environment variables are set
// - Example: validate_required_env(&["HOME", "USER"])?;

// check_env_conflicts(rules: &ConflictRules) -> Result<(), ConflictError>
// - Check for conflicting environment variable combinations
// - Example: DEBUG and QUIET cannot both be set

// sanitize_env_value(value: &str) -> String
// - Sanitize environment variable value (remove dangerous chars)
// - Example: sanitize_env_value("value;rm -rf /") -> "valuerm -rf "

// INTEGRATION POINTS:

// sync_env_to_global() -> ()
// - Sync environment variables to global context
// - Called by env_bootstrap()

// sync_global_to_env() -> ()
// - Sync global context variables back to environment
// - For child process inheritance

// This provides the bridge pattern:
// - env_bootstrap() - libraries get environment without CLI
// - cli::bootstrap() - CLI apps get full bootstrap (calls env_bootstrap + CLI setup)
