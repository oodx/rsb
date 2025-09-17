//! PTY wrapper for interactive test scenarios (feature `dev-pty`).
//!
//! Minimal, general-purpose abstraction over `portable-pty` to spawn
//! commands under a pseudo terminal and interact via read/write.
//!
//! This module is intended for tests and UATs that need true TTY
//! behavior (prompts, colors, terminal sizing, line buffering).
//!
//! Usage (example):
//! ```ignore
//! use rsb::dev::{PtyOptions, spawn_pty};
//!
//! let mut sess = spawn_pty("printf ok", &PtyOptions::default()).unwrap();
//! let out = sess.read_for(std::time::Duration::from_millis(200)).unwrap();
//! assert!(out.contains("ok"));
//! let _ = sess.wait();
//! ```

use std::io::{Error as IoError, ErrorKind, Read, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use portable_pty::{native_pty_system, CommandBuilder, PtySize};

#[derive(Debug, Clone)]
pub struct PtyOptions {
    pub cols: u16,
    pub rows: u16,
    pub cwd: Option<String>,
}

impl Default for PtyOptions {
    fn default() -> Self {
        Self {
            cols: 80,
            rows: 24,
            cwd: None,
        }
    }
}

pub struct PtySession {
    master: Box<dyn portable_pty::MasterPty + Send>,
    child: Box<dyn portable_pty::Child + Send>,
}

impl PtySession {
    /// Write bytes to the PTY as if typed.
    pub fn write_all(&mut self, data: &[u8]) -> std::io::Result<()> {
        let mut w = self.master.take_writer().map_err(to_io_err)?;
        w.write_all(data)
    }

    /// Convenience: write a line (appends newline).
    pub fn writeln(&mut self, s: &str) -> std::io::Result<()> {
        let mut w = self.master.take_writer().map_err(to_io_err)?;
        w.write_all(s.as_bytes())?;
        w.write_all(b"\n")
    }

    /// Read whatever is available within the duration. Returns empty string on timeout.
    pub fn read_for(&mut self, dur: Duration) -> std::io::Result<String> {
        let (tx, rx) = mpsc::channel();
        let mut reader = self.master.try_clone_reader().map_err(to_io_err)?;

        thread::spawn(move || {
            let mut buf = [0u8; 4096];
            match reader.read(&mut buf) {
                Ok(0) => {
                    let _ = tx.send(Ok(String::new()));
                }
                Ok(n) => {
                    let s = String::from_utf8_lossy(&buf[..n]).to_string();
                    let _ = tx.send(Ok(s));
                }
                Err(e) => {
                    let _ = tx.send(Err(e));
                }
            }
        });

        match rx.recv_timeout(dur) {
            Ok(res) => res,
            Err(_timeout) => Ok(String::new()),
        }
    }

    /// Resize the PTY.
    pub fn resize(&mut self, cols: u16, rows: u16) -> std::io::Result<()> {
        self.master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(to_io_err)
    }

    /// Wait for the child to exit and return its status code.
    pub fn wait(&mut self) -> std::io::Result<i32> {
        let status = self.child.wait().map_err(to_io_err)?;
        Ok(if status.success() { 0 } else { 1 })
    }

    /// Kill the child process.
    pub fn kill(&mut self) -> std::io::Result<()> {
        self.child.kill().map_err(to_io_err)
    }
}

/// Spawn a command inside a PTY. Uses `sh -lc <command>` for portability.
pub fn spawn_pty(command: &str, opts: &PtyOptions) -> std::io::Result<PtySession> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: opts.rows,
            cols: opts.cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(to_io_err)?;

    let mut cmd = CommandBuilder::new("sh");
    cmd.arg("-c");
    cmd.arg(command);
    if let Some(cwd) = &opts.cwd {
        cmd.cwd(cwd);
    }

    let child = pair.slave.spawn_command(cmd).map_err(to_io_err)?;
    drop(pair.slave); // Close slave in parent

    Ok(PtySession {
        master: pair.master,
        child,
    })
}

fn to_io_err<E: std::fmt::Display>(e: E) -> IoError {
    IoError::new(ErrorKind::Other, e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_pty_io() {
        let mut s = spawn_pty("printf ok", &PtyOptions::default()).unwrap();
        let out = s.read_for(Duration::from_millis(500)).unwrap();
        assert!(out.contains("ok"));
        let _ = s.wait();
    }
}
