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
    assert_eq!(get_var("opt_d"), "1");
    assert_eq!(get_var("opt_q"), "1");
    assert_eq!(get_var("opt_t"), "1");
    assert_eq!(get_var("opt_D"), "1");
    assert_eq!(get_var("opt_y"), "1");
    assert_eq!(get_var("opt_s"), "1");

    // Descriptive expansions (active only when built with --features stdopts)
    // If feature disabled, these will be empty; with feature, they must be "true".
    #[cfg(feature = "stdopts")]
    {
        assert_eq!(get_var("opt_debug"), "1");
        assert_eq!(get_var("opt_quiet"), "1");
        assert_eq!(get_var("opt_trace"), "1");
        assert_eq!(get_var("opt_dev_mode"), "1");
        assert_eq!(get_var("opt_yes"), "1");
        assert_eq!(get_var("opt_safe"), "1");
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
