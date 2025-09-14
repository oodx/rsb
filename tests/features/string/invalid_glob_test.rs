use rsb::prelude::*;

#[test]
fn test_invalid_glob_patterns_error_via_try_variants() {
    // Using patterns that would produce invalid regex in our simple glob->regex conversion
    let s = "document.txt";
    // Patterns with wildcards that render invalid regex on naive conversion
    let err = rsb::string::try_str_prefix(s, "*[", false).unwrap_err();
    match err {
        rsb::string::error::StringError::RegexCompile { pattern } => assert_eq!(pattern, "*["),
        _ => panic!("unexpected error: {:?}", err),
    }

    let err2 = rsb::string::try_str_suffix(s, "*[", false).unwrap_err();
    match err2 {
        rsb::string::error::StringError::RegexCompile { pattern } => assert_eq!(pattern, "*["),
        _ => panic!("unexpected error: {:?}", err2),
    }

    // Unbalanced '(' and ')'
    let e3 = rsb::string::try_str_prefix(s, "(*", true).unwrap_err();
    match e3 {
        rsb::string::error::StringError::RegexCompile { pattern } => assert_eq!(pattern, "(*"),
        _ => panic!("unexpected error: {:?}", e3),
    }
    let e4 = rsb::string::try_str_suffix(s, "*)", true).unwrap_err();
    match e4 {
        rsb::string::error::StringError::RegexCompile { pattern } => assert_eq!(pattern, "*)"),
        _ => panic!("unexpected error: {:?}", e4),
    }
}
