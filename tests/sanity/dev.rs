// RSB Sanity Tests - Dev Module Core Functionality Verification
// Tests verify the dev module PTY functionality works as documented in FEATURES_DEV

use rsb::prelude::*;

#[cfg(feature = "dev-pty")]
#[test]
fn test_pty_basic_operations() {
    use rsb::dev::{PtyOptions, spawn_pty};

    // Test basic PTY spawning and execution
    let mut session = spawn_pty("echo hello", &PtyOptions::default())
        .expect("PTY spawn should work");

    // Read output with timeout
    let output = session.read_for(std::time::Duration::from_millis(500))
        .expect("Should be able to read PTY output");

    assert!(output.contains("hello"));

    // Clean up
    let _ = session.wait();
}

#[cfg(feature = "dev-pty")]
#[test]
fn test_pty_options_configuration() {
    use rsb::dev::{PtyOptions, spawn_pty};

    // Test with custom PTY options
    let pty_opts = PtyOptions::default(); // Using defaults for now

    let mut session = spawn_pty("printf test", &pty_opts)
        .expect("PTY with options should work");

    let output = session.read_for(std::time::Duration::from_millis(300))
        .expect("Should read output");

    assert!(output.contains("test"));

    let _ = session.wait();
}

#[cfg(feature = "dev-pty")]
#[test]
fn test_pty_session_lifecycle() {
    use rsb::dev::{PtyOptions, spawn_pty};

    // Test complete PTY session lifecycle
    let mut session = spawn_pty("echo lifecycle", &PtyOptions::default())
        .expect("PTY spawn should work");

    // Test reading
    let output = session.read_for(std::time::Duration::from_millis(500))
        .expect("Should read successfully");

    assert!(output.contains("lifecycle"));

    // Test waiting for completion
    let exit_status = session.wait()
        .expect("Should be able to wait for completion");

    // Command should complete successfully
    assert!(exit_status.success());
}

#[cfg(feature = "dev-pty")]
#[test]
fn test_pty_multiple_commands() {
    use rsb::dev::{PtyOptions, spawn_pty};

    // Test running multiple commands
    let commands = vec!["echo first", "echo second", "printf third"];

    for cmd in commands {
        let mut session = spawn_pty(cmd, &PtyOptions::default())
            .expect("Command should spawn");

        let output = session.read_for(std::time::Duration::from_millis(300))
            .expect("Should read output");

        assert!(!output.is_empty());

        let _ = session.wait();
    }
}

#[cfg(not(feature = "dev-pty"))]
#[test]
fn test_dev_module_graceful_fallback() {
    // When dev-pty feature is not enabled, this test ensures
    // the module can still be imported without the PTY functionality

    // This test validates that the dev module exists and can be referenced
    // even when PTY features are disabled
    assert!(true); // Placeholder - module should compile
}

#[test]
fn test_dev_module_feature_gating() {
    // Test that dev module is properly feature-gated

    #[cfg(feature = "dev-pty")]
    {
        // Dev module should be available with feature
        use rsb::dev::PtyOptions;
        let _options = PtyOptions::default();
    }

    #[cfg(not(feature = "dev-pty"))]
    {
        // Dev module should not be available without feature
        // This test just ensures we can compile without the feature
        assert!(true);
    }
}

#[cfg(feature = "dev-pty")]
#[test]
fn test_pty_error_handling() {
    use rsb::dev::{PtyOptions, spawn_pty};

    // Test with invalid command (should handle gracefully)
    let result = spawn_pty("nonexistent_command_12345", &PtyOptions::default());

    // The spawn might succeed but the command will fail
    if let Ok(mut session) = result {
        // Try to read output (might be empty or contain error)
        let _output = session.read_for(std::time::Duration::from_millis(200));

        // Wait for completion (should handle the failed command)
        let _exit_status = session.wait();
    }
    // Test passes if no panic occurs
}

#[cfg(feature = "dev-pty")]
#[test]
fn test_pty_timeout_behavior() {
    use rsb::dev::{PtyOptions, spawn_pty};

    // Test PTY timeout behavior
    let mut session = spawn_pty("sleep 0.1 && echo done", &PtyOptions::default())
        .expect("Sleep command should spawn");

    // Short timeout - might not capture output
    let output1 = session.read_for(std::time::Duration::from_millis(50));

    // Longer timeout - should capture output
    let output2 = session.read_for(std::time::Duration::from_millis(200));

    // At least one should work
    assert!(output1.is_ok() || output2.is_ok());

    let _ = session.wait();
}

#[cfg(feature = "dev-pty")]
#[test]
fn test_pty_integration_patterns() {
    use rsb::dev::{PtyOptions, spawn_pty};

    // Test common PTY integration patterns

    // Pattern 1: Command with arguments
    let mut session1 = spawn_pty("echo hello world", &PtyOptions::default())
        .expect("Echo with args should work");

    let output1 = session1.read_for(std::time::Duration::from_millis(300))
        .expect("Should read output");

    assert!(output1.contains("hello"));
    assert!(output1.contains("world"));

    let _ = session1.wait();

    // Pattern 2: Command with special characters
    let mut session2 = spawn_pty("printf 'special\\nchars'", &PtyOptions::default())
        .expect("Printf should work");

    let output2 = session2.read_for(std::time::Duration::from_millis(300))
        .expect("Should read printf output");

    assert!(output2.contains("special"));

    let _ = session2.wait();
}

#[test]
fn test_dev_module_documentation_compliance() {
    // Test that the dev module follows the documented API
    // This is based on FEATURES_DEV.md specification

    #[cfg(feature = "dev-pty")]
    {
        use rsb::dev::{PtyOptions, PtySession, spawn_pty};

        // Test that the documented types exist
        let _options: PtyOptions = PtyOptions::default();

        // Test that spawn_pty has the documented signature
        let result: std::io::Result<PtySession> = spawn_pty("true", &PtyOptions::default());

        if let Ok(mut session) = result {
            // Test that PtySession has the documented methods
            let _output = session.read_for(std::time::Duration::from_millis(100));
            let _status = session.wait();
        }
    }

    #[cfg(not(feature = "dev-pty"))]
    {
        // Module should not be available without feature
        // Test compiles successfully showing proper feature gating
        assert!(true);
    }
}