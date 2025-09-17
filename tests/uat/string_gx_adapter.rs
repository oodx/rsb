// String + GX (Generators) Cross-Module Integration Tests
// Tests string transformations with random generation utilities

use rsb::prelude::*;

#[test]
fn uat_string_gx_random_case_transforms() {
    // Generate random strings and test case transformations
    let random_alnum = rsb::gx::string::get_rand_alnum(10);
    assert_eq!(random_alnum.len(), 10);

    // Test case transformations on generated strings
    let snake = rsb::string::to_snake_case(&random_alnum);
    let kebab = rsb::string::to_kebab_case(&random_alnum);
    let dot = rsb::string::to_dot_case(&random_alnum);

    // All should be valid transformations (no panics)
    assert!(!snake.is_empty());
    assert!(!kebab.is_empty());
    assert!(!dot.is_empty());

    // Test macro forms with generated content
    let snake_macro = snake!(random_alnum.as_str());
    let kebab_macro = kebab!(random_alnum.as_str());

    assert_eq!(snake, snake_macro);
    assert_eq!(kebab, kebab_macro);
}

#[test]
fn uat_string_gx_uuid_formatting() {
    // Generate UUID and test string formatting
    let uuid = rsb::gx::id::get_rand_uuid();
    assert!(!uuid.is_empty());

    // Test string operations on UUID
    let uuid_upper = rsb::string::str_upper(&uuid, true);
    let uuid_lower = rsb::string::str_lower(&uuid, true);

    // UUIDs should be properly formatted
    assert!(uuid.contains('-'));
    assert_eq!(uuid_upper.to_lowercase(), uuid_lower);

    // Test prefix/suffix operations
    let prefix_test = "uuid_";
    let prefixed = format!("{}{}", prefix_test, uuid);
    let stripped = rsb::string::str_prefix(&prefixed, prefix_test, false);
    assert_eq!(stripped, uuid);
}

#[test]
fn uat_string_gx_dict_processing() {
    // Test string processing with dictionary data
    let test_words = vec!["hello".to_string(), "world".to_string(), "test".to_string()];

    if let Some(random_word) = rsb::gx::collection::get_rand_from_slice(&test_words) {
        // Test case transformations on dictionary words
        let snake = rsb::string::to_snake_case(&random_word);
        let kebab = rsb::string::to_kebab_case(&random_word);

        // Should not be empty and should be valid transformations
        assert!(!snake.is_empty());
        assert!(!kebab.is_empty());

        // Test string utilities on dictionary content
        let quoted = rsb::string::utils::shell_single_quote(&random_word);
        assert!(quoted.starts_with('\''));
        assert!(quoted.ends_with('\''));
    }
}

#[test]
fn uat_string_gx_hex_processing() {
    // Generate hex strings and test ASCII processing
    let hex_string = rsb::gx::string::get_rand_hex(16);
    assert_eq!(hex_string.len(), 16);

    // Hex should be ASCII-safe already
    let ascii_filtered = rsb::string::utils::filter_ascii_strip(&hex_string);
    assert_eq!(ascii_filtered, hex_string); // Should be identical

    // Test case operations on hex
    let hex_upper = rsb::string::str_upper(&hex_string, true);
    let hex_lower = rsb::string::str_lower(&hex_string, true);

    // Hex strings should transform properly
    assert_ne!(hex_upper, hex_lower);
    assert_eq!(hex_upper.to_lowercase(), hex_lower);

    // Test substring operations on hex
    let sub = rsb::string::str_sub(&hex_string, 0, Some(8));
    assert_eq!(sub.len(), 8);
}