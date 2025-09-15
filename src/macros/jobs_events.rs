// --- Job Control Macros ---
// Namespaced re-exports for selective imports
pub use crate::{job, event, trap, hostname, user, home_dir, current_dir, pid_of, process_exists, kill_pid, kill_process, with_lock, lock, unlock};
// job!/event!/trap! moved to threads::macros
// Legacy hook aliases (removed to avoid macro name conflicts)

// --- System Info, Network, Process, Locking Macros ---
// Host info/path macros migrated to `hosts::macros` per MODULE_SPEC.
// Process/PID/Locking macros migrated to `os::macros`.

// curl!/get! moved to bash::macros
