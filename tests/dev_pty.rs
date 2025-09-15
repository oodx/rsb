#![cfg(feature = "dev-pty")]

// PTY wrapper sanity tests

#[test]
fn dev_pty_basic_read_write() {
    use rsb::dev::{spawn_pty, PtyOptions};
    use std::time::Duration;

    // Simple interactive shell flow under a PTY
    // Prints a prompt, waits for input, then echoes the answer
    let cmd = r#"printf 'Name? '; read name; printf 'Hello %s' "$name""#;
    let mut sess = spawn_pty(cmd, &PtyOptions::default()).expect("spawn pty");

    // Read prompt text
    let first = sess.read_for(Duration::from_millis(500)).expect("read prompt");
    assert!(first.contains("Name?"));

    // Send response and read output
    sess.writeln("RSB").expect("write input");
    let out = sess.read_for(Duration::from_millis(500)).expect("read output");
    assert!(out.contains("Hello RSB"), "output was: {}", out);

    let _ = sess.wait();
}

