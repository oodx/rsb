# Session: Prelude Audit Kickoff, Tests, and Findings

Date: 2025-09-15
Repo: rsb (new canonical), branch `main`

## Context
- Working directory: `/home/xnull/repos/code/rust/oodx/rsb`
- Target was to rehydrate session, read “Start Here” links, list tests, run smoke, and begin a prelude export audit.
- Note: `.session/rsb_complete/CONTINUE.md` and `TASKS.txt` referenced in prior context are not present in this repo.

## What I Read / Verified
- README.md: “Start Here” is present with overview, examples, and API references.
- Dev docs: `docs/tech/development/HOWTO_UPDATE_RSB.md`, `docs/tech/development/MODULE_SPEC.md`, `docs/tech/development/PRELUDE_POLICY.md` present.
  - `docs/tech/development/HOWTO_TEST.md` is not present in this repo.
- Reference docs: `docs/tech/reference/REBEL.md`, `docs/tech/reference/RSB_ARCH.md` present.
- Features docs: files exist under `docs/tech/features/` (Strings, Params, Global, Colors, Date, Tokens, Math, Prompts, Options, Host, Threads, CLI).

## Tests Run
- Made `bin/test.sh` executable locally and used it as entrypoint.
- `./bin/test.sh list` → lists many lanes (sanity, smoke, param, macros, global, host, cli, threads, colors, prompts, integration, etc.).
- `./bin/test.sh run smoke` → PASS
  - Sanity, UAT-global, and Param suites passed in the quick lane.
- `cargo test` (default) → FAIL currently due to math UAT compilation errors (details below).

## Prelude Audit (Policy Compliance)
- `src/prelude.rs` re-exports core types, helpers, and macros. It does NOT re-export optional visuals/logging macros (colored!, info!, warn!, error!, etc.).
- `src/lib.rs` gates `visual` under Cargo features; optional visuals require explicit imports.
- `prelude::macros` re-exports curated module-owned macros (param/string/case) and legacy groups; does not include visual macros.
- Cargo features present: `visual`, `colors-*`, `glyphs`, `prompts`, `visuals` umbrella.

Conclusion: Prelude export surface appears compliant with the policy (core-only, optional visuals not exported via prelude).

## Issues Found (Blocking full test run)
1) Core macros depend on optional visual macros internally
   - `src/math/macros.rs` references `$crate::error!(...)` in multiple macros (divide, sqrt, to_base, from_base, base_convert, ratio, modulo, etc.).
   - `src/threads/macros.rs` references `$crate::info!` and `$crate::error!`.
   - Visual macros are gated (not compiled by default), so default builds without `visual` feature fail when these macros are expanded (seen in math UAT).

2) Tests reference non-existent global macros
   - `tests/math_uat.rs` uses `set_var!` / `get_var!` macros, which are not defined in the current crate. The public API for globals is functions (`rsb::global::{set_var, get_var, has_var, ...}`), not macros.

## Evidence (snippets)
- Math macros call visual macro:
  - Example: in `src/math/macros.rs` line ~95: `$crate::error!("Division error: {}", e);`
- `cargo test` default error sample:
  - E0433 “could not find `error` in `$crate`” triggered from math macros expanded in `tests/math_uat.rs`.
  - “cannot find macro `set_var` in this scope” and “cannot find macro `get_var` in this scope” in math UAT.

## Minimal Patch Plan (Focused, compatible with policy)
1) Decouple core macros from visual logging
   - Replace internal uses of `$crate::error!(...)` (and `$crate::info!` in threads) with the core, non-visual helper:
     - `$crate::utils::glyph_stderr("error", &format!(...))`
     - `$crate::utils::glyph_stderr("info", &format!(...))`
   - Rationale: `utils::glyph_stderr` is available without the `visual` feature and respects quiet/debug/trace flags.

2) Update math UAT to use global functions instead of macros
   - Replace `set_var!(...)` / `get_var!(...)` with `rsb::global::set_var(...)` / `rsb::global::get_var(...)`.
   - Rationale: Align tests with the current public API and avoid introducing new macros into prelude.

3) Validate
   - Run `cargo test` (default) to verify compilation and behavior without visuals.
   - Run `cargo test --features visuals` to ensure optional visuals remain intact.
   - Re-run `./bin/test.sh run smoke` as a quick regression check.

## Optional Follow-ups
- Ensure any additional core macros that log on error also use `utils::glyph_stderr` to avoid hidden visual dependencies.
- Add a short note in math/threads macro files referencing PRELUDE_POLICY (core must not depend on optional visuals).
- If needed, add `docs/tech/development/HOWTO_TEST.md` pointing to `bin/test.sh` lanes and Cargo feature combos.

## Status Update / Next Action
- Smoke lane is green. Prelude export audit shows no prelude leakage of visual/log macros.
- Code check shows math and threads macros already decoupled from visual logging (use `utils::glyph_stderr`).
- Math UAT updated to use `rsb::global::{set_var, get_var}` functions (no missing macros).

Next:
- Validate locally: `cargo test` (default) and `cargo test --features visual` (or use `./bin/test.sh run smoke` then `./bin/test.sh run all`).
- If any help/inspect output shows raw color tags without `visual`, decide whether to strip tags in `utils::expand_colors_unified` for plain mode.
