# Bash Utilities (FEATURES_BASH)

Updated: 2025-09-13

Scope
- Curated wrappers and macros for common bash-style commands (curl, tar, zip).
- String-first, process-oriented helpers that delegate to the OS shell.

Module
- `rsb::bash` (module)
  - `bash::curl_get(url: &str) -> CmdResult`
  - `bash::curl_get_with_options(url: &str, options: &str) -> CmdResult`
  - `bash::curl_post(url: &str, data: &str) -> CmdResult`
  - `bash::curl::{get, get_opts, post}` — ergonomic aliases
  - Archives:
    - `bash::create_tar`, `create_tar_gz`, `extract_tar`, `list_tar`
    - `bash::create_zip`, `extract_zip`, `list_zip`

Safety
- Shell quoting: URLs and POST data are wrapped with `string::utils::shell_single_quote` to avoid injection and parsing issues.
- These wrappers call `sh -c` via `os::run_cmd_with_status` and return `CmdResult` (no panic).

Macros (module-owned)
- `curl!(url)` → String (stdout) or exits on failure
- `curl!(url, options: "-I")` → String
- `curl!(post: url, data: payload)` → String
- Archive macros (defined at crate level; re-exported under `bash::macros`):
  - `tar!(create: "a.tar", "file")`, `tar!(extract: "a.tar", to: "dest/")`, `tar!(list: "a.tar")`
  - `tar_gz!(create: "a.tar.gz", "file")`
  - `zip!(create: "a.zip", "dir")`, `zip!(extract: "a.zip", to: "dest/")`, `zip!(list: "a.zip")`
  - `pack!("archive.ext", paths...)` auto-detects format by extension

Examples
```rust
use rsb::prelude::*;
let body = curl!("https://example.com");
let head = curl!("https://example.com", options: "-I");
let resp = curl!(post: "https://example.com", data: "a=1&b=2");
```

Testing
- Sanity: `tests/bash_sanity.rs` → `tests/bash/sanity.rs`
- UAT: `tests/uat_bash.rs` → `tests/uat/bash.rs` (visible outputs, non-fatal on command errors)

Notes
- For programmatic HTTP, consider a gated Rust HTTP client in the future.
- Archive commands depend on system `tar`/`zip`/`unzip` availability.

