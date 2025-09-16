// Global Integration Tests
// Tests for RSB global store, expansion, and configuration integration

use rsb::prelude::*;

#[test]
fn global_integration_store_and_expansion() {
    // Test integration between global store and variable expansion
    set_var("INTEGRATION_BASE", "/tmp/rsb-test");
    set_var("INTEGRATION_NAME", "global-test");

    // Test that expansion works across multiple variables
    let path_template = "${INTEGRATION_BASE}/${INTEGRATION_NAME}/data";
    let expanded = expand_vars(&path_template);
    assert_eq!(expanded, "/tmp/rsb-test/global-test/data");

    // Test compound expansions
    set_var("FULL_PATH", &expanded);
    let final_path = expand_vars("Config: ${FULL_PATH}/config.toml");
    assert_eq!(final_path, "Config: /tmp/rsb-test/global-test/data/config.toml");
}

#[test]
fn global_integration_config_precedence() {
    // Test integration between environment and configuration
    set_var("CONFIG_PRIORITY", "config");
    set_var("ENV_PRIORITY", "env");

    // Simulate config loading scenario
    assert_eq!(get_var("CONFIG_PRIORITY"), "config");
    assert_eq!(get_var("ENV_PRIORITY"), "env");

    // Test that variables can be overridden
    set_var("CONFIG_PRIORITY", "overridden");
    assert_eq!(get_var("CONFIG_PRIORITY"), "overridden");
}

#[test]
fn global_integration_boolean_logic() {
    // Test integration of boolean helpers with real-world scenarios
    set_var("DEBUG_MODE", "1");
    set_var("QUIET_MODE", "0");
    set_var("VERBOSE_MODE", "true");

    assert!(is_true("DEBUG_MODE"));
    assert!(is_false("QUIET_MODE"));
    assert!(is_true("VERBOSE_MODE"));

    // Test scenario-based logic
    let should_log = is_true("DEBUG_MODE") && !is_true("QUIET_MODE");
    assert!(should_log);
}

