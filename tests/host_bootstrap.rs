use rsb::prelude::*;

#[test]
fn host_bootstrap_sets_expected_keys() {
    // Temp home and args
    let home = assert_fs::TempDir::new().unwrap();
    // Prefer XDG_HOME policy over altering HOME directly
    std::env::set_var("XDG_HOME", home.path());

    let demo = home.path().join("script.sh");
    std::fs::write(&demo, b"echo").unwrap();
    let args = vec![demo.to_string_lossy().to_string(), "a".into(), "b".into()];

    rsb::hosts::bootstrap(&args);

    // XDG and RSB
    assert!(rsb::global::get_var("XDG_HOME").ends_with(".local") || rsb::global::get_var("XDG_HOME").ends_with(home.path().to_string_lossy().as_ref()));
    assert!(!rsb::global::get_var("RSB_LIB_HOME").is_empty());

    // Script + args
    assert_eq!(rsb::global::get_var("SCRIPT_NAME"), "script.sh");
    assert_eq!(rsb::global::get_var("ARGC"), "3");
    assert_eq!(rsb::global::get_var("ARGV_1"), "a");
}

