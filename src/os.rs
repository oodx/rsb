// src/os.rs

// This module will contain operating system interactions, such as
// running commands, getting system info, job control, and trap handling.

// --- Command Execution ---
use crate::global::expand_vars;
use std::process::Command;
use std::sync::{Arc, Mutex};

// Module-owned macros for process helpers and locking
pub mod macros;

/// The result of a command execution, containing status, stdout, and stderr.
#[derive(Debug, Clone)]
pub struct CmdResult {
    pub status: i32,
    pub output: String,
    pub error: String,
}

/// Executes a shell command and returns a `CmdResult`.
pub fn run_cmd_with_status(cmd: &str) -> CmdResult {
    let expanded_cmd = expand_vars(cmd);
    // Mocked command outputs (primarily for tests)
    if let Some(mock_out) = MOCK_CMDS.lock().unwrap().get(&expanded_cmd).cloned() {
        return CmdResult { status: 0, output: mock_out, error: String::new() };
    }
    let output = Command::new("sh")
        .arg("-c")
        .arg(&expanded_cmd)
        .output();

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

/// Executes a shell command and returns its stdout, panicking on error.
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


// --- Job Control ---

use std::thread;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct JobHandle {
    pub id: u32,
    pub command: String,
    pub handle: Option<thread::JoinHandle<()>>,
    pub rx: std::sync::mpsc::Receiver<CmdResult>,
}

lazy_static! {
    pub static ref JOBS: Arc<Mutex<HashMap<u32, Arc<Mutex<JobHandle>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    pub static ref JOB_COUNTER: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    static ref MOCK_CMDS: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// Waits for a specific job to complete and returns its exit status.
/// This function will remove the job from the global JOBS map.
pub fn wait_on_job(job_id: u32, timeout: Option<std::time::Duration>) -> Result<CmdResult, String> {
    let job_arc = JOBS.lock().unwrap().remove(&job_id);

    if let Some(job_arc) = job_arc {
        if let Ok(job_handle) = Arc::try_unwrap(job_arc).map(|mutex| mutex.into_inner().unwrap()) {

            let result = if let Some(t) = timeout {
                job_handle.rx.recv_timeout(t)
            } else {
                job_handle.rx.recv().map_err(|_| std::sync::mpsc::RecvTimeoutError::Disconnected)
            };

            return match result {
                Ok(cmd_result) => {
                    if let Some(h) = job_handle.handle {
                        let _ = h.join(); // Join only on success
                    }
                    Ok(cmd_result)
                },
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    // On timeout, we don't join the handle. The job is orphaned.
                    Err("Timeout".to_string())
                },
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    if let Some(h) = job_handle.handle {
                        let _ = h.join(); // Join on disconnect
                    }
                    Err("Job channel disconnected".to_string())
                },
            };
        }
    }

    Err(format!("Job {} not found", job_id))
}


// --- Event System ---
use std::sync::atomic::{AtomicBool, Ordering};

// --- Signal Handling ---
static TRAP_INSTALLED: AtomicBool = AtomicBool::new(false);

/// The actual C-style signal handler.
extern "C" fn signal_handler(signal: i32) {
    let event_name = match signal {
        libc::SIGINT => "SIGINT",
        libc::SIGTERM => "SIGTERM",
        _ => "UNKNOWN_SIGNAL",
    };
    eprintln!("\nrsb-trap: Caught signal {}, exiting.", event_name);
    std::process::exit(128 + signal);
}

/// Installs the signal handlers for common termination signals.
pub fn install_signal_handlers() {
    if TRAP_INSTALLED
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        unsafe {
            libc::signal(libc::SIGINT, signal_handler as usize);
            libc::signal(libc::SIGTERM, signal_handler as usize);
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventData {
    pub event_type: String,
    pub data: HashMap<String, String>,
}

lazy_static! {
    // A registry for event handlers.
    pub static ref EVENT_HANDLERS: Arc<Mutex<HashMap<String, Vec<Box<dyn Fn(&EventData) + Send + Sync>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}


// --- System Information ---

/// Gets the current user's name from the context (`USER` variable).
pub fn get_user() -> String { crate::global::get_var("USER") }

/// Gets the current user's home directory from the context (`HOME` variable).
pub fn get_home() -> String { crate::global::get_var("HOME") }

/// Gets the current working directory from the context (`PWD` variable).
pub fn get_pwd() -> String { crate::global::get_var("PWD") }

/// Gets the system's hostname.
pub fn get_hostname() -> String { crate::hosts::get_hostname() }

/// Gets the system's architecture (e.g., `x86_64`, `aarch64`).
pub fn get_arch() -> String { crate::hosts::get_arch() }

/// Gets the operating system (e.g., `linux`, `macos`, `windows`).
pub fn get_os() -> String { crate::hosts::get_os() }


// --- OS Test Functions ---

/// Checks if a command is available in the system's PATH.
pub fn is_command(cmd: &str) -> bool { crate::hosts::is_command(cmd) }

// --- Archive Operations ---

/// Creates a tar archive using system tar command.
pub fn create_tar(archive_path: &str, source_paths: &[&str]) -> CmdResult { crate::bash::create_tar(archive_path, source_paths) }

/// Creates a compressed tar.gz archive using system tar command.
pub fn create_tar_gz(archive_path: &str, source_paths: &[&str]) -> CmdResult { crate::bash::create_tar_gz(archive_path, source_paths) }

/// Extracts a tar archive using system tar command.
pub fn extract_tar(archive_path: &str, dest_dir: Option<&str>) -> CmdResult { crate::bash::extract_tar(archive_path, dest_dir) }

/// Lists contents of a tar archive using system tar command.
pub fn list_tar(archive_path: &str) -> CmdResult { crate::bash::list_tar(archive_path) }

/// Creates a zip archive using system zip command.
pub fn create_zip(archive_path: &str, source_paths: &[&str]) -> CmdResult { crate::bash::create_zip(archive_path, source_paths) }

/// Extracts a zip archive using system unzip command.
pub fn extract_zip(archive_path: &str, dest_dir: Option<&str>) -> CmdResult { crate::bash::extract_zip(archive_path, dest_dir) }

/// Lists contents of a zip archive using system unzip command.
pub fn list_zip(archive_path: &str) -> CmdResult { crate::bash::list_zip(archive_path) }

// --- Additional System Information Functions ---

/// Gets the current username.
pub fn get_username() -> String { crate::hosts::get_username() }

/// Gets the current user's home directory.
pub fn get_home_dir() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "/".to_string())
}

/// Gets the current working directory.
pub fn get_current_dir() -> String {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string())
}

// --- Network Functions ---

/// Simple HTTP GET using curl.
pub fn http_get(url: &str) -> CmdResult { crate::bash::http_get(url) }

/// HTTP GET with custom options.
pub fn http_get_with_options(url: &str, options: &str) -> CmdResult { crate::bash::http_get_with_options(url, options) }

/// Simple HTTP POST using curl.
pub fn http_post(url: &str, data: &str) -> CmdResult { crate::bash::http_post(url, data) }

// --- Process Management Functions ---

/// Get process ID of a named process.
pub fn pid_of(process_name: &str) -> String {
    let result = run_cmd(&format!("pgrep '{}'", process_name));
    result.lines().next().unwrap_or("").trim().to_string()
}

/// Check if a process exists by name.
pub fn process_exists(process_name: &str) -> bool {
    !pid_of(process_name).is_empty()
}

/// Kill a process by PID.
pub fn kill_pid(pid: &str, signal: Option<&str>) -> CmdResult {
    let sig = signal.unwrap_or("TERM");
    let cmd = format!("kill -{} {}", sig, pid);
    run_cmd_with_status(&cmd)
}

/// Kill all processes by name.
pub fn kill_process(process_name: &str, signal: Option<&str>) -> CmdResult {
    let sig = signal.unwrap_or("TERM");
    let cmd = format!("pkill -{} '{}'", sig, process_name);
    run_cmd_with_status(&cmd)
}

// --- Locking Functions ---

use std::io::Write;

/// Create a lock file with PID.
pub fn create_lock(lock_path: &str) -> Result<(), String> {
    use std::fs::File;
    
    if std::path::Path::new(lock_path).exists() {
        // Check if the PID in the lock file is still running
        if let Ok(contents) = std::fs::read_to_string(lock_path) {
            let old_pid = contents.trim();
            if process_exists_by_pid(old_pid) {
                return Err(format!("Lock file exists and process {} is running", old_pid));
            }
            // Stale lock file, remove it
            let _ = std::fs::remove_file(lock_path);
        }
    }
    
    let mut file = File::create(lock_path)
        .map_err(|e| format!("Failed to create lock file: {}", e))?;
    
    let pid = std::process::id();
    write!(file, "{}", pid)
        .map_err(|e| format!("Failed to write PID to lock file: {}", e))?;
    
    Ok(())
}

/// Remove a lock file.
pub fn remove_lock(lock_path: &str) {
    let _ = std::fs::remove_file(lock_path);
}

/// Check if a process exists by PID.
pub fn process_exists_by_pid(pid: &str) -> bool {
    if pid.is_empty() {
        return false;
    }
    
    let result = run_cmd(&format!("ps -p {} -o pid=", pid));
    !result.trim().is_empty()
}

// --- Basic JSON Functions (shell-based) ---

/// Extract a value from JSON using jq (if available).
pub fn json_get(json_str: &str, path: &str) -> String {
    if !is_command("jq") {
        return String::new();
    }
    
    let cmd = format!("echo '{}' | jq -r '{}'", json_str.replace("'", "'\"'\"'"), path);
    run_cmd(&cmd).trim().to_string()
}

/// Extract a value from JSON file using jq (if available).
pub fn json_get_file(json_file: &str, path: &str) -> String {
    if !is_command("jq") {
        return String::new();
    }
    
    let cmd = format!("jq -r '{}' '{}'", path, json_file);
    run_cmd(&cmd).trim().to_string()
}
