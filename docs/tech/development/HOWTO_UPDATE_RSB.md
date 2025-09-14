# HOWTO: Update and Refactor RSB (Read Me First)

**WARNING**: Verify repository folder name before making changes.
- If you are working in `.../repos/code/rust/oodx/rsb2`, this is a temporary workspace due to issues with the original `rsb` checkout. Proceed, but expect a later consolidation back to `rsb`.
- If only `rsb/` exists and `rsb2/` does not, the consolidation has happened; use `rsb/` as the canonical path.

**Purpose**: Give a zero-context engineer or agent a fast, opinionated guide to make safe, consistent changes to RSB without rediscovering patterns. Summarize key paradigms (prelude policy, progressive enhancement, macros, tests, features) and where to find deeper docs.

# Chapter 1: Quick Orientation
- Code root: `src/`
  - `lib.rs`: module exports; do not casually add to prelude; keep optional packages behind features.
  - `prelude.rs`: exports only core APIs/macros; OPTIONAL packages (visuals, etc.) must be explicit imports.
  - `macros/`: legacy grouped macros (core, text, streams_exec, fs_data, time_math, visual, etc.). New work should prefer module‑owned macros (see Chapter 2).
  - `param/`: progressive helpers for `param!` macro. Owns `param!` in `src/param/macros.rs`.
  - `string/`: general string helpers and macros. `mod.rs` orchestrates `helpers.rs` and `macros.rs`.
  - `date/`: date/time helpers and macros. `mod.rs` orchestrates `helpers.rs` and `macros.rs`. Replaces legacy `time` module.
  - `visual/`: optional color/glyph/prompt packages behind feature flags.
  - `xcls/`, `streams.rs`, `streamable/`: stream utilities and helpers.

## 1.1: Tests Structure
- Tests root: `tests/` (see Chapter 4 for detailed structure and runner usage)

## 1.2: Essential Documentation
- `README.md` (top-level orientation + Visuals section)
- `FEATURES_COLORS.md` (visual colors quick stub)
- `FEATURES_PARAMS.md` (param progressive enhancement plan)
 - `FEATURES_STRINGS.md` (string helpers, macros, Unicode behavior)
 - `FEATURES_GLOBAL.md` (global store/expansion/config/introspection)
 - `docs/development/MODULE_SPECIFICATION.md` (module helper/macro/prelude exposure spec)
  - `rsb::prelude_ez` (EZ prelude for rapid prototyping; includes curated low-level helpers)
- `FEATURES_DATE.md` (date macros and helpers)
- `SESSION.md`, `SESSION_CURRENT.md` (what changed recently)
- `RSB_QUICK_REFERENCE.md`, `RSB_TACTICAL_GUIDE.md`, `RSB_ARCHITECTURE_PRINCIPLES.md`
- REBEL references: `../rebel/docs/REBEL.md`, `../rebel/docs/rsb-architecture.md`

EVERY NEW MAJOR FEATURE NEEEDS A FEATURE_<NAME>.md FILE BE SURE TO CREATE IT WHEN YOU MAKE ONE

# Chapter 2: Core Paradigms and Policies

## 2.1: Prelude Policy (Critical)
- The prelude is core-only. Do NOT export optional subsystems (visual/log macros, colors) via prelude.
- Optional components require explicit imports by callers, even when features are enabled.

## 2.2: Progressive Enhancement Pattern
- Prefer a small, stable macro/API surface with internal helpers organized for staged evolution.
- Example: `param!` macro uses `rsb::param::basic` helpers under the hood; `param::advanced` holds future tracing/features.
- Example: `visual::colors` exposes a runtime registry; named colors stay in a global HashMap (no enum) by stakeholder direction.

## 2.3: Module‑Owned Macros and Orchestrators
- **New policy**: each domain module owns its macros under `<module>/macros.rs` and exposes functions via `<module>/helpers.rs`.
- `<module>/mod.rs` acts as an orchestrator that re‑exports its helpers (`pub use helpers::*;`) and includes its macros module.
- We retain a unified import path via `prelude::macros` which re‑exports legacy grouped macros and module‑owned macros (e.g., `param`, `str_*`).
  - Example: `date!` moved from `macros/time_math.rs` to `date/macros.rs` and is re‑exported via the prelude alias.

## 2.4: String Helpers Consolidation
- String helper functions were moved from `utils` to the dedicated `string` module.
- Wildcard prefix/suffix removal uses anchored regex patterns and iterates on Unicode char boundaries to avoid UTF‑8 slicing panics.
- See `FEATURES_STRINGS.md` for details, Unicode notes, and tests. Grapheme‑aware behavior may be added behind a feature flag.

## 2.5: Features and Optional Packages
- Base feature is minimal. Optional packages are behind flags:
  - `visual` base + `colors-simple`, `colors-status`, `colors-named`, `glyphs`, `prompts`.
  - `visuals` umbrella aggregates color sets + glyphs + prompts.
- Ensure callers opt in explicitly; do not make visuals a transitive surprise.

## 2.6: Legacy Macro Organization
- All legacy macros live under `src/macros/` and export at crate root via `#[macro_export]`.
- Group macros logically (core, control_validation, text, time_math, fs_data, streams_exec, visual, etc.).
- Visual/log macros (e.g., `colored!`, `info!`) depend on `utils::expand_colors_unified` and should be considered optional.
- For inline tag macros like `colored!`: support a single-arg form to avoid format! brace conflicts.

## 2.7: Visual Colors and Registry (Optional)
- String-first runtime registry. Case-insensitive lookups.
- Backgrounds are off until explicitly enabled via `color_enable_with("...,bg")`.
- `colored("{...}")` expands inline tags; unknown tags pass through verbatim; glyph tags only render when glyphs are enabled.
- Named colors MUST remain in the global HashMap (not enums).

# Chapter 3: Testing Framework
## 3.1: Test Structure
Use the structure in `tests/README_TEST.md`. High-level:
- **Sanity**: `tests/sanity_main.rs` wrapper; includes `tests/sanity.rs` and `tests/sanity/baseline.rs` (visible demos)
- **Features**: `tests/features/<module>/*.rs` with a wrapper `tests/features_<module>.rs`
- **UAT**: `tests/uat/*.rs` with a wrapper `tests/uat_main.rs`
- **Shell tests**: `tests/sh/*.sh`

IMPORTANT! all tests must implement a sanity test (check core assumptions), and a visual uat test (show the commands being called and show the outputs) -- read `tests/README_TEST.md`

## 3.2: Test Runner
The test runner: `bin/test.sh`
- `./bin/test.sh list` shows available tests and auto-discovered wrappers.
- `./bin/test.sh run <name>` runs a mapped test or any auto-discovered wrapper (e.g., `features_colors`).
- `./bin/test.sh run smoke` quick lane; `./bin/test.sh run all` full lane.

## 3.3: Adding New Test Suites
Add new test suites by creating a wrapper `tests/<module>_<suite>.rs` and placing files under `tests/<module>/<suite>`. No need to edit `test.sh` thanks to auto-discovery. If you add named entries, follow the existing mapping style.

**Example**: New string suite - wrapper `tests/features_string.rs` with tests under `tests/features/string/`.

# Chapter 4: Development Patterns
## 4.1: Pre-Refactoring Checklist
Before refactoring:
- Confirm prelude policy impact (avoid adding exports there).
- Identify feature flags needed.
- Ensure tests cover both default profile and feature-enabled profile.

## 4.2: Module Creation/Refactoring
- Keep code in `src/<module>` directories when it grows beyond a single file.
- Prefer module‑owned macros: create `<module>/macros.rs` instead of adding to `src/macros/`; update `prelude::macros` re‑exports if needed.
- For progressive enhancement, expose a small public API and delegate to `::<module>::basic` internally, reserving `::<module>::advanced` for richer features later.

## 4.3: Visual Component Additions
- Add color names to the named palette map (registry), not enums. Update runtime registry if needed.
- Add UATs under `tests/uat/` and feature tests under `tests/features/colors/`.

# Chapter 5: Change Validation

## 5.1: Minimal Checklist for Changes
- [ ] Respect prelude policy (no optional exports in prelude)
- [ ] Use progressive helpers (`::<module>::basic`) for complex macros
- [ ] Gate optional features; keep default lean
- [ ] Add/adjust integration tests via wrappers (sanity/features/uat)
- [ ] Ensure `bin/test.sh list` shows your new wrappers
- [ ] Run: `cargo test` (default) and `cargo test --features visuals` if applicable
- [ ] Run: `./bin/test.sh run smoke` and `./bin/test.sh run all`
- [ ] Update `prelude::macros` if adding new module‑owned macros
- [ ] Update feature guides (`FEATURES_STRINGS.md`, `FEATURES_PARAMS.md`) if behavior changes

# Chapter 6: Common Commands
- `cargo test` — default tests (no visuals)
- `cargo test --features visuals` — visual suites
- `./bin/test.sh list` — discover tests
- `./bin/test.sh run sanity` — sanity package (core + baseline demos)
- `./bin/test.sh run colors` — color feature suites
- `./bin/test.sh run param` — param feature suites
- `./bin/test.sh run uat-colors` — visual UATs (requires visuals)
- `./bin/test.sh run smoke` — fast checks
- `./bin/test.sh run all` — full checks

# Chapter 7: Workspace Notes

## 7.1: rsb2 Temporary Workspace
- If you see `rsb2` in the path, you are in the temporary workspace. Continue refactors here; coordinate a later move to `rsb/`.
- After consolidation (only `rsb/` exists), remove this warning and update references.

# Chapter 8: Additional Resources

## 8.1: Architecture and Context
- For architectural intent, consult REBEL docs and `RSB_ARCHITECTURE_PRINCIPLES.md`.
- For current session context, read `SESSION_CURRENT.md`.

## 8.2: Module-Specific Quick Reference
- `param!` lives at `src/param/macros.rs`; helpers at `src/param/basic.rs`.
- Strings live at `src/string/` with `helpers.rs` and `macros.rs`. See `FEATURES_STRINGS.md` for Unicode and wildcard semantics.
- Module exposure pattern and naming conventions are defined in `docs/development/MODULE_SPECIFICATION.md`.
 - For dev/testing convenience, `rsb::dev` aggregates curated low-level helpers:
   - `rsb::dev::string` → `string::utils` (helpers, case, error, safety registry)
   - `rsb::dev::param` → `param::utils`
   - Note: stream items are intentionally deferred until the stream module reorg is complete.
