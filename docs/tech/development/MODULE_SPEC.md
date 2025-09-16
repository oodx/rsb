# RSB Module Specification (Helpers, Macros, Prelude, Integrations)

Updated: 2025-09-16

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
- `<module>/mod.rs` — orchestrator and re-exports. Owns the curated public surface. No business logic here.
- `<module>/utils.rs` — curated low-level helpers users may explicitly opt into ("utils" namespace).
- `<module>/helpers.rs` (optional) — internal implementations consumed by utils/orchestrator.
- `<module>/macros.rs` — module-owned macros. Prefer both forms when applicable:
  - Value form: consumes a provided value (e.g., `snake!(s)`).
  - Var form: fetches from Context first (e.g., `snake_var!("NAME")`).
- `<module>/error.rs` — typed error enums for consistent messaging.
- Streams integration — per-line wrappers for heavy transforms (e.g., case conversions) to process large inputs safely.

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

Prelude Policy (Amendment A)
- Re-export user-facing items and module-owned macros via `rsb::prelude::*`.
- Do not re-export internal submodules unless intentionally public and stable.
- Optional/visual features must not leak into prelude.
- Tests may import modules/macros directly as needed.

Param Macros
- `param!` stays crate-level; it delegates to helpers (e.g., `string::utils`, `string::case`).
- Keep `param!` as a DSL router. Push validation/logic into helpers.

ASCII-SAFE vs UNICODE-SAFE
- ASCII-SAFE: normalize output to ASCII (e.g., case transforms for filenames/configs). Non-ASCII is treated as a separator and stripped.
- UNICODE-SAFE: operate on Unicode scalars (e.g., substring via `chars()`), not graphemes.
- Docs: tag helpers with these labels; optionally maintain a small registry for debug listing.

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

Adapter Template (example)
```rust
// math/math_string_adapter.rs
#[cfg(feature = "string")]
pub fn parse_num_list(s: &str) -> Vec<f64> {
    // Example: reuse string helpers to split/sanitize
    rsb::string::utils::filter_ascii_strip(s)
        .split(',')
        .filter_map(|t| t.trim().parse::<f64>().ok())
        .collect()
}

#[cfg(not(feature = "string"))]
pub fn parse_num_list(_s: &str) -> Vec<f64> { Vec::new() } // safe fallback
```
And in `math/mod.rs`:
```rust
#[cfg(feature = "string")]
pub use self::math_string_adapter::parse_num_list;
```

Error and Logging Conventions
- Core modules must not depend on optional visual macros (`info!`, `warn!`, `error!`, etc.).
- Use `utils::stderrx(level, msg)` for best-effort, non-visual logging.
- Wrap examples/tests that use visual macros with `#[cfg(feature = "visual")]`.

Macro Surface Guidelines
- Keep macros thin; delegate logic to helpers.
- Prefer value + var forms for string‑oriented macros.
- Export module‑owned macros at crate root; re-export via `prelude::macros`.

Feature Flags (Cargo.toml)
```toml
[features]
default = []

# Visual base and components
visual = []
colors-simple = ["visual"]
colors-named  = ["visual", "colors-simple"]
colors-status = ["visual"]
glyphs = ["visual"]
prompts = ["visual", "colors-simple"]

# Umbrellas and convenience
colors  = ["visual", "colors-simple"]
visuals = ["visual", "colors-simple", "colors-named", "colors-status", "glyphs", "prompts"]

# Options helpers
stdopts = []
```

Streams Guidance
- Heavy, line-wise transformations belong in `streams` to handle large inputs safely.
- Provide ergonomic wrappers in modules for common stream patterns where it adds value.

Companion Preludes (development)
- `rsb::prelude_ez` — core prelude plus curated helpers/macros for prototyping; still respects feature gating.
- `rsb::prelude_dev` — dev/test helper namespaces (e.g., `prelude_dev::string`); unstable and intentionally narrow.

Maintenance Guidelines
- Adding to prelude: ensure broad usage, no std conflicts, no optional deps, stable API.
- Prefer adding to `prelude_ez` or `prelude_dev` rather than expanding the core prelude.
- Removing from prelude: provide deprecation period, migration notes, and (if necessary) a temporary feature flag.

Testing
- Include a baseline sanity test for each module without optional features.
- Gate adapter/integration tests with the required features; ensure runner lanes cover both default and feature-enabled profiles.
- Use `bin/test.sh` lanes (`smoke`, `all`) and cargo feature combos as documented in HOWTO_TEST.md.
