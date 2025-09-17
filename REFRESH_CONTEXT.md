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
- No commits yet; work happens in an open tree. Recent changes touch
  `src/cli/args.rs`, UAT demos, and new docs (`CHANGELOG.md`, `REFRESH_CONTEXT.md`).

## Critical Documentation & How to Reach It Quickly
- Architecture & philosophy: `./bin/test.sh docs rsb` (wraps `docs/tech/reference/REBEL.md`
  and `docs/tech/reference/RSB_ARCH.md`).
- Module specification: `./bin/test.sh docs spec` alias (points at
  `docs/tech/development/MODULE_SPEC.md`).
- Test organization standard: `./bin/test.sh docs org` → `docs/tech/development/TEST_ORGANIZATION.md`.
- Feature guides:
  - CLI: `./bin/test.sh docs cli` → `docs/tech/features/FEATURES_CLI.md`
  - Global store: `./bin/test.sh docs global` → `docs/tech/features/FEATURES_GLOBAL.md`
  - Params: `./bin/test.sh docs params` → `docs/tech/features/FEATURES_PARAMS.md`
  - Visuals: `./bin/test.sh docs visuals` → `docs/tech/features/FEATURES_VISUALS.md`
- Complete list of docs: `./bin/test.sh docs all`.
- Contributor onboarding: `HOWTO.md`, `CONTRIBUTING.md`, `README.md` (already reviewed).

## Essential Commands & Lanes
- List runner lanes: `./bin/test.sh list`
- Run sanity CLI lane: `cargo test --test sanity cli -- --nocapture`
- Run sanity visual lane: `cargo test --test sanity visual -- --nocapture`
- Run full UAT: `cargo test --test uat -- --nocapture`
- Docs hub: `./bin/test.sh docs`

## Active Agents & Protocols
- **#tina** (Testing Chicken): definitive validation. Run Tina after claiming any
  completion; no green status without her red laser sign-off.
- **#china** (Summary Chicken): progress summaries; double-check for optimism bias.
- Maintain brutal honesty—previous sessions contained false 100% completion claims.

## Outstanding Work
1. **Warning cleanup**
   - Decide whether to silence or address the unused-variable warnings in
     UAT demos (parse/com) without losing illustrative output.
   - Remove the deprecated bool helper usage in `src/com/bool.rs` or annotate with
     an intentional `#[allow]`.
2. **Test runner alignment**
   - Ensure `./bin/test.sh` lanes mirror the cargo commands above; document any
     divergence.
3. **Project truth maintenance**
   - Keep CHANGELOG up to date.
   - Update `.session/` notes after each meaningful milestone.

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
