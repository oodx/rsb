use rsb::prelude::*;

#[test]
fn os_user_and_paths() {
    // user/home/current dir should be non-empty
    assert!(!user!().is_empty());
    assert!(!home_dir!().is_empty());
    assert!(!current_dir!().is_empty());
    assert!(!hostname!().is_empty());
}

#[test]
fn os_pid_and_process_checks() {
    // Mock pgrep for a fake process
    mock_cmd!({"pgrep 'myproc'" => "4321\n"});
    let pid = pid_of!("myproc");
    assert_eq!(pid, "4321");
    assert!(process_exists!("myproc"));
    mock_cmd!(clear);
}

#[test]
fn os_http_helpers_with_mocks() {
    // GET
    mock_cmd!({"curl -s 'https://example.com'" => "ok"});
    let res = get!("https://example.com");
    assert_eq!(res, "ok");
    // GET with options
    mock_cmd!({"curl -I 'https://example.com'" => "200"});
    let res2 = get!("https://example.com", options: "-I");
    assert_eq!(res2, "200");
    // POST
    mock_cmd!({"curl -s -X POST -d 'x=1' 'https://api'" => "posted"});
    let post = curl!(post: "https://api", data: "x=1");
    assert_eq!(post, "posted");
    mock_cmd!(clear);
}

