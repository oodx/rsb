# PLAN_BASH

Status: Draft
Date: 2025-09-15

Docs: docs/tech/features/FEATURES_BASH.md
Modules: src/bash/{mod.rs, http.rs, archive.rs}
Tests: tests/bash/sanity.rs; tests/uat/bash.rs; wrappers present.

Gaps/Findings
- Network access in CI may be limited; tests should be resilient (use HEAD/timeout or stub).

Plan
- Confirm error mapping and outputs; document options.

Acceptance
- Sanity + UAT green locally; CI strategy documented.

