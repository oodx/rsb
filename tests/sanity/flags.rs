//! Sanity tests for flag commands (--help, --version)

use rsb::cli::{Args, check_flag_commands};
use rsb::global;
use serial_test::serial;

#[test]
#[serial]
fn sanity_help_flag_short() {
    let args = Args::from_strs(&["prog", "-h"]);
    let result = check_flag_commands(&args);
    assert!(result.is_some(), "Help flag should be detected");
    assert_eq!(result.unwrap(), 0, "Help should exit with code 0");
}

#[test]
#[serial]
fn sanity_help_flag_long() {
    let args = Args::from_strs(&["prog", "--help"]);
    
    let result = check_flag_commands(&args);
    assert!(result.is_some(), "Help flag should be detected");
    assert_eq!(result.unwrap(), 0, "Help should exit with code 0");
}

#[test]
#[serial]
fn sanity_version_flag_short() {
    let args = Args::from_strs(&["prog", "-v"]);
    
    let result = check_flag_commands(&args);
    assert!(result.is_some(), "Version flag should be detected");
    assert_eq!(result.unwrap(), 0, "Version should exit with code 0");
}

#[test]
#[serial]
fn sanity_version_flag_long() {
    let args = Args::from_strs(&["prog", "--version"]);
    
    let result = check_flag_commands(&args);
    assert!(result.is_some(), "Version flag should be detected");
    assert_eq!(result.unwrap(), 0, "Version should exit with code 0");
}

#[test]
#[serial]
fn sanity_topic_help_flag() {
    let args = Args::from_strs(&["prog", "build", "--help"]);
    
    let result = check_flag_commands(&args);
    assert!(result.is_some(), "Topic help flag should be detected");
    assert_eq!(result.unwrap(), 0, "Topic help should exit with code 0");
}

#[test]
#[serial]
fn sanity_no_flag() {
    let args = Args::from_strs(&["prog", "build"]);
    
    let result = check_flag_commands(&args);
    assert!(result.is_none(), "No flag should return None");
}

#[test]
#[serial]
fn sanity_flag_not_first() {
    // Flags anywhere in args should be detected
    let args = Args::from_strs(&["prog", "build", "--version"]);
    
    let result = check_flag_commands(&args);
    assert!(result.is_some(), "Version flag should be detected anywhere");
    assert_eq!(result.unwrap(), 0);
}

#[test]
#[serial]
fn sanity_multiple_args_with_help() {
    let args = Args::from_strs(&["prog", "build", "--debug", "--help"]);
    
    let result = check_flag_commands(&args);
    assert!(result.is_some(), "Help flag should be detected with other args");
    assert_eq!(result.unwrap(), 0);
}

#[test]
#[serial]
fn sanity_version_with_logo_art() {
    // Test that version uses RSB_LOGO_ART if present
    global::set_var("RSB_LOGO_ART", "TEST BANNER");
    let args = Args::from_strs(&["prog", "--version"]);
    
    let result = check_flag_commands(&args);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 0);
    // Cleanup
    global::unset_var("RSB_LOGO_ART");
}

#[test]
#[serial]
fn sanity_version_with_inf_copyright() {
    // Test that version uses inf_copyright from TOML snooping
    global::set_var("inf_copyright", "Copyright © 2025 Test Author");
    let args = Args::from_strs(&["prog", "--version"]);
    
    let result = check_flag_commands(&args);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 0);
    // Cleanup
    global::unset_var("inf_copyright");
}