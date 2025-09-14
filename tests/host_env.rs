// Host env bootstrap and helpers tests
use rsb::prelude::*;

#[test]
fn host_import_and_modes() {
    // Ensure a known state
    std::env::remove_var("TEST_ENV_A");
    std::env::remove_var("DEBUG");
    std::env::remove_var("DEV");
    std::env::remove_var("QUIET");
    std::env::remove_var("TRACE");

    // Set an env var and import
    std::env::set_var("TEST_ENV_A", "value_a");
    rsb::hosts::import_environment();
    assert_eq!(rsb::global::get_var("TEST_ENV_A"), "value_a");

    // Standard modes
    std::env::set_var("QUIET", "1");
    rsb::hosts::setup_standard_modes();
    assert!(rsb::global::is_true("QUIET_MODE"));
}

#[test]
fn host_env_helpers_and_sync() {
    // Direct get/set/has
    rsb::hosts::set_env_var("TEST_ENV_B", "b");
    assert!(rsb::hosts::has_env_var("TEST_ENV_B"));
    assert_eq!(rsb::hosts::get_env_var("TEST_ENV_B").unwrap(), "b");
    assert_eq!(rsb::global::get_var("TEST_ENV_B"), "b");

    // Sync global to env
    rsb::global::set_var("SYNC_ME", "42");
    rsb::hosts::global_to_env();
    assert_eq!(std::env::var("SYNC_ME").unwrap_or_default(), "42");
}
