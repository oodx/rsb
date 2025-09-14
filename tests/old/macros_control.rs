use rsb::prelude::*;

#[test]
fn control_and_validation_macros() {
    // test! conditions
    assert!(test!(-z ""));
    assert!(test!(-n "x"));
    assert!(test!("2", -gt, "1"));
    assert!(test!("abc", ==, "abc"));
    assert!(test!("abc", !=, "xyz"));

    // case! regex
    let mut matched = false;
    case!("v1.2.3", {
        "^v\\d+\\.\\d+\\.\\d+$" => { matched = true; },
        _ => { matched = false; }
    });
    assert!(matched);

    // for_in over array
    set_array("ARR", &["x","y","z"]);
    let mut acc = String::new();
    for_in!(val in "ARR" => { acc.push_str(&get_var("val")); });
    assert_eq!(acc, "xyz");
}

#[test]
fn with_lock_macro() {
    let lock_path = std::env::temp_dir().join("rsb_test.lock");
    let lock_path_str = lock_path.to_string_lossy();
    with_lock!(lock_path_str.as_ref() => {
        // inside critical section
        echo!("locked");
    });
    // ensure lock file removed
    assert!(!is_file(&lock_path_str));
}
