# PLAN_OPTIONS

Status: Draft
Date: 2025-09-15

Docs: docs/tech/features/FEATURES_OPTIONS.md
Modules: src/cli/{mod.rs, helpers.rs}; options! macro at crate root
Tests: tests/options.rs; tests/uat_stdopts.rs; sanity_main includes options integration; feature `stdopts` exists.

Gaps/Findings
- Ensure short-flag expansion under `stdopts`; document precedence with Args and global context.

Plan
- Validate MODULE_SPEC exposure for CLI; confirm no prelude leakage beyond intended APIs.

Acceptance
- Sanity + UAT green with and without `stdopts` feature.

