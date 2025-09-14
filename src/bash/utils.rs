//! Curated wrappers for common bash-style commands.
//! These return `CmdResult` and do not panic by themselves.

use crate::os::CmdResult;

// --- Network ---

/// Simple HTTP GET using curl.
pub fn curl_get(url: &str) -> CmdResult {
    let url_q = crate::string::utils::shell_single_quote(url);
    let cmd = format!("curl -s {}", url_q);
    crate::os::run_cmd_with_status(&cmd)
}

/// HTTP GET with custom options.
pub fn curl_get_with_options(url: &str, options: &str) -> CmdResult {
    let url_q = crate::string::utils::shell_single_quote(url);
    let cmd = format!("curl {} {}", options, url_q);
    crate::os::run_cmd_with_status(&cmd)
}

/// Simple HTTP POST using curl.
pub fn curl_post(url: &str, data: &str) -> CmdResult {
    let url_q = crate::string::utils::shell_single_quote(url);
    let data_q = crate::string::utils::shell_single_quote(data);
    let cmd = format!("curl -s -X POST -d {} {}", data_q, url_q);
    crate::os::run_cmd_with_status(&cmd)
}

// Friendly curl-style namespace aliases
pub mod curl {
    use super::*;
    pub fn get(url: &str) -> CmdResult { curl_get(url) }
    pub fn get_opts(url: &str, options: &str) -> CmdResult { curl_get_with_options(url, options) }
    pub fn post(url: &str, data: &str) -> CmdResult { curl_post(url, data) }
}

// Back-compat http_* names
pub fn http_get(url: &str) -> CmdResult { curl_get(url) }
pub fn http_get_with_options(url: &str, options: &str) -> CmdResult { curl_get_with_options(url, options) }
pub fn http_post(url: &str, data: &str) -> CmdResult { curl_post(url, data) }

// --- Archive Operations ---

/// Creates a tar archive using system tar command.
pub fn create_tar(archive_path: &str, source_paths: &[&str]) -> CmdResult {
    let paths = source_paths.join(" ");
    let cmd = format!("tar -cf '{}' {}", archive_path, paths);
    crate::os::run_cmd_with_status(&cmd)
}

/// Creates a compressed tar.gz archive using system tar command.
pub fn create_tar_gz(archive_path: &str, source_paths: &[&str]) -> CmdResult {
    let paths = source_paths.join(" ");
    let cmd = format!("tar -czf '{}' {}", archive_path, paths);
    crate::os::run_cmd_with_status(&cmd)
}

/// Extracts a tar archive using system tar command.
pub fn extract_tar(archive_path: &str, dest_dir: Option<&str>) -> CmdResult {
    let cmd = if let Some(dir) = dest_dir {
        format!("tar -xf '{}' -C '{}'", archive_path, dir)
    } else {
        format!("tar -xf '{}'", archive_path)
    };
    crate::os::run_cmd_with_status(&cmd)
}

/// Lists contents of a tar archive using system tar command.
pub fn list_tar(archive_path: &str) -> CmdResult {
    let cmd = format!("tar -tf '{}'", archive_path);
    crate::os::run_cmd_with_status(&cmd)
}

/// Creates a zip archive using system zip command.
pub fn create_zip(archive_path: &str, source_paths: &[&str]) -> CmdResult {
    let paths = source_paths.join(" ");
    let cmd = format!("zip -r '{}' {}", archive_path, paths);
    crate::os::run_cmd_with_status(&cmd)
}

/// Extracts a zip archive using system unzip command.
pub fn extract_zip(archive_path: &str, dest_dir: Option<&str>) -> CmdResult {
    let cmd = if let Some(dir) = dest_dir {
        format!("unzip '{}' -d '{}'", archive_path, dir)
    } else {
        format!("unzip '{}'", archive_path)
    };
    crate::os::run_cmd_with_status(&cmd)
}

/// Lists contents of a zip archive using system unzip command.
pub fn list_zip(archive_path: &str) -> CmdResult {
    let cmd = format!("unzip -l '{}'", archive_path);
    crate::os::run_cmd_with_status(&cmd)
}
