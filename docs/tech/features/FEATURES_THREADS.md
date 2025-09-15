# Threads & Jobs (FEATURES_THREADS)

Updated: 2025-09-13

Scope
- Provide thread utilities (sleep, benchmarking) and background job control with a simple, bash-like API.

Module
- `rsb::threads` (module)
  - `threads::sleep_ms(ms: u64)` — sleep for N milliseconds
  - `threads::bench(label, || { ... }) -> Duration` — measure elapsed time, logs info
  - `threads::start_background(cmd: &str) -> u32` — spawn background job (shell command), return job ID
  - `threads::wait(job_id, timeout_secs: Option<u64>) -> Result<i32, String>` — wait for job completion
  - `threads::list_jobs() -> Vec<(u32, String)>` — snapshot of running jobs

Macros (module-owned)
- `job!(background: "echo hi")` → job id
- `job!(wait: id)` → exit status (i32)
- `job!(timeout: secs, wait: id)` → exit status
- `job!(list)` → prints `[id] cmd` lines
- `event!(register name, handler)` / `event!(emit name, k => v, ...)` — uses OS event registry (temporary home)
- `trap!(handler, on: "SIGINT"|"SIGTERM"|"EXIT"|"COMMAND_ERROR")` — installs signal handlers, registers handler

Design
- Implementation leverages existing `os` job registry structures; migration to `threads` proper is planned.
- Fail-fast policy remains in higher-level macros that exit on error cases (consistent with RS).
- Logging: non‑visual paths use `utils::stderrx(level, msg)` so core builds do not depend on optional visual macros. Visual macros can be imported explicitly when the `visuals` feature is enabled.

Examples
```rust
use rsb::prelude::*;
let id = job!(background: "sleep 0.1 && echo done");
let status = job!(wait: id);
```

Testing
- Sanity: `tests/threads_sanity.rs` → `tests/threads/sanity.rs`
- UAT: `tests/uat_threads.rs` → `tests/uat/threads.rs` (visible demo of sleep/bench/jobs)
