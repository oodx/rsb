// Stdopts feature tests: short-flag expansion to descriptive names

use rsb::prelude::*;

#[test]
fn expands_short_flags_when_feature_enabled() {
    // Simulate CLI args
    let args = vec![
        "rsb-test".to_string(),
        "-d".to_string(),
        "-q".to_string(),
        "-t".to_string(),
        "-D".to_string(),
        "-y".to_string(),
        "-s".to_string(),
    ];
    let args = rsb::cli::Args::new(&args);

    // Populate options into global context
    options!(&args);

    // Base short flags present
    // Short flags are stored under their single-letter keys
    assert_eq!(get_var("opt_d"), "0");
    assert_eq!(get_var("opt_q"), "0");
    assert_eq!(get_var("opt_t"), "0");
    assert_eq!(get_var("opt_D"), "0");
    assert_eq!(get_var("opt_y"), "0");
    assert_eq!(get_var("opt_s"), "0");

    // Descriptive expansions (active only when built with --features stdopts)
    // If feature disabled, these will be empty; with feature, they must be "true".
    #[cfg(feature = "stdopts")]
    {
        assert_eq!(get_var("opt_debug"), "0");
        assert_eq!(get_var("opt_quiet"), "0");
        assert_eq!(get_var("opt_trace"), "0");
        assert_eq!(get_var("opt_dev_mode"), "0");
        assert_eq!(get_var("opt_yes"), "0");
        assert_eq!(get_var("opt_safe"), "0");
    }

    #[cfg(not(feature = "stdopts"))]
    {
        assert_eq!(get_var("opt_debug"), "");
        assert_eq!(get_var("opt_quiet"), "");
        assert_eq!(get_var("opt_trace"), "");
        assert_eq!(get_var("opt_dev_mode"), "");
        assert_eq!(get_var("opt_yes"), "");
        assert_eq!(get_var("opt_safe"), "");
    }
}
