# PLAN_STRINGS

Status: Draft
Date: 2025-09-15

Docs: docs/tech/features/FEATURES_STRINGS.md
Modules: src/string/{mod.rs, utils.rs, helpers.rs?, macros.rs, error.rs}
Tests: tests/features/string/{string_test.rs, case_test.rs, macros.rs, errors_test.rs, ascii_filter_test.rs, invalid_glob_test.rs}; sanity_main covers string basics.

Gaps/Findings
- Good coverage breadth. Verify wildcard prefix/suffix behavior matches doc (Unicode-safe slicing). Confirm failure modes in error.rs.

Plan
- Verify MODULE_SPEC: orchestrator re-exports, macros thin, utils curated.
- Sanity check prelude exports (no internal utils leakage beyond curated list).
- Add UAT demo under tests/uat/string.rs (if missing) showing common pipelines.
- Docs cross-links to REFERENCE where relevant.

Acceptance
- Sanity + UAT pass. Behavior matches FEATURE doc semantics; prelude surface matches policy.

