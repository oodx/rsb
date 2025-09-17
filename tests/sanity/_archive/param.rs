use rsb::prelude::*;

#[test]
fn sanity_param_basic_access() {
    // Test basic parameter access
    set_var("TEST_PARAM", "test_value");

    let value = param!("TEST_PARAM");
    assert_eq!(value, "test_value");

    // Test missing parameter
    let missing = param!("MISSING_PARAM");
    assert_eq!(missing, "");
}

#[test]
fn sanity_param_default_values() {
    // Test default value functionality
    set_var("EXISTING", "exists");
    set_var("EMPTY", "");

    let with_default = param!("MISSING", default: "fallback");
    assert_eq!(with_default, "fallback");

    let existing_default = param!("EXISTING", default: "fallback");
    assert_eq!(existing_default, "exists");

    let empty_default = param!("EMPTY", default: "fallback");
    assert_eq!(empty_default, "fallback");
}

#[test]
fn sanity_param_alt_values() {
    // Test alternative value functionality
    set_var("EXISTING", "exists");
    set_var("EMPTY", "");

    let with_alt = param!("MISSING", alt: "has_value");
    assert_eq!(with_alt, "");

    let existing_alt = param!("EXISTING", alt: "has_value");
    assert_eq!(existing_alt, "has_value");

    let empty_alt = param!("EMPTY", alt: "has_value");
    assert_eq!(empty_alt, "");
}

#[test]
fn sanity_param_length_operations() {
    // Test length operations
    set_var("TEST_STRING", "hello");

    let length = param!("TEST_STRING", len);
    assert_eq!(length, "5");

    let missing_length = param!("MISSING", len);
    assert_eq!(missing_length, "0");
}

#[test]
fn sanity_param_substring_operations() {
    // Test substring operations
    set_var("TEST_STRING", "0123456789");

    let sub1 = param!("TEST_STRING", sub: 2, 3);
    assert_eq!(sub1, "234");

    let sub2 = param!("TEST_STRING", sub: 0, 5);
    assert_eq!(sub2, "01234");
}

#[test]
fn sanity_param_prefix_suffix() {
    // Test prefix and suffix operations
    set_var("TEST_PATH", "/home/user/file.txt");

    let with_prefix = param!("TEST_PATH", prefix: "/home");
    assert_eq!(with_prefix, "user/file.txt");

    let with_suffix = param!("TEST_PATH", suffix: ".txt");
    assert_eq!(with_suffix, "/home/user/file");
}

#[test]
fn sanity_param_replacement() {
    // Test replacement operations
    set_var("TEST_STRING", "hello/world/test");

    let replace_first = param!("TEST_STRING", replace: "/" => "_");
    assert_eq!(replace_first, "hello_world/test");

    let replace_all = param!("TEST_STRING", replace: "/" => "_", all);
    assert_eq!(replace_all, "hello_world_test");
}

#[test]
fn sanity_param_case_operations() {
    // Test case conversion operations
    set_var("TEST_WORD", "hello");
    set_var("TEST_CAPS", "WORLD");

    let upper_first = param!("TEST_WORD", upper: first);
    assert_eq!(upper_first, "Hello");

    let upper_all = param!("TEST_WORD", upper);
    assert_eq!(upper_all, "HELLO");

    let lower_first = param!("TEST_CAPS", lower: first);
    assert_eq!(lower_first, "wORLD");

    let lower_all = param!("TEST_CAPS", lower);
    assert_eq!(lower_all, "world");
}