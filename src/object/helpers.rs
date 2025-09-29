//! Internal helper implementations for Object module

use std::collections::HashMap;

/// Load all global variables with a specific namespace prefix
pub fn load_globals_with_prefix(namespace: &str) -> HashMap<String, String> {
    let prefix = format!("{}_", namespace);
    let all_vars = crate::global::get_all_vars();
    let mut result = HashMap::new();

    for (key, value) in all_vars {
        if key.starts_with(&prefix) {
            // Strip the namespace prefix from the key
            let stripped_key = key.strip_prefix(&prefix).unwrap_or(&key);
            result.insert(stripped_key.to_string(), value);
        }
    }

    result
}

/// Normalize keys by converting dots, dashes, and camelCase to snake_case
pub fn normalize_key(key: &str) -> String {
    // Replace dots and dashes with underscores
    let key = key.replace('.', "_").replace('-', "_");

    // Use string module's to_snake_case for proper CamelCase handling
    crate::string::to_snake_case(&key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_key() {
        assert_eq!(normalize_key("dot.notation"), "dot_notation");
        assert_eq!(normalize_key("kebab-case"), "kebab_case");
        assert_eq!(normalize_key("CamelCase"), "camel_case");  // Fixed: properly splits CamelCase
        assert_eq!(normalize_key("mixed.dot-dash"), "mixed_dot_dash");
    }

    #[test]
    fn test_load_globals_with_prefix() {
        // Set up some test global variables
        crate::global::set_var("test_key1", "value1");
        crate::global::set_var("test_key2", "value2");
        crate::global::set_var("other_key", "other_value");

        let result = load_globals_with_prefix("test");
        assert_eq!(result.get("key1"), Some(&"value1".to_string()));
        assert_eq!(result.get("key2"), Some(&"value2".to_string()));
        assert_eq!(result.get("other_key"), None);
    }
}