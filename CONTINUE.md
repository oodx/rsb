# RSB Session Resume Guide

## Immediate Truths (2025-09-17)
- Visual macros are now module-owned (`src/visual/macros.rs`), re-exported via
  `rsb::visual::{macros::*, …}`. Legacy `src/macros/visual.rs` is gone.
- CLI sanity, visual sanity, and full UAT cargo suites are green.
- Full `cargo test` still fails: `tests/unit/features_prompts.rs` points at a
  non-existent `tests/unit/features/prompts/macros.rs` (real files live in
  `tests/unit/prompts/`). Fixing that wrapper is prerequisite for an all-green run.
- `./bin/test.sh` is currently broken: the linter path calls `boxy` themes that
  do not exist in the bundled tool (theme `magic`). Runner commands bail before
  running tests. Next task must focus on repairing `test.sh` (theme handling and
  any other drift) so we align with enforced project workflow.

## Rehydrate Checklist
1. **Docs to revisit**
   - Architecture & philosophy: `./bin/test.sh docs rsb`
   - MODULE_SPEC patterns: `./bin/test.sh docs modules`
   - Prelude rules: `docs/tech/development/PRELUDE_POLICY.md`
   - Modernization playbook: `docs/tech/development/HOWTO_UPDATE_RSB.md`
   - Feature gating roadmap: `docs/tech/development/FEATURES_GATING_PLAN.md`
   - Visual feature guide: `./bin/test.sh docs visuals`
 - Test organization: `./bin/test.sh docs org`

The project root keeps additional quick references:
- `CHANGELOG.md` – latest fixes made in this working copy
- `REFRESH_CONTEXT.md` – snapshot of current test status, commands, and outstanding work
- `CONTINUE.md` – this resume guide; update as the state evolves

2. **Command refresher**
   - Sanity (module focused): `cargo test --test sanity <module> -- --nocapture`
   - UAT: `cargo test --test uat -- --nocapture`
   - Full test runner (after repairs): `./bin/test.sh run sanity|uat|unit`
   - Linter (after repairs): `./bin/test.sh lint`

3. **Known follow-up tasks**
   - Repair `./bin/test.sh` themes/output so linting and runner commands work.
   - Update `tests/unit/features_prompts.rs` to reference the actual prompts tests
     (or relocate files) and restore a clean `cargo test` baseline.
   - Once the above are green, rerun Tina’s validation (`.eggs/red_egg*`) before
     claiming completion.

4. **Historical caveat**
   - `.session/PROJECT_STATUS_SUMMARY.md`, `FINAL_ACHIEVEMENT_REPORT.md`, and
     `TEST_TASKS*.txt` still proclaim 100% modernization with zero errors. Treat
     those as stale. `CHANGELOG.md` and `REFRESH_CONTEXT.md` now reflect the real
     state.

## Next Session Kickoff
1. Fix `./bin/test.sh` (theme handling, lint execution) and confirm runner commands
   work end-to-end.
2. Patch prompts unit wrapper, rerun unit/sanity/uat via runner.
3. Feed Tina (#tina) after the suites are verified to avoid future false greens.
