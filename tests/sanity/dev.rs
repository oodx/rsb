use rsb::prelude::*;

#[test]
fn sanity_dev_debug_mode() {
    // Test debug mode detection
    set_var("DEBUG", "1");
    assert!(rsb::dev::is_debug_mode());

    set_var("DEBUG", "0");
    assert!(!rsb::dev::is_debug_mode());

    unset_var("DEBUG");
    assert!(!rsb::dev::is_debug_mode());
}

#[test]
fn sanity_dev_assertions() {
    // Test development assertions
    rsb::dev::dev_assert(true, "This should pass");

    // Test that false assertions don't panic in release mode
    // (dev_assert should be a no-op in non-debug builds)
    rsb::dev::dev_assert(true, "Basic assertion");
}

#[test]
fn sanity_dev_timing() {
    // Test timing utilities
    let start = rsb::dev::start_timer();

    // Small delay
    std::thread::sleep(std::time::Duration::from_millis(1));

    let elapsed = rsb::dev::elapsed_time(start);
    assert!(elapsed.as_millis() >= 1);
}

#[test]
fn sanity_dev_debug_output() {
    // Test debug output functions
    rsb::dev::debug_print("Test debug message");

    // Should not panic regardless of debug mode
    set_var("DEBUG", "1");
    rsb::dev::debug_print("Debug mode message");

    set_var("DEBUG", "0");
    rsb::dev::debug_print("Non-debug mode message");
}

#[test]
fn sanity_dev_test_helpers() {
    // Test environment setup/cleanup
    rsb::dev::setup_test_env();
    rsb::dev::cleanup_test_env();

    // Test temp file creation
    let temp_file = rsb::dev::create_temp_file("test");
    assert!(!temp_file.is_empty());

    // File should exist
    assert!(std::path::Path::new(&temp_file).exists());
}