use rsb::prelude::*;
use assert_fs::TempDir;

#[test]
fn uat_hosts_env_demo() {
    println!("\n=== UAT: Host Env ===");
    println!("Setting TEST_UAT_A in process env to '123'");
    std::env::set_var("TEST_UAT_A", "123");

    println!("Calling import_environment()...");
    rsb::hosts::import_environment();
    let v = rsb::global::get_var("TEST_UAT_A");
    println!("Global TEST_UAT_A => {}", v);

    println!("Setting QUIET=1 in env; applying modes...");
    std::env::set_var("QUIET", "1");
    rsb::hosts::setup_standard_modes();
    println!("QUIET_MODE => {}", rsb::global::get_var("QUIET_MODE"));

    println!("env_bootstrap(): import + modes");
    rsb::hosts::env_bootstrap();
    println!("is_true(QUIET_MODE) => {}", rsb::global::is_true("QUIET_MODE"));

    println!("Mirroring via set_env_var('TEST_UAT_B','xyz')...");
    rsb::hosts::set_env_var("TEST_UAT_B", "xyz");
    println!("Global TEST_UAT_B => {}", rsb::global::get_var("TEST_UAT_B"));
    println!("Env TEST_UAT_B => {}", std::env::var("TEST_UAT_B").unwrap_or_default());

    assert_eq!(v, "123");
    assert!(rsb::global::is_true("QUIET_MODE"));
}

#[test]
fn uat_hosts_paths_demo() {
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