# PLAN_HOST

Status: Draft
Date: 2025-09-15

Docs: docs/tech/features/FEATURES_HOST.md
Modules: src/hosts/{mod.rs, env.rs, paths.rs, system.rs, global.rs}
Tests: tests/host_env.rs, tests/host_paths.rs; wrappers in tests/uat_host_env.rs, tests/uat_host_paths.rs.

Gaps/Findings
- Ensure XDG path derivations and fallbacks documented; environment hydration order clarified.

Plan
- Verify MODULE_SPEC split; confirm helpers are crate-internal or curated.

Acceptance
- Sanity + UAT green; docs match behavior.

