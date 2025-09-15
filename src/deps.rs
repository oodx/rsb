//! External dependencies re-exported for consumers of `rsb`.
//!
//! This module exposes commonly used third‑party crates so downstream
//! projects can depend on `rsb` alone and import these dependencies
//! from a single place without repeating them in their own `Cargo.toml`.
//!
//! Example:
//! ```rust
//! use rsb::deps::rand::{Rng, distr::Alphanumeric};
//! use rsb::deps::lazy_static::lazy_static;
//! ```

// Re-export selected external crates used internally by rsb
// Gated by the `deps` feature to keep the default surface lean.
#[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-base64"))] pub use base64;
#[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-chrono"))] pub use chrono;
#[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-glob"))] pub use glob;
#[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-lazy_static"))] pub use lazy_static;
#[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-libc"))] pub use libc;
#[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-rand"))] pub use rand;
#[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-regex"))] pub use regex;
#[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-serde"))] pub use serde;
#[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-serde_json"))] pub use serde_json;
#[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-urlencoding"))] pub use urlencoding;
#[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-uuid"))] pub use uuid;

/// Optional convenience prelude: `use rsb::deps::prelude::*;` to bring enabled crates into scope.
pub mod prelude {
    #[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-base64"))] pub use super::base64;
    #[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-chrono"))] pub use super::chrono;
    #[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-glob"))] pub use super::glob;
    #[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-lazy_static"))] pub use super::lazy_static;
    #[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-libc"))] pub use super::libc;
    #[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-rand"))] pub use super::rand;
    #[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-regex"))] pub use super::regex;
    #[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-serde"))] pub use super::serde;
    #[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-serde_json"))] pub use super::serde_json;
    #[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-urlencoding"))] pub use super::urlencoding;
    #[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-uuid"))] pub use super::uuid;
}
