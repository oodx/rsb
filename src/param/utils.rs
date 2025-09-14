//! Curated low-level helpers for param expansions.
//!
//! This module provides a stable surface for consumers who want direct access
//! to the underlying operations that power `param!`. Prefer using `param!` for
//! bash-like DSL ergonomics; use `param::utils` when you need fine-grained
//! control in code.
//!
//! See docs/development/MODULE_SPECIFICATION.md for module exposure guidelines.

pub use crate::param::basic::{
    get,
    sub as sub_abs,
    sub_rel,
    prefix,
    suffix,
    replace,
    upper,
    lower,
    len,
};

