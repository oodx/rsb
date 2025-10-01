//! Global Array Operations
//!
//! Array storage and manipulation using the global key-value store.

use super::{get_var, has_var, set_var};

/// Push an item onto a global array
pub fn array_push(key: &str, item: &str) {
    let mut items = get_array(key);
    items.push(item.to_string());
    let item_strs: Vec<&str> = items.iter().map(|s| s.as_str()).collect();
    set_array(key, &item_strs);
}

/// Get an item from a global array by index
pub fn array_get(key: &str, index: usize) -> String {
    get_array(key).get(index).cloned().unwrap_or_default()
}

/// Get the length of a global array
pub fn array_length(key: &str) -> usize {
    get_array(key).len()
}

/// Check if a global array contains an item
pub fn array_contains(key: &str, item: &str) -> bool {
    get_array(key).contains(&item.to_string())
}

/// Get all items from a global array
pub fn get_array(key: &str) -> Vec<String> {
    let length_key = format!("{}_LENGTH", key);
    if !has_var(&length_key) {
        let value = get_var(key);
        if value.is_empty() {
            return Vec::new();
        }
        return value.split_whitespace().map(|s| s.to_string()).collect();
    }
    let length: usize = get_var(&length_key).parse().unwrap_or(0);
    let mut items = Vec::new();
    for i in 0..length {
        let item_key = format!("{}_{}", key, i);
        if has_var(&item_key) {
            items.push(get_var(&item_key));
        }
    }
    items
}

/// Set a global array from a slice of strings
pub fn set_array(key: &str, items: &[&str]) {
    set_var(&format!("{}_LENGTH", key), &items.len().to_string());
    for (i, item) in items.iter().enumerate() {
        set_var(&format!("{}_{}", key, i), *item);
    }
    set_var(key, &items.join(" "));
}
