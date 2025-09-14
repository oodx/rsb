use rsb::prelude::*;

#[test]
fn adapter_apply_env_simple_only() {
    // Ensure mode flags are not present from previous state in this test binary
    rsb::global::unset_var("DEBUG_MODE");
    rsb::global::unset_var("DEV_MODE");
    rsb::global::unset_var("QUIET_MODE");
    rsb::global::unset_var("TRACE_MODE");

    std::env::set_var("TEST_SIMPLE_ENV", "val");
    rsb::global::apply_env_simple();
    assert_eq!(get_var("TEST_SIMPLE_ENV"), "val");

    // Simple adapter should not set mode flags
    assert_eq!(get_var("DEBUG_MODE"), "");
    assert_eq!(get_var("DEV_MODE"), "");
    assert_eq!(get_var("QUIET_MODE"), "");
    assert_eq!(get_var("TRACE_MODE"), "");
}

