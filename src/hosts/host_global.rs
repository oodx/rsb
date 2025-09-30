//! Host-level helpers that depend on Global store.
//!
//! This module provides functions that consume global context for host discovery.
//! Per MODULE_SPEC: hosts (consumer) depends on global (provider).

use crate::global;
use crate::hosts;

// === Global-Dependent System Information ===

/// Gets the current user's name from global context (USER variable).
pub fn get_user() -> String {
    global::get_var("USER")
}

/// Gets the current user's home directory from global context (HOME variable).
pub fn get_home() -> String {
    global::get_var("HOME")
}

/// Gets the current working directory from global context (PWD variable).
pub fn get_pwd() -> String {
    global::get_var("PWD")
}

// === Global Hydration ===

/// Hydrate Global from environment (env_bootstrap) and a list of config files.
/// Does not perform XDG/RSB path setup (pending host xdg implementation).
pub fn hydrate_env_and_configs(config_paths: &[&str]) {
    hosts::env_bootstrap();
    global::apply_config_files(config_paths);
}

/// Import only environment variables with a given prefix.
/// If `strip_prefix` is true, the stored key drops the prefix; otherwise it keeps it.
pub fn import_env_with_prefix(prefix: &str, strip_prefix: bool) {
    let up = prefix.to_string();
    for (k, v) in std::env::vars() {
        if k.starts_with(&up) {
            let key_to_store = if strip_prefix {
                k[up.len()..].to_string()
            } else {
                k.clone()
            };
            global::set_var(key_to_store, v);
        }
    }
}

// === Namespacing Helpers ===

/// Namespacing helpers delegate to core global namespace utilities.
pub fn ns_set(ns: &str, key: &str, value: &str) {
    global::ns_set(ns, key, value)
}
pub fn ns_get(ns: &str, key: &str) -> String {
    global::ns_get(ns, key)
}
pub fn ns_get_all(ns: &str) -> Vec<(String, String)> {
    global::ns_get_all(ns)
}
