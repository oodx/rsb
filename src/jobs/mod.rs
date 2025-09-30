//! Job Control Module (MODULE_SPEC aligned)
//!
//! Unified module for background shell jobs, process management, events,
//! signal handling, and locking. Consolidates functionality from old
//! threads/, process, signal, and events modules.

pub mod core;
pub mod macros;
pub mod process;
pub mod signal;
pub mod utils;

// Flatten exports
pub use core::*;
pub use process::*;
pub use signal::*;
pub use utils::*;
