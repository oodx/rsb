# RSB Truth/Booleans (REBEL Semantics)

Updated: 2025-09-15

Purpose
- Define and document RSB's REBEL boolean convention aligned with Unix/POSIX exit codes.
- Provide consistent macros, helpers, and conversions for booleans across the crate.

Summary
- Numeric truth: 0 = true (success), 1 = false (failure)
- String forms in the Global store: "0" = true, "1" = false
- Compatibility: textual aliases are accepted as input ("true"/"false", "yes"/"no", "on"/"off").
- Macros: `is_true!` and `is_false!` for ergonomic checks.

Module & API
- Module: `rsb::com`
  - Constants:
    - `com::TRUE: i32 = 0`, `com::FALSE: i32 = 1`
    - `com::TRUE_STR: &str = "0"`, `com::FALSE_STR: &str = "1"`
  - Helpers:
    - `com::bool_to_i32_rsb(bool) -> i32`
    - `com::i32_to_bool_rsb(i32) -> bool`
    - `com::is_true_val(&str) -> bool`, `com::is_false_val(&str) -> bool`
    - `com::is_true(var: &str) -> bool`, `com::is_false(var: &str) -> bool` (reads from Global)
  - Trait + conversions:
    - `com::ToRSBBool` implemented for `bool`, numeric ints, `&str`, `String`
    - `com::is_true_any(&T)`, `com::is_false_any(&T)` for generic checks
  - Macros (re-exported in prelude):
    - `is_true!(expr)` / `is_false!(expr)` — accept bool, number, string, or use `var: "KEY"` to read from Global

Conventions
- Options (CLI):
  - `--flag` sets `opt_flag = "0"` (true)
  - `--not-flag` sets `opt_flag = "1"` (false)
  - Short flags `-d -q` set `opt_d = "0"`, `opt_q = "0"`
  - Multi-flag `--multi=dq!ts` → `opt_d=0`, `opt_q=0`, `opt_t=1`, `opt_s=1`
- Environment Modes:
  - Presence of `DEBUG/DEV/QUIET/TRACE` sets `*_MODE = "0"` (true)
- Detectors (streamables) return string booleans "0"/"1" for true/false.

Examples
```rust
use rsb::prelude::*;

set_var("opt_quiet", com::TRUE_STR);         // "0"
assert!(is_true!(var: "opt_quiet"));
assert!(is_true!(0));                         // numeric true
assert!(is_false!(1));                        // numeric false
assert!(is_true!("yes"));                     // textual alias
```

Migration Notes
- Prior behavior in some utilities returned textual "true"/"false". These now return "0"/"1".
- Use `is_true!` / `is_false!` macros when testing values to avoid string comparisons.
- If textual booleans are required for interop, map `"0" => "true"`, `"1" => "false"` at boundaries.

Testing
- Core, smoke, visuals, progress, and dev-pty lanes pass with the new semantics.
- Updated tests to assert REBEL truth values.

Status
- MODERN: Yes — module owns truth helpers/macros, single source.
- SPEC_ALIGNED: Yes — orchestration in `com/mod.rs`, implementation in `com/utils.rs` and `com/macros.rs`.

Note (Status Update)
- We previously considered fully inverting legacy textual booleans to numeric REBEL values everywhere. After review, this is under active evaluation. The stable contract for consumers is to use `is_true!` / `is_false!` macros (or `com::is_true[_val]` helpers) rather than relying on raw representations. This shields callers if the internal representation is adjusted again. The current default remains REBEL‑aligned (0=true, 1=false) for flags and detectors.
