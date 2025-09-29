//! Global key/value store — core interfaces.

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

// Renamed from Context → Global
pub struct Global {
    vars: HashMap<String, String>,
}

impl Global {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
    pub fn set<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.vars.insert(key.into(), value.into());
    }
    pub fn get(&self, key: &str) -> String {
        self.vars.get(key).cloned().unwrap_or_default()
    }
    pub fn has(&self, key: &str) -> bool {
        self.vars.contains_key(key)
    }
    pub fn expand(&self, text: &str) -> String {
        let mut result = text.to_string();
        let braced_re = Regex::new(r"\$\{([A-Za-z_][A-Za-z0-9_]*)\}").unwrap();
        result = braced_re
            .replace_all(&result, |caps: &regex::Captures| {
                let var_name = &caps[1];
                self.vars.get(var_name).cloned().unwrap_or_default()
            })
            .to_string();
        let simple_re = Regex::new(r"\$([A-Za-z_][A-Za-z0-9_]*)").unwrap();
        result = simple_re
            .replace_all(&result, |caps: &regex::Captures| {
                let var_name = &caps[1];
                self.vars.get(var_name).cloned().unwrap_or_default()
            })
            .to_string();
        result
    }
    pub fn get_all_vars(&self) -> HashMap<String, String> {
        self.vars.clone()
    }
}

lazy_static! {
    pub static ref GLOBAL: Arc<Mutex<Global>> = Arc::new(Mutex::new(Global::new()));
}

pub fn set_var<K: Into<String>, V: Into<String>>(key: K, value: V) {
    GLOBAL.lock().unwrap().set(key, value)
}
pub fn get_var(key: &str) -> String {
    GLOBAL.lock().unwrap().get(key)
}
pub fn has_var(key: &str) -> bool {
    GLOBAL.lock().unwrap().has(key)
}
pub fn unset_var(key: &str) {
    GLOBAL.lock().unwrap().vars.remove(key);
}
pub fn expand_vars(text: &str) -> String {
    GLOBAL.lock().unwrap().expand(text)
}
pub fn get_all_vars() -> HashMap<String, String> {
    GLOBAL.lock().unwrap().get_all_vars()
}

// Clear functionality with protected keys and RSB_GLOBAL_RESET requirement

/// Get the protected keys from various sources
fn get_protected_keys() -> HashSet<String> {
    let mut protected = HashSet::new();

    // Default protected keys
    protected.insert("PATH".to_string());
    protected.insert("HOME".to_string());
    protected.insert("USER".to_string());
    protected.insert("SHELL".to_string());
    protected.insert("RSB_HOME".to_string());
    protected.insert("RSB_CONFIG".to_string());

    // Add keys from RSB_PROTECTED_KEYS environment variable
    if let Ok(keys_str) = std::env::var("RSB_PROTECTED_KEYS") {
        for key in keys_str.split(',') {
            protected.insert(key.trim().to_string());
        }
    }

    // TODO: In future, could read from Cargo.toml [package.metadata.rsb.protected_keys]

    protected
}

/// Clear all global variables except protected keys
/// Requires RSB_GLOBAL_RESET=1 to be set
///
/// Note: Acquires the global lock once for the entire operation after
/// checking the RSB_GLOBAL_RESET flag (which acquires its own lock).
pub fn clear_all() -> Result<usize, String> {
    // Check for RSB_GLOBAL_RESET flag (acquires and releases lock)
    if get_var("RSB_GLOBAL_RESET") != "1" {
        return Err("RSB_GLOBAL_RESET must be set to 1 to clear globals".to_string());
    }

    let protected = get_protected_keys();
    let mut global = GLOBAL.lock().unwrap();
    let initial_count = global.vars.len();

    // Keep only protected keys
    global.vars.retain(|key, _| protected.contains(key));

    let removed_count = initial_count - global.vars.len();
    Ok(removed_count)
}

/// Clear global variables matching a prefix pattern
/// Requires RSB_GLOBAL_RESET=1 to be set
pub fn clear_prefix(prefix: &str) -> Result<usize, String> {
    // Check for RSB_GLOBAL_RESET flag
    if get_var("RSB_GLOBAL_RESET") != "1" {
        return Err("RSB_GLOBAL_RESET must be set to 1 to clear globals".to_string());
    }

    let protected = get_protected_keys();
    let mut global = GLOBAL.lock().unwrap();
    let initial_count = global.vars.len();

    // Remove keys matching prefix unless protected
    global.vars.retain(|key, _| {
        !key.starts_with(prefix) || protected.contains(key)
    });

    let removed_count = initial_count - global.vars.len();
    Ok(removed_count)
}

/// Clear global variables matching a suffix pattern
/// Requires RSB_GLOBAL_RESET=1 to be set
pub fn clear_suffix(suffix: &str) -> Result<usize, String> {
    // Check for RSB_GLOBAL_RESET flag
    if get_var("RSB_GLOBAL_RESET") != "1" {
        return Err("RSB_GLOBAL_RESET must be set to 1 to clear globals".to_string());
    }

    let protected = get_protected_keys();
    let mut global = GLOBAL.lock().unwrap();
    let initial_count = global.vars.len();

    // Remove keys matching suffix unless protected
    global.vars.retain(|key, _| {
        !key.ends_with(suffix) || protected.contains(key)
    });

    let removed_count = initial_count - global.vars.len();
    Ok(removed_count)
}

/// Clear global variables matching a regex pattern
/// Requires RSB_GLOBAL_RESET=1 to be set
pub fn clear_pattern(pattern: &str) -> Result<usize, String> {
    // Check for RSB_GLOBAL_RESET flag
    if get_var("RSB_GLOBAL_RESET") != "1" {
        return Err("RSB_GLOBAL_RESET must be set to 1 to clear globals".to_string());
    }

    let re = match Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => return Err(format!("Invalid regex pattern: {}", e)),
    };

    let protected = get_protected_keys();
    let mut global = GLOBAL.lock().unwrap();
    let initial_count = global.vars.len();

    // Remove keys matching pattern unless protected
    global.vars.retain(|key, _| {
        !re.is_match(key) || protected.contains(key)
    });

    let removed_count = initial_count - global.vars.len();
    Ok(removed_count)
}
