// --- Job Control Macros ---
// Namespaced re-exports for selective imports
pub use crate::{job, event, trap, hostname, user, home_dir, current_dir, pid_of, process_exists, kill_pid, kill_process, with_lock, lock, unlock};
// job!/event!/trap! moved to threads::macros
// Legacy hook aliases (removed to avoid macro name conflicts)

// --- System Info, Network, Process, Locking Macros ---
// Host info/path macros migrated to `hosts::macros` per MODULE_SPEC.

// curl!/get! moved to bash::macros

#[macro_export]
macro_rules! pid_of { ($process:expr) => { $crate::os::pid_of($process) }; }
#[macro_export]
macro_rules! process_exists { ($process:expr) => { $crate::os::process_exists($process) }; }

#[macro_export]
macro_rules! kill_pid {
    ($pid:expr) => {{
        match $crate::os::kill_pid($pid, None) {
            result if result.status == 0 => { $crate::okay!("Process {} terminated", $pid); },
            result => { $crate::error!("Failed to kill process {}: {}", $pid, result.error); std::process::exit(result.status); }
        }
    }};
    ($pid:expr, signal: $sig:expr) => {{
        match $crate::os::kill_pid($pid, Some($sig)) {
            result if result.status == 0 => { $crate::okay!("Process {} terminated with {}", $pid, $sig); },
            result => { $crate::error!("Failed to kill process {}: {}", $pid, result.error); std::process::exit(result.status); }
        }
    }};
}

#[macro_export]
macro_rules! kill_process {
    ($process:expr) => {{
        match $crate::os::kill_process($process, None) {
            result if result.status == 0 => { $crate::okay!("Killed all {} processes", $process); },
            result => { $crate::error!("Failed to kill {}: {}", $process, result.error); std::process::exit(result.status); }
        }
    }};
    ($process:expr, signal: $sig:expr) => {{
        match $crate::os::kill_process($process, Some($sig)) {
            result if result.status == 0 => { $crate::okay!("Killed all {} processes with {}", $process, $sig); },
            result => { $crate::error!("Failed to kill {}: {}", $process, result.error); std::process::exit(result.status); }
        }
    }};
}

// --- Locking Macros ---
#[macro_export]
macro_rules! with_lock {
    ($lock_path:expr => $body:block) => {{
        match $crate::os::create_lock($lock_path) {
            Ok(_) => { let result = $body; $crate::os::remove_lock($lock_path); result },
            Err(e) => { $crate::error!("Failed to acquire lock: {}", e); std::process::exit(1); }
        }
    }};
}

#[macro_export]
macro_rules! lock {
    ($lock_path:expr) => {{
        match $crate::os::create_lock($lock_path) {
            Ok(_) => { $crate::okay!("Lock acquired: {}", $lock_path); },
            Err(e) => { $crate::error!("Failed to acquire lock: {}", e); std::process::exit(1); }
        }
    }};
}

#[macro_export]
macro_rules! unlock { ($lock_path:expr) => { $crate::os::remove_lock($lock_path); $crate::okay!("Lock released: {}", $lock_path); }; }
