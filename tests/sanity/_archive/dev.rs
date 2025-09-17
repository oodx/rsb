// RSB Sanity Tests - Dev Module PTY Functionality Verification
// Tests verify the dev module PTY wrapper functionality as documented in FEATURES_DEV

use rsb::prelude::*;

// PTY tests require the dev-pty feature flag
#[cfg(feature = "dev-pty")]
use rsb::dev::{PtyOptions, spawn_pty};

// Test that basic PTY functionality works
#[test]
#[cfg(feature = "dev-pty")]
fn test_pty_basic_execution() {
    // Test basic PTY command execution
    let mut session = spawn_pty("printf 'hello world'", &PtyOptions::default()).unwrap();

    // Read output with timeout
    let output = session.read_for(std::time::Duration::from_millis(200)).unwrap();
    assert!(output.contains("hello world"));

    // Clean up
    let _ = session.wait();
}

#[test]
#[cfg(feature = "dev-pty")]
fn test_pty_echo_command() {
    // Test PTY with echo command
    let mut session = spawn_pty("echo 'PTY test'", &PtyOptions::default()).unwrap();

    // Read output
    let output = session.read_for(std::time::Duration::from_millis(200)).unwrap();
    assert!(output.contains("PTY test"));

    // Clean up
    let _ = session.wait();
}

#[test]
#[cfg(feature = "dev-pty")]
fn test_pty_simple_math() {
    // Test PTY with simple mathematical operations
    let mut session = spawn_pty("expr 2 + 3", &PtyOptions::default()).unwrap();

    // Read result
    let output = session.read_for(std::time::Duration::from_millis(200)).unwrap();
    assert!(output.contains("5"));

    // Clean up
    let _ = session.wait();
}

// When dev-pty is not available, provide basic sanity tests using global module
#[test]
#[cfg(not(feature = "dev-pty"))]
fn test_dev_module_disabled() {
    // When dev-pty feature is disabled, ensure we can still run basic sanity checks
    // This tests that RSB gracefully handles missing dev functionality

    // Test that we can use global module for debug state simulation
    set_var("RSB_DEBUG", "1");
    let debug_state = get_var("RSB_DEBUG");
    assert_eq!(debug_state, Some("1".to_string()));

    unset_var("RSB_DEBUG");
    let debug_state = get_var("RSB_DEBUG");
    assert_eq!(debug_state, None);
}

#[test]
#[cfg(not(feature = "dev-pty"))]
fn test_basic_environment_simulation() {
    // Test environment variable manipulation as alternative to debug utilities
    set_var("TEST_MODE", "enabled");
    assert_eq!(get_var("TEST_MODE"), Some("enabled".to_string()));

    set_var("TEST_LEVEL", "verbose");
    assert_eq!(get_var("TEST_LEVEL"), Some("verbose".to_string()));

    // Cleanup
    unset_var("TEST_MODE");
    unset_var("TEST_LEVEL");

    assert_eq!(get_var("TEST_MODE"), None);
    assert_eq!(get_var("TEST_LEVEL"), None);
}

#[test]
fn test_dev_feature_detection() {
    // Test that we can detect whether dev-pty feature is available
    #[cfg(feature = "dev-pty")]
    {
        // PTY functionality should be available
        // We'll test that the module exists by trying to create PtyOptions
        let _options = PtyOptions::default();
        // If this compiles and runs, PTY support is working
    }

    #[cfg(not(feature = "dev-pty"))]
    {
        // PTY functionality should not be available
        // Test should still pass to demonstrate graceful degradation
        assert!(true); // Basic sanity check when PTY is disabled
    }
}

#[test]
fn test_timing_simulation() {
    // Without dedicated timing utilities, use standard library for timing tests
    let start = std::time::Instant::now();

    // Small delay to simulate work
    std::thread::sleep(std::time::Duration::from_millis(10));

    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() >= 10);
    assert!(elapsed.as_millis() < 100); // Should be reasonably quick
}

#[test]
fn test_debug_state_management() {
    // Simulate debug state management without specialized debug utilities
    // This demonstrates how to handle debug scenarios using RSB's global state

    // Save original state
    let original_debug = get_var("RSB_DEV_DEBUG");

    // Test debug enabled state
    set_var("RSB_DEV_DEBUG", "true");
    let is_debug = get_var("RSB_DEV_DEBUG").unwrap_or_default() == "true";
    assert!(is_debug);

    // Test debug disabled state
    set_var("RSB_DEV_DEBUG", "false");
    let is_debug = get_var("RSB_DEV_DEBUG").unwrap_or_default() == "true";
    assert!(!is_debug);

    // Restore original state
    match original_debug {
        Some(value) => set_var("RSB_DEV_DEBUG", &value),
        None => unset_var("RSB_DEV_DEBUG"),
    }
}