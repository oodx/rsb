//! Sanity tests for Options Cleanup functionality

use rsb::prelude::*;
use rsb::cli::{Args, OptionsStrategy, options};

// Helper to create string vec from string literals
fn vec_str(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test_options_strategy_default() {
    // Default strategy should keep all arguments
    let mut args = Args::new(&vec_str(&["prog", "file1", "--debug", "file2", "-q"]));
    let context = options(&args);
    args.apply_options_strategy(OptionsStrategy::Default, &context);

    let expected = vec_str(&["prog", "file1", "--debug", "file2", "-q"]);
    assert_eq!(args.all(), expected.as_slice());
}

#[test]
fn test_options_strategy_sort() {
    // Sort strategy should move flags to the end
    let mut args = Args::new(&vec_str(&["prog", "file1", "--debug", "file2", "-q", "file3"]));
    let context = options(&args);
    args.apply_options_strategy(OptionsStrategy::Sort, &context);

    let all = args.all();
    // Program name stays first
    assert_eq!(all[0], "prog");
    // Positional args come next
    assert!(all[1..4].contains(&"file1".to_string()));
    assert!(all[1..4].contains(&"file2".to_string()));
    assert!(all[1..4].contains(&"file3".to_string()));
    // Flags come last
    assert!(all[4..].contains(&"--debug".to_string()));
    assert!(all[4..].contains(&"-q".to_string()));
}

#[test]
fn test_options_strategy_remove() {
    // Remove strategy should remove processed flags
    let mut args = Args::new(&vec_str(&["prog", "file1", "--debug", "file2", "-q"]));
    let context = options(&args);
    args.apply_options_strategy(OptionsStrategy::Remove, &context);

    let remaining = args.all();
    let expected = vec_str(&["prog", "file1", "file2"]);
    assert_eq!(remaining, expected.as_slice());
    assert!(!remaining.contains(&"--debug".to_string()));
    assert!(!remaining.contains(&"-q".to_string()));
}

#[test]
fn test_options_with_values() {
    let mut args = Args::new(&vec_str(&["prog", "--config=app.conf", "file1", "--verbose"]));
    let context = options(&args);

    // Verify options were set
    assert_eq!(get_var("opt_config"), "app.conf");
    assert_eq!(get_var("opt_verbose"), "true");

    args.apply_options_strategy(OptionsStrategy::Remove, &context);
    let expected = vec_str(&["prog", "file1"]);
    assert_eq!(args.all(), expected.as_slice());
}

#[test]
fn test_flag_boundary_detection() {
    // This pattern could be problematic: --flag value (space-separated)
    let args = Args::new(&vec_str(&["prog", "--flag", "value", "--safe=value"]));
    let context = options(&args);

    // Should detect potential boundary issue
    assert!(context.has_boundary_issues);
}

#[test]
fn test_strategy_from_config() {
    // Test loading strategy from environment
    set_var("RSB_OPTIONS_MODE", "sort");
    assert_eq!(OptionsStrategy::from_config(), OptionsStrategy::Sort);

    set_var("RSB_OPTIONS_MODE", "remove");
    assert_eq!(OptionsStrategy::from_config(), OptionsStrategy::Remove);

    set_var("RSB_OPTIONS_MODE", "invalid");
    assert_eq!(OptionsStrategy::from_config(), OptionsStrategy::Default);

    unset_var("RSB_OPTIONS_MODE");
    assert_eq!(OptionsStrategy::from_config(), OptionsStrategy::Default);
}

#[test]
fn test_options_macro_default() {
    let mut args = Args::new(&vec_str(&["prog", "file1", "--debug", "file2"]));
    rsb::options!(&mut args);

    // Options should be set
    assert_eq!(get_var("opt_debug"), "true");
}

#[test]
fn test_options_macro_with_strategy() {
    let mut args = Args::new(&vec_str(&["prog", "file1", "--verbose", "file2"]));
    rsb::options!(&mut args, strategy: "remove");

    // Options should be set
    assert_eq!(get_var("opt_verbose"), "true");
    // Flags should be removed
    assert!(!args.all().contains(&"--verbose".to_string()));
}

#[test]
fn test_options_ex_macro() {
    let mut args = Args::new(&vec_str(&["prog", "file1", "--trace", "file2"]));
    rsb::options_ex!(&mut args, OptionsStrategy::Sort);

    // Options should be set
    assert_eq!(get_var("opt_trace"), "true");
    // Flags should be at the end
    let all = args.all();
    assert!(all.last() == Some(&"--trace".to_string()));
}

// Cleanup helper
fn cleanup() {
    unset_var("RSB_OPTIONS_MODE");
    unset_var("opt_debug");
    unset_var("opt_verbose");
    unset_var("opt_trace");
    unset_var("opt_config");
}