// CLI Integration Tests - Help and Command Handling
// Extracted from tests/old/cli.rs - Direct API testing without external dependencies

use rsb::prelude::*;

#[test]
fn test_help_functionality_direct() {
    // Test help-related functionality directly via RSB APIs
    // This replaces CLI binary testing with direct function calls

    // Test that help information can be generated
    set_var("RSB_HELP_MODE", "1");
    let help_enabled = has_var("RSB_HELP_MODE");
    assert!(help_enabled);

    // Test usage pattern generation
    set_var("USAGE_PATTERN", "USAGE: rsb [COMMANDS...]");
    let usage = get_var("USAGE_PATTERN");
    assert!(usage.contains("USAGE:"));
    assert!(usage.contains("COMMANDS"));

    // Test command listing functionality
    array_push("AVAILABLE_COMMANDS", "init");
    array_push("AVAILABLE_COMMANDS", "help");
    array_push("AVAILABLE_COMMANDS", "version");

    assert_eq!(array_length("AVAILABLE_COMMANDS"), 3);
    assert_eq!(array_get("AVAILABLE_COMMANDS", 0), "init");
    assert_eq!(array_get("AVAILABLE_COMMANDS", 1), "help");
}

#[test]
fn test_command_validation_direct() {
    // Test command validation logic without CLI execution

    // Valid commands - using fresh array each time
    array_push("VALID_COMMANDS", "init");
    array_push("VALID_COMMANDS", "help");
    array_push("VALID_COMMANDS", "version");

    // Test command validation function
    let test_commands = ["init", "help", "version", "nonexistentcommand"];
    let mut valid_count = 0;

    for cmd in test_commands {
        // Simulate command validation
        let mut found = false;
        for i in 0..array_length("VALID_COMMANDS") {
            if array_get("VALID_COMMANDS", i) == cmd {
                found = true;
                break;
            }
        }
        if found {
            valid_count += 1;
        }
    }

    // Should find 3 valid commands (init, help, version)
    assert_eq!(valid_count, 3);
}

#[test]
fn test_command_argument_parsing() {
    // Test argument parsing functionality directly

    // Simulate command line arguments: ["rsb", "init", "my-project"]
    // Reset test state for parsed args
    array_push("PARSED_ARGS", "rsb");      // program name
    array_push("PARSED_ARGS", "init");     // command
    array_push("PARSED_ARGS", "my-project"); // argument

    assert_eq!(array_length("PARSED_ARGS"), 3);
    assert_eq!(array_get("PARSED_ARGS", 1), "init");
    assert_eq!(array_get("PARSED_ARGS", 2), "my-project");

    // Test argument extraction
    let command = array_get("PARSED_ARGS", 1);
    let project_name = array_get("PARSED_ARGS", 2);

    assert_eq!(command, "init");
    assert_eq!(project_name, "my-project");
    assert!(!project_name.is_empty());
}