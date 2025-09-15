# PLAN_TOKENS

Status: Draft
Date: 2025-09-15

Docs: docs/tech/features/FEATURES_TOKENS.md
Modules: src/token/{mod.rs, parse.rs, helpers.rs, error.rs}
Tests: tests/tokens/features/comprehensive.rs; tests/tokens_sanity.rs; UAT tests/uat/tokens.rs

Gaps/Findings
- Helpers have several dead_code warnings; ensure intended API is surfaced or trim internals.

Plan
- Validate parse and normalization logic; ensure error variants covered; document stream interop.

Acceptance
- Sanity + UAT pass; no dead_code for intended helpers; docs aligned.

