# RSB Module Specification (Helper Exposure + Macros + Prelude)

Updated: 2025-09-12

##  Purpose
- Define a consistent pattern for how modules expose low-level helpers, macros, errors, and guards.
- Keep the user-facing surface ergonomic and predictable while allowing advanced/low-level usage.

## Principles
- Single source of truth per module; avoid duplicate helpers scattered across the codebase.
- Keep macros thin; push logic into helper functions.
- Curate the prelude; re-export only what typical apps need.
- Prefer ASCII-first naming/case transforms; document Unicode semantics for other helpers.
- use concise UNIX-style names for files, variables and modules, only use longer names when absolutely necessary or when additional clarity or distinction is required! e.g. `my_long_name` => `long` or `calculate_first_derivative` => `calc_deriv`; shorter names indicate a level of generality `calc` vs `calc_deriv` <== additional specificity.

## Structure (per module)
- `<module>/mod.rs`: orchestrator and re-exports. Owns the curated public surface, no other functions or implementations are allowed. All implementations need their own file adjacent to mod.rs.
- `<module>/utils.rs`: curated low-level helpers users may explicitly opt into ("utils" namespace).
- `<module>/helpers.rs` (optional): internal helper implementation files consumed by utils and the orchestrator.
- `<module>/macros.rs`: module-owned macros. Prefer two forms when applicable:
  - Value-form macro: consumes a provided string
  - Var-form macro: fetches from Context and then applies the operation
- `<module>/error.rs`: typed error enums for consistent messaging.
- Guards: central guard helper(s) for common policies (size/time).
- Streams: per-line wrappers for heavy transforms (e.g., case conversions), to enable processing large inputs safely.

Prelude Policy (Amendment A alignment)
- Re-export user-facing items and module-owned macros via `rsb::prelude::*`.
- Avoid re-exporting "utils" or internal submodules unless they are intentionally part of the public surface.
- Tests may import modules/macros directly as needed.

## Param Macros
- `param!` stays crate-level; it delegates to module helpers (e.g., `string::utils` / `string::case`).
- Avoid business logic inside `param!`—keep it as a DSL router to helpers.

ASCII-SAFE vs UNICODE-SAFE
- ASCII-SAFE: functions that normalize to ASCII-only output (e.g., case transforms for filenames/configs). Non-ASCII is treated as a separator and stripped.
- UNICODE-SAFE: functions that respect Unicode scalar semantics (e.g., substring via `chars()`), but may not be grapheme-aware.
- Doc guidance:
  - Mark helpers in docs with these labels.
  - Optional: maintain a static, hand-curated registry for debug listing (no proc-macro/reflection required).

Future
- Optional feature flags (e.g., transliteration for slugs).
- Grapheme-aware alternatives behind a `string-graphemes` feature.
