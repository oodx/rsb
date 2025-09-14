use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use rsb::prelude::*;

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
fn test_date_and_benchmark_macros() {
    let mut cmd = get_example_cmd();
    cmd.env("DEBUG", "1")
       .env("CARGO_TEST", "1") // Explicitly mark as test environment
       .arg("date-test");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Epoch:"))
        .stderr(predicate::str::contains("Benchmark completed in"));
}

#[test]
fn test_file_in_macro() {
    // Create a temporary directory for testing
    let temp_dir = std::env::temp_dir().join(format!("rsb_file_in_test_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    
    // Create test files using standard rust filesystem functions
    std::fs::write(temp_dir.join("file1.txt"), "content1").unwrap();
    std::fs::write(temp_dir.join("file2.txt"), "content2").unwrap();

    let mut cmd = get_example_cmd();
    cmd.env("CARGO_TEST", "1") // Explicitly mark as test environment
       .arg("file-in-test")
       .arg(&temp_dir);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("file2.txt"));
}

#[test]
fn test_path_split_macro() {
    let mut cmd = get_example_cmd();
    cmd.env("CARGO_TEST", "1") // Explicitly mark as test environment
       .arg("path-test")
       .arg("/tmp/some/file.txt");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Parent: /tmp/some"))
        .stdout(predicate::str::contains("Filename: file.txt"));
}

#[test]
fn test_math_macro() {
    let mut cmd = get_example_cmd();
    cmd.env("CARGO_TEST", "1") // Explicitly mark as test environment
       .arg("math-test");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("C = 26.25"))
        .stdout(predicate::str::contains("C += 1.75 -> 28"));
}

#[test]
fn test_cap_stream_macro() {
    let mut cmd = get_example_cmd();
    // Use a writable temp directory for XDG_TMP
    let tmp = std::env::temp_dir();
    cmd.env("CARGO_TEST", "1") // Explicitly mark as test environment
       .env("XDG_TMP", tmp.to_string_lossy().to_string())
       .arg("cap-stream-test");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Temp file exists."));
}

#[test]
fn test_trap_on_err() {
    let mut cmd = get_example_cmd();
    cmd.env("CARGO_TEST", "1") // Explicitly mark as test environment
       .env("DEBUG_MODE", "1") // Enable info! messages
       .arg("trap-test");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("ERROR TRAP: Command 'run!' failed with status"))
        .stdout(predicate::str::contains("Final error count: 1"));
}

#[test]
fn test_random_macros() {
    let mut cmd = get_example_cmd();
    cmd.env("CARGO_TEST", "1") // Explicitly mark as test environment
       .arg("random-test");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^rand_alnum: .{10}\n").unwrap())
        .stdout(predicate::str::is_match(r"rand_uuid: ........-....-....-....-............\n").unwrap());
}

#[test]
fn test_dict_macros() {
    let mut cmd = get_example_cmd();
    cmd.env("CARGO_TEST", "1") // Explicitly mark as test environment
       .arg("dict-test");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Random word:"))
        .stdout(predicate::str::contains("Generated words:"));
}
