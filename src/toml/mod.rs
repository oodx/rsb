//! TOML Snooping - Extract custom metadata from Cargo.toml into global store
//!
//! This module provides functionality to extract configuration sections from
//! Cargo.toml's `[package.metadata.*]` sections and store them as global variables
//! with namespace prefixes. Supports rsb, hub, and inf namespaces by default.
//!
//! # Features
//! - Extracts [package.metadata.rsb/hub/inf] sections from Cargo.toml
//! - Converts keys to snake_case automatically
//! - Handles arrays using RSB convention (LENGTH + indexed storage)
//! - Integrates with Object<T> system via global store
//! - Lazy initialization with static SNOOPER instance
//!
//! # Usage
//! ```ignore
//! use rsb::toml::enable_toml_snooping;
//!
//! // Enable TOML snooping with default namespaces (rsb, hub, inf)
//! enable_toml_snooping();
//!
//! // Access snooped values via global store
//! let hub_url = rsb::global::get_var("hub_api_url");
//! ```

// Module orchestration - implementation in snooper.rs
mod snooper;

// Re-export public types
pub use snooper::TomlSnooper;

// Public API
use lazy_static::lazy_static;
use std::sync::Mutex;

// Global static SNOOPER instance
lazy_static! {
    static ref SNOOPER: Mutex<TomlSnooper> = Mutex::new(TomlSnooper::new());
}

/// Enable TOML snooping with default namespaces (rsb, hub, inf)
///
/// This function should typically be called during bootstrap initialization.
/// It extracts configuration from `[package.metadata.*]` sections in Cargo.toml
/// and stores them as global variables with namespace prefixes.
///
/// # Example
/// ```ignore
/// use rsb::toml::enable_toml_snooping;
///
/// enable_toml_snooping();
///
/// // Access via global store
/// let url = rsb::global::get_var("hub_api_url");
/// ```
pub fn enable_toml_snooping() {
    if let Ok(mut snooper) = SNOOPER.lock() {
        snooper.enable();
    }
}

/// Add a custom namespace to snoop from Cargo.toml
///
/// This must be called before `enable_toml_snooping()` to take effect.
///
/// # Example
/// ```ignore
/// use rsb::toml::{snoop_namespace, enable_toml_snooping};
///
/// snoop_namespace("custom");
/// enable_toml_snooping();
///
/// // Now custom_* variables will be available
/// let value = rsb::global::get_var("custom_my_setting");
/// ```
pub fn snoop_namespace(namespace: &str) {
    if let Ok(mut snooper) = SNOOPER.lock() {
        snooper.add_namespace(namespace);
    }
}

/// Check if TOML snooping is enabled
pub fn is_enabled() -> bool {
    SNOOPER
        .lock()
        .map(|s| s.enabled)
        .unwrap_or(false)
}

/// Check if a namespace is being snooped
pub fn has_namespace(namespace: &str) -> bool {
    SNOOPER
        .lock()
        .map(|s| s.namespaces.contains(&namespace.to_string()))
        .unwrap_or(false)
}