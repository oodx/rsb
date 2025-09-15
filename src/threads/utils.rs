//! Curated thread utilities and job control wrappers
//!
//! This module exposes a simple API for sleeping, benchmarking, spawning
//! background jobs, waiting on jobs, and listing jobs. It delegates to
//! `crate::os` for the underlying execution and registries.

use std::time::{Duration, Instant};

/// Sleep for the specified number of milliseconds.
pub fn sleep_ms(ms: u64) { std::thread::sleep(Duration::from_millis(ms)); }

/// Benchmark a closure and return the elapsed time.
pub fn bench<F: FnOnce()>(label: &str, f: F) -> Duration {
    let start = Instant::now();
    f();
    let dur = start.elapsed();
    crate::utils::stderrx("info", &format!("[bench] {} took: {:?}", label, dur));
    dur
}

/// Start a background job that runs the given shell command.
/// Returns a job ID that can be waited on.
pub fn start_background(command: &str) -> u32 {
    use std::sync::{Arc, Mutex};
    let mut counter = crate::os::JOB_COUNTER.lock().unwrap();
    *counter += 1;
    let job_id = *counter;
    let cmd_string = command.to_string();
    let (tx, rx) = std::sync::mpsc::channel();
    let cmd_string_for_thread = cmd_string.clone();

    let handle = std::thread::spawn(move || {
        let result = crate::os::run_cmd_with_status(&cmd_string_for_thread);
        let _ = tx.send(result);
    });

    let job_handle = crate::os::JobHandle {
        id: job_id,
        command: cmd_string,
        handle: Some(handle),
        rx,
    };
    crate::os::JOBS.lock().unwrap().insert(job_id, Arc::new(Mutex::new(job_handle)));
    crate::utils::stderrx("info", &format!("[{}] Started background job", job_id));
    job_id
}

/// Wait for a background job by ID. Optional timeout in seconds.
pub fn wait(job_id: u32, timeout_secs: Option<u64>) -> Result<i32, String> {
    let timeout = timeout_secs.map(Duration::from_secs);
    match crate::os::wait_on_job(job_id, timeout) {
        Ok(result) => Ok(result.status),
        Err(e) => Err(e),
    }
}

/// List current background jobs as (id, command) pairs.
pub fn list_jobs() -> Vec<(u32, String)> {
    let jobs = crate::os::JOBS.lock().unwrap();
    let mut out = Vec::new();
    for (id, job_mutex) in jobs.iter() {
        let job = job_mutex.lock().unwrap();
        out.push((*id, job.command.clone()));
    }
    out
}
