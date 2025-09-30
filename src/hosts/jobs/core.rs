//! Background Job Control
//!
//! Background shell job execution, tracking, and waiting.

use super::super::command::{run_cmd_with_status, CmdResult};
use super::process::{JobHandle, JOB_COUNTER, JOBS};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Start a background job that runs the given shell command.
/// Returns a job ID that can be waited on.
pub fn start_background(command: &str) -> u32 {
    let mut counter = JOB_COUNTER.lock().unwrap();
    *counter += 1;
    let job_id = *counter;
    let cmd_string = command.to_string();
    let (tx, rx) = std::sync::mpsc::channel();
    let cmd_string_for_thread = cmd_string.clone();

    let handle = thread::spawn(move || {
        let result = run_cmd_with_status(&cmd_string_for_thread);
        let _ = tx.send(result);
    });

    let job_handle = JobHandle {
        id: job_id,
        command: cmd_string,
        handle: Some(handle),
        rx,
    };
    JOBS.lock()
        .unwrap()
        .insert(job_id, Arc::new(Mutex::new(job_handle)));
    crate::utils::stderrx("info", &format!("[{}] Started background job", job_id));
    job_id
}

/// Wait for a background job by ID. Optional timeout in seconds.
pub fn wait(job_id: u32, timeout_secs: Option<u64>) -> Result<i32, String> {
    let timeout = timeout_secs.map(Duration::from_secs);
    match super::process::wait_on_job(job_id, timeout) {
        Ok(result) => Ok(result.status),
        Err(e) => Err(e),
    }
}

/// List current background jobs as (id, command) pairs.
pub fn list_jobs() -> Vec<(u32, String)> {
    let jobs = JOBS.lock().unwrap();
    let mut out = Vec::new();
    for (id, job_mutex) in jobs.iter() {
        let job = job_mutex.lock().unwrap();
        out.push((*id, job.command.clone()));
    }
    out
}
