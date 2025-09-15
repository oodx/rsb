# RSB Development Utilities (MODERN + SPEC_ALIGNED)

Updated: 2025-09-15

Purpose
- Provide optional, test-focused utilities to aid development without affecting default builds.
- Enable PTY-backed testing to exercise true interactive/TTY code paths (prompts, colors, terminal sizing).

Feature Flags
- `dev-pty` — Enables `rsb::dev::pty` (pseudo-terminal wrapper built on `portable-pty`). Off by default.

Module Layout (SPEC)
- `src/dev/mod.rs` — Namespace for development utilities (feature-gated, not exported via prelude).
- `src/dev/pty.rs` — Minimal PTY wrapper: spawn under PTY, read/write, resize, wait.

Public API (curated)
```rust
#[cfg(feature = "dev-pty")]
use rsb::dev::{PtyOptions, PtySession, spawn_pty};

#[cfg(feature = "dev-pty")]
{
    let mut sess = spawn_pty("printf ok", &PtyOptions::default()).unwrap();
    let out = sess.read_for(std::time::Duration::from_millis(200)).unwrap();
    assert!(out.contains("ok"));
    let _ = sess.wait();
}
```

Design Notes
- Gated and non-invasive: default builds don’t include PTY dependencies.
- Preludes: not exported via `rsb::prelude`; import explicitly under the feature.
- Intended primarily for tests/UAT where real TTY behavior is required (prompts, colors, progress redraws).

How to Use
- Enable feature: `cargo test --features dev-pty`
- In tests or local tools, import from `rsb::dev` and orchestrate interactions (write input, read output, resize terminal).
- Combine with existing test runner timeouts (`bin/test.sh`) to ensure stability.

Status
- MODERN: Yes — isolated namespace, curated API, feature-gated.
- SPEC_ALIGNED: Yes — documented, minimal surface, not leaked via prelude.

Future Work
- Add small PTY-backed tests for `visual::prompts` to prove TTY branches.
- Consider optional adapters for other PTY backends behind separate features if needed.

