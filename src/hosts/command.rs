//! Command Execution
//!
//! Shell command execution with global variable expansion.
//! Per MODULE_SPEC: hosts (consumer) depends on global (provider) for expand_vars.

use crate::global;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::process::Command;
use std::sync::{Arc, Mutex};

/// The result of a command execution, containing status, stdout, and stderr.
#[derive(Debug, Clone)]
pub struct CmdResult {
    pub status: i32,
    pub output: String,
    pub error: String,
}

lazy_static! {
    static ref MOCK_CMDS: Arc<Mutex<HashMap<String, String>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

/// Executes a shell command and returns a `CmdResult`.
/// Expands global variables in the command string.
pub fn run_cmd_with_status(cmd: &str) -> CmdResult {
    let expanded_cmd = global::expand_vars(cmd);

    // Mocked command outputs (primarily for tests)
    if let Some(mock_out) = MOCK_CMDS.lock().unwrap().get(&expanded_cmd).cloned() {
        return CmdResult {
            status: 0,
            output: mock_out,
            error: String::new(),
        };
    }

    let output = Command::new("sh").arg("-c").arg(&expanded_cmd).output();

    match output {
        Ok(out) => CmdResult {
            status: out.status.code().unwrap_or(1),
            output: String::from_utf8_lossy(&out.stdout).to_string(),
            error: String::from_utf8_lossy(&out.stderr).to_string(),
        },
        Err(e) => CmdResult {
            status: 1,
            output: String::new(),
            error: e.to_string(),
        },
    }
}

/// Executes a shell command and returns its stdout, exiting on error.
pub fn run_cmd(cmd: &str) -> String {
    let result = run_cmd_with_status(cmd);
    if result.status != 0 {
        crate::event!(emit "COMMAND_ERROR", "source" => "cmd!", "command" => cmd, "status" => &result.status.to_string(), "stderr" => &result.error);
        eprintln!("Command failed: {}", cmd);
        eprintln!("Stderr: {}", result.error);
        std::process::exit(result.status);
    }
    result.output
}

/// Executes a shell command and captures its output, similar to `$(...)` in bash.
pub fn shell_exec(cmd: &str, silent: bool) -> Result<String, CmdResult> {
    let result = run_cmd_with_status(cmd);
    if result.status == 0 {
        Ok(result.output.trim().to_string())
    } else {
        if !silent {
            // The error message is now part of the CmdResult.
        }
        Err(result)
    }
}

/// Set mocked command outputs. Intended for tests.
pub fn set_mock_cmds(pairs: &[(&str, &str)]) {
    let mut map = MOCK_CMDS.lock().unwrap();
    map.clear();
    for (cmd, out) in pairs {
        map.insert((*cmd).to_string(), (*out).to_string());
    }
}

/// Clear all mocked commands.
pub fn clear_mock_cmds() {
    MOCK_CMDS.lock().unwrap().clear();
}
