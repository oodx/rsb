# RSB Session Resume Guide

## Immediate Truths (2025-09-17)
- Visual macros now live in `src/visual/macros.rs` (legacy `src/macros/visual.rs`
  removed). Re-exports wired through `visual::mod.rs`.
- CLI sanity, visual sanity, and UAT cargo suites pass; prompts unit wrapper was
  repointed to `tests/unit/prompts/*.rs` so the full unit suite builds.
- `./bin/test.sh` has a partial repair: Boxy theme failures now fall back to
  stderr, but the script still violates BashFX-v3 structure (no `main`,
  `options`, or layered dispatch). Legendary rewrite is the next major task.
- Unit runner currently fails due to legacy expectations in
  `tests/unit/streams/core.rs` and `tests/unit/string/errors_test.rs` (non-critical
  for upcoming BashFX migration but worth revisiting post-rewrite).

## Rehydrate Checklist
1. **Docs to revisit**
   - Architecture & philosophy: `./bin/test.sh docs rsb`
   - MODULE_SPEC patterns: `./bin/test.sh docs modules`
   - Prelude rules: `docs/tech/development/PRELUDE_POLICY.md`
   - Modernization playbook: `docs/tech/development/HOWTO_UPDATE_RSB.md`
   - Feature gating roadmap: `docs/tech/development/FEATURES_GATING_PLAN.md`
   - Visual feature guide: `./bin/test.sh docs visuals`
   - Test organization: `./bin/test.sh docs org`
   - BashFX fundamentals (external reference): `docs/tech/reference/BASHFX-v3.md`

Repository quick references:
- `CHANGELOG.md` – latest code/documentation deltas
- `REFRESH_CONTEXT.md` – live test status + outstanding work
- `CONTINUE.md` – this resume guide; update with each major phase change

2. **Command refresher**
   - Sanity focus: `cargo test --test sanity <module> -- --nocapture`
   - UAT: `cargo test --test uat -- --nocapture`
   - Runner lanes (post-rewrite): `./bin/test.sh run sanity|uat|unit`
   - Linter (post-rewrite): `./bin/test.sh lint`

3. **Next major objective**
   - Design a BashFX-v3 compliant legendary scaffold for `test.sh` (explicit
     `options`, `main`, dispatcher tree) before porting existing commands.
   - Stage a new `fx-testsh` repo under `repos/shell/bashfx/` for development and
     eventually sync the rewritten script back into RSB.
   - Capture Boxy fallback decisions and helper library requirements for the
     rewrite (see `BASHFX_TESTSH_PLAN`).

4. **Historical caveat**
   - `.session/PROJECT_STATUS_SUMMARY.md`, `FINAL_ACHIEVEMENT_REPORT.md`, and
     `TEST_TASKS*.txt` remain outdated (claiming full modernization). Rely on the
     docs above for accurate state tracking.

## Next Session Kickoff
1. Review `BASHFX_TESTSH_PLAN` for the BashFX legend scaffold and helper inventory.
2. Establish or sync into `repos/shell/bashfx/fx-testsh` once accessible.
3. Begin implementing the scaffold (options/main/dispatch) and port lint/run/docs
   commands in iterative slices, validating Boxy output at each step.
