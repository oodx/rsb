#![cfg(feature = "prompts")]
use rsb::prelude::*;
use rsb::{confirm, confirm_default, ask, select};

fn setup_test_context() {
    // Ensure clean state
    unset_var("opt_yes");
    unset_var("opt_quiet");
}

#[test]
fn test_confirm_macro_exists() {
    setup_test_context();
    set_var("opt_yes", "true");

    // Test thin macro delegates to function
    let result = confirm!("Test confirmation?");
    assert!(result);

    unset_var("opt_yes");
}

#[test]
fn test_confirm_default_macro_exists() {
    setup_test_context();
    set_var("opt_quiet", "true");

    // Test with default true
    let result_true = confirm_default!("Install now?", true);
    assert!(result_true);

    // Test with default false
    let result_false = confirm_default!("Delete everything?", false);
    assert!(!result_false);

    unset_var("opt_quiet");
}

#[test]
fn test_ask_macro_variants() {
    setup_test_context();
    set_var("opt_quiet", "true");

    // Test single argument form (no default)
    let result_no_default = ask!("Enter your name");
    assert_eq!(result_no_default, "");

    // Test two argument form (with default)
    let result_with_default = ask!("Enter your name", "anonymous");
    assert_eq!(result_with_default, "anonymous");

    unset_var("opt_quiet");
}

#[test]
fn test_select_macro_variants() {
    setup_test_context();
    set_var("opt_quiet", "true");

    let options = &["alpha", "beta", "gamma"];

    // Test without default (uses index 0)
    let result_no_default = select!("Choose option", options);
    assert_eq!(result_no_default, "alpha");

    // Test with default index
    let result_with_default = select!("Choose option", options, 1);
    assert_eq!(result_with_default, "beta");

    unset_var("opt_quiet");
}

#[test]
fn test_macros_follow_thin_pattern() {
    setup_test_context();
    set_var("opt_yes", "true");

    // Verify macros delegate to the actual functions
    // by ensuring they return same results as direct function calls
    let macro_result = confirm!("Proceed?");
    let function_result = rsb::visual::prompts::confirm("Proceed?");
    assert_eq!(macro_result, function_result);

    unset_var("opt_yes");
}
