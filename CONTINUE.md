# RSB Session Resume Guide

## Immediate Truths (2025-09-17)
- Legendary runner scaffold (`fx-testsh` v2.2.0) now powers `bin/test.sh`; the
  `--rsb` profile pre-wires lane aliases, doc overrides, and Boxy ceremony.
- Sanity, smoke, unit, and regression suites are green through the runner; cargo
  still surfaces intentional warnings for unused token helpers and style nits.
- Visual macros remain in `src/visual/macros.rs` with curated re-exports via
  `visual::mod.rs`.
- Integration host-paths tests guard `HOME`/`XDG_*` mutations with a mutex and
  temp seeding; monitor for regressions if new env consumers appear.

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
   - Keep docs aligned with the generated runner (lane tables, profile notes).
   - Ensure cargo stays warning-free after runner-driven test passes.
   - Prep the final PR narrative once docs/tests settle.

4. **Historical caveat**
   - `.session/PROJECT_STATUS_SUMMARY.md`, `FINAL_ACHIEVEMENT_REPORT.md`, and
     `TEST_TASKS*.txt` remain outdated (claiming full modernization). Rely on the
     docs above for accurate state tracking.

## Next Session Kickoff
1. Run `./bin/test.sh --rsb lint` and `./bin/test.sh --rsb run sanity` as the
   quick health check before continuing edits.
2. Skim the refreshed docs (`docs/tech/reference/RSB_TEST_RUNNER.md`,
   `docs/tech/reference/RSB_TESTSH_INTEGRATION.md`) to confirm terminology
   sticks.
3. Draft the PR summary / changelog entry once the checks stay green.

## Latest Sync Update
- Runner swapped to fx-testsh scaffold (`bin/test.sh`); review before landing.
- Docs: fx sync checklist appended to `docs/tech/development/HOWTO_TEST.md`; verify final wording.
- FS/hosts/parse/param tests adjusted for new semantics; keep an eye on warnings (unused helpers, deprecated bool shim removal).
- Environment guard added for integration host paths; confirm expected TMP behavior (`XDG_TMP_HOME` seeded per test).
- All suites currently green (`./bin/test.sh --rsb run sanity|smoke`, `cargo test --all --quiet`).
