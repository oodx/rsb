//! JQ JSON Utilities
//!
//! Shell-based JSON parsing using jq (if available).

use crate::hosts::command::run_cmd;
use crate::hosts::os::is_command;

/// Extract a value from JSON using jq (if available).
pub fn jq_get(json_str: &str, path: &str) -> String {
    if !is_command("jq") {
        return String::new();
    }

    let cmd = format!(
        "echo '{}' | jq -r '{}'",
        json_str.replace("'", "'\"'\"'"),
        path
    );
    run_cmd(&cmd).trim().to_string()
}

/// Extract a value from JSON file using jq (if available).
pub fn jq_get_file(json_file: &str, path: &str) -> String {
    if !is_command("jq") {
        return String::new();
    }

    let cmd = format!("jq -r '{}' '{}'", path, json_file);
    run_cmd(&cmd).trim().to_string()
}
