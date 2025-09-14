use rsb::prelude::*;
use assert_fs::TempDir;

#[test]
fn uat_host_paths_demo() {
    println!("\n=== UAT: Host Paths (XDG + RSB) ===");
    let tmp = TempDir::new().unwrap();
    std::env::set_var("HOME", tmp.path());
    for k in ["XDG_CONFIG_HOME","XDG_CACHE_HOME","XDG_DATA_HOME","XDG_HOME","XDG_LIB_HOME","XDG_ETC_HOME","XDG_BIN_HOME","XDG_TMP"] { std::env::remove_var(k); }

    println!("Calling setup_xdg_paths()...");
    rsb::hosts::setup_xdg_paths();
    println!("XDG_HOME => {}", rsb::global::get_var("XDG_HOME"));
    println!("XDG_DATA_HOME => {}", rsb::global::get_var("XDG_DATA_HOME"));
    println!("Ensuring directories...");
    rsb::hosts::ensure_xdg_directories();

    println!("Calling setup_rsb_paths()...");
    rsb::hosts::setup_rsb_paths();
    println!("RSB_LIB_HOME => {}", rsb::global::get_var("RSB_LIB_HOME"));
    println!("RSB_BIN_HOME => {}", rsb::global::get_var("RSB_BIN_HOME"));

    let args = vec![tmp.path().join("demo.sh").to_string_lossy().to_string()];
    println!("Calling setup_execution_context(args)...");
    rsb::hosts::setup_execution_context(&args);
    println!("SCRIPT_NAME => {}", rsb::global::get_var("SCRIPT_NAME"));
    println!("SCRIPT_DIR => {}", rsb::global::get_var("SCRIPT_DIR"));
}
