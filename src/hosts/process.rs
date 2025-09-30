//! Process Management and Job Control
//!
//! Process queries, job tracking, and locking mechanisms.

use super::command::{run_cmd, run_cmd_with_status, CmdResult};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

// === Job Control ===

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
}

/// Waits for a specific job to complete and returns its exit status.
/// This function will remove the job from the global JOBS map.
pub fn wait_on_job(
    job_id: u32,
    timeout: Option<std::time::Duration>,
) -> Result<CmdResult, String> {
    let job_arc = JOBS.lock().unwrap().remove(&job_id);

    if let Some(job_arc) = job_arc {
        if let Ok(job_handle) = Arc::try_unwrap(job_arc).map(|mutex| mutex.into_inner().unwrap()) {
            let result = if let Some(t) = timeout {
                job_handle.rx.recv_timeout(t)
            } else {
                job_handle
                    .rx
                    .recv()
                    .map_err(|_| std::sync::mpsc::RecvTimeoutError::Disconnected)
            };

            return match result {
                Ok(cmd_result) => {
                    if let Some(h) = job_handle.handle {
                        let _ = h.join(); // Join only on success
                    }
                    Ok(cmd_result)
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    // On timeout, we don't join the handle. The job is orphaned.
                    Err("Timeout".to_string())
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    if let Some(h) = job_handle.handle {
                        let _ = h.join(); // Join on disconnect
                    }
                    Err("Job channel disconnected".to_string())
                }
            };
        }
    }

    Err(format!("Job {} not found", job_id))
}

// === Process Management ===

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

/// Check if a process exists by PID.
pub fn process_exists_by_pid(pid: &str) -> bool {
    if pid.is_empty() {
        return false;
    }

    let result = run_cmd(&format!("ps -p {} -o pid=", pid));
    !result.trim().is_empty()
}

// === Locking ===

/// Create a lock file with PID.
pub fn create_lock(lock_path: &str) -> Result<(), String> {
    use std::fs::File;

    if std::path::Path::new(lock_path).exists() {
        // Check if the PID in the lock file is still running
        if let Ok(contents) = std::fs::read_to_string(lock_path) {
            let old_pid = contents.trim();
            if process_exists_by_pid(old_pid) {
                return Err(format!(
                    "Lock file exists and process {} is running",
                    old_pid
                ));
            }
            // Stale lock file, remove it
            let _ = std::fs::remove_file(lock_path);
        }
    }

    let mut file =
        File::create(lock_path).map_err(|e| format!("Failed to create lock file: {}", e))?;

    let pid = std::process::id();
    write!(file, "{}", pid).map_err(|e| format!("Failed to write PID to lock file: {}", e))?;

    Ok(())
}

/// Remove a lock file.
pub fn remove_lock(lock_path: &str) {
    let _ = std::fs::remove_file(lock_path);
}
