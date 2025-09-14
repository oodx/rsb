//! Prompts Helpers - Enhanced prompt functions with timeout support
//!
//! This module provides timeout-enhanced versions of the basic prompt functions.
//! It integrates with the global context to read timeout configuration from:
//! 1. --prompt-timeout flag (opt_prompt_timeout)
//! 2. PROMPT_TIMEOUT environment variable
//!
//! Timeout implementation uses thread-based approach for cross-platform compatibility.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::global::get_var;
use super::interactive::{confirm, confirm_default, ask, select};

/// Parse timeout from global context
/// Priority: opt_prompt_timeout (CLI flag) > PROMPT_TIMEOUT (env var) > None
fn parse_timeout_from_context() -> Option<u64> {
    // Check --prompt-timeout flag first
    let opt_val = get_var("opt_prompt_timeout");
    if !opt_val.is_empty() {
        if let Ok(timeout) = opt_val.parse::<u64>() {
            return Some(timeout);
        }
    }

    // Check PROMPT_TIMEOUT environment variable
    let env_val = get_var("PROMPT_TIMEOUT");
    if !env_val.is_empty() {
        if let Ok(timeout) = env_val.parse::<u64>() {
            return Some(timeout);
        }
    }

    None
}

/// Execute a function with timeout using a simpler polling approach
/// Returns Some(result) if completed within timeout, None if timed out
fn with_timeout<F, R>(timeout: Duration, f: F) -> Option<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let result = Arc::new(Mutex::new(None));
    let result_clone = Arc::clone(&result);
    let completed = Arc::new(Mutex::new(false));
    let completed_clone = Arc::clone(&completed);

    let _handle = thread::spawn(move || {
        let value = f();
        *result_clone.lock().unwrap() = Some(value);
        *completed_clone.lock().unwrap() = true;
    });

    // Poll for completion with timeout
    let start = Instant::now();
    while start.elapsed() < timeout {
        if *completed.lock().unwrap() {
            // Extract result
            if let Ok(mut result_guard) = result.lock() {
                return result_guard.take();
            }
        }
        thread::sleep(Duration::from_millis(10));
    }

    None // Timeout occurred
}

/// Confirm with timeout support
/// Returns `default_value` on timeout
pub fn confirm_with_timeout(message: &str, timeout_override: Option<u64>, default_value: bool) -> bool {
    let timeout_secs = timeout_override
        .or_else(parse_timeout_from_context)
        .unwrap_or(30); // 30 second default

    let timeout_duration = Duration::from_secs(timeout_secs);
    let message_owned = message.to_string();

    // Use timeout wrapper
    with_timeout(timeout_duration, move || confirm(&message_owned))
        .unwrap_or(default_value) // Return default on timeout
}

/// Confirm with default and timeout support
pub fn confirm_default_with_timeout(message: &str, default: bool, timeout_override: Option<u64>) -> bool {
    let timeout_secs = timeout_override
        .or_else(parse_timeout_from_context)
        .unwrap_or(30);

    let timeout_duration = Duration::from_secs(timeout_secs);
    let message_owned = message.to_string();

    with_timeout(timeout_duration, move || confirm_default(&message_owned, default))
        .unwrap_or(default) // Return provided default on timeout
}

/// Ask with timeout support
/// Returns `default_value` on timeout
pub fn ask_with_timeout(message: &str, default: Option<&str>, timeout_override: Option<u64>) -> String {
    let timeout_secs = timeout_override
        .or_else(parse_timeout_from_context)
        .unwrap_or(30);

    let timeout_duration = Duration::from_secs(timeout_secs);
    let message_owned = message.to_string();
    let default_owned = default.map(|s| s.to_string());

    with_timeout(timeout_duration, move || {
        ask(&message_owned, default_owned.as_deref())
    })
    .unwrap_or_else(|| default.unwrap_or("").to_string()) // Return default on timeout
}

/// Select with timeout support
/// Returns option at `default_index` on timeout
pub fn select_with_timeout(message: &str, options: &[&str], default_index: Option<usize>, timeout_override: Option<u64>) -> String {
    if options.is_empty() {
        return String::new();
    }

    let timeout_secs = timeout_override
        .or_else(parse_timeout_from_context)
        .unwrap_or(30);

    let timeout_duration = Duration::from_secs(timeout_secs);
    let default_idx = default_index.unwrap_or(0).min(options.len().saturating_sub(1));
    let message_owned = message.to_string();
    let options_owned: Vec<String> = options.iter().map(|s| s.to_string()).collect();

    with_timeout(timeout_duration, move || {
        let options_refs: Vec<&str> = options_owned.iter().map(|s| s.as_str()).collect();
        select(&message_owned, &options_refs, default_index)
    })
    .unwrap_or_else(|| options[default_idx].to_string()) // Return default option on timeout
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::global::{set_var, unset_var};

    #[test]
    fn test_parse_timeout_from_context() {
        // Test CLI flag priority
        set_var("opt_prompt_timeout", "15");
        set_var("PROMPT_TIMEOUT", "25");
        assert_eq!(parse_timeout_from_context(), Some(15));

        // Test env var fallback
        unset_var("opt_prompt_timeout");
        assert_eq!(parse_timeout_from_context(), Some(25));

        // Test no timeout set
        unset_var("PROMPT_TIMEOUT");
        assert_eq!(parse_timeout_from_context(), None);
    }

    #[test]
    fn test_timeout_functions_with_quiet_mode() {
        // In quiet mode, functions should return immediately (no actual timeout needed)
        set_var("opt_quiet", "1");

        let result = confirm_with_timeout("Test?", Some(1), false);
        assert_eq!(result, false); // quiet mode returns false for confirm

        let result = ask_with_timeout("Name?", Some("default"), Some(1));
        assert_eq!(result, "default"); // quiet mode returns provided default

        unset_var("opt_quiet");
    }
}