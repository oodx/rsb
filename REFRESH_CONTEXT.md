# RSB Refresh Context

## High-Level Overview
- Project: **RSB (Rebel String-Based)** framework at `rsb/`
- Philosophy: REBEL principles favour pragmatic, bash-like ergonomics over
  ceremonial Rust purity. Modern modules must be SPEC_ALIGNED (see Module Spec).
- Test infrastructure is organized and enforced through `./bin/test.sh`; use the
  runner for day-to-day work and reach for raw `cargo test` only when debugging a
  very small slice.

## Current Status (2025-09-17)
- ✅ Visual sanity lane: `cargo test --test sanity visual -- --nocapture`
- ✅ CLI sanity lane: args contract restored; `cargo test --test sanity cli -- --nocapture`
- ✅ UAT suite: `cargo test --test uat -- --nocapture` (passes with benign warnings
  about unused demo variables and deprecated helpers).
- ⚠️ Full `cargo test` currently fails because `tests/unit/features/prompts/macros.rs`
  is referenced but missing (wrapper expects `tests/unit/features/prompts/...` while
  the existing files live in `tests/unit/prompts/`).
- No commits yet; work happens in an open tree. Recent changes touch
  `src/cli/args.rs`, UAT demos, and new docs (`CHANGELOG.md`, `REFRESH_CONTEXT.md`).

### Historical documentation caveat
Archived session files (`.session/PROJECT_STATUS_SUMMARY.md`,
`FINAL_ACHIEVEMENT_REPORT.md`, `TEST_TASKS*.txt`) still proclaim "100% test
modernization" with zero compilation errors. Treat them as inaccurate history: the
current repo required fixes to get the CLI sanity lane passing, UAT was broken,
and unit prompts tests still block a full run.

## Critical Documentation & How to Reach It Quickly
- Core architecture & philosophy: `./bin/test.sh docs rsb` (routes to
  `docs/tech/reference/REBEL.md` + `docs/tech/reference/RSB_ARCH.md`).
- Module specification & ownership patterns: `./bin/test.sh docs modules`
  → `docs/tech/development/MODULE_SPEC.md`.
- Prelude rules: `docs/tech/development/PRELUDE_POLICY.md` (also referenced from
  HOWTO_UPDATE_RSB).
- Modernization playbook: `docs/tech/development/HOWTO_UPDATE_RSB.md`.
- Feature gating roadmap: `docs/tech/development/FEATURES_GATING_PLAN.md`.
- Test organization standard: `./bin/test.sh docs org`
  → `docs/tech/development/TEST_ORGANIZATION.md`.
- Feature guides (available via `./bin/test.sh docs <feature>`):
  - CLI → `docs/tech/features/FEATURES_CLI.md`
  - Global store → `docs/tech/features/FEATURES_GLOBAL.md`
  - Params → `docs/tech/features/FEATURES_PARAMS.md`
  - Visuals → `docs/tech/features/FEATURES_VISUALS.md`
- Complete documentation inventory: `./bin/test.sh docs all`.
- Contributor onboarding references: `HOWTO.md`, `CONTRIBUTING.md`, `README.md`.

## Essential Commands & Lanes
- Preferred testing entry point: `./bin/test.sh` (lists commands and docs hub).
- List runner lanes: `./bin/test.sh list`
- Run sanity suite via runner: `./bin/test.sh run sanity`
- Run UAT suite via runner: `./bin/test.sh run uat`
- Targeted cargo commands (useful for debugging but second to the runner):
  - `cargo test --test sanity cli -- --nocapture`
  - `cargo test --test sanity visual -- --nocapture`
  - `cargo test --test uat -- --nocapture`

## Active Agents & Protocols
- **#tina** (Testing Chicken): definitive validation. Run Tina after claiming any
  completion; no green status without her red laser sign-off.
- **#china** (Summary Chicken): progress summaries; double-check for optimism bias.
- Maintain brutal honesty—previous sessions contained false 100% completion claims.

## Outstanding Work
1. **Restore unit test completeness**
   - Fix the missing `tests/unit/features/prompts/macros.rs` reference (either move
     the existing files under `tests/unit/features/prompts/` or update the wrapper
     to point at `tests/unit/prompts/`). This currently blocks `cargo test` and
     will also affect `test.sh run unit` once activated.
2. **Prompts unit wrapper repair**
   - `tests/unit/features_prompts.rs` still references
     `tests/unit/features/prompts/macros.rs`. Update the wrapper (or move the
     prompts files) so the unit suite passes.
3. **Warning cleanup**
   - Decide whether to silence or address the unused-variable warnings in
     UAT demos (parse/com) without losing illustrative output.
   - Remove or acknowledge the deprecated bool helper usage in `src/com/bool.rs`.
4. **Runner alignment & documentation honesty**
   - Audit `./bin/test.sh` lanes so they surface the same pass/fail state that
     cargo commands revealed (especially once the prompts path is fixed).
   - Counter the optimistic `.session/PROJECT_STATUS_*` and `TEST_TASKS*`
     documents with accurate status notes (CHANGELOG + future session logs).

## Guardrails
- Avoid API signature changes unless a contract is broken—downstream projects
  already consume the framework.
- Preserve ASCII-first formatting in code and documentation.
- Do not delete or overwrite user-created files without explicit direction.
- Use `rustfmt` on touched files; repo-wide `cargo fmt` currently fails because of a
  missing test module include (`tests/unit/features/prompts/macros.rs`). Format files
  individually until that gap is resolved.

## Validation Checklist Before Claiming Success
1. Run targeted cargo tests for touched modules.
2. Run relevant `./bin/test.sh` lanes.
3. Capture Tina’s validation (`.eggs/red_egg.*`) when suites are expected to pass.
4. Update CHANGELOG and session notes.
5. Report any remaining red tests explicitly—never assume green by omission.

## Quick Path to Zero-Context Resume
1. Read `REFRESH_CONTEXT.md` (this file).
2. Review `CHANGELOG.md` for the latest deltas.
3. Inspect `.session/SESSION_12_CONTINUATION.md` for historical context.
4. Consult Tina’s last validation egg: `.eggs/red_egg.1.rsb-test-modernization-validation.txt`.
5. Rerun the sanity CLI and UAT commands above to confirm the live state.
6. Continue with the “Outstanding Work” list, feeding Tina after each verified fix.
