// moved from tests/param_helpers.rs
use rsb::param::basic as p;
use rsb::prelude::*;

#[test]
fn test_basic_helpers() {
    set_var("FOO", "/home/user/file.txt");
    assert_eq!(p::get("FOO"), "/home/user/file.txt");
    assert_eq!(p::sub(&p::get("FOO"), 0, Some(5)), "/home");
    assert_eq!(p::prefix(&p::get("FOO"), "/home", false), "/user/file.txt");
    assert_eq!(p::suffix(&p::get("FOO"), ".txt", false), "/home/user/file");
    assert_eq!(
        p::replace(&p::get("FOO"), "/", "_", false),
        "_home/user/file.txt"
    );
    assert_eq!(p::upper(&p::get("FOO"), true), "/HOME/USER/FILE.TXT");
    assert_eq!(p::lower(&p::get("FOO"), true), "/home/user/file.txt");
    assert!(p::len("FOO") > 0);
}
