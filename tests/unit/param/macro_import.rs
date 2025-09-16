use rsb::prelude::*;

#[test]
fn test_param_macro_via_prelude_alias() {
    use rsb::prelude::macros::*;
    set_var("P", "v");
    assert_eq!(param!("P"), "v");
    assert_eq!(param!("MISSING", default: "d"), "d");
}

