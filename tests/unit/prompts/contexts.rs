#![cfg(feature = "prompts")]
use rsb::prelude::*;
use rsb::visual::prompts::{confirm, ask, select};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref TEST_LOCK: Mutex<()> = Mutex::new(());
}

fn setup_clean_context() {
    unset_var("opt_yes");
    unset_var("opt_quiet");
}

#[test]
fn test_prompts_respect_global_context() {
    let _lock = TEST_LOCK.lock().unwrap();
    setup_clean_context();

    // Test global context integration through opt flags
    set_var("opt_quiet", "true");

    let result = confirm("Global context test?");
    assert!(!result);

    // Test context persistence across calls
    let name = ask("Name", Some("test"));
    assert_eq!(name, "test");

    unset_var("opt_quiet");
}

#[test]
fn test_context_isolation() {
    let _lock = TEST_LOCK.lock().unwrap();
    setup_clean_context();

    // Each prompt should read context fresh, not cache
    set_var("opt_yes", "true");
    let result1 = confirm("First call?");
    assert!(result1);

    // Change context mid-test
    unset_var("opt_yes");
    set_var("opt_quiet", "true");

    let result2 = confirm("Second call?");
    assert!(!result2); // Should read new context

    unset_var("opt_quiet");
}

#[test]
fn test_non_tty_fallback_behavior() {
    let _lock = TEST_LOCK.lock().unwrap();
    setup_clean_context();

    // In non-TTY (like CI), prompts should use defaults without blocking
    // This test validates the non-interactive behavior paths

    // confirm() returns false in non-TTY when no opt_yes
    // ask() returns default/empty in non-TTY
    // select() returns first option in non-TTY

    // These are tested indirectly through opt_quiet which simulates non-TTY
    set_var("opt_quiet", "true");

    assert!(!confirm("Non-TTY confirm"));
    assert_eq!(ask("Non-TTY ask", None), "");
    assert_eq!(ask("Non-TTY ask", Some("default")), "default");
    assert_eq!(select("Pick", &["a", "b"], None), "a");

    unset_var("opt_quiet");
}

#[test]
fn test_color_integration() {
    let _lock = TEST_LOCK.lock().unwrap();
    setup_clean_context();
    set_var("opt_quiet", "true"); // Avoid actual TTY interaction

    // Enable colors to test prompt rendering with color codes
    #[cfg(feature = "colors-simple")]
    {
        use rsb::visual::colors::{color_mode, color_enable_with};
        color_mode("always");
        color_enable_with("simple");

        // The functions should work with colors enabled
        let result = confirm("Colored prompt test?");
        assert!(!result); // In quiet mode

        let answer = ask("Colored ask test", Some("default"));
        assert_eq!(answer, "default");
    }

    unset_var("opt_quiet");
}
