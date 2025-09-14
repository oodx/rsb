use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use rsb::prelude::*;

/// Builds the example binary and returns a Command prepared to run it.
fn get_example_cmd() -> Command {
    // Run `cargo build` only once.
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let mut cmd = Command::new("cargo");
        cmd.args(["build", "--example", "showcase"]);
        let status = cmd.status().expect("Failed to build example");
        if !status.success() {
            panic!("Failed to build example");
        }
    });

    // Get the absolute path to the binary
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let binary_path = std::path::Path::new(&manifest_dir)
        .join("target/debug/examples/showcase");

    let cmd = Command::new(binary_path);
    cmd
}

#[test]
fn test_help_command() {
    let mut cmd = get_example_cmd();
    cmd.env("CARGO_TEST", "1") // Explicitly mark as test environment
       .arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("COMMANDS:"));
}

#[test]
fn test_unknown_command() {
    let mut cmd = get_example_cmd();
    cmd.env("CARGO_TEST", "1") // Explicitly mark as test environment
       .arg("nonexistentcommand");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unknown command"));
}

#[test]
fn test_init_command() {
    // Create a temporary directory for testing
    let temp_dir = std::env::temp_dir().join(format!("rsb_test_{}", std::process::id()));
    let project_name = "test-project";
    let project_path = temp_dir.join(project_name);

    let mut cmd = get_example_cmd();
    // Ensure temp directory exists
    std::fs::create_dir_all(&temp_dir).unwrap();
    
    // Run the init command from within the temp directory
    cmd.current_dir(&temp_dir)
       .env("DEBUG", "1") // Enable info/okay messages
       .env("CARGO_TEST", "1") // Explicitly mark as test environment
       .arg("init")
       .arg(project_name);

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Project initialized successfully!"));

    assert!(project_path.exists());
    assert!(project_path.join("README.md").exists());
}

#[test]
fn test_config_set_get() {
    // Create a temporary directory for testing
    let temp_dir = std::env::temp_dir().join(format!("rsb_config_test_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).unwrap();

    let mut cmd_set = get_example_cmd();
    cmd_set.current_dir(&temp_dir)
        .env("DEBUG", "1") // Enable info/okay messages
        .env("CARGO_TEST", "1") // Explicitly mark as test environment
        .arg("config")
        .arg("set")
        .arg("TEST_KEY_PERSIST")
        .arg("VALUE_PERSIST");
    cmd_set.assert().success();

    let mut cmd_get = get_example_cmd();
    cmd_get.current_dir(&temp_dir)
        .env("CARGO_TEST", "1") // Explicitly mark as test environment
        .arg("config")
        .arg("get")
        .arg("TEST_KEY_PERSIST");
    cmd_get.assert()
        .success()
        .stdout(predicate::str::contains("VALUE_PERSIST"));
}

#[test]
fn test_meta_parsing() {
    // Create a temporary directory for testing
    let temp_dir = std::env::temp_dir().join(format!("rsb_meta_test_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let meta_file_path = temp_dir.join("test.sh");
    
    // Write the meta file using standard rust filesystem functions
    std::fs::write(&meta_file_path, "# author : RSB Developer\n# version: 1.0.0\n\necho 'hello'").unwrap();

    let mut cmd = get_example_cmd();
    cmd.current_dir(&temp_dir)
        .env("CARGO_TEST", "1") // Explicitly mark as test environment
        .arg("meta-test")
        .arg(&meta_file_path);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Author: RSB Developer"))
        .stdout(predicate::str::contains("Version: 1.0.0"));
}
