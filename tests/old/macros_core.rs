// core (bootstrap/args/get_env)
use rsb::prelude::*;

#[test]
fn core_bootstrap_and_args() {
    // Simulate process args
    let args_vec = args!();
    assert!(!args_vec.is_empty());

    // Ensure env variables can be imported
    set_var("FOO", "");
    std::env::set_var("FOO", "bar");
    get_env!();
    assert_eq!(get_var("FOO"), "bar");
}

