// moved from tests/param_test.rs
use rsb::prelude::*;

#[test]
fn test_comprehensive_param_bash_expansion() {
    set_var("TEST_VAR", "/path/to/file.txt");
    set_var("EMPTY_VAR", "");
    set_var("NUMBERS", "12345");
    set_var("WORD", "hello");
    set_var("CAPS", "WORLD");
    set_var("HOME", "/home/testuser");
    set_var("NUM1", "10");
    set_var("NUM2", "5");
    set_var("opt_layout", "k1=v1,k2=v2");

    let home_val = param!("HOME");
    let opt_layout_val = param!("opt_layout", default: "none");
    assert_eq!(home_val, "/home/testuser");
    assert_eq!(opt_layout_val, "k1=v1,k2=v2");

    let empty_default = param!("EMPTY_VAR", default: "fallback");
    let home_default = param!("HOME", default: "fallback");
    let empty_alt = param!("EMPTY_VAR", alt: "has_value");
    let home_alt = param!("HOME", alt: "has_value");
    assert_eq!(empty_default, "fallback");
    assert_eq!(home_default, "/home/testuser");
    assert_eq!(empty_alt, "");
    assert_eq!(home_alt, "has_value");

    let home_len = param!("HOME", len);
    assert_eq!(home_len, 14);

    let home_sub1 = param!("HOME", sub: 1);
    let home_sub2 = param!("HOME", sub: 0, 5);
    let numbers_sub = param!("NUMBERS", sub: 2, 2);
    assert_eq!(home_sub1, "home/testuser");
    assert_eq!(home_sub2, "/home");
    assert_eq!(numbers_sub, "34");

    // Negative substring indices (relative indexing)
    let tail3 = param!("TEST_VAR", sub: -3);
    assert_eq!(tail3, "txt");
    let mid_rel = param!("TEST_VAR", sub: -7, 3);
    assert_eq!(mid_rel, "ile");

    let home_prefix1 = param!("HOME", prefix: "/home");
    let home_prefix2 = param!("HOME", prefix: "/ho", longest);
    let test_var_prefix = param!("TEST_VAR", prefix: "/path/to");
    assert_eq!(home_prefix1, "/testuser");
    let _ = home_prefix2; // not asserted here
    assert_eq!(test_var_prefix, "/file.txt");

    let test_var_suffix1 = param!("TEST_VAR", suffix: ".txt");
    let test_var_suffix2 = param!("TEST_VAR", suffix: "file.txt");
    assert_eq!(test_var_suffix1, "/path/to/file");
    assert_eq!(test_var_suffix2, "/path/to/");

    let test_var_replace1 = param!("TEST_VAR", replace: "/" => "_");
    let test_var_replace2 = param!("TEST_VAR", replace: "/" => "_", all);
    let opt_layout_replace = param!("opt_layout", replace: "," => ";");
    assert_eq!(test_var_replace1, "_path/to/file.txt");
    assert_eq!(test_var_replace2, "_path_to_file.txt");
    assert_eq!(opt_layout_replace, "k1=v1;k2=v2");

    let word_upper_first = param!("WORD", upper: first);
    let caps_lower_first = param!("CAPS", lower: first);
    let test_var_upper_first = param!("TEST_VAR", upper: first);
    let test_var_upper = param!("TEST_VAR", upper);
    let word_lower = param!("WORD", lower);
    assert_eq!(word_upper_first, "Hello");
    assert_eq!(caps_lower_first, "wORLD");
    let _ = test_var_upper_first; // not asserted here
    assert_eq!(test_var_upper, "/PATH/TO/FILE.TXT");
    assert_eq!(word_lower, "hello");

    // Required variable behavior (${VAR:?msg})
    set_var("REQ_PRESENT", "value");
    // When present and non-empty, returns the value. Missing case now hard-exits (covered in UAT/behavior docs)
    assert_eq!(param!("REQ_PRESENT", require: "must be set"), "value");

    let num_42 = to_number!("42");
    let num_param = to_number!(param!("NUM1"));
    let num_invalid = to_number!("invalid", default: 99);
    assert_eq!(num_42, 42);
    assert_eq!(num_param, 10);
    assert_eq!(num_invalid, 99);
}

#[test]
fn test_pattern_case_transforms_first_match() {
    use rsb::prelude::*;
    set_var("NAME", "foo_bar_baz");
    // Uppercase first match of pattern 'ba*' -> first matching substring is 'bar_baz'; only first char 'b' uppercased
    assert_eq!(param!("NAME", upper: "ba*"), "foo_Bar_baz");

    set_var("PHRASE", "alpha BETA gamma");
    // Lowercase first match of pattern 'BE*' -> matches 'BETA'; only first char 'B' lowercased
    assert_eq!(param!("PHRASE", lower: "BE*"), "alpha bETA gamma");
}

#[test]
fn test_options_integration_with_comprehensive_params() {
    let test_args = vec![
        "program".to_string(),
        "command".to_string(),
        "--verbose".to_string(),
        "--config=test.conf".to_string(),
        "-d".to_string(),
        "--layout=k1=v1,k2=v2".to_string(),
    ];

    let args = rsb::cli::Args::new(&test_args);
    options!(&args);

    assert_eq!(param!("opt_verbose"), "true");
    assert_eq!(param!("opt_config"), "test.conf");
    assert_eq!(param!("opt_d"), "true");
    assert_eq!(param!("opt_layout"), "k1=v1,k2=v2");

    assert_eq!(param!("opt_config", suffix: ".conf"), "test");
    assert_eq!(param!("opt_layout", replace: "," => ";"), "k1=v1;k2=v2");
    assert_eq!(param!("opt_config", upper), "TEST.CONF");
}
