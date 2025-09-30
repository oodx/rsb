//! Bash Commands Module (MODULE_SPEC aligned)
//!
//! Curated wrappers around common bash/system commands (curl, tar, zip, jq, etc.).
//! Provides a stable, string-first surface and defers to hosts for low-level
//! command execution and result handling.

pub mod utils;
pub use utils::*;

// JQ JSON utilities
pub mod jq;

// Expose bash-related macros via this module for discoverability
pub mod macros;
