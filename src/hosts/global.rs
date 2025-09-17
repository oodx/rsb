//! Host-level helpers that hydrate the Global store and provide optional namespacing.
//!
//! This module composes host env discovery with global adapters.

/// Hydrate Global from environment (env_bootstrap) and a list of config files.
/// Does not perform XDG/RSB path setup (pending host xdg implementation).
pub fn hydrate_env_and_configs(config_paths: &[&str]) {
    crate::hosts::env_bootstrap();
    crate::global::apply_config_files(config_paths);
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
            crate::global::set_var(key_to_store, v);
        }
    }
}

/// Namespacing helpers delegate to core global namespace utilities.
pub fn ns_set(ns: &str, key: &str, value: &str) {
    crate::global::ns_set(ns, key, value)
}
pub fn ns_get(ns: &str, key: &str) -> String {
    crate::global::ns_get(ns, key)
}
pub fn ns_get_all(ns: &str) -> Vec<(String, String)> {
    crate::global::ns_get_all(ns)
}
