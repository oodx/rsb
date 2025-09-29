//! Sanity tests for global clear functionality

use rsb::prelude::*;
use std::sync::Mutex;

// Global mutex to serialize tests that modify RSB_GLOBAL_RESET
// This prevents race conditions when tests run in parallel
static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_clear_all_requires_reset_flag() {
    let _guard = TEST_MUTEX.lock().unwrap();

    // Ensure RSB_GLOBAL_RESET is not set from previous tests
    unset_var("RSB_GLOBAL_RESET");

    // Use unique variable names for this test
    unset_var("noreset_key1");
    unset_var("noreset_key2");

    // Should fail without RSB_GLOBAL_RESET flag
    set_var("noreset_key1", "test_value");
    set_var("noreset_key2", "another_value");

    let result = rsb::global::clear_all();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("RSB_GLOBAL_RESET"));

    // Keys should still exist
    assert_eq!(get_var("noreset_key1"), "test_value");
    assert_eq!(get_var("noreset_key2"), "another_value");

    // Clean up
    unset_var("noreset_key1");
    unset_var("noreset_key2");
}

#[test]
fn test_clear_all_with_reset_flag() {
    let _guard = TEST_MUTEX.lock().unwrap();

    // Use unique variable names for this test
    unset_var("clearall_temp1");
    unset_var("clearall_temp2");
    unset_var("clearall_HOME");
    unset_var("clearall_RESET");

    // Set the reset flag with unique name
    set_var("RSB_GLOBAL_RESET", "1");

    // Set some test keys with unique names
    set_var("clearall_temp1", "value1");
    set_var("clearall_temp2", "value2");
    set_var("HOME", "/home/test");

    // Clear all
    let result = rsb::global::clear_all();
    assert!(result.is_ok());

    // Non-protected keys should be gone
    assert_eq!(get_var("clearall_temp1"), "");
    assert_eq!(get_var("clearall_temp2"), "");

    // Protected keys should remain
    assert_eq!(get_var("HOME"), "/home/test");

    // Clean up
    unset_var("RSB_GLOBAL_RESET");
    unset_var("HOME");
}

#[test]
fn test_clear_prefix() {
    let _guard = TEST_MUTEX.lock().unwrap();

    // Use unique variable names for this test
    unset_var("prefix_test_one");
    unset_var("prefix_test_two");
    unset_var("prefix_keep");
    unset_var("RSB_GLOBAL_RESET");

    set_var("RSB_GLOBAL_RESET", "1");

    // Set various keys with unique prefix
    set_var("prefix_test_one", "1");
    set_var("prefix_test_two", "2");
    set_var("prefix_keep", "3");
    set_var("PATH", "/usr/bin");

    // Clear keys with "prefix_test_" prefix
    let result = rsb::global::clear_prefix("prefix_test_");
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count, 2);

    // Check results
    assert_eq!(get_var("prefix_test_one"), "");
    assert_eq!(get_var("prefix_test_two"), "");
    assert_eq!(get_var("prefix_keep"), "3");
    assert_eq!(get_var("PATH"), "/usr/bin");

    // Clean up
    unset_var("RSB_GLOBAL_RESET");
    unset_var("prefix_keep");
    unset_var("PATH");
}

#[test]
fn test_clear_suffix() {
    let _guard = TEST_MUTEX.lock().unwrap();

    // Use unique variable names for this test
    unset_var("suffix_file_temp");
    unset_var("suffix_cache_temp");
    unset_var("suffix_permanent");
    unset_var("RSB_GLOBAL_RESET");

    set_var("RSB_GLOBAL_RESET", "1");

    // Set various keys with unique prefix
    set_var("suffix_file_temp", "1");
    set_var("suffix_cache_temp", "2");
    set_var("suffix_permanent", "3");

    // Clear keys with "_temp" suffix
    let result = rsb::global::clear_suffix("_temp");
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count, 2);

    // Check results
    assert_eq!(get_var("suffix_file_temp"), "");
    assert_eq!(get_var("suffix_cache_temp"), "");
    assert_eq!(get_var("suffix_permanent"), "3");

    // Clean up
    unset_var("RSB_GLOBAL_RESET");
    unset_var("suffix_permanent");
}

#[test]
fn test_clear_pattern() {
    let _guard = TEST_MUTEX.lock().unwrap();

    // Use unique variable names for this test
    unset_var("pattern_app_config_1");
    unset_var("pattern_app_data_2");
    unset_var("pattern_system");
    unset_var("RSB_GLOBAL_RESET");

    set_var("RSB_GLOBAL_RESET", "1");

    // Set various keys with unique prefix
    set_var("pattern_app_config_1", "1");
    set_var("pattern_app_data_2", "2");
    set_var("pattern_system", "3");
    set_var("USER", "testuser");

    // Clear keys matching pattern "pattern_app_.*_[0-9]"
    let result = rsb::global::clear_pattern(r"pattern_app_.*_[0-9]");
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count, 2);

    // Check results
    assert_eq!(get_var("pattern_app_config_1"), "");
    assert_eq!(get_var("pattern_app_data_2"), "");
    assert_eq!(get_var("pattern_system"), "3");
    assert_eq!(get_var("USER"), "testuser");

    // Clean up
    unset_var("RSB_GLOBAL_RESET");
    unset_var("pattern_system");
    unset_var("USER");
}

#[test]
fn test_clear_pattern_invalid_regex() {
    let _guard = TEST_MUTEX.lock().unwrap();

    // Clean up any previous test state
    unset_var("RSB_GLOBAL_RESET");

    set_var("RSB_GLOBAL_RESET", "1");

    // Try with invalid regex
    let result = rsb::global::clear_pattern("[invalid");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid regex"));

    // Clean up
    unset_var("RSB_GLOBAL_RESET");
}

#[test]
fn test_protected_keys_from_env() {
    let _guard = TEST_MUTEX.lock().unwrap();

    // Use unique variable names for this test
    std::env::remove_var("RSB_PROTECTED_KEYS");
    unset_var("ENVTEST_KEY1");
    unset_var("ENVTEST_KEY2");
    unset_var("envtest_unprotected");
    unset_var("RSB_GLOBAL_RESET");

    // Set custom protected keys via environment
    std::env::set_var("RSB_PROTECTED_KEYS", "ENVTEST_KEY1,ENVTEST_KEY2");
    set_var("RSB_GLOBAL_RESET", "1");

    // Set some keys with unique names
    set_var("ENVTEST_KEY1", "protected1");
    set_var("ENVTEST_KEY2", "protected2");
    set_var("envtest_unprotected", "removeme");

    // Clear all
    let result = rsb::global::clear_all();
    assert!(result.is_ok());

    // Protected keys should remain
    assert_eq!(get_var("ENVTEST_KEY1"), "protected1");
    assert_eq!(get_var("ENVTEST_KEY2"), "protected2");
    assert_eq!(get_var("envtest_unprotected"), "");

    // Clean up
    std::env::remove_var("RSB_PROTECTED_KEYS");
    unset_var("RSB_GLOBAL_RESET");
    unset_var("ENVTEST_KEY1");
    unset_var("ENVTEST_KEY2");
}

#[test]
fn test_clear_operations_return_count() {
    let _guard = TEST_MUTEX.lock().unwrap();

    // Use unique variable names for this test
    unset_var("count_remove1");
    unset_var("count_remove2");
    unset_var("count_remove3");
    unset_var("RSB_GLOBAL_RESET");

    set_var("RSB_GLOBAL_RESET", "1");

    // Set up test data with unique prefix
    set_var("count_remove1", "1");
    set_var("count_remove2", "2");
    set_var("count_remove3", "3");
    set_var("HOME", "/home/test");

    // Test clear_prefix returns correct count
    let result = rsb::global::clear_prefix("count_remove");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3);

    // Clean up
    unset_var("RSB_GLOBAL_RESET");
    unset_var("HOME");
}

#[test]
fn test_protected_keys_not_removed_by_pattern() {
    let _guard = TEST_MUTEX.lock().unwrap();

    // Use unique variable names for this test
    unset_var("PROT_HOMEDIR");
    unset_var("PROT_HOMEWORK");
    unset_var("RSB_GLOBAL_RESET");

    set_var("RSB_GLOBAL_RESET", "1");

    // Set HOME (protected) and other keys with unique prefix
    set_var("HOME", "/home/user");
    set_var("PROT_HOMEDIR", "/homedir");
    set_var("PROT_HOMEWORK", "todo");

    // Try to clear all keys starting with "PROT_HOME"
    let result = rsb::global::clear_prefix("PROT_HOME");
    assert!(result.is_ok());

    // HOME should remain (different prefix), others removed
    assert_eq!(get_var("HOME"), "/home/user");
    assert_eq!(get_var("PROT_HOMEDIR"), "");
    assert_eq!(get_var("PROT_HOMEWORK"), "");

    // Clean up
    unset_var("RSB_GLOBAL_RESET");
    unset_var("HOME");
}