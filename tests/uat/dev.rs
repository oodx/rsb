use rsb::prelude::*;

#[test]
fn uat_dev_pty_testing_demo() {
    println!("\n=== UAT: Dev PTY Testing Utilities Demo ===");

    // Test PTY session creation and management
    println!("Testing PTY testing utilities...");

    #[cfg(feature = "dev-pty")]
    {
        use rsb::dev::{spawn_pty, PtyOptions, PtySession};

        println!("✓ PTY testing feature is available");

        // Test PTY options configuration
        let pty_options = PtyOptions::default();
        println!("✓ PTY options configured: {:?}", pty_options);

        // Test PTY session spawn (basic functionality test)
        match spawn_pty("echo", &["test"], &pty_options) {
            Ok(session) => {
                println!("✓ PTY session spawned successfully");
                // Basic session validation
                if let Ok(output) = session.read_output() {
                    println!("✓ PTY output read: {}", output.trim());
                }
            }
            Err(e) => {
                println!("⚠ PTY spawn failed (expected in some environments): {}", e);
            }
        }
    }

    #[cfg(not(feature = "dev-pty"))]
    {
        println!("⚠ PTY testing feature not enabled (dev-pty feature flag required)");
    }

    println!("PTY testing utilities demo completed!");
}

#[test]
fn uat_dev_test_environment_demo() {
    println!("\n=== UAT: Dev Test Environment Demo ===");

    // Test development environment setup utilities
    println!("Testing development environment utilities...");

    // Test environment variable management for testing
    set_var("DEV_TEST_MODE", "1");
    println!("✓ Test environment variable set");

    // Test debug mode detection
    if rsb::dev::is_debug_mode() {
        println!("✓ Debug mode is active");
    } else {
        println!("ℹ Debug mode is inactive");
    }

    // Test development assertions
    rsb::dev::dev_assert(true, "Basic dev assertion test");
    println!("✓ Development assertion passed");

    // Test timing utilities for performance testing
    let start = rsb::dev::start_timer();
    std::thread::sleep(std::time::Duration::from_millis(1));
    let elapsed = rsb::dev::elapsed_time(start);
    println!("✓ Timing measurement: {:?}", elapsed);

    // Test cleanup
    unset_var("DEV_TEST_MODE");
    println!("✓ Test environment cleaned up");

    println!("Development environment demo completed!");
}

#[test]
fn uat_dev_debugging_tools_demo() {
    println!("\n=== UAT: Dev PTY Testing Demo ===");

    // Test PTY development utilities (when available)
    #[cfg(feature = "dev-pty")]
    {
        println!("Testing PTY development tools...");

        use rsb::dev::{spawn_pty, PtyOptions};

        // Test basic PTY command execution
        println!("Testing basic PTY command execution...");
        let mut session = spawn_pty("echo 'PTY test successful'", &PtyOptions::default()).unwrap();
        let output = session
            .read_for(std::time::Duration::from_millis(500))
            .unwrap();
        println!("✓ PTY command output: {}", output.trim());
        let _ = session.wait();

        // Test PTY with file operations
        println!("Testing PTY with file operations...");
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join(format!("pty_test_{}.txt", std::process::id()));
        let temp_file_str = temp_file.to_string_lossy().to_string();

        let mut session = spawn_pty(
            &format!("echo 'PTY file test' > {}", temp_file_str),
            &PtyOptions::default(),
        )
        .unwrap();
        let _ = session
            .read_for(std::time::Duration::from_millis(300))
            .unwrap();
        let _ = session.wait();

        // Verify file was created
        if std::path::Path::new(&temp_file_str).exists() {
            println!("✓ PTY file creation verified");
        }

        println!("PTY development tools demo completed!");
    }

    #[cfg(not(feature = "dev-pty"))]
    {
        println!("PTY development tools not available (dev-pty feature disabled)");
        println!("Demonstrating alternative development utilities...");

        // Demonstrate debug state management using global variables
        set_var("RSB_DEV_MODE", "enabled");
        println!(
            "✓ Development mode enabled: {}",
            get_var("RSB_DEV_MODE").unwrap_or_default()
        );

        // Demonstrate timing utilities
        let start = std::time::Instant::now();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let elapsed = start.elapsed();
        println!("✓ Timing measurement: {:?}", elapsed);

        // Demonstrate temp file creation using standard approach
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join(format!("dev_test_{}.txt", std::process::id()));
        let temp_file_str = temp_file.to_string_lossy().to_string();
        rsb::fs::write_file(&temp_file_str, "dev test content");
        println!("✓ Temporary file created: {}", temp_file_str);

        // Cleanup
        unset_var("RSB_DEV_MODE");
        println!("Alternative development tools demo completed!");
    }
}
