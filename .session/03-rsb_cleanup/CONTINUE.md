# RSB Continuation Guide (Next Session)

Date: 2025-09-15
Branch: main
Repo: rsb

Quick snapshot
- Prelude policy: clean — no visual/log macros exported via prelude.
- Math/Threads: macros already decoupled from visual logging (use utils::glyph_stderr).
- Math UAT: uses rsb::global::{set_var, get_var} functions (no missing macros).
- CLI help/inspect/stack: use utils::expand_colors_unified for color tag handling.

Checklist (start here)
- Run quick tests:
  - ./bin/test.sh list
  - ./bin/test.sh run smoke
- Run full tests:
  - cargo test
  - cargo test --features visuals
  - ./bin/test.sh run all
- Verify CLI introspection output (no raw tags when visuals on):
  - cargo run --example showcase --features visuals -- help
  - cargo run --example showcase --features visuals -- inspect
  - cargo run --example showcase --features visuals -- stack

Open decisions
- Plain mode color tags: expand_colors_unified currently returns text unchanged without visual; consider stripping inline tags in plain mode if output readability matters for default builds.

If failures occur
- Re-check any math/threads macro calls still using error!/info! inside other modules.
- Confirm tests don’t rely on legacy global macros; prefer rsb::global functions.

Links
- Current session context: .session/SESSION_CURRENT.md
- Update guide: docs/tech/development/HOWTO_UPDATE_RSB.md
- CLI feature doc: docs/tech/features/FEATURES_CLI.md
