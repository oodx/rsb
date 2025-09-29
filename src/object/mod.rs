//! Object module - Flexible string-based configuration containers
//!
//! Provides JavaScript-like Object with phantom type parameters for shape hinting.
//! All values are strings following RSB's string-biased philosophy.

use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Index;

mod helpers;
mod utils;
pub mod macros;

// Re-export public utilities
pub use utils::*;

/// Generic Object with phantom type T for shape hinting
pub struct Object<T = ()> {
    inner: HashMap<String, String>,
    namespace: String,
    _phantom: PhantomData<T>,
}

impl<T> Object<T> {
    /// Create a new empty Object with the given namespace
    pub fn new(namespace: impl Into<String>) -> Self {
        Self {
            inner: HashMap::new(),
            namespace: namespace.into(),
            _phantom: PhantomData,
        }
    }

    /// Create an Object from global variables with the given namespace prefix
    /// For example, if namespace is "hub", it will load all variables starting with "hub_"
    pub fn from_global(namespace: impl Into<String>) -> Self {
        let namespace = namespace.into();
        let inner = helpers::load_globals_with_prefix(&namespace);

        Self {
            inner,
            namespace,
            _phantom: PhantomData,
        }
    }

    /// Get a value by key, returning empty string if not found
    pub fn get(&self, key: &str) -> &str {
        let normalized_key = helpers::normalize_key(key);
        self.inner.get(&normalized_key).map(|s| s.as_str()).unwrap_or("")
    }

    /// Get a value by key with a default fallback
    pub fn get_or<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        let normalized_key = helpers::normalize_key(key);
        self.inner.get(&normalized_key).map(|s| s.as_str()).unwrap_or(default)
    }

    /// Check if a key exists
    pub fn has(&self, key: &str) -> bool {
        let normalized_key = helpers::normalize_key(key);
        self.inner.contains_key(&normalized_key)
    }

    /// Set a value
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let normalized_key = helpers::normalize_key(&key.into());
        self.inner.insert(normalized_key, value.into());
    }

    /// Get the underlying HashMap
    pub fn as_map(&self) -> &HashMap<String, String> {
        &self.inner
    }

    /// Get all keys
    pub fn keys(&self) -> Vec<&String> {
        self.inner.keys().collect()
    }

    /// Get the namespace
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Write all values back to global with namespace prefix
    pub fn sync_to_global(&self) {
        for (key, value) in &self.inner {
            crate::global::set_var(&format!("{}_{}", self.namespace, key), value);
        }
    }

    /// Change the phantom type for documentation purposes
    pub fn as_type<U>(self) -> Object<U> {
        Object {
            inner: self.inner,
            namespace: self.namespace,
            _phantom: PhantomData,
        }
    }
}

// Implement Index trait for bracket notation
impl<T> Index<&str> for Object<T> {
    type Output = str;

    fn index(&self, key: &str) -> &Self::Output {
        self.get(key)
    }
}

// Implement Debug for Object
impl<T> std::fmt::Debug for Object<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Object")
            .field("namespace", &self.namespace)
            .field("inner", &self.inner)
            .finish()
    }
}

// Implement Clone for Object
impl<T> Clone for Object<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            namespace: self.namespace.clone(),
            _phantom: PhantomData,
        }
    }
}

// Marker types for documentation
pub struct HubShape;
pub struct InfShape;
pub struct RsbShape;

// Type aliases for clarity
pub type AnyObject = Object<()>;
pub type HubConfig = Object<HubShape>;
pub type InfConfig = Object<InfShape>;
pub type RsbConfig = Object<RsbShape>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_creation() {
        let mut obj = Object::<()>::new("test");
        obj.set("key", "value");
        assert_eq!(&obj["key"], "value");
        assert_eq!(obj.get("key"), "value");
    }

    #[test]
    fn test_get_or_default() {
        let obj = Object::<()>::new("test");
        assert_eq!(obj.get_or("missing", "default"), "default");
        assert_eq!(obj.get("missing"), "");
    }

    #[test]
    fn test_has_key() {
        let mut obj = Object::<()>::new("test");
        obj.set("exists", "yes");
        assert!(obj.has("exists"));
        assert!(!obj.has("missing"));
    }

    #[test]
    fn test_type_conversion() {
        let mut generic: Object = Object::new("test");
        generic.set("data", "value");

        let typed: Object<HubShape> = generic.as_type();
        assert_eq!(&typed["data"], "value");
    }

    #[test]
    fn test_key_normalization() {
        let mut obj = Object::<()>::new("test");
        obj.set("dot.notation", "value1");
        obj.set("kebab-case", "value2");
        obj.set("CamelCase", "value3");

        // Keys are normalized to snake_case and lowercase
        assert_eq!(&obj["dot_notation"], "value1");
        assert_eq!(&obj["kebab_case"], "value2");
        assert_eq!(&obj["camelcase"], "value3");
    }

    #[test]
    fn test_namespace() {
        let obj = Object::<()>::new("myapp");
        assert_eq!(obj.namespace(), "myapp");
    }

    #[test]
    fn test_keys_listing() {
        let mut obj = Object::<()>::new("test");
        obj.set("key1", "value1");
        obj.set("key2", "value2");

        let keys = obj.keys();
        assert_eq!(keys.len(), 2);
    }

    #[test]
    fn test_from_global() {
        // Set up some global variables
        crate::global::set_var("test_key1", "value1");
        crate::global::set_var("test_key2", "value2");

        let obj = Object::<()>::from_global("test");
        assert_eq!(obj.get("key1"), "value1");
        assert_eq!(obj.get("key2"), "value2");
    }

    #[test]
    fn test_sync_to_global() {
        let mut obj = Object::<()>::new("sync_test");
        obj.set("key", "value");
        obj.sync_to_global();

        // This would set sync_test_key="value" in global store
        assert_eq!(crate::global::get_var("sync_test_key"), "value");
    }
}