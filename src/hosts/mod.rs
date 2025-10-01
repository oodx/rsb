#![allow(unused_imports)]
//! Host Environment Discovery Module
//!
//! Clean orchestrator pattern — host discovers environment, global stores it,
//! CLI builds interfaces. MODULE_SPEC alignment: keep implementation modules
//! private, expose curated surface at module root.
//!
//! RSB Philosophy: host discovers → global stores → cli builds

// Host-level helpers that depend on global (cross-module)
pub mod host_global;

// Operating system information (pure, no cross-module deps)
pub mod os;

// Command execution (depends on global for expand_vars)
pub mod command;

// Include macros module
pub mod macros;

// Implementation modules (kept private)
mod bootstrap;
mod env;
mod host_path;
mod rsb_path;
mod xdg_path;

// Curated surface - re-export stable host helpers
pub use bootstrap::*;
pub use command::*;
pub use env::*;
pub use host_global::*;
pub use host_path::*;
pub use os::*;
pub use rsb_path::*;
pub use xdg_path::*;
