use rsb::prelude::*;

#[test]
fn test_filter_ascii_strip() {
    use rsb::string::utils::filter_ascii_strip;
    assert_eq!(filter_ascii_strip("Hello🌍World"), "HelloWorld");
    assert_eq!(filter_ascii_strip("Crème brûlée"), "Crme brle");
}

#[test]
fn test_filter_ascii_sanitize() {
    use rsb::string::utils::{
        filter_ascii_sanitize, filter_ascii_sanitize_default, ASCII_INVALID_MARKER,
    };
    assert_eq!(
        filter_ascii_sanitize("Hello🌍World", ASCII_INVALID_MARKER),
        "Hello#INV#World"
    );
    assert_eq!(
        filter_ascii_sanitize_default("Crème brûlée"),
        "Cr#INV#me br#INV#l#INV#e"
    );
}
