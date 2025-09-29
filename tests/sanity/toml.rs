//! Sanity tests for TOML Snooping module

use rsb::toml::*;
use rsb::global::get_var;
use std::fs;
use std::env;
use serial_test::serial;

// Helper to create test Cargo.toml in temp directory
fn setup_test_toml(content: &str) -> std::path::PathBuf {
    let temp_dir = env::temp_dir().join(format!("rsb_test_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let cargo_path = temp_dir.join("Cargo.toml");
    fs::write(&cargo_path, content).unwrap();
    temp_dir
}

fn cleanup_test_dir(dir: &std::path::Path) {
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn sanity_default_namespaces() {
    // Default namespaces should exist
    assert!(has_namespace("hub"));
    assert!(has_namespace("inf"));
    assert!(has_namespace("rsb"));
}

#[test]
#[serial]
fn sanity_enable_toml_snooping() {
    // Create test environment
    let test_toml = r#"
[package]
name = "test"
version = "0.1.0"

[package.metadata.rsb]
test_key = "test_value"
"#;

    let test_dir = setup_test_toml(test_toml);
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&test_dir).unwrap();

    // Enable snooping
    enable_toml_snooping();

    // Check that value was snooped
    assert_eq!(get_var("rsb_test_key"), "test_value");

    // Cleanup
    env::set_current_dir(original_dir).unwrap();
    cleanup_test_dir(&test_dir);
}

#[test]
#[serial]
fn sanity_snoop_multiple_namespaces() {
    let test_toml = r#"
[package]
name = "test"
version = "0.1.0"

[package.metadata.hub]
api_url = "https://api.hub.test"

[package.metadata.inf]
team = "RSB Core"

[package.metadata.rsb]
mode = "production"
"#;

    let test_dir = setup_test_toml(test_toml);
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&test_dir).unwrap();

    enable_toml_snooping();

    // Check all namespaces were snooped
    assert_eq!(get_var("hub_api_url"), "https://api.hub.test");
    assert_eq!(get_var("inf_team"), "RSB Core");
    assert_eq!(get_var("rsb_mode"), "production");

    env::set_current_dir(original_dir).unwrap();
    cleanup_test_dir(&test_dir);
}

#[test]
#[serial]
fn sanity_snake_case_conversion() {
    let test_toml = r#"
[package]
name = "test"
version = "0.1.0"

[package.metadata.hub]
apiUrl = "test"
maxRetries = "5"
team-name = "test-team"
"#;

    let test_dir = setup_test_toml(test_toml);
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&test_dir).unwrap();

    enable_toml_snooping();

    // Keys should be converted to snake_case
    assert_eq!(get_var("hub_api_url"), "test");
    assert_eq!(get_var("hub_max_retries"), "5");
    assert_eq!(get_var("hub_team_name"), "test-team");

    env::set_current_dir(original_dir).unwrap();
    cleanup_test_dir(&test_dir);
}

#[test]
#[serial]
fn sanity_array_storage() {
    let test_toml = r#"
[package]
name = "test"
version = "0.1.0"

[package.metadata.hub]
features = ["auth", "cache", "metrics"]
"#;

    let test_dir = setup_test_toml(test_toml);
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&test_dir).unwrap();

    enable_toml_snooping();

    // Array should be stored with LENGTH + indexed values
    assert_eq!(get_var("hub_features_LENGTH"), "3");
    assert_eq!(get_var("hub_features_0"), "auth");
    assert_eq!(get_var("hub_features_1"), "cache");
    assert_eq!(get_var("hub_features_2"), "metrics");

    env::set_current_dir(original_dir).unwrap();
    cleanup_test_dir(&test_dir);
}

#[test]
#[serial]
fn sanity_value_types() {
    let test_toml = r#"
[package]
name = "test"
version = "0.1.0"

[package.metadata.rsb]
string_val = "test"
int_val = 42
bool_val = true
float_val = 3.14
"#;

    let test_dir = setup_test_toml(test_toml);
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&test_dir).unwrap();

    enable_toml_snooping();

    // All values stored as strings
    assert_eq!(get_var("rsb_string_val"), "test");
    assert_eq!(get_var("rsb_int_val"), "42");
    assert_eq!(get_var("rsb_bool_val"), "true");
    assert_eq!(get_var("rsb_float_val"), "3.14");

    env::set_current_dir(original_dir).unwrap();
    cleanup_test_dir(&test_dir);
}

#[test]
#[serial]
fn sanity_custom_namespace() {
    let test_toml = r#"
[package]
name = "test"
version = "0.1.0"

[package.metadata.custom]
key = "custom_value"
"#;

    let test_dir = setup_test_toml(test_toml);
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&test_dir).unwrap();

    // Add custom namespace before enabling
    snoop_namespace("custom");
    enable_toml_snooping();

    assert_eq!(get_var("custom_key"), "custom_value");

    env::set_current_dir(original_dir).unwrap();
    cleanup_test_dir(&test_dir);
}

#[test]
#[serial]
fn sanity_is_enabled() {
    let test_toml = r#"
[package]
name = "test"
version = "0.1.0"
"#;

    let test_dir = setup_test_toml(test_toml);
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&test_dir).unwrap();

    enable_toml_snooping();

    // Should be enabled after calling enable_toml_snooping
    assert!(is_enabled());

    env::set_current_dir(original_dir).unwrap();
    cleanup_test_dir(&test_dir);
}

#[test]
#[serial]
fn sanity_has_namespace() {
    // Default namespaces should exist
    assert!(has_namespace("hub"));
    assert!(has_namespace("inf"));
    assert!(has_namespace("rsb"));

    // Add a unique custom namespace for this test
    let unique_ns = format!("test_ns_{}", uuid::Uuid::new_v4().to_string().split('-').next().unwrap());
    assert!(!has_namespace(&unique_ns));

    // Add custom namespace
    snoop_namespace(&unique_ns);
    assert!(has_namespace(&unique_ns));
}

#[test]
#[serial]
fn sanity_no_cargo_toml() {
    // Test graceful handling when Cargo.toml doesn't exist
    let temp_dir = env::temp_dir().join(format!("rsb_no_toml_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&temp_dir).unwrap();

    // Should not panic when Cargo.toml doesn't exist
    enable_toml_snooping();

    env::set_current_dir(original_dir).unwrap();
    cleanup_test_dir(&temp_dir);
}