//! UAT tests for CLI args to global functionality with visual demonstrations

use rsb::prelude::*;

#[test]
fn uat_cli_args_basic_demo() {
    println!("\n🚀 UAT: CLI Args to Global Demo");
    println!("=================================");

    // Simulate a typical command line
    let args = vec![
        "/usr/bin/myapp".to_string(),
        "build".to_string(),
        "--verbose".to_string(),
        "target/debug".to_string(),
    ];

    println!("\n📝 Simulated command:");
    println!("  $ /usr/bin/myapp build --verbose target/debug");

    // Store args in global
    rsb::cli::cli_to_global(&args);

    println!("\n📦 Stored in global:");
    println!("  cli_prog = {}", cli_prog!());
    println!("  cli_argc = {}", cli_argc!());
    println!("  cli_arg_1 = {}", cli_arg!(1));
    println!("  cli_arg_2 = {}", cli_arg!(2));
    println!("  cli_arg_3 = {}", cli_arg!(3));
    println!("  cli_args = {}", cli_args!());

    // Demonstrate bash-like access
    println!("\n🔧 Bash-like access:");
    println!("  $0 equivalent: {}", get_var("cli_argv_0"));
    println!("  $1 equivalent: {}", cli_arg!(1));
    println!("  $# equivalent: {}", cli_argc!());
    println!("  $@ equivalent: {}", cli_args!());

    // Verify values
    assert_eq!(cli_prog!(), "/usr/bin/myapp");
    assert_eq!(cli_argc!(), 3);
    assert_eq!(cli_arg!(1), "build");
    assert_eq!(cli_arg!(2), "--verbose");
    assert_eq!(cli_arg!(3), "target/debug");
}

#[test]
fn uat_cli_args_macros_demo() {
    println!("\n🎯 UAT: CLI Args Macros Demo");
    println!("==============================");

    let args = vec![
        "calculator".to_string(),
        "add".to_string(),
        "10".to_string(),
        "20".to_string(),
    ];

    rsb::cli::cli_to_global(&args);

    println!("\n📝 Command: calculator add 10 20");
    println!("\n🔍 Using helper macros:");

    // cli_prog! - get program name
    println!("  Program: {}", cli_prog!());

    // cli_argc! - get argument count
    println!("  Arg count: {}", cli_argc!());

    // cli_arg! - get specific argument
    println!("  Operation: {}", cli_arg!(1));
    println!("  First number: {}", cli_arg!(2));
    println!("  Second number: {}", cli_arg!(3));

    // cli_has_arg! - check if arg exists
    println!("\n✅ Argument existence:");
    println!("  Has arg 1? {}", cli_has_arg!(1));
    println!("  Has arg 4? {}", cli_has_arg!(4));

    // cli_argv! - get args as vector
    println!("\n📚 Args as vector:");
    let argv = cli_argv!();
    for (i, arg) in argv.iter().enumerate() {
        println!("    [{}] = {}", i, arg);
    }

    assert_eq!(argv.len(), 3);
    assert_eq!(argv[0], "add");
}

#[test]
fn uat_cli_bootstrap_integration() {
    println!("\n🔄 UAT: Bootstrap Integration Demo");
    println!("====================================");

    let args = vec![
        "deploy.sh".to_string(),
        "production".to_string(),
        "--dry-run".to_string(),
        "v1.2.3".to_string(),
    ];

    println!("\n📝 Command: deploy.sh production --dry-run v1.2.3");

    // Bootstrap automatically stores args
    println!("\n🚀 Calling cli_bootstrap...");
    rsb::cli::cli_bootstrap(&args);

    println!("✅ Args automatically stored in global!");

    println!("\n📦 Retrieved from global:");
    println!("  Script: {}", cli_prog!());
    println!("  Environment: {}", cli_arg!(1));
    println!("  Flag: {}", cli_arg!(2));
    println!("  Version: {}", cli_arg!(3));

    // Show how to use in application logic
    println!("\n💡 Usage example:");
    if cli_arg!(1) == "production" {
        println!("  ⚠️  Production deployment detected!");
    }
    if cli_arg!(2) == "--dry-run" {
        println!("  🔒 Dry run mode - no changes will be made");
    }
    println!("  📌 Deploying version: {}", cli_arg!(3));

    assert_eq!(cli_prog!(), "deploy.sh");
    assert_eq!(cli_argc!(), 3);
}

#[test]
fn uat_cli_args_empty_demo() {
    println!("\n🔲 UAT: No Arguments Demo");
    println!("==========================");

    let args = vec!["standalone".to_string()];

    rsb::cli::cli_to_global(&args);

    println!("\n📝 Command: standalone (no arguments)");
    println!("\n📦 Stored values:");
    println!("  cli_prog = {}", cli_prog!());
    println!("  cli_argc = {}", cli_argc!());
    println!("  cli_args = '{}'", cli_args!());

    println!("\n🔍 Checking for arguments:");
    if cli_argc!() == 0 {
        println!("  ℹ️  No arguments provided - using defaults");
    }

    let argv = cli_argv!();
    println!("  Args vector is empty: {}", argv.is_empty());

    assert_eq!(cli_argc!(), 0);
    assert!(argv.is_empty());
}

#[test]
fn uat_cli_args_with_paths() {
    println!("\n📁 UAT: Path Arguments Demo");
    println!("============================");

    let args = vec![
        "backup.rs".to_string(),
        "/home/user/documents".to_string(),
        "/mnt/backup/2024-01-01".to_string(),
        "--recursive".to_string(),
    ];

    rsb::cli::cli_to_global(&args);

    println!("\n📝 Command: backup.rs /home/user/documents /mnt/backup/2024-01-01 --recursive");

    println!("\n🗂️ Parsed paths:");
    println!("  Source: {}", cli_arg!(1));
    println!("  Destination: {}", cli_arg!(2));
    println!("  Options: {}", cli_arg!(3));

    // Demonstrate path processing
    println!("\n💡 Path validation example:");
    let source = cli_arg!(1);
    let dest = cli_arg!(2);

    if source.starts_with("/") {
        println!("  ✅ Source is absolute path");
    }
    if dest.contains("backup") {
        println!("  ✅ Destination appears to be backup location");
    }
    if cli_arg!(3) == "--recursive" {
        println!("  ✅ Recursive mode enabled");
    }

    assert_eq!(source, "/home/user/documents");
    assert_eq!(dest, "/mnt/backup/2024-01-01");
}

#[test]
fn uat_cli_args_semicolon_handling() {
    println!("\n🔗 UAT: Semicolon Join Demo");
    println!("============================");

    let args = vec![
        "process".to_string(),
        "file1.txt".to_string(),
        "file2.txt".to_string(),
        "file3.txt".to_string(),
    ];

    rsb::cli::cli_to_global(&args);

    println!("\n📝 Command: process file1.txt file2.txt file3.txt");

    println!("\n📦 Joined representation:");
    let joined = cli_args!();
    println!("  cli_args = '{}'", joined);

    println!("\n🔄 Splitting back:");
    for (i, file) in joined.split(';').enumerate() {
        println!("  File {}: {}", i + 1, file);
    }

    // Show vector conversion
    println!("\n📚 As vector:");
    let files = cli_argv!();
    println!("  {} files to process", files.len());
    for file in &files {
        println!("    - {}", file);
    }

    assert_eq!(joined, "file1.txt;file2.txt;file3.txt");
    assert_eq!(files.len(), 3);
}