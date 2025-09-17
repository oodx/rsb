// CLI Integration Tests - Configuration and Meta Information
// Extracted from tests/old/new_features.rs and dispatch.rs

use rsb::prelude::*;

#[test]
fn test_configuration_parsing() {
    // Test configuration file parsing without CLI dependencies

    // Create temporary config content using correct format for meta_keys! (header meta only)
    let config_content =
        "# project_name: test-project\n# version: 1.0.0\n# author: Test Author\nBody content\n";

    // Write to temporary file and test parsing
    let temp_dir = std::env::temp_dir();
    let config_file = temp_dir.join("rsb_test_config.conf");
    std::fs::write(&config_file, config_content).unwrap();

    // Test config parsing via meta_keys! macro
    meta_keys!(config_file.to_str().unwrap(), into: "CONFIG");

    assert_eq!(get_var("CONFIG_project_name"), "test-project");
    assert_eq!(get_var("CONFIG_version"), "1.0.0");
    assert_eq!(get_var("CONFIG_author"), "Test Author");

    // Cleanup
    std::fs::remove_file(config_file).ok();
}

#[test]
fn test_project_initialization_logic() {
    // Test project initialization functionality directly

    let temp_dir = std::env::temp_dir().join("rsb_init_test");
    let project_name = "test-project";

    // Test directory creation logic
    let project_path = temp_dir.join(project_name);
    std::fs::create_dir_all(&project_path).unwrap();

    assert!(project_path.exists());
    assert!(project_path.is_dir());

    // Test path operations
    let project_path_str = project_path.to_string_lossy();
    set_var("PROJECT_PATH", project_path_str.as_ref());

    // Test path manipulation
    path_split!(&get_var("PROJECT_PATH"), into: "PATH_INFO");
    assert_eq!(get_var("PATH_INFO_file_name"), project_name);

    // Cleanup
    std::fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_meta_information_extraction() {
    // Test meta information extraction without CLI

    // Set up project meta information
    set_var("PROJECT_META_NAME", "test-rsb-project");
    set_var("PROJECT_META_VERSION", "0.1.0");
    set_var("PROJECT_META_DESCRIPTION", "A test RSB project");

    // Test meta information access
    assert_eq!(get_var("PROJECT_META_NAME"), "test-rsb-project");
    assert_eq!(get_var("PROJECT_META_VERSION"), "0.1.0");
    assert!(!get_var("PROJECT_META_DESCRIPTION").is_empty());

    // Test meta information formatting
    let formatted = format!(
        "{} v{}: {}",
        get_var("PROJECT_META_NAME"),
        get_var("PROJECT_META_VERSION"),
        get_var("PROJECT_META_DESCRIPTION")
    );

    assert!(formatted.contains("test-rsb-project"));
    assert!(formatted.contains("0.1.0"));
    assert!(formatted.contains("A test RSB project"));
}

#[test]
fn test_environment_mode_detection() {
    // Test environment and mode detection logic

    // Test test mode detection
    set_var("CARGO_TEST", "1");
    assert!(has_var("CARGO_TEST"));

    // Test debug mode
    set_var("DEBUG", "1");
    assert!(has_var("DEBUG"));

    // Test mode-specific behavior
    if has_var("CARGO_TEST") {
        set_var("TEST_MODE_ACTIVE", "true");
    }

    assert_eq!(get_var("TEST_MODE_ACTIVE"), "true");
}
