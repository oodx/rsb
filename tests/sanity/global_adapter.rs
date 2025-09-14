use rsb::prelude::*;

#[test]
fn sanity_global_adapter_simple() {
    println!("\n=== SANITY: Global Adapter (Simple Env) ===");
    std::env::set_var("SANITY_SIMPLE", "yes");
    rsb::global::apply_env_simple();
    println!("SANITY_SIMPLE => {}", get_var("SANITY_SIMPLE"));
    assert_eq!(get_var("SANITY_SIMPLE"), "yes");
}

