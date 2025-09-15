# RSB Development Continuation Guide
Updated: 2025-09-12 (late)
Purpose: Restore full context after reset and continue this workstream quickly

Context note: This workspace is `rsb2` under `.../repos/code/rust/oodx`. If a canonical `rsb/` exists and `rsb2/` is gone, use the consolidated repo. See HOWTO_UPDATE_RSB.md for a quick primer.

## Narrative: Work Completed So Far
1) Visual package (optional) tightened and validated
- Feature-gated `src/visual/` with colors (simple, status, named), backgrounds, and glyphs; runtime registry with case-insensitive lookups; background toggles.
- Inline tag expansion via `colored!` with safe single-arg form to avoid format! conflicts; unknown tags pass through.
- Named colors remain in the global HashMap by design. No enum conversion for the named palette.
- Visual macros split from core; visuals removed from prelude to enforce explicit opt-in.
- UAT and sanity tests organized under wrappers; example gated behind `visuals`.

2) Prelude/feature-gating policy enforced
- Prelude contains only core exports. Optional systems (visuals/log macros) require explicit import.

3) Param macro progressive enhancement and fixes
- Introduced `src/param/{basic,advanced}.rs`; `param!` delegates to `param::basic` helpers.
- Added negative substring support (relative indices) via `sub_rel` and macro wiring.
- Consolidated param tests under `tests/features/param/` and added UAT focused on visible outputs.

4) Tests + runner structure
- Folderized tests with wrappers for Cargo:
  - Sanity: `tests/sanity_main.rs` includes `sanity.rs` (core) + `sanity/baseline.rs` (visible demos).
  - Features: `tests/features_colors.rs`, `tests/features_param.rs` include color/param suites.
  - UAT: `tests/uat_main.rs` includes `tests/uat/*.rs`.
  - Shell: `tests/sh/*.sh`.
- `bin/test.sh`:
  - Lists mapped tests and auto-discovers wrappers (`tests/*.rs`).
  - Runs mapped names or wrapper base names. Added `smoke` (quick) and `all` (comprehensive) suites.

5) Documentation
- Added `FEATURES_COLORS.md`, `FEATURES_PARAMS.md`, and `HOWTO_UPDATE_RSB.md`.
- Updated README Visuals section for explicit imports and runtime enabling.

6) String module + Param macro ownership
- Introduced `src/string/` with `helpers.rs`, `macros.rs`, and new `case.rs`; `mod.rs` orchestrates re‑exports.
- Moved string helpers into `string`; added wildcard prefix/suffix (glob) with safe char-boundaries and tests.
- Added `string::case` helpers and macros: snake/kebab/slug/dot/space/camel; generalized case in `param!(..., case: ...)`.
- Patterned case transforms in param (`${VAR^pat}` / `${VAR,pat}`) implemented.
- `${VAR:?msg}` now hard‑exits after printing to stderr.
- Added ASCII‑only normalization for case helpers; documented ASCII‑SAFE vs UNICODE‑SAFE.
- New ASCII utilities: `filter_ascii_strip`, `filter_ascii_sanitize(#INV#)`.
- Centralized string errors via `StringError` (line limit + regex compile); added tests for invalid globs.
- New curated surfaces: `string::utils`, `param::utils`; added `rsb::dev` and `rsb::prelude_ez` for fast prototyping.
- Updated docs: FEATURES_STRINGS/FEATURES_PARAMS; added `docs/development/MODULE_SPECIFICATION.md`.

7) Global module (core-only) established
- Implemented `rsb::global` per module spec with core store and helpers:
  - `global::store` (renamed `Global` struct; set/get/has/unset/expand/get_all)
  - `global::utils` (is_true/is_false, is_token_stream)
  - `global::config` (parse/load/save/export)
  - `global::registry` (function registry, call stack, colors/glyphs + helpers)
- `prelude` now exports `rsb::global::*`; `context` forwards to `global` (no behavior change).
- Added features and UAT wrappers for `global` (core ops + visible demo). All tests green.

8) Global adapters + host env
- Added `rsb::global::adapter` helpers:
  - Simple (no host): `import_env_simple`, `apply_env_simple`, `hydrate_simple`, `apply_config_files`.
  - Host-enhanced: `apply_env` (env + mode flags), `hydrate_env_and_files`.
- Added `rsb::dev_host::env` with `env_bootstrap`, `import_environment`, `setup_standard_modes`, and env helpers (get/set/has, sync).
- Added `rsb::dev_host::global` composer + prefix importer `import_env_with_prefix`.
- Created Global namespacing utilities (`rsb::global::ns`):
  - Supports `NS__KEY` and `NS::KEY`; `ns_set`, `ns_set_cc`, `ns_get`, `ns_get_all`, overlay helpers.
- Tests: adapter (simple/host), host env (sanity/UAT), global ns.

## Concepts Encountered
- Progressive enhancement: macros as thin front-ends with helper modules for staged evolution.
- Prelude policy: core-only; optional systems remain explicit.
- Visual runtime toggles: NO_COLOR, RSB_COLOR=never, color_mode; background enable via spec.
- Test wrappers to cleanly run nested folders with Cargo; test.sh auto-discovery.

## Key Documents
- HOWTO_UPDATE_RSB.md (zero-context primer)
- tests/README_TEST.md (test patterns and runner usage)
- FEATURES_COLORS.md, FEATURES_PARAMS.md
- FEATURES_BASH.md, FEATURES_THREADS.md
- SESSION.md, SESSION_CURRENT.md
- RSB_QUICK_REFERENCE.md, RSB_TACTICAL_GUIDE.md, RSB_ARCHITECTURE_PRINCIPLES.md
- REBEL: ../rebel/docs/REBEL.md, ../rebel/docs/rsb-architecture.md
- Module exposure spec: docs/development/MODULE_SPECIFICATION.md (read on hydration)
 - Global details: FEATURES_GLOBAL.md (adapters, namespacing, config)
 - Context → Global plan: CONTEXT_MIGRATION.txt

9) Module Spec alignment (helpers → utils)
- string, cli, date: helpers are internal; curated surfaces exposed via `utils`; prelude re-exports updated.
- hosts: impl modules made private; curated `hosts::utils` exposed; system info/command checks live in `hosts::system`.

10) New threads module (jobs/events/traps)
- Added `rsb::threads` with `utils` and module-owned macros.
- Provides `sleep_ms`, `bench`, `start_background`, `wait`, `list_jobs`.
- Moved `job!/event!/trap!` into `threads::macros` (crate-level export maintained).

11) New bash module (curl/tar/zip)
- Added `rsb::bash` with safe curl wrappers and archive helpers.
- `bash::macros` defines `curl!` (GET/POST) and re-exports archive macros.
- `string::utils::shell_single_quote` added for POSIX-safe quoting.
- `os` delegates curl/archive helpers to `bash`.

12) Examples + Binary
- `examples/minimal_cli.rs`, `examples/strings.rs`, `examples/threads.rs` added.
- `src/dummy_cli.rs` + [[bin]] `dummy-cli` for a working sample CLI.

13) Tests (sanity + UAT expanded)
- Added sanity/UAT wrappers for `threads` and `bash` per tests/README_TEST.md.
- All suites green under default features.

## Current Mission
- Complete param gaps; preserve bash-like behavior where feasible and valuable.
- Keep visuals optional and tidy; expand prompts as needed.
- Prepare module reorg RFC; keep changes non-breaking.

Global/Host migration focus (high priority, scoped)
- Finish context → global forwarding and trim `context.rs` (keep bootstrap until CLI step).
- Host: implement XDG/RSB paths and script awareness under `rsb::dev_host` and wire a CLI bootstrap.

New items captured during this session:
- Decide on `Context::expand` policy vs bash‑style patterns (RSB-018); ideally steer to `param!` or implement via syntax engine.
- Optional grapheme‑aware string ops behind a feature flag (RSB-STR-002).
- Remaining param case‑pattern transforms `${VAR^pat}` / `${VAR,pat}` (RSB-PARAM-004).
- Date follow-ups: UTC/millis variants and parsing robustness done; consider arithmetic helpers and duration modes.

## Tasks and Issues
- Tasks: `TASKS.txt` (ticket-style; use for planning and status)
- Issues/defects: `ISSUES.md` (gap tracking, especially for param bash-compat)

## Next Steps
Short‑term priorities (docs and safety)
1) String error handling: add StringError + guards + try_* surfaces (see STRING_ERROR_PLAN.md)
2) CLI bootstrap surface validated; macros now in cli; hosts bootstrap complete
3) RSB‑STREAM‑020: Stream module reorg plan (unblocks per‑line stream filters)

Backlog (tracked in TASKS.txt)
- RSB‑STR‑010: ASCII filter utilities (DONE)
- RSB‑STREAM‑011: Stream per‑line ASCII filters (BLOCKED by reorg)
- RSB‑ASC‑012: asc100 interop/marker alignment (investigation)
- RSB‑ARCH‑023: RFC to consolidate low‑level validation with asc100
- RSB‑STR‑006: PascalCase/Title Case
- RSB‑DATE‑003/004/005: Date QoL

## Quick Commands
```bash
./bin/test.sh list
./bin/test.sh run smoke
./bin/test.sh run all

cargo test
cargo test --features visuals
```

- Date module migration
- Introduced `src/date/` with `helpers.rs` and `macros.rs`; moved `date!` out of `macros/time_math.rs`.
- Migrated old `time.rs` helpers into `date::helpers` and added UTC/millis helpers and typed errors.
- Added feature tests (`features_date`) and UAT (`uat_date`).
Ready to continue. 🚀
