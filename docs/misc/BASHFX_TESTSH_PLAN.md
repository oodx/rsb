# fx-testsh Rebuild Plan

## Mission
- Rebuild `test.sh` as a BashFX-v3 legendary script inside the forthcoming `fx-testsh` project.
- Preserve the existing command surface (`lint`, `run <lane>`, `docs <topic>`, `status`, etc.) while aligning the internals with the BashFX dispatcher, ordinality, and ceremony patterns.
- Centralize helper logic into composable libraries so the script can be maintained as infrastructure-critical tooling.

## Key References (relative to `fx-testsh/docs`)
- `bashfx/BASHFX-v3.md` – canonical architecture, naming, and dispatcher rules.
   - `bashfx/options.sh.txt` stanard options implementation example
   - `bashfx/stderr.sh.txt`  standard ansii colors and printer function (can be udpated)

- `build.sh` is copied from ../fx-buildsh the `protobuild.sh` script is a powerful utility for the BashFX build.sh pattern of creating modular scripts and building them from a parts/ directory. This file must be configured for the project to use correctly; but its here just in case we decide that we want parts and builds for a large script.

**Reference files from the RSB (Rebel) Framework**
- `rebel/RSB_TEST_RUNNER.md` *(to be authored)* – mirror of `ref/test.sh` commentary and RSB-specific lane expectations.
- `rebel/TEST_ORGANIZATION.md` – enforcement goals for lint mode.
- `rebel/PRELUDE_POLICY.md` – Prelude usage rules mirrored from RSB.
- `rebel/HOWTO_UPDATE_RSB.md` – change-management expectations when syncing back to RSB.

Note that the test.sh solution as is, is primarily designed in support of Rebel/RSB/Rust, but we should leave room to expand to other language domains. 

## Snapshot of Current Script (from `ref/test.sh`)
- Single-level `case` statement mixes CLI parsing, visuals, and business logic.
- No `options()` / `main()` / `dispatch()` separation; no ordinality or sub-dispatchers.
- Helper naming is inconsistent with `_foo` / `__foo` prefixes; large monolithic functions.
- Ceremony output assumes successful Boxy invocation; fallback handling now lives in RSB but must be re-homed cleanly.
- Responsibilities blurred across linting, test discovery, docs, and status reporting.

## Target Legendary Scaffold
```
main()
  └── options "$@"
  └── dispatch "${args[@]}"

options()
  └── set opt_* flags, populate ARGS array (strip flags)

dispatch()
  ├── do_help
  ├── do_status
  ├── do_lint
  ├── do_run
  │     └── dispatch_run (sub-dispatcher for lanes)
  ├── do_docs
  │     └── dispatch_docs (sub-dispatcher for topics)
  ├── do_list
  └── do_version / do_logo (optional legendary adornments)

_helpers()
  ├── `_validate_test_structure`
  ├── `_print_boxy`
  ├── `_run_cargo_lane`
  ├── `_report_summary`
  └── `__debug` style low-level utilities
```
- High-order functions (`do_*`) own user-facing guards and ceremony.
- Mid-order helpers (`_foo`) break down workflows (lint validation, doc lookup, lane execution).
- Low-order helpers (`__foo`) encapsulate literal shell operations and should be broadly reusable.

## Migration Strategy
1. **Extract Baseline**
   - Keep current script under `ref/test.sh` for reference.
   - Catalog existing lanes (`lint`, `run <lane>`, `docs`, `status`, `help`, `list`).

2. **Scaffold Legendary Script**
   - Implement `options`, `main`, `dispatch`, sub-dispatchers with empty bodies but accurate ceremony.
   - Port the Boxy wrapper as `_boxy_emit` with stderr fallback.
   - Add standard shell guards (set -euo pipefail, `trap` for cleanup) per BashFX guidance.

3. **Port Commands Incrementally**
   - Start with `do_help`, `do_status`, `do_list` for discovery utilities.
   - Integrate lint enforcement (`do_lint` → `_lint_tests` → `__validate_structure`).
   - Rebuild lane execution (`do_run`) and doc routing (`do_docs`).
   - Preserve existing env var overrides (DOCS paths, TEST_DIR) while normalizing naming.

4. **Library Extraction**
   - Evaluate common helpers for potential library folders (`lib/boxy.sh`, `lib/test_org.sh`).
   - Follow BashFX directory conventions (`lib/`, `parts/` once build.sh pattern adopted).

5. **Adopt build.sh when ready**
   - Once the script stabilizes, split into `.parts/` with a generated `build.sh` to assemble the final artifact.

## Outstanding Questions / TODOs
- Clarify whether Tina integration lives inside `do_run` or as a follow-up command.
- Decide on permanent locations for Boxy helper and test-organization library (shared vs. script-local).
- Define version/metadata banner for the legendary script (consider embedded doc pattern).
- Review failing legacy unit tests (`streams`, `string/errors`) after the script rewrite to ensure runner output expectations stay honest.

## Next Actions (blocking the rewrite)
- Wait for sandbox access to `repos/shell/bashfx/` and the new `fx-testsh` repo.
- Once available, bootstrap legendary scaffold using this plan.
- Sync progress notes back into RSB when major milestones are reached (lint lane functional, run lanes restored, docs dispatcher live).
