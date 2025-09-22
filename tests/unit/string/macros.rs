use rsb::prelude::*;

#[test]
fn test_string_macros_basic() {
    // str_in!
    assert!(str_in!("Bar", in: "Foo Bar Baz"));
    assert!(!str_in!("Qux", in: "Foo Bar Baz"));

    // str_line!
    assert_eq!(str_line!('-', 5), "-----");

    // Context-bound macros: str_trim!/str_len!
    set_var("FOO", "  Bar Baz  ");
    assert_eq!(str_trim!("FOO"), "Bar Baz");
    assert_eq!(str_len!("FOO"), get_var("FOO").len());

    // str_explode! into array
    str_explode!("a,b,c", on: ',', into: "ARR");
    assert_eq!(array_length("ARR"), 3);
    assert_eq!(array_get("ARR", 1), "b");
}

#[test]
fn test_string_macros_via_prelude_alias() {
    // Ensure prelude::macros alias exposes string macros
    use rsb::prelude::macros::*;
    assert!(str_in!("lo", in: "hello"));
    assert_eq!(str_line!('=', 3), "===");
}
