#[cfg(test)]
mod tests {
    use rsb::prelude::*;
    use rsb::com::*;

    #[test]
    fn test_boolean_constants() {
        // Test canonical constants
        assert_eq!(TRUE, true);
        assert_eq!(FALSE, false);
        assert_eq!(TRUE_STR, "true");
        assert_eq!(FALSE_STR, "false");
    }

    #[test]
    fn test_is_true_val_basic() {
        // Basic true values
        assert!(is_true_val("true"));
        assert!(is_true_val("TRUE"));
        assert!(is_true_val("True"));
        assert!(is_true_val("yes"));
        assert!(is_true_val("YES"));
        assert!(is_true_val("on"));
        assert!(is_true_val("ON"));
        assert!(is_true_val("1"));

        // Basic false values
        assert!(!is_true_val("false"));
        assert!(!is_true_val("FALSE"));
        assert!(!is_true_val("no"));
        assert!(!is_true_val("NO"));
        assert!(!is_true_val("off"));
        assert!(!is_true_val("OFF"));
        assert!(!is_true_val("0"));
        assert!(!is_true_val(""));
    }

    #[test]
    fn test_is_true_val_numeric() {
        // Non-zero numbers are true
        assert!(is_true_val("1"));
        assert!(is_true_val("42"));
        assert!(is_true_val("-1"));
        assert!(is_true_val("100"));

        // Zero is false
        assert!(!is_true_val("0"));
        assert!(!is_true_val("-0"));

        // Invalid numbers are false
        assert!(!is_true_val("abc"));
        assert!(!is_true_val("1.5")); // Not an i64
    }

    #[test]
    fn test_is_false_val_symmetry() {
        // Function symmetry: is_false_val should be !is_true_val
        let test_values = ["true", "false", "yes", "no", "1", "0", "abc", ""];

        for val in test_values {
            assert_eq!(is_false_val(val), !is_true_val(val),
                      "Symmetry failed for value: {}", val);
        }
    }

    #[test]
    fn test_global_variable_access() {
        // Test is_true/is_false with global variables
        set_var("test_true", TRUE_STR);
        set_var("test_false", FALSE_STR);
        set_var("test_yes", "yes");
        set_var("test_empty", "");

        assert!(is_true("test_true"));
        assert!(!is_false("test_true"));

        assert!(!is_true("test_false"));
        assert!(is_false("test_false"));

        assert!(is_true("test_yes"));
        assert!(!is_false("test_yes"));

        assert!(!is_true("test_empty"));
        assert!(is_false("test_empty"));

        // Non-existent variable should be false
        assert!(!is_true("nonexistent_var"));
        assert!(is_false("nonexistent_var"));
    }

    #[test]
    fn test_macros_basic() {
        // Test is_true!/is_false! macros with direct values
        assert!(is_true!(true));
        assert!(is_true!(1));
        assert!(is_true!("yes"));
        assert!(is_true!("1"));

        assert!(is_false!(false));
        assert!(is_false!(0));
        assert!(is_false!("no"));
        assert!(is_false!("0"));
    }

    #[test]
    fn test_macros_with_variables() {
        // Test is_true!/is_false! macros with variable access
        set_var("macro_test_true", "true");
        set_var("macro_test_false", "false");

        assert!(is_true!(var: "macro_test_true"));
        assert!(!is_false!(var: "macro_test_true"));

        assert!(!is_true!(var: "macro_test_false"));
        assert!(is_false!(var: "macro_test_false"));
    }

    #[test]
    fn test_to_bool_trait() {
        // Test ToBool trait implementations
        assert!(is_true_any(&true));
        assert!(!is_true_any(&false));

        assert!(is_true_any(&1));
        assert!(is_true_any(&-1));
        assert!(!is_true_any(&0));

        assert!(is_true_any(&"yes"));
        assert!(is_true_any(&"true".to_string()));
        assert!(!is_true_any(&"no"));

        // Test is_false_any symmetry
        assert!(!is_false_any(&true));
        assert!(is_false_any(&false));
        assert!(!is_false_any(&1));
        assert!(is_false_any(&0));

        // Test ExitKind ToBool: Success = true, any failure = false
        assert!(is_true_any(&ExitKind::Success));
        assert!(!is_true_any(&ExitKind::Failure));
        assert!(!is_true_any(&ExitKind::SystemFailure));
        assert!(!is_true_any(&ExitKind::LogicFailure));
        assert!(!is_true_any(&ExitKind::UserFailure));
    }

    #[test]
    fn test_exit_kind_enum() {
        // Test ExitKind enum values
        assert_eq!(ExitKind::Success.code(), 0);
        assert_eq!(ExitKind::Failure.code(), 1);
        assert_eq!(ExitKind::SystemFailure.code(), 2);
        assert_eq!(ExitKind::LogicFailure.code(), 3);
        assert_eq!(ExitKind::UserFailure.code(), 4);
    }

    #[test]
    fn test_exit_code_classification() {
        // Test exit code helper functions
        assert!(is_success(0));
        assert!(!is_success(1));
        assert!(!is_success(2));
        assert!(!is_success(-1));

        // is_fail should catch any non-zero exit code
        assert!(!is_fail(0));
        assert!(is_fail(1));
        assert!(is_fail(2));
        assert!(is_fail(127));
        assert!(is_fail(-1));
    }

    #[test]
    fn test_as_exit_trait() {
        use std::process::ExitCode as StdExitCode;

        // Test bool to ExitCode conversion
        assert_eq!(true.as_exit(), StdExitCode::SUCCESS);
        // Note: false.as_exit() creates ExitCode::from(1), harder to test equality

        // Test ExitKind enum conversion
        assert_eq!(ExitKind::Success.as_exit(), StdExitCode::SUCCESS);

        // Test ExitKind conversion (removed unused integer conversions)
        // Integer conversions removed as they weren't used elsewhere in codebase
    }

    #[test]
    fn test_whitespace_handling() {
        // Test that whitespace is properly trimmed
        assert!(is_true_val("  true  "));
        assert!(is_true_val("\ttrue\n"));
        assert!(!is_true_val("  false  "));
        assert!(!is_true_val("\tfalse\n"));
    }

    #[test]
    fn test_case_insensitive() {
        // Test case insensitive parsing
        let true_variants = ["true", "TRUE", "True", "TrUe", "yes", "YES", "Yes", "on", "ON", "On"];
        let false_variants = ["false", "FALSE", "False", "FaLsE", "no", "NO", "No", "off", "OFF", "Off"];

        for variant in true_variants {
            assert!(is_true_val(variant), "Failed for: {}", variant);
        }

        for variant in false_variants {
            assert!(!is_true_val(variant), "Failed for: {}", variant);
        }
    }
}