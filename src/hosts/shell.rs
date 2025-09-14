//! Shell Detection, Interaction, and Execution Context
//!
//! Minimal execution context setup used during host bootstrap.

/// Extract script metadata from argv[0] and current working directory.
/// Sets Global keys: SCRIPT_NAME, SCRIPT_PATH, SCRIPT_DIR, PWD.
pub fn setup_execution_context(args: &[String]) {
    if args.is_empty() {
        // Still record current directory as PWD for completeness
        let pwd = std::env::current_dir().map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|_| ".".to_string());
        crate::global::set_var("PWD", &pwd);
        return;
    }
    let script_path = &args[0];
    let script_name = std::path::Path::new(script_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("script");
    let script_dir = std::path::Path::new(script_path)
        .parent()
        .and_then(|p| p.to_str())
        .unwrap_or(".");
    crate::global::set_var("SCRIPT_NAME", script_name);
    crate::global::set_var("SCRIPT_PATH", script_path);
    crate::global::set_var("SCRIPT_DIR", script_dir);
    let pwd = std::env::current_dir().map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|_| ".".to_string());
    crate::global::set_var("PWD", &pwd);
}

// Placeholder for potential shell_exec wrapper and shell detection helpers.
