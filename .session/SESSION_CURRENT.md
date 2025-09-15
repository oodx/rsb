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
1) Choose next concept (e.g., GLOBAL or DATE) and repeat: analyze, fill gaps, add/adjust tests, update PLAN, commit.
2) Continue using smoke lane for quick validation, and targeted cargo test for new/changed tests.

## Notes
- If full `cargo test` fails due to linker disk space, prioritize targeted lanes (`smoke`, feature subsets) and visuals lane.
