//! Operating System Information
//!
//! Pure OS/system discovery without cross-module dependencies.
//! For global-context-aware functions (get_user, get_home, get_pwd), see host_global.rs

use std::process::{Command, Stdio};

// === System Description ===

/// Gets the system hostname
pub fn get_hostname() -> String {
    if let Ok(name) = Command::new("hostname").output() {
        String::from_utf8_lossy(&name.stdout).trim().to_string()
    } else {
        "localhost".to_string()
    }
}

/// CPU architecture (x86_64, aarch64, etc.)
pub fn get_arch() -> String {
    std::env::consts::ARCH.to_string()
}

/// Operating system (linux, macos, windows)
pub fn get_os() -> String {
    std::env::consts::OS.to_string()
}

/// OS family (unix, windows)
pub fn get_family() -> String {
    std::env::consts::FAMILY.to_string()
}

