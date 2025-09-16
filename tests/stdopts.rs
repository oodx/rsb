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
    assert_eq!(get_var("opt_d"), "true");
    assert_eq!(get_var("opt_q"), "true");
    assert_eq!(get_var("opt_t"), "true");
    assert_eq!(get_var("opt_D"), "true");
    assert_eq!(get_var("opt_y"), "true");
    assert_eq!(get_var("opt_s"), "true");

    // Descriptive expansions (active only when built with --features stdopts)
    // If feature disabled, these will be empty; with feature, they must be "true".
    #[cfg(feature = "stdopts")]
    {
        assert_eq!(get_var("opt_debug"), "true");
        assert_eq!(get_var("opt_quiet"), "true");
        assert_eq!(get_var("opt_trace"), "true");
        assert_eq!(get_var("opt_dev_mode"), "true");
        assert_eq!(get_var("opt_yes"), "true");
        assert_eq!(get_var("opt_safe"), "true");
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
