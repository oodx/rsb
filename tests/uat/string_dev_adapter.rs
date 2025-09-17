// String + Dev Cross-Module Integration Tests
// Tests string utilities with PTY development helpers

#[cfg(feature = "dev-pty")]
use rsb::dev::{PtyOptions, spawn_pty};
use rsb::prelude::*;

#[test]
#[cfg(feature = "dev-pty")]
fn uat_string_dev_pty_output_processing() {
    // Test string processing of PTY command output
    let mut sess = spawn_pty("echo 'Hello World'", &PtyOptions::default())
        .expect("PTY spawn should work");

    let output = sess.read_for(std::time::Duration::from_millis(500))
        .expect("PTY read should work");

    // Process PTY output with string utilities
    let cleaned = rsb::string::utils::filter_ascii_strip(&output);
    let trimmed = cleaned.trim();

    // Test case transformations on PTY output
    let snake = rsb::string::to_snake_case(trimmed);
    assert_eq!(snake, "hello_world");

    let kebab = rsb::string::to_kebab_case(trimmed);
    assert_eq!(kebab, "hello-world");

    let _ = sess.wait();
}

#[test]
#[cfg(feature = "dev-pty")]
fn uat_string_dev_command_generation() {
    // Test generating shell commands with string utilities
    let base_cmd = "echo";
    let message = "Test Message With Spaces";

    // Use string utilities to prepare shell-safe command
    let quoted_message = rsb::string::utils::shell_single_quote(message);
    let full_command = format!("{} {}", base_cmd, quoted_message);

    // Execute via PTY and verify output
    let mut sess = spawn_pty(&full_command, &PtyOptions::default())
        .expect("Generated command should execute");

    let output = sess.read_for(std::time::Duration::from_millis(500))
        .expect("Command output should be readable");

    // Verify the original message is preserved in output
    assert!(output.contains("Test Message With Spaces"));

    let _ = sess.wait();
}

#[test]
#[cfg(not(feature = "dev-pty"))]
fn uat_string_dev_fallback_test() {
    // Test string utilities work without dev-pty feature
    let test_str = "Hello World";

    // These should work regardless of dev-pty availability
    assert_eq!(rsb::string::to_snake_case(test_str), "hello_world");
    assert_eq!(rsb::string::to_kebab_case(test_str), "hello-world");

    let quoted = rsb::string::utils::shell_single_quote(test_str);
    assert!(quoted.starts_with('\''));
    assert!(quoted.ends_with('\''));
}