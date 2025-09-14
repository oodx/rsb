use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use assert_fs::TempDir;
use assert_fs::fixture::{PathChild, FileWriteStr};

/// Builds the example binary and returns a Command prepared to run it.
fn get_example_cmd() -> Command {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let mut cmd = Command::new("cargo");
        cmd.args(["build", "--example", "showcase"]);
        let status = cmd.status().expect("Failed to build example");
        if !status.success() {
            panic!("Failed to build example");
        }
    });

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let binary_path = std::path::Path::new(&manifest_dir)
        .join("target/debug/examples/showcase");

    Command::new(binary_path)
}

#[test]
fn test_array_macros() {
    let mut cmd = get_example_cmd();
    cmd.env("DEBUG", "1").arg("array-test");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Length: 3"))
        .stdout(predicate::str::contains("Item 1: b"))
        .stdout(predicate::str::contains("new array: a b c d"));
}

#[test]
fn test_system_macros() {
    let mut cmd = get_example_cmd();
    cmd.env("DEBUG", "1").arg("system-test");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Line: ----------"))
        .stdout(predicate::str::contains("Random number:"));
}
