#![allow(unused_imports)]
//! Host Environment Discovery Module
//!
//! Clean orchestrator pattern — host discovers environment, global stores it,
//! CLI builds interfaces. MODULE_SPEC alignment: keep implementation modules
//! private, expose curated surface via `hosts::utils` and at module root.
//!
//! RSB Philosophy: host discovers → global stores → cli builds

// Public curated surface
pub mod utils;
pub use utils::*;

// Host-level hydration + namespacing helpers
pub mod global;

// Include macros module
pub mod macros;

// Implementation modules (kept private)
mod env;
mod paths;
mod xdg_path;
mod rsb_path;
mod host_path;
mod virt_path;
mod shell;
mod system;
pub use system::*;
mod bootstrap;
