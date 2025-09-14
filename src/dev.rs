//! RSB Dev Surface — low-level utilities for development/testing
//!
//! This module aggregates curated low-level helpers from various modules under
//! a single `rsb::dev` namespace to speed up prototyping and testing.
//!
//! IMPORTANT: This surface is for dev/test convenience. Stability is not
//! guaranteed; prefer the normal module surfaces for production code.
//!
//! Current inclusions (stream items intentionally deferred until stream reorg):
//! - `string::utils` (helpers, case, error, safety registry)
//! - `param::utils` (curated param helpers)

pub mod string {
    pub use crate::string::utils::*;
}

pub mod param {
    pub use crate::param::utils::*;
}

