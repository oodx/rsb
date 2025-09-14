//! Bash Commands Module (MODULE_SPEC aligned)
//!
//! Curated wrappers around common bash/system commands (curl, tar, zip, etc.).
//! Provides a stable, string-first surface and defers to `os` for low-level
//! command execution and result handling.

pub mod utils;
pub use utils::*;

// Expose bash-related macros via this module for discoverability
pub mod macros;
