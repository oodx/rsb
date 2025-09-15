# PLAN_COLORS

Status: Draft
Date: 2025-09-15

Docs: docs/tech/features/FEATURES_COLORS.md
Modules: src/visual/colors/{mod.rs, registry.rs, simple.rs, status.rs}
Tests: tests/features/colors/{sanity.rs, runtime.rs}; UAT under tests/uat/colors*.rs and tests/uat/visual.rs

Gaps/Findings
- Feature-gated; ensure no leakage via prelude; require explicit `use rsb::visual::colors::*`.
- Verify runtime toggles and glyph integration behind features.

Plan
- Validate registry completeness and tag expansion; ensure {bg:*} semantics in docs.
- UAT to show enabling modes and runtime toggles.

Acceptance
- cargo test --features visuals passes; default cargo test unaffected.

