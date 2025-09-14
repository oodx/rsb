// Job Control (from macros/jobs_events.rs)

// MACRO TO MOVE FROM macros/jobs_events.rs:

// job!(command) -> JobHandle
// - Launch background job/process
// - Returns handle for job control
// - Example: job!("long_running_script.sh") -> JobHandle

// job!(name, command) -> JobHandle
// - Named background job with identifier
// - Allows job tracking and management
// - Example: job!("backup", "rsync -av /data/ /backup/")

// job_wait!(handle) -> ExitStatus
// - Wait for job completion
// - Returns exit status of background job
// - Example: job_wait!(handle) -> ExitStatus::success()

// job_kill!(handle) -> ()
// - Terminate background job
// - Sends SIGTERM to job process
// - Example: job_kill!(backup_job)