#![cfg(feature = "prompts")]
use rsb::prelude::*;
use rsb::visual::prompts::{confirm, confirm_default, ask, select, default_from};

fn setup_clean_context() {
    unset_var("opt_yes");
    unset_var("opt_quiet");
    unset_var("test_key");
}

#[test]
fn test_confirm_with_opt_yes() {
    setup_clean_context();
    set_var("opt_yes", "true");

    // Should always return true when opt_yes is set
    let result = confirm("Delete important files?");
    assert!(result);

    unset_var("opt_yes");
}

#[test]
fn test_confirm_with_opt_quiet() {
    setup_clean_context();
    set_var("opt_quiet", "true");

    // Should return false in quiet mode (conservative default)
    let result = confirm("Proceed with installation?");
    assert!(!result);

    unset_var("opt_quiet");
}

#[test]
fn test_confirm_default_respects_defaults() {
    setup_clean_context();
    set_var("opt_quiet", "true");

    // Should respect the provided default in quiet mode
    let result_true = confirm_default("Continue?", true);
    assert!(result_true);

    let result_false = confirm_default("Delete?", false);
    assert!(!result_false);

    unset_var("opt_quiet");
}

#[test]
fn test_ask_with_defaults() {
    setup_clean_context();
    set_var("opt_quiet", "true");

    // No default provided - should return empty string
    let result_empty = ask("Enter name", None);
    assert_eq!(result_empty, "");

    // Default provided - should return default
    let result_default = ask("Enter name", Some("anonymous"));
    assert_eq!(result_default, "anonymous");

    unset_var("opt_quiet");
}

#[test]
fn test_select_with_options() {
    setup_clean_context();
    set_var("opt_quiet", "true");

    let options = &["red", "green", "blue"];

    // No default index - should use index 0
    let result_first = select("Choose color", options, None);
    assert_eq!(result_first, "red");

    // With default index
    let result_second = select("Choose color", options, Some(1));
    assert_eq!(result_second, "green");

    // Default index out of bounds - should clamp to last valid
    let result_clamped = select("Choose color", options, Some(10));
    assert_eq!(result_clamped, "blue");

    unset_var("opt_quiet");
}

#[test]
fn test_select_empty_options() {
    setup_clean_context();
    set_var("opt_quiet", "true");

    let empty_options: &[&str] = &[];
    let result = select("Choose", empty_options, None);
    assert_eq!(result, "");

    unset_var("opt_quiet");
}

#[test]
fn test_default_from_helper() {
    setup_clean_context();

    // Key not set - should return fallback
    let result_fallback = default_from("nonexistent_key", "fallback");
    assert_eq!(result_fallback, "fallback");

    // Key set - should return value from context
    set_var("test_key", "context_value");
    let result_context = default_from("test_key", "fallback");
    assert_eq!(result_context, "context_value");

    unset_var("test_key");
}

#[test]
fn test_opt_yes_overrides_opt_quiet() {
    setup_clean_context();

    // Both flags set - opt_yes should take precedence
    set_var("opt_yes", "true");
    set_var("opt_quiet", "true");

    let result = confirm("Test precedence?");
    assert!(result); // opt_yes wins

    unset_var("opt_yes");
    unset_var("opt_quiet");
}
