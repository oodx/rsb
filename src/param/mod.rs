//! Param module (progressive enhancement)
//!
//! The `param!` macro is exported at crate root (see `src/param/macros.rs`).
//! This module organizes non-macro helpers with a progressive enhancement pattern:
//! - `basic`: core transforms used by `param!` (substring, prefix/suffix strip, replace, case ops).
//! - `advanced`: reserved for future, more complex expansions (e.g., pattern-aware, tracing).
//!
//! Callers should not depend on these helpers directly unless building custom behavior.
//! They exist to keep `param!` implementation clean and to allow staged evolution.

pub mod advanced;
pub mod basic;
pub mod macros;

// Curated low-level surface for consumers
pub mod utils;
