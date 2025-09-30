//! Threads and Jobs Module (MODULE_SPEC aligned)
//!
//! Forwards to hosts::jobs for all functionality. This module exists for
//! backward compatibility and will eventually be deprecated.

pub mod macros;
pub mod utils;

// Forward all exports to hosts::jobs
pub use crate::hosts::jobs::*;
