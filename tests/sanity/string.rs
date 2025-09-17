// RSB Sanity Tests - String Module Core Functionality Verification
// Tests verify the string module functions and macros work as documented in FEATURES_STRINGS

use rsb::prelude::*;

#[test]
fn test_basic_string_operations() {
    // Test core string functions from rsb::string module

    // Test substring operations (Unicode-safe)
    let test_str = "Hello, World!";
    let sub1 = rsb::string::str_sub(test_str, 0, Some(5));
    assert_eq!(sub1, "Hello");

    let sub2 = rsb::string::str_sub(test_str, 7, Some(5));
    assert_eq!(sub2, "World");

    let sub3 = rsb::string::str_sub(test_str, 7, None);
    assert_eq!(sub3, "World!");

    // Test Unicode safety with multi-byte characters
    let unicode_str = "Hello, 世界!";
    let unicode_sub = rsb::string::str_sub(unicode_str, 7, Some(2));
    assert_eq!(unicode_sub, "世界");
}

#[test]
fn test_prefix_suffix_operations() {
    // Test prefix and suffix removal
    let test_str = "prefix_content_suffix";

    // Test prefix removal
    let no_prefix = rsb::string::str_prefix(test_str, "prefix_", false);
    assert_eq!(no_prefix, "content_suffix");

    // Test suffix removal
    let no_suffix = rsb::string::str_suffix(test_str, "_suffix", false);
    assert_eq!(no_suffix, "prefix_content");

    // Test no match (should return original)
    let no_match_prefix = rsb::string::str_prefix(test_str, "nomatch", false);
    assert_eq!(no_match_prefix, test_str);

    let no_match_suffix = rsb::string::str_suffix(test_str, "nomatch", false);
    assert_eq!(no_match_suffix, test_str);
}

#[test]
fn test_string_replacement() {
    // Test string replacement functionality
    let test_str = "Hello World Hello";

    // Test replace first occurrence
    let replace_first = rsb::string::str_replace(test_str, "Hello", "Hi", false);
    assert_eq!(replace_first, "Hi World Hello");

    // Test replace all occurrences
    let replace_all = rsb::string::str_replace(test_str, "Hello", "Hi", true);
    assert_eq!(replace_all, "Hi World Hi");

    // Test no match (should return original)
    let no_match = rsb::string::str_replace(test_str, "xyz", "abc", false);
    assert_eq!(no_match, test_str);
}

#[test]
fn test_case_operations() {
    // Test case conversion helpers
    let test_str = "hello world";

    // Test upper case operations
    let upper_first = rsb::string::str_upper(test_str, false);
    assert!(upper_first.starts_with('H'));
    assert_eq!(upper_first, "Hello world");

    let upper_all = rsb::string::str_upper(test_str, true);
    assert_eq!(upper_all, "HELLO WORLD");

    // Test lower case operations
    let mixed_str = "HELLO WORLD";
    let lower_first = rsb::string::str_lower(mixed_str, false);
    assert!(lower_first.starts_with('h'));
    assert_eq!(lower_first, "hELLO WORLD");

    let lower_all = rsb::string::str_lower(mixed_str, true);
    assert_eq!(lower_all, "hello world");
}

#[test]
fn test_case_transformations() {
    // Test case transformation functions (re-exported from string::case)
    let test_input = "HelloWorldExample";

    // Test various case transformations
    let snake = rsb::string::to_snake_case(test_input);
    assert_eq!(snake, "hello_world_example");

    let kebab = rsb::string::to_kebab_case(test_input);
    assert_eq!(kebab, "hello-world-example");

    let dot = rsb::string::to_dot_case(test_input);
    assert_eq!(dot, "hello.world.example");

    let space = rsb::string::to_space_case(test_input);
    assert_eq!(space, "hello world example");

    // Test camel case conversion from snake_case
    let snake_input = "hello_world_example";
    let camel = rsb::string::to_camel_case(snake_input);
    assert_eq!(camel, "helloWorldExample");
}

#[test]
fn test_case_macros() {
    // Test case conversion macros (value forms)
    let test_value = "TestCaseExample";

    // Test macro forms
    let snake_result = snake!(test_value);
    assert_eq!(snake_result, "test_case_example");

    let kebab_result = kebab!(test_value);
    assert_eq!(kebab_result, "test-case-example");

    let camel_result = camel!("test_case_example");
    assert_eq!(camel_result, "testCaseExample");

    let dot_result = dot!(test_value);
    assert_eq!(dot_result, "test.case.example");

    let space_result = space!(test_value);
    assert_eq!(space_result, "test case example");
}

#[test]
fn test_ascii_filtering() {
    // Test ASCII filtering utilities
    let unicode_str = "Hello🌍World";

    // Test ASCII stripping
    let stripped = rsb::string::utils::filter_ascii_strip(unicode_str);
    assert_eq!(stripped, "HelloWorld");

    // Test ASCII sanitization with default marker
    let sanitized = rsb::string::utils::filter_ascii_sanitize_default(unicode_str);
    assert!(sanitized.contains("Hello"));
    assert!(sanitized.contains("World"));
    assert!(sanitized.contains("#INV#")); // Invalid character marker
}

#[test]
fn test_shell_helpers() {
    // Test shell utility functions
    let test_str = "Hello 'world' with spaces";

    // Test POSIX shell single quoting
    let quoted = rsb::string::utils::shell_single_quote(test_str);
    assert!(quoted.starts_with('\''));
    assert!(quoted.ends_with('\''));
    assert!(quoted.contains("Hello"));
    assert!(quoted.contains("world"));
}

#[test]
fn test_string_macros() {
    // Test string helper macros

    // Test substring containment
    let haystack = "The quick brown fox";
    assert!(str_in!("quick", in: haystack));
    assert!(str_in!("fox", in: haystack));
    assert!(!str_in!("elephant", in: haystack));

    // Test line generation
    let line = str_line!('-', 10);
    assert_eq!(line, "----------");
    assert_eq!(line.len(), 10);

    let shorter_line = str_line!('=', 5);
    assert_eq!(shorter_line, "=====");
}

#[test]
fn test_context_variable_macros() {
    // Test macros that work with global context variables

    // Set up test variables in global context
    set_var("TEST_STRING", "TestValue");
    set_var("PADDED_STRING", "  trimme  ");

    // Test length macro (returns usize, not string)
    let length = str_len!("TEST_STRING");
    assert_eq!(length, 9); // "TestValue" is 9 characters

    // Test trim macro
    let trimmed = str_trim!("PADDED_STRING");
    assert_eq!(trimmed, "trimme");

    // Test case conversion var macros
    let snake_var = snake_var!("TEST_STRING");
    assert_eq!(snake_var, "test_value");

    let kebab_var = kebab_var!("TEST_STRING");
    assert_eq!(kebab_var, "test-value");

    // Clean up test variables
    unset_var("TEST_STRING");
    unset_var("PADDED_STRING");
}

#[test]
fn test_edge_cases() {
    // Test edge cases and boundary conditions

    // Test empty string
    let empty = "";
    let empty_sub = rsb::string::str_sub(empty, 0, Some(5));
    assert_eq!(empty_sub, "");

    let empty_snake = rsb::string::to_snake_case(empty);
    assert_eq!(empty_snake, "");

    // Test single character
    let single = "A";
    let single_lower = rsb::string::str_lower(single, true);
    assert_eq!(single_lower, "a");

    // Test boundary substring operations
    let boundary_str = "test";
    let boundary_sub = rsb::string::str_sub(boundary_str, 0, Some(10)); // Beyond length
    assert_eq!(boundary_sub, "test"); // Should not panic, return available

    // Test case transformation with numbers and special chars
    let mixed_input = "test123_VALUE";
    let mixed_snake = rsb::string::to_snake_case(mixed_input);
    assert_eq!(mixed_snake, "test_123_value"); // Numbers split from letters
}