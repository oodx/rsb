use rsb::prelude::*;
use assert_fs::TempDir;

#[test]
fn sanity_host_paths() {
    println!("\n=== SANITY: Host XDG + RSB Paths ===");
    // Use ephemeral HOME
    let tmp = TempDir::new().unwrap();
    std::env::set_var("HOME", tmp.path());
    for k in ["XDG_CONFIG_HOME","XDG_CACHE_HOME","XDG_DATA_HOME","XDG_HOME","XDG_LIB_HOME","XDG_ETC_HOME","XDG_BIN_HOME","XDG_TMP"] { std::env::remove_var(k); }

    rsb::hosts::setup_xdg_paths();
    rsb::hosts::setup_rsb_paths();

    println!("XDG_HOME => {}", rsb::global::get_var("XDG_HOME"));
    println!("XDG_DATA_HOME => {}", rsb::global::get_var("XDG_DATA_HOME"));
    println!("XDG_TMP_HOME => {}", rsb::global::get_var("XDG_TMP_HOME"));
    println!("RSB_DATA_HOME => {}", rsb::global::get_var("RSB_DATA_HOME"));

    assert!(!rsb::global::get_var("XDG_HOME").is_empty());
    assert!(!rsb::global::get_var("RSB_DATA_HOME").is_empty());
}
