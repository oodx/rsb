# RSB Progress (MODERN + SPEC_ALIGNED)

Updated: 2025-09-15

Purpose
- Provide a framework‑agnostic, zero‑dependency core for progress reporting.
- Offer terminal reporters and styles suitable for CLI apps.
- Keep the surface curated and feature‑gated to maintain lean defaults.

Feature Flag
- `progress` — enables the progress module (`rsb::progress`). Not enabled by default.

Module Layout (SPEC)
- `src/progress/core.rs` — Core types, events, reporter trait, task lifecycle.
- `src/progress/styles.rs` — Spinner and bar style definitions and helpers.
- `src/progress/terminal.rs` — Terminal reporter (stdout/stderr), rate/ETA rendering.
- `src/progress/manager.rs` — ProgressManager for task orchestration.
- `src/progress/mod.rs` — Curated public API and tiny prelude.

Public API (curated)
```rust
#[cfg(feature = "progress")]
use rsb::progress::{
  ProgressManager, ProgressStyle,
  ProgressReporter, ProgressTask,
  TerminalReporter, TerminalConfig,
};

// Optional convenience prelude
#[cfg(feature = "progress")]
use rsb::progress::prelude::*;
```

Quick Start
```rust
#[cfg(feature = "progress")]
{
    use rsb::progress::{ProgressManager, ProgressStyle};

    let mut progress = ProgressManager::new();
    let task = progress.start_task("Processing files", ProgressStyle::Bar { total: 10 });

    for i in 0..10 {
        task.update(i + 1, &format!("Processing file {}", i + 1));
    }

    task.complete("All files processed");
}
```

Design Notes
- Zero‑dependency core: relies only on `std`.
- Terminal reporter supports:
  - ANSI colors toggled via `TerminalConfig { use_colors: bool, .. }`.
  - Unicode spinners toggled via `use_unicode`.
  - Output stream selection (`use_stderr`).
  - Update throttling (`update_interval_ms`).
  - Clear‑on‑complete behavior.
- No prelude exports: import explicitly to honor prelude policy.
- Logging: for non‑visual diagnostics, prefer `utils::stderrx(level, msg)` in adjacent code. Progress module itself writes directly to chosen stream.

Testing (HOWTO_TEST)
- Unit tests live under the module (`src/progress/mod.rs`) and compile when `--features progress` is enabled.
- Run:
  - `cargo test --features progress`
  - Optionally with visuals for colored output elsewhere: `cargo test --features visuals,progress`
- Test lanes (runner): add as needed (not required for core CI); typical full runs include `./bin/test.sh run all`.

Status
- MODERN: Yes — modularized, curated API, no wildcard exports.
- SPEC_ALIGNED: Yes — feature gated, documented, and tested with unit tests.

