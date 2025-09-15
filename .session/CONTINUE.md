# RSB Continuation Guide (Next Session)

Date: 2025-09-15
Branch: main
Repo: rsb (canonical)

Quick snapshot
- Logger: `stderrx` is the internal non‑visual logger (shim for `glyph_stderr` retained).
- CLI: `pre_dispatch!` now auto‑registers handlers and supports `desc:` like `dispatch!`.
- Progress: integrated behind `progress` feature; rate formatting policy unchanged (>=1.0 → 1 decimal, <1.0 → 2 decimals).
- Docs: README “Start Here” + docs index use table format with relative links; Progress feature doc added.
- Deps: `rsb::deps` supports per‑dependency flags and an umbrella (`deps-all`, alias `deps`).

Start here (zero‑context)
- Read:
  - README “Start Here” table: `README.md`
  - Docs index: `docs/tech/INDEX.md`
  - Module spec: `docs/tech/development/MODULE_SPEC.md`
  - Progress & Colors feature docs:
    - `docs/tech/features/FEATURES_PROGRESS.md`
    - `docs/tech/features/FEATURES_COLORS.md`
  - Session summary: `.session/SESSION_05_progress_integration_and_deps.md`

Validate environment
- List wrappers: `./bin/test.sh list`
- Quick lane: `./bin/test.sh run smoke`
- Core tests: `cargo test`
- Visuals: `cargo test --features visuals`
- Progress: `cargo test --features progress`

Feature flags cheat
- Visual umbrella: `--features visuals` (colors + glyphs + prompts)
- Progress: `--features progress`
- Deps:
  - Per‑dep: `--features deps-chrono` (then `use rsb::deps::chrono;`)
  - All deps: `--features deps` (then `use rsb::deps::*;`)

Next tasks (execution order)
1) Legacy macro migration
   - Identify remaining items in `src/macros/` suitable for MODULE_SPEC migration.
   - Move into module‑owned `macros.rs`; adjust `prelude::macros` if needed.
2) Optional: unify math `random_list!` error logging to `stderrx`.
3) Optional CI: add lanes for smoke + visuals + progress.
4) Keep README and docs index in sync when adding modules/features.

Important policies
- Prelude policy: do not export optional subsystems via `rsb::prelude`.
- Progress rate formatting must remain as is (two‑mode). Discuss via RFC before any behavior change.

Useful paths
- CLI dispatch helpers: `src/cli/{macros.rs,dispatch.rs}`
- Logger and helpers: `src/utils.rs`, `src/macros/{stderr.rs,visual.rs}`
- Progress module: `src/progress/{mod.rs,core.rs,manager.rs,terminal.rs,styles.rs}`
- Deps surface: `src/deps.rs`, feature flags in `Cargo.toml`

How to resume quickly
- Run: `./bin/test.sh run smoke` then `cargo test --features visuals,progress`
- Skim: `.session/SESSION_05_progress_integration_and_deps.md` for the last session summary
- Begin with the Legacy macro migration task (above), or pick the optional items if preferred.

Agents / tooling
- No special services needed; use Cargo and the provided test runner.
- This repo’s coding agent previously performed the refactors; you can proceed directly with the checklist.
