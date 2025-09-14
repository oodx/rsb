use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

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
fn dispatch_no_args_shows_help() {
    let mut cmd = get_example_cmd();
    cmd.env("CARGO_TEST", "1");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("COMMANDS:"));
}

#[test]
fn pre_dispatch_forwards_args() {
    let mut cmd = get_example_cmd();
    cmd.env("CARGO_TEST", "1")
        .env("DEBUG", "1")
        .arg("install")
        .arg("--force");

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Force installation enabled."))
        .stderr(predicate::str::contains("Simulating package installation..."));
}

#[test]
fn dispatch_forwards_all_args() {
    let mut cmd = get_example_cmd();
    cmd.env("CARGO_TEST", "1")
        .env("DEBUG", "1")
        .arg("build")
        .arg("release")
        .arg("--clean")
        .arg("--version=2.3.4")
        .arg("output=/tmp/rsb-build-test")
        .arg("features=a,b");

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Enabling features: a, b"))
        .stderr(predicate::str::contains("Building"))
        .stderr(predicate::str::contains("for target: release"))
        .stderr(predicate::str::contains("Build successful!"));
}

