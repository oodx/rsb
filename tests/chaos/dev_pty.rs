#![cfg(feature = "dev-pty")]

// PTY wrapper sanity tests

#[test]
fn dev_pty_basic_read_write() {
    use rsb::dev::{spawn_pty, PtyOptions};
    use std::time::Duration;

    // Simple interactive shell flow under a PTY
    // Prints a prompt, waits for input, then echoes the answer
    let cmd = r#"printf 'Name? '; read name; sleep 0.1; printf 'Hello %s' "$name""#;
    let mut sess = spawn_pty(cmd, &PtyOptions::default()).expect("spawn pty");

    // Immediately send response; the script will be waiting on read
    sess.writeln("RSB").expect("write input");
    let mut out = String::new();
    for _ in 0..5 {
        out.push_str(&sess.read_for(Duration::from_millis(500)).unwrap_or_default());
        if out.contains("Hello RSB") { break; }
    }
    assert!(out.contains("Hello RSB"), "output was: {}", out);

    let _ = sess.wait();
}
