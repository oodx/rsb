# PLAN_COLORS

Status: Completed
Date: 2025-09-15

Docs: docs/tech/features/FEATURES_COLORS.md
Modules: src/visual/colors/{mod.rs, registry.rs, simple.rs, status.rs}
Tests: tests/features/colors/{sanity.rs, runtime.rs}; UAT under tests/uat/colors*.rs and tests/uat/visual.rs

Gaps/Findings
- Feature-gated; ensure no leakage via prelude; require explicit `use rsb::visual::colors::*`.
- Enabling `--features visuals` used to fail due to macro redefinitions (`error!` existed in both the visual bundle and `macros/stderr.rs`). Issue resolved by relocating the visual macros to `src/visual/macros.rs` and gating the core fallbacks behind `#[cfg(not(feature = "visual"))]`.
- Verify runtime toggles and glyph integration behind features.

Plan
- Validate registry completeness and tag expansion; ensure {bg:*} semantics in docs.
- UAT to show enabling modes and runtime toggles.
- Resolve macro conflict by providing a single source of truth for `error!/warn!/info!`:
  - Option A: add fallback macros under `#[cfg(not(feature = "visual"))]` and define visual ones only when `feature = "visual"` (ensure core fallback not compiled with visual).
  - Option B: rename internal core fallback macros (not preferred; breaks API).
  - Target: A.

Acceptance
- cargo test --features visuals passes; default cargo test unaffected.

Result
- Resolved macro duplication by gating core log macros under `#[cfg(not(feature="visual"))]` and removing an unconditional legacy `error!` macro.
- Ran `cargo test --features visuals --test features_colors` and prompts UAT — passed.
