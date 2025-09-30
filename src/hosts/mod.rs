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

// Host-level helpers that depend on global (cross-module)
pub mod host_global;

// Operating system information (pure, no cross-module deps)
pub mod os;

// Command execution (depends on global for expand_vars)
pub mod command;

// Job control (process mgmt, events, signals, background jobs)
pub mod jobs;

// Include macros module
pub mod macros;

// Implementation modules (kept private)
mod bootstrap;
mod env;
mod host_path;
mod rsb_path;
mod xdg_path;
