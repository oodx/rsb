// --- Stream Macros ---
// Namespaced re-exports for selective imports
pub use crate::{cat, cmd, pipe, stream, run, shell};
#[macro_export]
macro_rules! cat {
    ($path:expr) => { $crate::streams::Stream::from_file($path) };
    ($($path:expr),+) => { $crate::streams::Stream::from_files(&[$($path),+]) };
}

#[macro_export]
macro_rules! cmd { ($command:expr) => { $crate::streams::Stream::from_cmd($command) }; }

#[macro_export]
macro_rules! pipe { ($input:expr) => { $crate::streams::Stream::from_string(&$input.to_string()) }; }

#[macro_export]
macro_rules! stream {
    (var: $var:expr) => { $crate::streams::Stream::from_var($var) };
    (files: $($path:expr),+) => { $crate::streams::Stream::from_files(&[$($path),+]) };
    (cmd: $command:expr) => { $crate::streams::Stream::from_cmd($command) };
    (string: $content:expr) => { $crate::streams::Stream::from_string($content) };
    (array: $arr:expr) => { $crate::streams::Stream::from_vec($arr) };
    (delimited: $content:expr, on: $delim:expr) => { $crate::streams::Stream::from_delimited_string($content, $delim) };
}

#[macro_export]
macro_rules! run {
    ($($arg:tt)*) => {
        {
            let cmd_str = format!($($arg)*);
            match $crate::os::shell_exec(&cmd_str, false) {
                Ok(output) => output,
                Err(res) => {
                    $crate::event!(emit "COMMAND_ERROR", "source" => "run!", "command" => &cmd_str, "status" => &res.status.to_string(), "stderr" => &res.error);
                    $crate::fatal!("Shell command failed: {}", cmd_str);
                    
                    // Detect if running in test environment
                    let is_test = std::env::var("CARGO_TEST").is_ok() || std::thread::current().name().map_or(false, |name| name.contains("test"));
                    if !is_test {
                        std::process::exit(res.status);
                    } else {
                        // In test mode, return empty string instead of exiting
                        String::new()
                    }
                }
            }
        }
    };
    ($($arg:tt)*, silent) => {
        match $crate::os::shell_exec(&format!($($arg)*), true) {
            Ok(output) => output,
            Err(_) => String::new(),
        }
    };
}

#[macro_export]
macro_rules! shell {
    ($($arg:tt)*) => {
        {
            let cmd_str = format!($($arg)*);
            $crate::os::run_cmd_with_status(&cmd_str)
        }
    };
}
