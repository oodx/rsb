//! RSB Dev Prelude — curated low-level helpers for development/testing
//!
//! Aggregates selected, low-level helpers under `rsb::prelude_dev` to speed up
//! prototyping and test authoring. Prefer `rsb::prelude::*` for production code,
//! or `rsb::prelude_ez::*` for a broader convenience surface.
//!
//! Stability is not guaranteed. Surfaces may be adjusted as modules evolve.

pub mod string {
    pub use crate::string::utils::*;
}

pub mod param {
    pub use crate::param::utils::*;
}
