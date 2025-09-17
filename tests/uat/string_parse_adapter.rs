// String + Parse Cross-Module Integration Tests
// Tests string transformations with file parsing operations

use rsb::prelude::*;

#[test]
fn uat_string_parse_template_case_transforms() {
    // Test string case transformations in template context
    let template_content = "Hello {{NAME}} from {{PLACE}}";

    // Set variables for template processing
    set_var("NAME", "World");
    set_var("PLACE", "RSB");

    // Process template and test case transformations
    let processed = rsb::parse::template_replace_str(template_content);
    assert_eq!(processed, "Hello World from RSB");

    // Test case transformations on template result
    let snake = rsb::string::to_snake_case(&processed);
    assert_eq!(snake, "hello_world_from_rsb");

    let kebab = rsb::string::to_kebab_case(&processed);
    assert_eq!(kebab, "hello-world-from-rsb");

    // Clean up
    unset_var("NAME");
    unset_var("PLACE");
}

#[test]
fn uat_string_parse_content_filtering() {
    // Test string filtering with parsed content
    let mixed_content = "Line 1: Hello 🌍\nLine 2: ASCII content\nLine 3: More 📝 text";

    // Parse into lines
    let lines: Vec<&str> = mixed_content.lines().collect();

    // Filter each line through string utilities
    let filtered_lines: Vec<String> = lines.iter()
        .map(|line| rsb::string::utils::filter_ascii_strip(line))
        .collect();

    // Should have removed Unicode characters
    assert_eq!(filtered_lines[0], "Line 1: Hello ");
    assert_eq!(filtered_lines[1], "Line 2: ASCII content");
    assert_eq!(filtered_lines[2], "Line 3: More  text");

    // Test case transformations on filtered content
    let joined = filtered_lines.join(" ");
    let snake = rsb::string::to_snake_case(&joined);
    assert!(snake.contains("line_1"));
    assert!(snake.contains("ascii_content"));
}

#[test]
fn uat_string_parse_sed_replace_integration() {
    // Test string utilities with sed-style replacements
    let content = "Hello World, this is a Test";

    // Use sed_replace macro with string case results
    let snake_version = rsb::string::to_snake_case(content);
    let modified = sed_replace!(snake_version, "hello", "hi");
    assert_eq!(modified, "hi_world_this_is_a_test");

    // Test with kebab case
    let kebab_version = rsb::string::to_kebab_case(content);
    let kebab_modified = sed_replace!(kebab_version, "hello", "hi");
    assert_eq!(kebab_modified, "hi-world-this-is-a-test");
}

#[test]
fn uat_string_parse_multiline_processing() {
    // Test string processing with multiline content
    let multiline = "FIRST_LINE\nSecond Line\nTHIRD_line";

    // Process each line with string transformations
    let processed_lines: Vec<String> = multiline.lines()
        .map(|line| {
            // Apply consistent case transformation
            rsb::string::to_snake_case(line)
        })
        .collect();

    assert_eq!(processed_lines[0], "first_line");
    assert_eq!(processed_lines[1], "second_line");
    assert_eq!(processed_lines[2], "third_line");

    // Test rejoining and further processing
    let rejoined = processed_lines.join("\n");
    let final_result = sed_replace!(rejoined, "line", "item");

    assert!(final_result.contains("first_item"));
    assert!(final_result.contains("second_item"));
    assert!(final_result.contains("third_item"));
}

#[test]
fn uat_string_parse_variable_case_transforms() {
    // Test case transformation macros with parse context
    set_var("TEST_VALUE", "HelloWorldExample");

    // Test string case macros with context variables
    let snake_result = snake_var!("TEST_VALUE");
    assert_eq!(snake_result, "hello_world_example");

    let kebab_result = kebab_var!("TEST_VALUE");
    assert_eq!(kebab_result, "hello-world-example");

    // Test in template context
    let template = "File name: {{TEST_VALUE}}";
    let processed = rsb::parse::template_replace_str(template);
    assert_eq!(processed, "File name: HelloWorldExample");

    // Transform the template result
    let snake_filename = rsb::string::to_snake_case(&processed);
    assert!(snake_filename.contains("hello_world_example"));

    unset_var("TEST_VALUE");
}