//! Sanity test for prelude imports
//!
//! Ensures all prelude variants are importable and contain expected items.

use rsb::prelude::*;

#[test]
fn sanity_prelude_standard() {
    // Should have global functions
    set_var("test_key", "test_value");
    assert_eq!(get_var("test_key"), "test_value");
    assert!(has_var("test_key"));

    // Should have Args
    let _args = Args::new(&[]);
}

#[test]
fn sanity_prelude_guards() {
    use rsb::prelude::guards::*;

    // Boolean guards
    assert!(is_true_val("1"));
    assert!(is_false_val("0"));

    // Math predicates
    assert!(is_even(2));
    assert!(is_odd(3));
    assert!(is_positive(1.0));
    assert!(is_negative(-1.0));
    assert!(is_zero(0.0));

    // String guards
    assert!(is_name("valid_name"));
    assert!(!is_name("invalid name")); // spaces not allowed

    // File system guards (just check they're callable)
    let _result = is_file("/tmp/nonexistent");
    let _result = is_dir("/tmp");
}

#[test]
fn sanity_prelude_dev() {
    // Global functions should be available (standard prelude is already imported)
    set_var("dev_test", "value");
    assert_eq!(get_var("dev_test"), "value");

    // Standard library items should be available
    let text = "hello world";
    assert!(text.contains("hello"));
}

#[test]
fn sanity_prelude_ez() {
    use rsb::prelude::ez::*;

    // Should have everything from standard prelude
    set_var("ez_test", "value");
    assert_eq!(get_var("ez_test"), "value");

    // Should have guards
    assert!(is_even(2));

    // Should have standard library
    let text = "test";
    assert!(text.len() > 0);
}

#[test]
fn sanity_prelude_scope_isolation() {
    // Test that we can use different preludes in different scopes
    {
        use rsb::prelude::*;
        set_var("scope1", "value1");
    }

    {
        use rsb::prelude::guards::*;
        assert!(is_even(4));
    }

    {
        use rsb::prelude::ez::*;
        assert!(has_var("scope1"));
    }
}
