# RSB Truth/Booleans (Rust-Native)

Updated: 2025-09-16

Purpose
- Use Rust-native booleans (`true`/`false`) across RSB.
- Provide a clear bridge to Unix/POSIX exit codes where needed.

Summary
- Global store uses textual booleans: `"true"` / `"false"` for flags.
- Parsers accept common aliases: `yes/no`, `on/off`, and numeric `1/0` (non-zero → true).
- Macros: `is_true!` and `is_false!` read booleans ergonomically.
- Exit codes: `ExitCodeKind::{Success,Failure,AnnotatedFailure}` map to 0/1/2 with a simple `AsExit` bridge.

Module & API
- Module: `rsb::com`
  - Constants:
    - `com::TRUE: bool = true`, `com::FALSE: bool = false`
    - `com::TRUE_STR: &str = "true"`, `com::FALSE_STR: &str = "false"`
  - Helpers:
    - `com::is_true_val(&str) -> bool`, `com::is_false_val(&str) -> bool`
    - `com::is_true(var: &str) -> bool`, `com::is_false(var: &str) -> bool` (reads from Global)
    - Exit codes: `com::ExitCodeKind::{Success,Failure,AnnotatedFailure}` and `com::AsExit` trait
    - Exit classification helpers: `com::is_success(i32)`, `com::is_fail(i32)`, `com::is_other_fail(i32)`
  - Trait + conversions:
    - `com::ToRSBBool` implemented for `bool`, integer types (non-zero → true), `&str`, `String`
    - `com::is_true_any(&T)`, `com::is_false_any(&T)` for generic checks
  - Macros (re-exported in prelude):
    - `is_true!(expr)` / `is_false!(expr)` — accept bool, number, string, or `var: "KEY"` to read from Global

Conventions
- Options (CLI):
  - `--flag` sets `opt_flag = "true"`
  - `--not-flag` sets `opt_flag = "false"`
  - Short flags `-d -q` set `opt_d = "true"`, `opt_q = "true"`
  - Multi-flag `--multi=dq!ts` → `opt_d=true`, `opt_q=true`, `opt_t=false`, `opt_s=false`
- Environment Modes:
  - Presence of `DEBUG/DEV/QUIET/TRACE` sets `*_MODE = "true"`

Examples
```rust
use rsb::prelude::*;

set_var("opt_quiet", com::TRUE_STR);         // "true"
assert!(is_true!(var: "opt_quiet"));
assert!(is_true!(1));                         // numeric non-zero → true
assert!(is_false!(0));                        // numeric zero → false
assert!(is_true!("yes"));                     // textual alias

// ExitCode bridge
use std::process::ExitCode;
use rsb::com::{AsExit, ExitCodeKind};

fn main() -> ExitCode {
    let ok = true;
    if ok { ExitCodeKind::Success.as_exit() } else { ExitCodeKind::Failure.as_exit() }
}
```

Migration Notes
- Prior experimental behavior used numeric strings (`"0"`/`"1"`) in the store. We reverted to textual booleans.
- Use `is_true!` / `is_false!` macros when testing values to avoid string comparisons.
- For exit status handling use the new `ExitCodeKind` and `AsExit` bridge.

Status
- MODERN: Yes — single source of truth for booleans and exit codes.
- SPEC_ALIGNED: Yes — orchestration in `com/mod.rs`, implementation in `com/utils.rs` and `com/macros.rs`.

