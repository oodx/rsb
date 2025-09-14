//! Global adapter helpers: hydrate Global from env and/or config files.
//!
//! These are thin orchestrators that delegate to host/global modules.

/// Apply environment variables into the global store and set standard modes.
pub fn apply_env() {
    crate::hosts::import_environment();
    crate::hosts::setup_standard_modes();
}

/// Load one or more config files into the global store.
pub fn apply_config_files(paths: &[&str]) {
    for p in paths { crate::global::load_config_file(p); }
}

/// Hydrate the global store from env, then load config files in order.
pub fn hydrate_env_and_files(paths: &[&str]) {
    apply_env();
    apply_config_files(paths);
}

/// Simple import: mirror process environment into Global without modes/XDG/script.
pub fn import_env_simple() {
    for (k, v) in std::env::vars() {
        crate::global::set_var(&k, &v);
    }
}

/// Apply environment (simple) then return; no modes are set.
pub fn apply_env_simple() { import_env_simple(); }

/// Hydrate using simple env import (no modes), then optional config files.
pub fn hydrate_simple(paths: &[&str]) {
    apply_env_simple();
    apply_config_files(paths);
}
