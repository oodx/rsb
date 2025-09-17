//! Global key/value store — core interfaces.

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
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
