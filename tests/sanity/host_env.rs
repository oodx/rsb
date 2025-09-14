use rsb::prelude::*;

#[test]
fn sanity_host_env() {
    println!("\n=== SANITY: Host Env Import + Modes ===");

    // Prepare environment
    std::env::set_var("SANITY_ENV_K", "v");
    std::env::set_var("QUIET", "1");

    // Import and set modes
    rsb::hosts::import_environment();
    rsb::hosts::setup_standard_modes();

    // Visible outputs
    println!("ENV SANITY_ENV_K => {}", rsb::global::get_var("SANITY_ENV_K"));
    println!("MODE QUIET_MODE => {}", rsb::global::get_var("QUIET_MODE"));

    // Sanity asserts
    assert_eq!(rsb::global::get_var("SANITY_ENV_K"), "v");
    assert!(rsb::global::is_true("QUIET_MODE"));
}
