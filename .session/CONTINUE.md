# RSB Continuation Guide (Next Session)

Date: 2025-09-15
Branch: main
Repo: rsb (canonical)

Quick snapshot
- Logger: `stderrx` is the internal non‑visual logger (shim for `glyph_stderr` retained).
- CLI: `pre_dispatch!` now auto‑registers handlers and supports `desc:` like `dispatch!`.
- Progress: integrated behind `progress` feature; rate formatting policy unchanged (>=1.0 → 1 decimal, <1.0 → 2 decimals).
- Docs: README “Start Here” + docs index use table format with relative links; Progress and FS feature docs added.
- Deps: `rsb::deps` supports per‑dependency flags and an umbrella (`deps-all`, alias `deps`).

New since last session
- FS moduleization (MODULE_SPEC): fs moved to `src/fs/{mod.rs,utils.rs,macros.rs}`; legacy `src/macros/fs_data.rs` removed. Added `docs/tech/features/FEATURES_FS.md`.
- Dev PTY (feature `dev-pty`): `rsb::dev::pty` for PTY-backed tests; added `tests/dev_pty.rs`. HOWTO_TEST updated. Test runner gains timeout wrapper via `timeout`/`gtimeout` (env `RSB_TEST_TIMEOUT`, default 600s).
- Macro ownership: host info/path macros under `hosts::macros`; streams under `streams::macros`; OS pid/lock under `os::macros`.

Start here (zero‑context)
- Read:
  - README “Start Here” table: `README.md`
  - Docs index: `docs/tech/INDEX.md`
  - Module spec: `docs/tech/development/MODULE_SPEC.md`
  - Progress, FS & Colors feature docs:
    - `docs/tech/features/FEATURES_PROGRESS.md`
    - `docs/tech/features/FEATURES_FS.md`
    - `docs/tech/features/FEATURES_COLORS.md`
  - Session summary: `.session/SESSION_05_progress_integration_and_deps.md`

Validate environment
- List wrappers: `./bin/test.sh list`
- Quick lane: `./bin/test.sh run smoke`
- Core tests: `cargo test`
- Visuals: `cargo test --features visuals`
- Progress: `cargo test --features progress`
- Dev PTY: `cargo test --features dev-pty --test dev_pty`

Feature flags cheat
- Visual umbrella: `--features visuals` (colors + glyphs + prompts)
- Progress: `--features progress`
- Deps:
  - Per‑dep: `--features deps-chrono` (then `use rsb::deps::chrono;`)
  - All deps: `--features deps` (then `use rsb::deps::*;`)

Next tasks (execution order)
1) Split `json_dict_random` macros per MODULE_SPEC
   - Move randomness helpers (`rand_*`, `rand_range!`) under `gx` (string/id/collection).
   - Keep `json_*`/`dict!`/`gen_dict!`/`rand_dict!` curated (either small `json` helper or under `gx`).
   - Re‑export macros at crate root; behavior unchanged.
2) Optional CI: add lanes for smoke + visuals + progress (document in HOWTO_TEST if added).
3) Legacy macro migration audit & prelude policy checks.
4) Keep README and docs index in sync when adding modules/features.

Important policies
- Prelude policy: do not export optional subsystems via `rsb::prelude`.
- Progress rate formatting must remain as is (two‑mode). Discuss via RFC before any behavior change.

Useful paths
- FS module: `src/fs/{mod.rs,utils.rs,macros.rs}`
- CLI dispatch helpers: `src/cli/{macros.rs,dispatch.rs}`
- Logger and helpers: `src/utils.rs`, `src/macros/{stderr.rs,visual.rs}`
- Progress module: `src/progress/{mod.rs,core.rs,manager.rs,terminal.rs,styles.rs}`
- Deps surface: `src/deps.rs`, feature flags in `Cargo.toml`

How to resume quickly
- Run:
  - `./bin/test.sh run smoke`
  - `cargo test`
  - `cargo test --features visuals`
  - `cargo test --features progress`
  - optional: `cargo test --features dev-pty --test dev_pty`
- Skim: `.session/SESSION_05_progress_integration_and_deps.md` and this file.
- Begin with the `json_dict_random` split (above), or add CI lanes.

Rehydration prompt (paste this to resume context)

You are working in the RSB repo. CWD: rsb (project root). Branch: main.

Context rehydration (read these first)
- README.md (Start Here table)
- docs/tech/INDEX.md (feature and dev docs index)
- docs/tech/development/MODULE_SPEC.md (module/spec alignment)
- docs/tech/features/FEATURES_FS.md (FS module)
- docs/tech/features/FEATURES_PROGRESS.md (progress module)
- .session/SESSION_05_progress_integration_and_deps.md (last session summary)
- .session/CONTINUE.md (this continuation guide)
- .session/SESSION_CURRENT.md (latest status)
- .session/TASKS.txt (current tasks)
- Cargo.toml (feature flags)

Current state
- FS moduleized (mod.rs orchestrator, utils.rs impl, macros.rs). Legacy fs_data removed.
- Dev PTY feature added (rsb::dev::pty) with sanity test.
- Progress integrated behind feature; rate formatting policy unchanged.
- Logger: utils::stderrx; shim glyph_stderr retained for compatibility.
- CLI: pre_dispatch! supports desc and auto registers handlers.
- Deps: rsb::deps supports per-dep and umbrella features.

Policies
- Prelude policy: optional subsystems do not leak via rsb::prelude.
- Progress rate formatting unchanged.
- Open brief RFC for any user-visible output changes.

Quick validation
- ./bin/test.sh run smoke
- cargo test
- cargo test --features visuals
- cargo test --features progress
- optional: cargo test --features dev-pty --test dev_pty

Next tasks
1) Split json_dict_random macros per MODULE_SPEC (gx/json split; re-export macros)
2) Optional CI lanes for smoke/visuals/progress (document in HOWTO_TEST)
3) Macro migration audit and prelude policy check

Agents / tooling
- No special services needed; use Cargo and the provided test runner.
- This repo’s coding agent previously performed the refactors; you can proceed directly with the checklist.
