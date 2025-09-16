use rsb::prelude::*;

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
