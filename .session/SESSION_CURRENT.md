# Session: Per-Concept Plans, Math Decoupling, Next Steps

Date: 2025-09-15
Repo: rsb (new canonical), branch `main`

## Summary
- Added plan files for each feature concept in `.session/PLAN_*.md`.
- Began implementation with Math: decoupled math macros from visual logging and updated math UAT global usage to function-based API.
- Prelude audit confirms optional visuals/log macros are not exported via prelude.

## Plans Added
- PLAN_STRINGS.md, PLAN_PARAMS.md, PLAN_GLOBAL.md, PLAN_COLORS.md, PLAN_DATE.md, PLAN_TOKENS.md,
  PLAN_PROMPTS.md, PLAN_OPTIONS.md, PLAN_HOST.md, PLAN_THREADS.md, PLAN_CLI.md, PLAN_BASH.md, PLAN_OPTIONS_STDOPTS.md

- Math concept: macros decoupled from visual logging; math UAT adjusted; smoke lane green.
- Strings concept: UAT added (`tests/uat/string.rs` + wrapper) and verified via targeted test run. Plan marked Completed.
- Tokens concept: existing sanity, feature, and UAT tests all passing; plan marked Completed.
- Global concept: feature, core, and adapter tests passing; plan marked Completed.
- Date concept: feature + UAT tests passing; plan marked Completed.
- Options concept: default and stdopts feature tests passing; plans marked Completed.
- CLI concept: E2E shell test passed; plan marked Completed.
- CLI concept: E2E shell test passed; plan marked Completed.
- Colors/Visuals: visuals tests passing after macro fix; plan marked Completed.
- Host concept: env and paths tests passing; plan marked Completed.
- Threads concept: sanity + UAT passing; plan marked Completed.
- Bash concept: sanity + UAT passing; plan marked Completed.
- Params concept: comprehensive features passing; plan marked Completed.

## Next Actions
1) Legacy macro migration: identify remaining items in `src/macros/` suitable for MODULE_SPEC migration and schedule moves into module-owned `macros.rs` with curated exports; update `prelude::macros` accordingly (deferred for a focused pass).
2) Verify any remaining doc cross-links opportunistically during future changes.

## Notes
- If full `cargo test` fails due to linker disk space, prioritize targeted lanes (`smoke`, feature subsets) and visuals lane.

## Wrap-up (This Pass)
- All major concept suites validated green (sanity, features, UAT; visuals included).
- Docs index added and README cross-linked.
- Plans for each concept saved in `.session/PLAN_*.md` and marked Completed.
- CLI built-ins polished: inspect/help/stack now expand color templates correctly; handler registry stores clean names (no pointer strings).

## Continuation Checklist
- Validate lanes on a fresh workspace: `./bin/test.sh run smoke`, key wrappers, and visuals features.
- Begin incremental migration of legacy macros in `src/macros/` to module-owned `macros.rs` per MODULE_SPEC.
- Keep prelude policy intact (core-only; visuals/loggers opt-in).
- Drafted modular feature-gating plan at `docs/tech/development/FEATURES_GATING_PLAN.md` for per-concept basic/advanced enablement.
