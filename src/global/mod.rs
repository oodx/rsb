//! Global state module (core-only)
//!
//! Curated surface for global key/value store and simple helpers.
//! This module intentionally excludes env/opts/host discovery and CLI
//! orchestration — those belong to `rsb::host` and `rsb::cli`.

mod store;
pub use store::*;

mod utils;
pub use utils::*;

mod config;
pub use config::*;

mod adapter;
pub use adapter::*;

mod ns;
pub use ns::*;
pub mod registry;
pub use registry::*;

// No module-owned macros at the moment; functions are the primary interface.
