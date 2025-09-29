//! Curated low-level helpers for Object module

use super::{Object, HubShape, InfShape, RsbShape};

/// Create an Object from global variables with the given namespace prefix
pub fn get_object<T>(namespace: &str) -> Object<T> {
    Object::from_global(namespace)
}

/// Get the hub configuration object
pub fn get_hub() -> Object<HubShape> {
    Object::from_global("hub")
}

/// Get the inf configuration object
pub fn get_inf() -> Object<InfShape> {
    Object::from_global("inf")
}

/// Get the rsb configuration object
pub fn get_rsb() -> Object<RsbShape> {
    Object::from_global("rsb")
}