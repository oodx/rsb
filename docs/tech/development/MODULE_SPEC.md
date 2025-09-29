Updated: 2025-09-28

MODULE_SPEC v3


## Standard Crate Structure

*RSB Standard Folders*
- `<proj>` the assumed project root
- `<proj>/bin` — our standard tool scripts like `test.sh` `deploy.dh` `snap.sh` `build.sh` among others stored here
- `<proj>/docs` — reference and status docs 
- `<proj>/docs/proc` — our meta process docs includes ROADMAP, TASKS, and other status notes
- `<proj>/docs/ref` — reference docs like concepts, architectures, patterns, strategies, guidelines
- `<proj>/docs/feats` — usually per sub-module or feature readme for the project FEATURES_<name>.md
- `<proj>/src` — all primary source code lives here
- `<proj>/tests` — structured test suite here (sanity and uat tests required)

*RSB Optional Folders*
- `<proj>/conf` — location for project specific configurations 
- `<proj>/data` — stateful data or databases
- `<proj>/examples` — uat feature examples
- `<proj>/benches` — benchmark runners
- `<proj>/tmp` — local temp folder as needed (must always be removed before checkin)

## Standard Src Specification

- `src/lib.rs` - entry point for project api (library usage)
- `src/main.rs`  — primary executable entrypoint (no business logic here, just orchestration)
- `src/prelude.rs`  — global prelude orchestrator
- `src/lang.rs`  — standard simple string constnat location (whenever other lang,i8ln, or string patterns are not explicitly used)
- `src/deps.rs`  — standard location for re-exporting non-rsb and non-hub based crates (see HOWTO_HUB.md)
- `src/bin` — cli and repl
- `src/<pkg>`  — vanity package namespace for all code and sub modules
- `src/<pkg>/<module>/*`  — see Sub Module Specification section

*IMPORTANT*  no other top level files or modules are allowed under `src` legacy files need to be refactored or reorgd into a proper submodule space.


## Sub Module Specification (Helpers, Macros, Prelude, Integrations)

Purpose
- Define a consistent pattern for how modules expose low-level helpers, macros, errors, guards, and cross‑module integrations.
- Keep the user-facing surface ergonomic and predictable while allowing advanced/low-level usage.

Design Principles
- Single source of truth per module; avoid duplicate helpers scattered across the codebase.
- Keep macros thin; push logic into helper functions.
- Curate the prelude; re-export only what typical apps need (see PRELUDE_POLICY.md).
- Prefer ASCII-first naming/case transforms; document Unicode semantics where relevant.
- Reuse existing low-level helpers across modules; isolate integrations to avoid hard/circular deps.

Module Layout (per module)
- Keep code in `src/<module>` directories when it grows beyond a single file.
- Module directory names should be short and concise.
- `<module>/mod.rs` — orchestrator and re-exports. Owns the curated public surface. No business logic here.
- `<module>/utils.rs` — curated low-level helpers users may explicitly opt into ("utils" namespace).
- `<module>/helpers.rs` (optional) — internal implementations consumed by utils/orchestrator.
- `<module>/macros.rs` — module-owned macros. Prefer both forms when applicable:
- `<module>/error.rs` — typed error enums for consistent messaging.
- For progressive enhancement, expose a small public API and delegate to `::<module>::basic` internally, reserving `::<module>::advanced` for richer features later.
- **REQUIRED:** Create sanity and UAT tests for new modules or tests will be blocked.

Macro Surface Guidelines
- Keep macros thin; delegate logic to helpers.
- Export module‑owned macros at crate root; re-export via `prelude::macros`.

Cross Module Integration
- try to keep module files pure as in they dont depend on other RSB modules; this enables progressive enhancement with feature flags
- when you need to create functions that dep on other modules use the cross module integration pattern (these are non exhaustive examples) =>
  consider 4 modules
  (mod A) apple.rs
  (mod B) banana.rs
  (mod C) knife.rs
  (mod D) hammer.rs
  This pattern requires you to create seperate integration modules in the parent module. 
  Examples:
    apple.rs uses knife.rs
      - LITERAL SMOOSH  => apple_knife.rs  (A + C)
      - DESCRIPTIVE     => apple_cutter.rs (what C applies to A)
      - ADAPTER         => apple_knifer_adp.rs (add _adp suffix)
    knife.rs uses hammer.rs (maybe just the handle)
      - PART EXTRACTION => knife_grip.rs (A uses a part of C only)
    apple.rs uses banana.rs
      - MISC UTILS      => apple_banana_utils (utils using A+B)


Cross‑Module Integrations (Adapters)
- Goal: reuse helpers across modules without hard/circular dependencies.
- Location: Consumer module only, in a dedicated file.
  - Naming: `<module>_<dep>_adapter.rs` or `<module>_<dep>_shared.rs`.
  - Examples:
    - `math/math_string_adapter.rs` — math uses string helpers for parse/format.
    - `threads/threads_global_adapter.rs` — threads consults global flags.
- Isolation and gating:
  - Adapter is the only place referencing the foreign module.
  - Gate with `#[cfg(feature = "<dep-feature>")]` or a grouping like `integrations`.
  - Provide graceful fallbacks under `#[cfg(not(feature = "<dep-feature>"))]` (no‑op/minimal behavior).
- Ownership:
  - Choose a primary module (the one that owns the user-facing API) and place the adapter there.
  - Avoid mutual adapters or circular feature dependencies.
- Exposure:
  - Re-export only the adapter helpers actually needed by users via the primary module’s `mod.rs`.
  - Never re-export entire foreign modules from adapters.
- Tests:
  - Add adapter feature tests under `tests/features/<module>/` and gate with the required features.
  - Maintain at least one sanity test for the primary module that does not require the adapter feature.


Prelude Policy (Amendment A)
- Re-export user-facing items and module-owned macros via `<project>::prelude::*`.
- Do not re-export internal submodules unless intentionally public and stable.
- Optional features must not leak into prelude.
- Tests may import modules/macros directly as needed.
