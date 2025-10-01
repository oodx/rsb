//! Host Interaction Macros
//!
//! Curated host-related macros. These wrap the `hosts` module utilities
//! and provide a stable, user-friendly surface aligned with MODULE_SPEC.

// Import environment variables into Global via Host layer
#[macro_export]
macro_rules! get_env {
    () => {
        $crate::hosts::import_environment();
    };
}

// Optional host-only bootstrap (without CLI)
#[macro_export]
macro_rules! host_bootstrap {
    () => {
        $crate::hosts::bootstrap_from_env();
    };
}

// Host/system info wrappers
#[macro_export]
macro_rules! hostname {
    () => {
        $crate::hosts::get_hostname()
    };
}

#[macro_export]
macro_rules! user {
    () => {
        $crate::hosts::host_global::get_user()
    };
}

// Host path wrappers
#[macro_export]
macro_rules! home_dir {
    () => {
        $crate::hosts::get_home_dir()
    };
}

#[macro_export]
macro_rules! current_dir {
    () => {
        $crate::hosts::get_current_dir()
    };
}

// === Process Management Macros ===

#[macro_export]
macro_rules! pid_of {
    ($process:expr) => {
        $crate::jobs::pid_of($process)
    };
}

#[macro_export]
macro_rules! process_exists {
    ($process:expr) => {
        $crate::jobs::process_exists($process)
    };
}

#[macro_export]
macro_rules! kill_pid {
    ($pid:expr) => {{
        match $crate::jobs::kill_pid($pid, None) {
            result if result.status == 0 => {
                $crate::okay!("Process {} terminated", $pid);
            }
            result => {
                $crate::error!("Failed to kill process {}: {}", $pid, result.error);
                std::process::exit(result.status);
            }
        }
    }};
    ($pid:expr, signal: $sig:expr) => {{
        match $crate::jobs::kill_pid($pid, Some($sig)) {
            result if result.status == 0 => {
                $crate::okay!("Process {} terminated with {}", $pid, $sig);
            }
            result => {
                $crate::error!("Failed to kill process {}: {}", $pid, result.error);
                std::process::exit(result.status);
            }
        }
    }};
}

#[macro_export]
macro_rules! kill_process {
    ($process:expr) => {{
        match $crate::jobs::kill_process($process, None) {
            result if result.status == 0 => {
                $crate::okay!("Killed all {} processes", $process);
            }
            result => {
                $crate::error!("Failed to kill {}: {}", $process, result.error);
                std::process::exit(result.status);
            }
        }
    }};
    ($process:expr, signal: $sig:expr) => {{
        match $crate::jobs::kill_process($process, Some($sig)) {
            result if result.status == 0 => {
                $crate::okay!("Killed all {} processes with {}", $process, $sig);
            }
            result => {
                $crate::error!("Failed to kill {}: {}", $process, result.error);
                std::process::exit(result.status);
            }
        }
    }};
}

// === Locking Macros ===

#[macro_export]
macro_rules! with_lock {
    ($lock_path:expr => $body:block) => {{
        match $crate::jobs::create_lock($lock_path) {
            Ok(_) => {
                let result = $body;
                $crate::jobs::remove_lock($lock_path);
                result
            }
            Err(e) => {
                $crate::error!("Failed to acquire lock: {}", e);
                std::process::exit(1);
            }
        }
    }};
}

#[macro_export]
macro_rules! lock {
    ($lock_path:expr) => {{
        match $crate::jobs::create_lock($lock_path) {
            Ok(_) => {
                $crate::okay!("Lock acquired: {}", $lock_path);
            }
            Err(e) => {
                $crate::error!("Failed to acquire lock: {}", e);
                std::process::exit(1);
            }
        }
    }};
}

#[macro_export]
macro_rules! unlock {
    ($lock_path:expr) => {
        $crate::jobs::remove_lock($lock_path);
        $crate::okay!("Lock released: {}", $lock_path);
    };
}

// === Validation Macros ===

#[macro_export]
macro_rules! require_command {
    ($cmd:expr) => {
        $crate::validate!($crate::hosts::command::is_command($cmd), "Command not found: {}", $cmd);
    };
}

// === JSON Macros (jq-backed helpers) ===

#[macro_export]
macro_rules! json_get {
    ($json:expr, $path:expr) => {
        $crate::bash::jq::json_get($json, $path)
    };
}

#[macro_export]
macro_rules! json_get_file {
    ($file:expr, $path:expr) => {
        $crate::bash::jq::json_get_file($file, $path)
    };
}

// === Test Helpers ===

#[macro_export]
macro_rules! mock_cmd {
    ({ $($cmd:expr => $out:expr),* $(,)? }) => {{
        let pairs: &[(&str, &str)] = &[ $( ($cmd, $out) ),* ];
        $crate::hosts::command::set_mock_cmds(pairs);
    }};
    (clear) => {{
        $crate::hosts::command::clear_mock_cmds();
    }};
}
