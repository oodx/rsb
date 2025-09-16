use rsb::prelude::*;

#[test]
fn text_macros_basic() {
    set_var("FOO", "Bar Baz");
    assert_eq!(to_number!("42"), 42);
    assert_eq!(to_number!("x", default: 7), 7);
    assert!(str_in!("Bar", in: &get_var("FOO")));
    assert_eq!(param!("FOO", upper), "BAR BAZ");
    assert_eq!(param!("FOO", lower), "bar baz");
    assert_eq!(param!("FOO", replace: "Baz" => "Qux"), "Bar Qux");
    assert_eq!(str_trim!("FOO"), "Bar Baz");
    assert_eq!(str_len!("FOO"), 7);
    assert_eq!(str_line!('-', 5), "-----");

    // explode into array
    str_explode!("a,b,c", on: ',', into: "ARR");
    assert_eq!(array_length("ARR"), 3);
    assert_eq!(array_get("ARR", 1), "b");
}
// text
