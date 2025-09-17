use rsb::prelude::*;

#[test]
fn sanity_cli_args_basic() {
    // Test basic Args construction and access
    let args = Args::new(&["bin".into(), "arg1".into(), "arg2".into()]);

    assert_eq!(args.get(1), Some("arg1".to_string()));
    assert_eq!(args.get(2), Some("arg2".to_string()));
    assert_eq!(args.get(3), None);

    assert_eq!(args.get_or(1, "default"), "arg1");
    assert_eq!(args.get_or(3, "default"), "default");

    assert_eq!(args.len(), 3);
}

#[test]
fn sanity_cli_args_flags() {
    // Test flag detection and consumption
    let mut args = Args::new(&["bin".into(), "--verbose".into(), "--config=test.conf".into(), "-d".into()]);

    assert!(args.has("--verbose"));
    assert!(args.has("-d"));
    assert!(!args.has("--nonexistent"));

    // Test flag with value
    assert_eq!(args.has_val("--config"), Some("test.conf".to_string()));
    assert_eq!(args.has_val("--missing"), None);

    // Test flag consumption
    assert!(args.has_pop("--verbose"));
    assert!(!args.has("--verbose")); // Should be consumed
}

#[test]
fn sanity_cli_args_kv_arrays() {
    // Test key=value and key:array parsing
    let args = Args::new(&["bin".into(), "name=value".into(), "colors:red,green,blue".into()]);

    assert_eq!(args.get_kv("name"), Some("value".to_string()));
    assert_eq!(args.get_kv("missing"), None);

    let colors = args.get_array("colors");
    assert_eq!(colors, Some(vec!["red".to_string(), "green".to_string(), "blue".to_string()]));
    assert_eq!(args.get_array("missing"), None);
}

#[test]
fn sanity_cli_args_remaining() {
    // Test remaining args functionality
    let args = Args::new(&["bin".into(), "cmd".into(), "arg1".into(), "arg2".into()]);

    let remaining = args.remaining();
    assert_eq!(remaining, vec!["cmd", "arg1", "arg2"]);

    let all = args.all();
    assert_eq!(all, vec!["bin", "cmd", "arg1", "arg2"]);

    let joined = args.join(" ");
    assert_eq!(joined, "bin cmd arg1 arg2");
}

#[test]
fn sanity_cli_args_expansion() {
    // Test template expansion
    let args = Args::new(&["bin".into(), "first".into(), "second".into()]);

    // Test positional expansion
    let template = "Args: $1 and $2, total: $#";
    let expanded = args.expand(template);
    assert!(expanded.contains("first"));
    assert!(expanded.contains("second"));
    assert!(expanded.contains("3")); // Total count including bin
}

#[test]
fn sanity_cli_bootstrap() {
    // Test CLI bootstrap functionality
    let test_args = vec!["test_bin".to_string(), "--test-flag".to_string()];

    // Test bootstrap with provided args
    let args = rsb::cli::cli_bootstrap(&test_args);
    assert_eq!(args.get(0), Some("test_bin".to_string()));
    assert!(args.has("--test-flag"));

    // Bootstrap should set up global environment
    // Verify some basic global state is initialized
    assert!(!rsb::global::get_var("HOME").is_empty());
}