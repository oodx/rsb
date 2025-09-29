//! UAT tests for global clear functionality with visual demonstrations

use rsb::prelude::*;
use std::sync::Mutex;

// Global mutex to serialize tests that modify RSB_GLOBAL_RESET
// This prevents race conditions when tests run in parallel
static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn uat_clear_all_demonstration() {
    let _guard = TEST_MUTEX.lock().unwrap();

    println!("\n🧹 UAT: Clear All Globals Demo");
    println!("================================");

    // Clean up any previous test state
    unset_var("RSB_GLOBAL_RESET");
    unset_var("HOME");
    unset_var("app_config");
    unset_var("temp_cache");
    unset_var("session_id");

    // Setup test environment
    set_var("RSB_GLOBAL_RESET", "1");
    set_var("HOME", "/home/demo");
    set_var("app_config", "production");
    set_var("temp_cache", "data");
    set_var("session_id", "abc123");

    println!("\n📝 Initial state:");
    println!("  HOME = {}", get_var("HOME"));
    println!("  app_config = {}", get_var("app_config"));
    println!("  temp_cache = {}", get_var("temp_cache"));
    println!("  session_id = {}", get_var("session_id"));

    // Clear all globals
    match rsb::global::clear_all() {
        Ok(count) => {
            println!("\n✅ Cleared {} non-protected variables", count);
        }
        Err(e) => {
            println!("\n❌ Error: {}", e);
        }
    }

    println!("\n📝 After clear_all:");
    println!("  HOME = {} (protected)", get_var("HOME"));
    println!("  app_config = {}", get_var("app_config"));
    println!("  temp_cache = {}", get_var("temp_cache"));
    println!("  session_id = {}", get_var("session_id"));

    // Verify protected key remains
    assert_eq!(get_var("HOME"), "/home/demo");
    // Verify others are cleared
    assert_eq!(get_var("app_config"), "");
    assert_eq!(get_var("temp_cache"), "");
    assert_eq!(get_var("session_id"), "");

    // Clean up
    unset_var("RSB_GLOBAL_RESET");
    unset_var("HOME");
}

#[test]
fn uat_clear_prefix_demonstration() {
    let _guard = TEST_MUTEX.lock().unwrap();

    println!("\n🎯 UAT: Clear by Prefix Demo");
    println!("================================");

    // Clean up any previous test state
    unset_var("RSB_GLOBAL_RESET");
    unset_var("temp_file1");
    unset_var("temp_file2");
    unset_var("temp_cache");
    unset_var("permanent_data");

    // Setup
    set_var("RSB_GLOBAL_RESET", "1");
    set_var("temp_file1", "data1");
    set_var("temp_file2", "data2");
    set_var("temp_cache", "cache");
    set_var("permanent_data", "important");

    println!("\n📝 Initial state:");
    println!("  temp_file1 = {}", get_var("temp_file1"));
    println!("  temp_file2 = {}", get_var("temp_file2"));
    println!("  temp_cache = {}", get_var("temp_cache"));
    println!("  permanent_data = {}", get_var("permanent_data"));

    // Clear all with "temp_" prefix
    match rsb::global::clear_prefix("temp_") {
        Ok(count) => {
            println!("\n✅ Cleared {} variables with 'temp_' prefix", count);
        }
        Err(e) => {
            println!("\n❌ Error: {}", e);
        }
    }

    println!("\n📝 After clear_prefix('temp_'):");
    println!("  temp_file1 = {}", get_var("temp_file1"));
    println!("  temp_file2 = {}", get_var("temp_file2"));
    println!("  temp_cache = {}", get_var("temp_cache"));
    println!("  permanent_data = {} (preserved)", get_var("permanent_data"));

    // Verify results
    assert_eq!(get_var("temp_file1"), "");
    assert_eq!(get_var("temp_file2"), "");
    assert_eq!(get_var("temp_cache"), "");
    assert_eq!(get_var("permanent_data"), "important");

    // Clean up
    unset_var("RSB_GLOBAL_RESET");
    unset_var("permanent_data");
}

#[test]
fn uat_clear_pattern_demonstration() {
    let _guard = TEST_MUTEX.lock().unwrap();

    println!("\n🔍 UAT: Clear by Pattern Demo");
    println!("================================");

    // Clean up any previous test state
    unset_var("RSB_GLOBAL_RESET");
    unset_var("cache_user_123");
    unset_var("cache_session_456");
    unset_var("cache_global");
    unset_var("config_main");

    // Setup
    set_var("RSB_GLOBAL_RESET", "1");
    set_var("cache_user_123", "data1");
    set_var("cache_session_456", "data2");
    set_var("cache_global", "data3");
    set_var("config_main", "settings");

    println!("\n📝 Initial state:");
    println!("  cache_user_123 = {}", get_var("cache_user_123"));
    println!("  cache_session_456 = {}", get_var("cache_session_456"));
    println!("  cache_global = {}", get_var("cache_global"));
    println!("  config_main = {}", get_var("config_main"));

    // Clear cache entries with numeric suffixes
    match rsb::global::clear_pattern(r"cache_.*_\d+") {
        Ok(count) => {
            println!("\n✅ Cleared {} variables matching pattern 'cache_.*_\\d+'", count);
        }
        Err(e) => {
            println!("\n❌ Error: {}", e);
        }
    }

    println!("\n📝 After clear_pattern:");
    println!("  cache_user_123 = {}", get_var("cache_user_123"));
    println!("  cache_session_456 = {}", get_var("cache_session_456"));
    println!("  cache_global = {} (no numeric suffix)", get_var("cache_global"));
    println!("  config_main = {} (different prefix)", get_var("config_main"));

    // Verify results
    assert_eq!(get_var("cache_user_123"), "");
    assert_eq!(get_var("cache_session_456"), "");
    assert_eq!(get_var("cache_global"), "data3");
    assert_eq!(get_var("config_main"), "settings");

    // Clean up
    unset_var("RSB_GLOBAL_RESET");
    unset_var("cache_global");
    unset_var("config_main");
}

#[test]
fn uat_reset_flag_protection() {
    let _guard = TEST_MUTEX.lock().unwrap();

    println!("\n🔒 UAT: Reset Flag Protection Demo");
    println!("===================================");

    // Clean up any previous test state
    unset_var("RSB_GLOBAL_RESET");
    unset_var("important_data");

    // Try to clear without the flag
    set_var("important_data", "do_not_delete");

    println!("\n📝 Attempting clear without RSB_GLOBAL_RESET flag...");
    println!("  important_data = {}", get_var("important_data"));

    match rsb::global::clear_all() {
        Ok(_) => {
            println!("❌ Unexpected success!");
        }
        Err(e) => {
            println!("✅ Protected: {}", e);
        }
    }

    println!("\n📝 Data remains safe:");
    println!("  important_data = {} (unchanged)", get_var("important_data"));

    assert_eq!(get_var("important_data"), "do_not_delete");

    // Clean up
    unset_var("important_data");
}

#[test]
fn uat_custom_protected_keys() {
    let _guard = TEST_MUTEX.lock().unwrap();

    println!("\n🛡️ UAT: Custom Protected Keys Demo");
    println!("====================================");

    // Clean up any previous test state
    std::env::remove_var("RSB_PROTECTED_KEYS");
    unset_var("RSB_GLOBAL_RESET");
    unset_var("APP_SECRET");
    unset_var("DB_PASSWORD");
    unset_var("temp_data");

    // Set custom protected keys
    std::env::set_var("RSB_PROTECTED_KEYS", "APP_SECRET,DB_PASSWORD");
    set_var("RSB_GLOBAL_RESET", "1");

    // Setup test data
    set_var("APP_SECRET", "super_secret_key");
    set_var("DB_PASSWORD", "secure_pass");
    set_var("temp_data", "can_be_cleared");

    println!("\n📝 Initial state with custom protected keys:");
    println!("  APP_SECRET = {} (protected)", get_var("APP_SECRET"));
    println!("  DB_PASSWORD = {} (protected)", get_var("DB_PASSWORD"));
    println!("  temp_data = {}", get_var("temp_data"));

    // Clear all
    match rsb::global::clear_all() {
        Ok(count) => {
            println!("\n✅ Cleared {} non-protected variables", count);
        }
        Err(e) => {
            println!("\n❌ Error: {}", e);
        }
    }

    println!("\n📝 After clear_all with custom protection:");
    println!("  APP_SECRET = {} (protected)", get_var("APP_SECRET"));
    println!("  DB_PASSWORD = {} (protected)", get_var("DB_PASSWORD"));
    println!("  temp_data = {}", get_var("temp_data"));

    // Verify protected keys remain
    assert_eq!(get_var("APP_SECRET"), "super_secret_key");
    assert_eq!(get_var("DB_PASSWORD"), "secure_pass");
    assert_eq!(get_var("temp_data"), "");

    // Clean up
    std::env::remove_var("RSB_PROTECTED_KEYS");
    unset_var("RSB_GLOBAL_RESET");
    unset_var("APP_SECRET");
    unset_var("DB_PASSWORD");
}

#[test]
fn uat_clear_suffix_demonstration() {
    let _guard = TEST_MUTEX.lock().unwrap();

    println!("\n🔚 UAT: Clear by Suffix Demo");
    println!("================================");

    // Clean up any previous test state
    unset_var("RSB_GLOBAL_RESET");
    unset_var("user_cache");
    unset_var("app_cache");
    unset_var("system_cache");
    unset_var("user_config");

    // Setup
    set_var("RSB_GLOBAL_RESET", "1");
    set_var("user_cache", "cache1");
    set_var("app_cache", "cache2");
    set_var("system_cache", "cache3");
    set_var("user_config", "config1");

    println!("\n📝 Initial state:");
    println!("  user_cache = {}", get_var("user_cache"));
    println!("  app_cache = {}", get_var("app_cache"));
    println!("  system_cache = {}", get_var("system_cache"));
    println!("  user_config = {}", get_var("user_config"));

    // Clear all with "_cache" suffix
    match rsb::global::clear_suffix("_cache") {
        Ok(count) => {
            println!("\n✅ Cleared {} variables with '_cache' suffix", count);
        }
        Err(e) => {
            println!("\n❌ Error: {}", e);
        }
    }

    println!("\n📝 After clear_suffix('_cache'):");
    println!("  user_cache = {}", get_var("user_cache"));
    println!("  app_cache = {}", get_var("app_cache"));
    println!("  system_cache = {}", get_var("system_cache"));
    println!("  user_config = {} (different suffix)", get_var("user_config"));

    // Verify results
    assert_eq!(get_var("user_cache"), "");
    assert_eq!(get_var("app_cache"), "");
    assert_eq!(get_var("system_cache"), "");
    assert_eq!(get_var("user_config"), "config1");

    // Clean up
    unset_var("RSB_GLOBAL_RESET");
    unset_var("user_config");
}