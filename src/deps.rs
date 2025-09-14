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
pub use base64;
pub use chrono;
pub use glob;
pub use lazy_static;
pub use libc;
pub use rand;
pub use regex;
pub use serde;
pub use serde_json;
pub use urlencoding;
pub use uuid;
