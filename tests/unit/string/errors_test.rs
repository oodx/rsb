use rsb::prelude::*;

#[test]
fn try_prefix_invalid_pattern_errors() {
    let s = "document.txt";
    let err = rsb::string::try_str_prefix(s, "[", false).unwrap_err();
    match err {
        rsb::string::error::StringError::RegexCompile { pattern } => assert_eq!(pattern, "["),
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn case_size_guard_try_variant_errors() {
    // create a string > 64KiB
    let large = "a".repeat(65 * 1024);
    let err = rsb::string::case::try_to_snake_case(&large).unwrap_err();
    match err {
        rsb::string::error::StringError::SizeLimitExceeded { limit, length } => {
            assert_eq!(limit, 64 * 1024);
            assert_eq!(length, large.len());
        }
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn substring_try_out_of_bounds_errors() {
    let s = "hello";
    let err = rsb::string::try_str_sub_abs(s, 99, None).unwrap_err();
    match err {
        rsb::string::error::StringError::IndexOutOfBounds { index, len } => {
            assert_eq!(index, 99);
            assert_eq!(len, s.chars().count());
        }
        _ => panic!("unexpected error: {:?}", err),
    }
}
