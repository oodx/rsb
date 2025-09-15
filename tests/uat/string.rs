use rsb::prelude::*;

#[test]
fn uat_string_demo() {
    // Case transforms
    assert_eq!(rsb::string::to_dot_case("Log File Name"), "log.file.name");
    assert_eq!(rsb::string::to_kebab_case("Hello World"), "hello-world");
    
    // Substring by character indices (Unicode-safe)
    let s = "hello world";
    assert_eq!(str_sub(s, 6, Some(5)), "world");

    // Prefix/Suffix removal (literal)
    assert_eq!(rsb::string::str_prefix("prefix_value", "prefix_", false), "value");
    assert_eq!(rsb::string::str_suffix("value.txt", ".txt", false), "value");

    // ASCII sanitize default
    let mixed = "hi ☃";
    let sanitized = rsb::string::utils::filter_ascii_sanitize_default(mixed);
    assert_eq!(sanitized, format!("hi {}", rsb::string::utils::ASCII_INVALID_MARKER));
}

