use rsb::prelude::*;

#[test]
fn store_set_get_has_unset() {
    // Use rsb::global interface
    rsb::global::set_var("GL_TEST_KEY", "value");
    assert!(rsb::global::has_var("GL_TEST_KEY"));
    assert_eq!(rsb::global::get_var("GL_TEST_KEY"), "value");
    rsb::global::unset_var("GL_TEST_KEY");
    assert!(!rsb::global::has_var("GL_TEST_KEY"));
}

#[test]
fn expansion_basic_substitution() {
    rsb::global::set_var("HOME", "/home/test");
    let s1 = rsb::global::expand_vars("$HOME/docs");
    let s2 = rsb::global::expand_vars("Path: ${HOME}/docs");
    assert_eq!(s1, "/home/test/docs");
    assert_eq!(s2, "Path: /home/test/docs");
}

#[test]
fn integer_boolean_helpers() {
    rsb::global::set_var("opt_quiet", "true");
    rsb::global::set_var("opt_trace", "false");
    assert!(rsb::global::is_true("opt_quiet"));
    assert!(rsb::global::is_false("opt_trace"));
}

#[test]
fn token_stream_validation() {
    assert!(rsb::global::is_token_stream("a=1,b=2"));
    assert!(rsb::global::is_token_stream("a=1;b=2"));
    assert!(rsb::global::is_token_stream("a=1"));
    assert!(!rsb::global::is_token_stream("=bad"));
}
