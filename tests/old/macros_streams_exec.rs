use rsb::prelude::*;

#[test]
fn stream_exec_macros() {
    // pipe + grep + to_string
    let out = pipe!("a\nb\na")
        .grep("a")
        .to_string();
    assert_eq!(out, "a\na");

    // run! with mocked command
    mock_cmd!({"echo hello" => "hello\n"});
    let s = run!("echo hello");
    assert_eq!(s, "hello");
    mock_cmd!(clear);

    // shell! returns status and output
    mock_cmd!({"printf 1" => "1"});
    let res = shell!("printf 1");
    assert_eq!(res.status, 0);
}
// streams_exec
