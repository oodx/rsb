//! UAT tests for flag commands with visual demonstrations

use rsb::cli::{Args, check_flag_commands};
use rsb::global;
use serial_test::serial;

#[test]
#[serial]
fn uat_help_flag_demo() {
    println!("\n=== Help Flag Demo ===\n");

    println!("Testing: prog --help");
    let args = Args::from_strs(&["myapp", "--help"]);
    println!("Args: {:?}", args.all());

    let result = check_flag_commands(&args);
    println!("Result: {:?}", result);
    println!("\n✨ Help flag detected and handled!");

    assert!(result.is_some());
    assert_eq!(result.unwrap(), 0);
}

#[test]
#[serial]
fn uat_version_flag_demo() {
    println!("\n=== Version Flag Demo ===\n");

    println!("Testing: prog --version");
    let args = Args::from_strs(&["myapp", "--version"]);
    println!("Args: {:?}", args.all());

    let result = check_flag_commands(&args);
    println!("Result: {:?}", result);
    println!("\n✨ Version flag detected and handled!");

    assert!(result.is_some());
    assert_eq!(result.unwrap(), 0);
}

#[test]
#[serial]
fn uat_version_with_banner_demo() {
    println!("\n=== Version with Banner Demo ===\n");

    // Set up ASCII art banner
    let banner = r#"
█▀▀▄ █▀▀ █▀▀▄
█▄▄▀ ▀▀█ █▄▄▀
▀ ▀▄ ▀▀▀ ▀ ▀▄
"#;

    global::set_var("RSB_LOGO_ART", banner);
    global::set_var("inf_copyright", "Copyright © 2025 RSB Framework");
    global::set_var("inf_build_info", "Build info: with rust 1.75.0");

    println!("Set RSB_LOGO_ART, inf_copyright, inf_build_info");
    println!("\nTesting: prog --version");
    println!("Expected output:");
    println!("  - ASCII banner");
    println!("  - Version: X.Y.Z | License: LICENSE");
    println!("  - Copyright line");
    println!("  - Build info");

    let args = Args::from_strs(&["myapp", "--version"]);
    let result = check_flag_commands(&args);

    println!("\n✨ Version with custom banner and metadata!");

    // Cleanup
    global::unset_var("RSB_LOGO_ART");
    global::unset_var("inf_copyright");
    global::unset_var("inf_build_info");

    assert!(result.is_some());
    assert_eq!(result.unwrap(), 0);
}

#[test]
#[serial]
fn uat_topic_help_flag_demo() {
    println!("\n=== Topic-Specific Help Flag Demo ===\n");

    // Register a help command
    global::register_function("help", "Show help for commands");

    println!("Testing: prog build --help");
    let args = Args::from_strs(&["myapp", "build", "--help"]);
    println!("Args: {:?}", args.all());
    println!("\nBehavior:");
    println!("  1. Detects 'build' topic before --help flag");
    println!("  2. Checks if 'help' command is registered (it is!)");
    println!("  3. Shows tip to user about 'prog help build'");
    println!("  4. Falls through to generic help");

    let result = check_flag_commands(&args);

    println!("\n✨ Topic help provides guidance even without help router!");

    assert!(result.is_some());
    assert_eq!(result.unwrap(), 0);
}

#[test]
#[serial]
fn uat_flag_detection_anywhere_demo() {
    println!("\n=== Flag Detection Anywhere Demo ===\n");

    println!("RSB idiom: Flags work anywhere in arguments\n");

    let test_cases = vec![
        vec!["prog", "--help"],
        vec!["prog", "build", "--help"],
        vec!["prog", "build", "--debug", "--help"],
        vec!["prog", "--version"],
        vec!["prog", "test", "--verbose", "--version"],
    ];

    for (i, case) in test_cases.iter().enumerate() {
        println!("Test {}: {:?}", i + 1, case);
        let args = Args::from_strs(case);
        let result = check_flag_commands(&args);
        println!("  → Detected: {}", result.is_some());
        assert!(result.is_some(), "Flag should be detected in test case {}", i + 1);
    }

    println!("\n✨ Flags detected anywhere in argument list!");
}

#[test]
#[serial]
fn uat_no_flag_passthrough_demo() {
    println!("\n=== No Flag Passthrough Demo ===\n");

    println!("Testing: prog build --debug");
    let args = Args::from_strs(&["myapp", "build", "--debug"]);
    println!("Args: {:?}", args.all());

    let result = check_flag_commands(&args);
    println!("Result: {:?}", result);
    println!("\nBehavior:");
    println!("  - No --help or --version flag detected");
    println!("  - Returns None to continue to dispatch");
    println!("  - Command 'build' will be dispatched normally");

    println!("\n✨ Non-flag commands pass through to dispatch!");

    assert!(result.is_none());
}