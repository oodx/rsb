//! Threads and Jobs Module (MODULE_SPEC aligned)
//!
//! Provides thread utilities (sleep, benchmark) and job control helpers
//! built on top of the crate's OS execution primitives. The internal
//! implementation details (handles, registries) remain in `os` for now.
//!
//! Public surface is curated via `threads::utils` and re-exported at
//! the module root for convenience.

pub mod utils;
pub use utils::*;
// Module-owned macros for jobs/events/traps
pub mod macros;
