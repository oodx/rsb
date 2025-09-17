// --- Job Control Macros ---
// Namespaced re-exports for selective imports
pub use crate::{
    current_dir, event, home_dir, hostname, job, kill_pid, kill_process, lock, pid_of,
    process_exists, trap, unlock, user, with_lock,
};
// job!/event!/trap! moved to threads::macros
// Legacy hook aliases (removed to avoid macro name conflicts)

// --- System Info, Network, Process, Locking Macros ---
// Host info/path macros migrated to `hosts::macros` per MODULE_SPEC.
// Process/PID/Locking macros migrated to `os::macros`.

// curl!/get! moved to bash::macros
