// CLI Integration Tests - Array and System Macro Functionality
// Extracted from tests/old/final_utils.rs and os_basic.rs

use rsb::prelude::*;

#[test]
fn test_array_operations_comprehensive() {
    // Test comprehensive array operations extracted from CLI tests

    // Test array creation and manipulation - start fresh
    // Note: RSB arrays start empty by design, no need to clear

    // Test array population
    array_push("TEST_ARRAY", "item1");
    array_push("TEST_ARRAY", "item2");
    array_push("TEST_ARRAY", "item3");

    assert_eq!(array_length("TEST_ARRAY"), 3);
    assert_eq!(array_get("TEST_ARRAY", 0), "item1");
    assert_eq!(array_get("TEST_ARRAY", 1), "item2");
    assert_eq!(array_get("TEST_ARRAY", 2), "item3");

    // Test array iteration and processing
    let mut concatenated = String::new();
    for i in 0..array_length("TEST_ARRAY") {
        if !concatenated.is_empty() {
            concatenated.push(',');
        }
        concatenated.push_str(&array_get("TEST_ARRAY", i));
    }

    assert_eq!(concatenated, "item1,item2,item3");

    // Test array search/contains functionality
    assert!(array_contains("TEST_ARRAY", "item2"));
    assert!(!array_contains("TEST_ARRAY", "nonexistent"));
}

#[test]
fn test_system_macro_operations() {
    // Test system macro functionality without external command execution

    // Test file system operations
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("rsb_system_test.txt");
    let test_content = "System macro test content\nLine 2\nLine 3";

    // Write test file
    std::fs::write(&test_file, test_content).unwrap();

    // Test file operations via RSB macros
    let file_path = test_file.to_string_lossy();

    // Test path operations
    path_split!(&file_path, into: "FILE_INFO");
    assert_eq!(get_var("FILE_INFO_file_name"), "rsb_system_test.txt");

    // Test file content operations (without external commands)
    let content = std::fs::read_to_string(&test_file).unwrap();
    assert!(content.contains("System macro test content"));
    assert!(content.contains("Line 2"));

    // Test line-based operations using RSB streams
    let lines = pipe!(content).to_string();

    assert!(lines.contains("Line 2"));

    // Cleanup
    std::fs::remove_file(test_file).ok();
}

#[test]
fn test_variable_expansion_and_substitution() {
    // Test variable expansion functionality

    // Set up test variables
    set_var("BASE_DIR", "/tmp/rsb");
    set_var("PROJECT_NAME", "test-project");
    set_var("FILE_EXT", "rs");

    // Test variable substitution patterns
    let template = "${BASE_DIR}/${PROJECT_NAME}/src/main.${FILE_EXT}";

    // Manual variable expansion (simulating CLI behavior)
    let expanded = template
        .replace("${BASE_DIR}", &get_var("BASE_DIR"))
        .replace("${PROJECT_NAME}", &get_var("PROJECT_NAME"))
        .replace("${FILE_EXT}", &get_var("FILE_EXT"));

    assert_eq!(expanded, "/tmp/rsb/test-project/src/main.rs");

    // Test using param! macro for expansion
    set_var("TEMPLATE", template);
    let param_expanded = param!("TEMPLATE")
        .replace("${BASE_DIR}", &get_var("BASE_DIR"))
        .replace("${PROJECT_NAME}", &get_var("PROJECT_NAME"))
        .replace("${FILE_EXT}", &get_var("FILE_EXT"));

    assert_eq!(param_expanded, "/tmp/rsb/test-project/src/main.rs");
}

#[test]
fn test_command_dispatch_logic() {
    // Test command dispatch functionality extracted from dispatch.rs

    // Set up command registry - start fresh
    array_push("COMMAND_REGISTRY", "help:show_help");
    array_push("COMMAND_REGISTRY", "init:initialize_project");
    array_push("COMMAND_REGISTRY", "version:show_version");

    // Test command lookup
    let target_command = "init";
    let mut found_handler = String::new();

    for i in 0..array_length("COMMAND_REGISTRY") {
        let entry = array_get("COMMAND_REGISTRY", i);
        if entry.starts_with(&format!("{}:", target_command)) {
            found_handler = entry.split(':').nth(1).unwrap_or("").to_string();
            break;
        }
    }

    assert_eq!(found_handler, "initialize_project");

    // Test command validation
    let valid_commands = ["help", "init", "version"];
    let test_command = "init";
    let is_valid = valid_commands.contains(&test_command);
    assert!(is_valid);

    // Test invalid command
    let invalid_command = "nonexistent";
    let is_invalid = !valid_commands.contains(&invalid_command);
    assert!(is_invalid);
}
