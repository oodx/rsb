use rsb::prelude::*;

#[test]
fn sanity_string_substring() {
    // Test basic substring operations
    let test_str = "Hello, World!";

    let sub1 = rsb::string::str_sub(test_str, 0, Some(5));
    assert_eq!(sub1, "Hello");

    let sub2 = rsb::string::str_sub(test_str, 7, Some(5));
    assert_eq!(sub2, "World");

    let sub3 = rsb::string::str_sub(test_str, 7, None);
    assert_eq!(sub3, "World!");

    // Test Unicode safety
    let unicode_str = "Hello, 世界!";
    let unicode_sub = rsb::string::str_sub(unicode_str, 7, Some(2));
    assert_eq!(unicode_sub, "世界");
}

#[test]
fn sanity_string_prefix_suffix() {
    // Test prefix and suffix removal
    let test_str = "prefix_content_suffix";

    let no_prefix = rsb::string::str_prefix(test_str, "prefix_", false);
    assert_eq!(no_prefix, "content_suffix");

    let no_suffix = rsb::string::str_suffix(test_str, "_suffix", false);
    assert_eq!(no_suffix, "prefix_content");

    // Test no match
    let no_match = rsb::string::str_prefix(test_str, "nomatch", false);
    assert_eq!(no_match, test_str);
}

#[test]
fn sanity_string_replacement() {
    // Test string replacement
    let test_str = "Hello World Hello";

    let replace_first = rsb::string::str_replace(test_str, "Hello", "Hi", false);
    assert_eq!(replace_first, "Hi World Hello");

    let replace_all = rsb::string::str_replace(test_str, "Hello", "Hi", true);
    assert_eq!(replace_all, "Hi World Hi");

    // Test no match
    let no_match = rsb::string::str_replace(test_str, "xyz", "abc", false);
    assert_eq!(no_match, test_str);
}

#[test]
fn sanity_string_case_conversion() {
    // Test case conversion helpers
    let test_str = "hello world";

    let upper_first = rsb::string::str_upper(test_str, false);
    assert!(upper_first.starts_with('H'));

    let upper_all = rsb::string::str_upper(test_str, true);
    assert_eq!(upper_all, "HELLO WORLD");

    let mixed_str = "HELLO WORLD";
    let lower_first = rsb::string::str_lower(mixed_str, false);
    assert!(lower_first.starts_with('h'));

    let lower_all = rsb::string::str_lower(mixed_str, true);
    assert_eq!(lower_all, "hello world");
}

#[test]
fn sanity_string_case_transformations() {
    // Test case transformation functions
    let test_str = "HelloWorldExample";

    let snake = rsb::string::case::to_snake_case(test_str);
    assert_eq!(snake, "hello_world_example");

    let kebab = rsb::string::case::to_kebab_case(test_str);
    assert_eq!(kebab, "hello-world-example");

    let dot = rsb::string::case::to_dot_case(test_str);
    assert_eq!(dot, "hello.world.example");

    let space = rsb::string::case::to_space_case(test_str);
    assert_eq!(space, "hello world example");

    let camel = rsb::string::case::to_camel_case("hello_world_example");
    assert_eq!(camel, "helloWorldExample");
}

#[test]
fn sanity_string_case_macros() {
    // Test case conversion macros
    let test_value = "TestCaseExample";

    let snake_result = snake!(test_value);
    assert_eq!(snake_result, "test_case_example");

    let kebab_result = kebab!(test_value);
    assert_eq!(kebab_result, "test-case-example");

    let camel_result = camel!("test_case_example");
    assert_eq!(camel_result, "testCaseExample");
}

#[test]
fn sanity_string_ascii_filtering() {
    // Test ASCII filtering utilities
    let unicode_str = "Hello🌍World";

    let stripped = rsb::string::utils::filter_ascii_strip(unicode_str);
    assert_eq!(stripped, "HelloWorld");

    let sanitized = rsb::string::utils::filter_ascii_sanitize_default(unicode_str);
    assert!(sanitized.contains("Hello"));
    assert!(sanitized.contains("World"));
    assert!(sanitized.contains("#INV#")); // Invalid character marker
}

#[test]
fn sanity_string_shell_helpers() {
    // Test shell utility functions
    let test_str = "Hello 'world' with spaces";

    let quoted = rsb::string::utils::shell_single_quote(test_str);
    assert!(quoted.starts_with('\''));
    assert!(quoted.ends_with('\''));
    assert!(quoted.contains("Hello"));
}