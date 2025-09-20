# Backup: SESSION_CURRENT.md (2025-09-15)

# Session: FS Moduleization, PTY Dev Utils, Progress/Visuals Stable

Date: 2025-09-15
Repo: rsb (canonical), branch `main`

## Summary
- FS: Migrated to MODULE_SPEC structure
  - Added `src/fs/mod.rs` (orchestrator), `src/fs/utils.rs` (impl), `src/fs/macros.rs` (module‑owned macros).
  - Removed legacy `src/macros/fs_data.rs`. All macros exported at crate root remain available.
  - Added `docs/tech/features/FEATURES_FS.md` and linked from README and docs index.
- Dev PTY: Added optional `dev-pty` feature and wrapper
  - `rsb::dev::pty` provides a light PTY session for interactive/TTY tests.
  - Added `tests/dev_pty.rs` sanity; updated HOWTO_TEST.
- Progress: Fixed doctest import; visuals/progress lanes remain green.
- Host/Streams macros migration: host info/path macros live under `hosts::macros`; streams macros under `streams::macros`; OS pid/lock macros under `os::macros`.
- Test runner: Added timeout wrapper (`timeout`/`gtimeout`) via `RSB_TEST_TIMEOUT`.

## Status
- Tests
  - `cargo test`: PASS
  - `cargo test --features visuals`: PASS
  - `cargo test --features progress`: PASS
  - `cargo test --features dev-pty --test dev_pty`: PASS
- Policy checks
  - Prelude policy: compliant (no optional visual/log exports via prelude).
  - Progress rate formatting: two‑mode behavior (>=1.0 → 1 decimal, <1.0 → 2 decimals) preserved.

## What Changed (Key Paths)
- Logger: `src/utils.rs` (`stderrx`, shim), macros under `src/macros/{stderr.rs,visual.rs}`.
- CLI: `src/cli/{macros.rs,dispatch.rs}`; registry printing in `src/global/registry.rs`.
- Progress: `src/progress/{mod.rs,core.rs,manager.rs,terminal.rs,styles.rs}`; docs `docs/tech/features/FEATURES_PROGRESS.md`.
- Colors: `docs/tech/features/FEATURES_COLORS.md` rewritten.
- Deps: features in `Cargo.toml`; code in `src/deps.rs`.
- Docs landing: `README.md` and `docs/tech/INDEX.md` use table layout and relative links.

## Next Actions
1) Split `json_dict_random` legacy macros:
   - Move `rand_*` and `rand_range!` under `gx` (string/id/collection).
   - Keep `json_*`/`dict!`/`gen_dict!`/`rand_dict!` curated; consider `json` helper or place under `gx`.
   - Re‑export macros at crate root for compatibility.
2) Optional CI lanes: smoke, visuals, progress (document in HOWTO_TEST if added).
3) Continue legacy macro migration audit; ensure prelude policy compliance.

## Notes
- The deps surface now supports both per‑dependency opt‑in (e.g., `deps-chrono`) and a full umbrella (`deps`/`deps-all`).
- For visuals, default builds print plain text (tags stripped); visuals render when the `visuals` feature is enabled.
