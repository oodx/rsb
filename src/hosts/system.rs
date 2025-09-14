//! System Information (user, hostname, process and basic OS checks)

/// Gets the system hostname
pub fn get_hostname() -> String {
    if let Ok(name) = std::process::Command::new("hostname").output() {
        String::from_utf8_lossy(&name.stdout).trim().to_string()
    } else {
        "localhost".to_string()
    }
}

/// Gets the current username
pub fn get_username() -> String {
    if let Ok(user) = std::env::var("USER") {
        user
    } else if let Ok(user) = std::env::var("USERNAME") { // Windows
        user
    } else {
        // Fallback try whoami
        if let Ok(out) = std::process::Command::new("whoami").output() {
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        } else {
            "unknown".to_string()
        }
    }
}

/// CPU architecture (x86_64, aarch64, etc.)
pub fn get_arch() -> String { std::env::consts::ARCH.to_string() }

/// Operating system (linux, macos, windows)
pub fn get_os() -> String { std::env::consts::OS.to_string() }

/// Check if a command exists in PATH
pub fn is_command(cmd: &str) -> bool {
    // which
    if std::process::Command::new("which")
        .arg(cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    { return true; }

    // command -v
    if std::process::Command::new("command")
        .arg("-v")
        .arg(cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    { return true; }

    false
}
