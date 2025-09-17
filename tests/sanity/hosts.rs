use rsb::prelude::*;

#[test]
fn sanity_hosts_environment_discovery() {
    // Test environment discovery functionality
    rsb::hosts::discover_environment();

    // Should populate basic environment variables
    assert!(!get_var("HOME").is_empty());
    assert!(!get_var("USER").is_empty() || !get_var("USERNAME").is_empty());
}

#[test]
fn sanity_hosts_xdg_paths() {
    // Test XDG path management
    rsb::hosts::setup_xdg_paths();

    // Should set up XDG environment variables
    assert!(!get_var("XDG_CONFIG_HOME").is_empty());
    assert!(!get_var("XDG_DATA_HOME").is_empty());
    assert!(!get_var("XDG_CACHE_HOME").is_empty());
}

#[test]
fn sanity_hosts_bootstrap() {
    // Test host bootstrap functionality
    let test_args = vec!["test_bin".to_string(), "--test".to_string()];
    rsb::hosts::bootstrap(&test_args);

    // Bootstrap should set up basic environment
    assert!(!get_var("HOME").is_empty());
    assert!(!get_var("HOSTNAME").is_empty() || !get_var("COMPUTERNAME").is_empty());
}

#[test]
fn sanity_hosts_path_utilities() {
    // Test path utility functions
    let home_path = rsb::hosts::get_home_path();
    assert!(!home_path.is_empty());

    let config_path = rsb::hosts::get_config_path();
    assert!(!config_path.is_empty());

    let cache_path = rsb::hosts::get_cache_path();
    assert!(!cache_path.is_empty());
}

#[test]
fn sanity_hosts_os_detection() {
    // Test OS detection capabilities
    let os_info = rsb::hosts::get_os_info();
    assert!(!os_info.is_empty());

    // Should detect at least one of the major platforms
    let is_recognized = os_info.contains("linux") ||
                       os_info.contains("windows") ||
                       os_info.contains("macos") ||
                       os_info.contains("darwin");
    assert!(is_recognized);
}

#[test]
fn sanity_hosts_hostname_detection() {
    // Test hostname detection
    let hostname = rsb::hosts::get_hostname();
    assert!(!hostname.is_empty());

    // Should also be available in global store after bootstrap
    if get_var("HOSTNAME").is_empty() {
        // Try alternative hostname variable
        assert!(!get_var("COMPUTERNAME").is_empty() || !hostname.is_empty());
    }
}