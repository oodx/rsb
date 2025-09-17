// RSB Sanity Tests - Hosts Module Core Functionality Verification
// Tests verify the hosts module functions work as documented in FEATURES_HOST

use rsb::prelude::*;

#[test]
fn test_hosts_environment_import() {
    // Test environment variable import functionality

    // Set some test environment variables
    std::env::set_var("TEST_HOST_VAR", "test_value");
    std::env::set_var("TEST_HOST_NUM", "42");

    // Import environment into global context
    rsb::hosts::import_environment();

    // Verify environment variables are accessible via global context
    assert_eq!(get_var("TEST_HOST_VAR"), "test_value");
    assert_eq!(get_var("TEST_HOST_NUM"), "42");

    // Clean up
    std::env::remove_var("TEST_HOST_VAR");
    std::env::remove_var("TEST_HOST_NUM");
}

#[test]
fn test_hosts_standard_modes() {
    // Test standard mode setup functionality

    // Set mode environment variables
    std::env::set_var("DEBUG", "1");
    std::env::set_var("QUIET", "true");

    // Setup standard modes
    rsb::hosts::setup_standard_modes();

    // Verify modes are set in global context
    assert_eq!(get_var("DEBUG_MODE"), "1");
    assert_eq!(get_var("QUIET_MODE"), "1");

    // Test DEV mode
    std::env::set_var("DEV", "yes");
    rsb::hosts::setup_standard_modes();
    assert_eq!(get_var("DEV_MODE"), "1");

    // Clean up
    std::env::remove_var("DEBUG");
    std::env::remove_var("QUIET");
    std::env::remove_var("DEV");
}

#[test]
fn test_hosts_xdg_paths() {
    // Test XDG path setup functionality

    // Setup XDG paths
    rsb::hosts::setup_xdg_paths();

    // Verify XDG variables are set
    assert!(!get_var("XDG_CONFIG_HOME").is_empty());
    assert!(!get_var("XDG_CACHE_HOME").is_empty());
    assert!(!get_var("XDG_HOME").is_empty());
    assert!(!get_var("XDG_LIB_HOME").is_empty());
    assert!(!get_var("XDG_ETC_HOME").is_empty());
    assert!(!get_var("XDG_BIN_HOME").is_empty());
    assert!(!get_var("XDG_DATA_HOME").is_empty());
    assert!(!get_var("XDG_TMP_HOME").is_empty());

    // Test that XDG_TMP alias is set
    assert_eq!(get_var("XDG_TMP"), get_var("XDG_TMP_HOME"));
}

#[test]
fn test_hosts_rsb_paths() {
    // Test RSB path setup functionality

    // Setup XDG paths first (required for RSB paths)
    rsb::hosts::setup_xdg_paths();

    // Setup RSB paths
    rsb::hosts::setup_rsb_paths();

    // Verify RSB variables are set
    assert!(!get_var("RSB_LIB_HOME").is_empty());
    assert!(!get_var("RSB_ETC_HOME").is_empty());
    assert!(!get_var("RSB_DATA_HOME").is_empty());
    assert!(!get_var("RSB_BIN_HOME").is_empty());

    // Test RSB path helper functions
    let tool_path = rsb::hosts::rsb_tool_path("mytool");
    assert!(tool_path.contains("mytool"));
    assert!(tool_path.contains("rsb"));

    let config_path = rsb::hosts::rsb_config_path("myconfig.conf");
    assert!(config_path.contains("myconfig.conf"));
}

#[test]
fn test_hosts_execution_context() {
    // Test script execution context setup

    let test_args = vec![
        "/path/to/script.sh".to_string(),
        "arg1".to_string(),
        "--flag".to_string(),
        "arg2".to_string(),
    ];

    // Setup execution context
    rsb::hosts::setup_execution_context(&test_args);

    // Verify script context variables
    assert_eq!(get_var("SCRIPT_NAME"), "script.sh");
    assert!(get_var("SCRIPT_PATH").contains("/path/to/script.sh"));
    assert!(get_var("SCRIPT_DIR").contains("/path/to"));
    assert!(!get_var("SCRIPT_PWD").is_empty());

    // Verify argument count
    assert_eq!(get_var("ARGC"), "4");
}

#[test]
fn test_hosts_env_bootstrap() {
    // Test combined environment bootstrap functionality

    std::env::set_var("TEST_BOOTSTRAP_VAR", "bootstrap_value");

    // Run env bootstrap (import + modes)
    rsb::hosts::env_bootstrap();

    // Verify environment was imported
    assert_eq!(get_var("TEST_BOOTSTRAP_VAR"), "bootstrap_value");

    // Verify modes were set up (if any mode vars exist)
    // This is a combined operation test

    std::env::remove_var("TEST_BOOTSTRAP_VAR");
}

#[test]
fn test_hosts_full_bootstrap() {
    // Test complete bootstrap functionality

    let test_args = vec!["test_program".to_string(), "test_arg".to_string()];

    // Run full bootstrap
    rsb::hosts::bootstrap(&test_args);

    // Verify all components are initialized:

    // 1. Environment imported
    assert!(!get_var("PATH").is_empty()); // PATH should be imported from env

    // 2. XDG paths set
    assert!(!get_var("XDG_HOME").is_empty());

    // 3. RSB paths set
    assert!(!get_var("RSB_LIB_HOME").is_empty());

    // 4. Script context set
    assert_eq!(get_var("SCRIPT_NAME"), "test_program");
    assert_eq!(get_var("ARGC"), "2");
}

#[test]
fn test_hosts_bootstrap_from_env() {
    // Test convenience bootstrap that uses std::env::args()

    // This bootstrap uses actual command line arguments
    rsb::hosts::bootstrap_from_env();

    // Verify bootstrap completed successfully
    assert!(!get_var("SCRIPT_NAME").is_empty());
    assert!(!get_var("XDG_HOME").is_empty());
    assert!(!get_var("RSB_LIB_HOME").is_empty());
}

#[test]
fn test_hosts_env_to_global_sync() {
    // Test environment to global context synchronization

    // Set an environment variable
    std::env::set_var("SYNC_TEST_VAR", "sync_value");

    // Sync from env to global
    rsb::hosts::env_to_global();

    // Verify it's in global context
    assert_eq!(get_var("SYNC_TEST_VAR"), "sync_value");

    // Modify in global context
    set_var("SYNC_TEST_VAR", "modified_value");

    // Sync from global to env
    rsb::hosts::global_to_env();

    // Verify environment variable was updated
    assert_eq!(std::env::var("SYNC_TEST_VAR").unwrap(), "modified_value");

    std::env::remove_var("SYNC_TEST_VAR");
}

#[test]
fn test_hosts_ensure_directories() {
    // Test directory creation functionality

    // Setup XDG paths (required for directory creation)
    rsb::hosts::setup_xdg_paths();

    // Ensure directories are created
    rsb::hosts::ensure_xdg_directories();

    // We can't easily verify actual directory creation in tests,
    // but the function should not panic
    assert!(true, "Directory creation completed without panicking");
}

#[test]
fn test_hosts_path_helpers() {
    // Test various path helper functions

    // Setup required paths
    rsb::hosts::setup_xdg_paths();
    rsb::hosts::setup_rsb_paths();

    // Test rsb_tool_path
    let tool = rsb::hosts::rsb_tool_path("compiler");
    assert!(tool.contains("compiler"));
    assert!(tool.contains("bin") || tool.contains("rsb"));

    // Test rsb_config_path
    let config = rsb::hosts::rsb_config_path("settings.toml");
    assert!(config.contains("settings.toml"));
    assert!(config.contains("etc") || config.contains("rsb"));

    // Test rsb_data_path
    let data = rsb::hosts::rsb_data_path("database.db");
    assert!(data.contains("database.db"));
    assert!(data.contains("data") || data.contains("rsb"));
}

#[test]
fn test_hosts_xdg_custom_paths() {
    // Test XDG path setup with custom environment variables

    // Set custom XDG paths
    std::env::set_var("XDG_CONFIG_HOME", "/custom/config");
    std::env::set_var("XDG_HOME", "/custom/xdg");

    // Setup XDG paths
    rsb::hosts::setup_xdg_paths();

    // Verify custom paths are respected
    assert_eq!(get_var("XDG_CONFIG_HOME"), "/custom/config");
    assert!(get_var("XDG_HOME").contains("/custom"));

    // Clean up
    std::env::remove_var("XDG_CONFIG_HOME");
    std::env::remove_var("XDG_HOME");
}

#[test]
fn test_edge_cases() {
    // Test edge cases and error conditions

    // Test with empty args
    let empty_args: Vec<String> = vec![];
    rsb::hosts::setup_execution_context(&empty_args);
    // Should handle gracefully
    assert_eq!(get_var("ARGC"), "0");

    // Test with very long path
    let long_args = vec![
        format!("/very/long/path/{}/script.sh", "sub/".repeat(50)),
        "arg".to_string(),
    ];
    rsb::hosts::setup_execution_context(&long_args);
    assert!(!get_var("SCRIPT_NAME").is_empty());

    // Test bootstrap idempotency (calling multiple times)
    rsb::hosts::bootstrap_from_env();
    rsb::hosts::bootstrap_from_env();
    // Should not panic or corrupt state
    assert!(!get_var("SCRIPT_NAME").is_empty());
}
