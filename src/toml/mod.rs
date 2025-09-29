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

use lazy_static::lazy_static;
use std::path::PathBuf;
use std::sync::Mutex;
use toml::Value;

use crate::global::set_var;
use crate::string::to_snake_case;

/// TOML Snooper for extracting metadata sections from Cargo.toml
///
/// Note: This struct's fields are private. Use the public API functions
/// (enable_toml_snooping, snoop_namespace, is_enabled, has_namespace)
/// to interact with the global SNOOPER instance.
#[derive(Debug, Clone)]
pub struct TomlSnooper {
    pub(crate) enabled: bool,
    pub(crate) namespaces: Vec<String>,
}

impl TomlSnooper {
    /// Create a new TomlSnooper with default namespaces
    pub fn new() -> Self {
        Self {
            enabled: false,
            namespaces: vec!["hub".into(), "inf".into(), "rsb".into()],
        }
    }

    /// Enable snooping and extract metadata from Cargo.toml
    pub fn enable(&mut self) {
        self.enabled = true;
        self.snoop_cargo_toml();
    }

    /// Add a custom namespace to snoop
    pub fn add_namespace(&mut self, namespace: &str) {
        if !self.namespaces.contains(&namespace.to_string()) {
            self.namespaces.push(namespace.to_string());
        }
    }

    /// Main snooping logic - find and parse Cargo.toml
    fn snoop_cargo_toml(&self) {
        match find_cargo_toml() {
            Ok(cargo_path) => {
                if let Ok(content) = std::fs::read_to_string(&cargo_path) {
                    if let Ok(toml) = content.parse::<Value>() {
                        self.extract_metadata(&toml);
                    }
                }
            }
            Err(_) => {
                // Cargo.toml not found - this is not an error condition
                // (might be running in a context without Cargo.toml)
            }
        }
    }

    /// Extract metadata sections from parsed TOML
    fn extract_metadata(&self, toml: &Value) {
        for namespace in &self.namespaces {
            if let Some(metadata) = toml
                .get("package")
                .and_then(|p| p.get("metadata"))
                .and_then(|m| m.get(namespace.as_str()))
            {
                self.store_namespace_values(namespace, metadata);
            }
        }
    }

    /// Store namespace values in global store with snake_case conversion
    fn store_namespace_values(&self, namespace: &str, values: &Value) {
        if let Value::Table(table) = values {
            for (key, value) in table {
                // Convert key to snake_case
                let snake_key = to_snake_case(key);
                let global_key = format!("{}_{}", namespace, snake_key);

                // Store based on value type
                match value {
                    Value::String(s) => set_var(&global_key, s),
                    Value::Integer(i) => set_var(&global_key, &i.to_string()),
                    Value::Boolean(b) => set_var(&global_key, if *b { "true" } else { "false" }),
                    Value::Float(f) => set_var(&global_key, &f.to_string()),
                    Value::Array(arr) => {
                        // Store array using RSB convention: LENGTH + indexed
                        set_var(&format!("{}_LENGTH", global_key), &arr.len().to_string());
                        for (i, item) in arr.iter().enumerate() {
                            let item_value = match item {
                                Value::String(s) => s.clone(),
                                Value::Integer(i) => i.to_string(),
                                Value::Boolean(b) => {
                                    if *b {
                                        "true".to_string()
                                    } else {
                                        "false".to_string()
                                    }
                                }
                                Value::Float(f) => f.to_string(),
                                _ => continue, // Skip complex nested types
                            };
                            set_var(&format!("{}_{}", global_key, i), &item_value);
                        }
                    }
                    _ => {
                        // Skip complex types (nested tables, datetime, etc.)
                    }
                }
            }
        }
    }
}

impl Default for TomlSnooper {
    fn default() -> Self {
        Self::new()
    }
}

/// Find Cargo.toml by walking up directory tree
fn find_cargo_toml() -> Result<PathBuf, std::io::Error> {
    let mut path = std::env::current_dir()?;
    loop {
        let cargo_path = path.join("Cargo.toml");
        if cargo_path.exists() {
            return Ok(cargo_path);
        }
        if !path.pop() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Cargo.toml not found",
            ));
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snooper_initialization() {
        let snooper = TomlSnooper::new();
        assert!(!snooper.enabled);
        assert_eq!(snooper.namespaces.len(), 3);
        assert!(snooper.namespaces.contains(&"hub".to_string()));
        assert!(snooper.namespaces.contains(&"inf".to_string()));
        assert!(snooper.namespaces.contains(&"rsb".to_string()));
    }

    #[test]
    fn test_add_namespace() {
        let mut snooper = TomlSnooper::new();
        snooper.add_namespace("custom");
        assert_eq!(snooper.namespaces.len(), 4);
        assert!(snooper.namespaces.contains(&"custom".to_string()));
    }

    #[test]
    fn test_add_namespace_duplicate() {
        let mut snooper = TomlSnooper::new();
        snooper.add_namespace("hub");
        assert_eq!(snooper.namespaces.len(), 3); // Should not add duplicate
    }
}