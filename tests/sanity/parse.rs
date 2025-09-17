use rsb::prelude::*;

#[test]
fn sanity_parse_line_operations() {
    let content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";

    // Test line extraction
    let lines = sed_lines!(content, 2, 4);
    assert!(lines.contains("Line 2"));
    assert!(lines.contains("Line 3"));
    assert!(lines.contains("Line 4"));
    assert!(!lines.contains("Line 1"));
    assert!(!lines.contains("Line 5"));
}

#[test]
fn sanity_parse_pattern_operations() {
    let content = "Before\nTarget line\nAfter";

    // Test pattern-based extraction
    let around = sed_around!(content, "Target", 1);
    assert!(around.contains("Before"));
    assert!(around.contains("Target line"));
    assert!(around.contains("After"));
}

#[test]
fn sanity_parse_replacement() {
    let content = "Hello world";

    // Test string replacement
    let replaced = sed_replace!(content, "Hello", "Hi");
    assert_eq!(replaced, "Hi world");

    // Test no match
    let no_match = sed_replace!(content, "xyz", "abc");
    assert_eq!(no_match, "Hello world");
}

#[test]
fn sanity_parse_template_operations() {
    let template = "Start\n<!-- MARKER -->\nEnd";
    let insertion = "Middle content";

    // Test template insertion
    let result = sed_template!(template, "<!-- MARKER -->", insertion);
    assert!(result.contains("Start"));
    assert!(result.contains("Middle content"));
    assert!(result.contains("End"));
    assert!(!result.contains("<!-- MARKER -->"));
}

#[test]
fn sanity_parse_file_operations() {
    // Create temporary test file
    let content = "File line 1\nFile line 2\nFile line 3";
    let temp_file = rsb::dev::create_temp_file(content);

    // Test file line extraction
    let lines = sed_lines_file!(&temp_file, 1, 2);
    assert!(lines.contains("File line 1"));
    assert!(lines.contains("File line 2"));
    assert!(!lines.contains("File line 3"));

    // Test file pattern extraction
    let around = sed_around_file!(&temp_file, "line 2", 1);
    assert!(around.contains("File line 1"));
    assert!(around.contains("File line 2"));
    assert!(around.contains("File line 3"));
}

#[test]
fn sanity_parse_edge_cases() {
    // Test empty content
    let empty = "";
    let empty_lines = sed_lines!(empty, 1, 1);
    assert!(empty_lines.is_empty());

    // Test single line
    let single = "Only line";
    let single_result = sed_lines!(single, 1, 1);
    assert_eq!(single_result.trim(), "Only line");

    // Test out of bounds
    let short = "Line 1\nLine 2";
    let out_of_bounds = sed_lines!(short, 1, 10);
    assert!(out_of_bounds.contains("Line 1"));
    assert!(out_of_bounds.contains("Line 2"));
}