# PLAN_MATH

Status: In Progress
Owner: agent
Date: 2025-09-15

Concept: Math (library functions + macros)

Docs
- Feature doc: docs/tech/features/FEATURES_MATH.md (present)

Module Surfaces (expected per MODULE_SPEC)
- src/math/mod.rs (orchestrator): exposes helpers, re-exports curated surface
- src/math/helpers.rs (optional) and/or utils.rs: implementation helpers
- src/math/macros.rs: module-owned macros (thin), no visual/log macro dependency

Current Tests
- Sanity: tests/math_sanity.rs (present)
- UAT: tests/math_uat.rs (present, rich demo)

Gaps / Findings
- Core/visual coupling: math macros used `$crate::error!` for error reporting which is a visual/log macro not in the prelude. This breaks default builds that don’t enable visual features.
- UAT globals: tests used `set_var!`/`get_var!` macros that don’t exist; use `rsb::global::{set_var, get_var}` functions instead.

Plan to Compliance
1) Decouple macros from visual logging
   - Change `$crate::error!(...)` to `$crate::utils::stderrx("error", &format!(...))` in `src/math/macros.rs`.
   - Rationale: stderrx is core, feature-neutral, respects quiet/debug modes.
2) Ensure MODULE_SPEC shape
   - Verify orchestrator, helpers, utils split; macros thin and delegating to helpers.
   - Confirm curated exports in prelude remain core-only; no macro leakage.
3) Test stabilization
   - Fix UAT to use function-based globals.
   - Run: `cargo test` (default) and `cargo test --features visuals`.
   - Add a minimal feature test under `tests/features/math/` if desired (sanity ops), but existing sanity+UAT likely sufficient.
4) Docs
   - Confirm/refresh `FEATURES_MATH.md` examples using function-based globals
   - Note error reporting behavior is core-only.

Acceptance Criteria
- Default `cargo test` passes without visual features.
- `cargo test --features visuals` passes.
- Sanity + UAT demonstrate expected operations; errors log via core utils.
- No visual/log macros exported via prelude; macros compile feature-neutrally.

Notes
- This plan has step 1+3 implemented in this session; proceeding to run validations and then commit.
