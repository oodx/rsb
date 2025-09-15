# RSB — CONTINUE.md (Session Handoff)

Purpose
- Provide a clear, opinionated entry point to understand RSB quickly and contribute effectively.
- Capture what was just done, what to read next, how to validate locally, and what tasks are pending.

Project Summary
- RSB is an opinionated Rust framework for “too big for bash, too small for Rust” tools.
- String‑first design with a bash‑like macro DSL (echo!, cmd!, cat!, validate!, param!, dispatch!, etc.).
- Architecture borrows from BashFX: function ordinality, predictable structure, optional visuals behind feature flags.
- REBEL philosophy: accessibility over purity, pragmatic safety, fail fast with clear messages.

Read This First (in order)
1) README: project overview, philosophy, macro DSL quick look
   - README.md → section “Start Here” for core links
2) Development guide: how to make changes safely; policies
   - docs/tech/development/HOWTO_UPDATE_RSB.md
3) Module Spec: helper exposure, module‑owned macros, prelude policy
   - docs/tech/development/MODULE_SPEC.md
4) Tests: structure, wrappers, runner usage
   - docs/tech/development/HOWTO_TEST.md
   - tests/README_TEST.md
5) Architecture/Philosophy references
   - docs/tech/reference/RSB_ARCH.md
   - docs/tech/reference/REBEL.md
6) Feature docs (skim relevant areas)
   - docs/tech/features/FEATURES_*.md (Strings, Params, Global, Colors, Date, Tokens, Math, Prompts, Options, Host, Threads, CLI)

Local Workflow (quick)
- List tests: `./bin/test.sh list`
- Run smoke/all: `./bin/test.sh run smoke` | `./bin/test.sh run all`
- Visuals: `cargo test --features visuals`
- Default tests: `cargo test`
- Example CLIs: see examples/ and tests/sh/ scripts

Contribution Guidelines (must‑knows)
- Prelude policy: prelude is core‑only. Optional visuals/log macros remain explicit (import when needed).
- Module‑owned macros: each domain’s macros live in `<module>/macros.rs`; `mod.rs` orchestrates helpers + re‑exports curated APIs.
- Progressive enhancement: thin macro surfaces delegate to helpers (::<module>::basic); reserve advanced areas for future growth.
- String bias: prefer string‑first interfaces; hide heavy types; adopt streams where practical for a Unix‑like flow.
- Function ordinality: public orchestration → crate helpers → low‑level system functions; scope errors accordingly (user/app/system fault).
- Tests: every module should have a sanity test and a visual UAT; use wrappers so `test.sh` auto‑discovers suites.

State of the Repo (this session)
- History aligned and authors normalized in the previous rsb.old repo; new repo (this one) contains the “newer code”.
- Docs updated:
  - README includes “Start Here” with core links.
  - HOWTO_UPDATE_RSB now points to MODULE_SPEC.md; a redirect stub exists at MODULE_SPECIFICATION.md.
  - Feature docs validated present under docs/tech/features.

Validate Locally (before pushing changes)
- `./bin/test.sh list`
- `./bin/test.sh run smoke`
- `./bin/test.sh run all`
- `cargo test` and `cargo test --features visuals`

Pending Tasks (top)
- Prelude audit: ensure optional visuals/log macros are not exported via prelude.
- Cross‑link docs: add pointers where helpful (dev/reference hub page or docs index).
- Run and stabilize tests: default and visuals feature profiles; adjust wrappers if discovery misses any suites.
- Module‑owned macro migration: move legacy macros from src/macros/ into their modules; update prelude::macros re‑exports.
- Verify remaining doc references for stale paths; fix any lingering mismatches.
- Optional: add docs/index.md to hub key guides; CI to run `smoke` lane and visuals on demand.

Suggested Next Steps (hands‑on)
1) Prelude audit
   - Inspect prelude exports; confirm visuals/loggers remain opt‑in.
2) Tests
   - Run smoke/all and cargo tests; capture any failures and file follow‑ups.
3) Macro migration (incremental)
   - Choose one area (e.g., date, string, or tokens) and move macros to `<module>/macros.rs`; update orchestrator and prelude::macros.
4) Doc pass
   - Add a docs index if helpful; ensure README “Start Here” stays current.

Current Status Snapshot (2025-09-15)
- All major concept tests are green (sanity, features, UAT; visuals included).
- Visuals macro duplication resolved; core log macros gated under non-visual.
- Docs index (`docs/tech/INDEX.md`) added; README cross-links in place.

Quick Pointers
- Patterns: docs/tech/reference/RSB_ARCH.md
- Philosophy: docs/tech/reference/REBEL.md
- Features: docs/tech/features/FEATURES_*.md
- Tests: docs/tech/development/HOWTO_TEST.md, tests/README_TEST.md, bin/test.sh
- Dev policy: docs/tech/development/HOWTO_UPDATE_RSB.md, docs/tech/development/MODULE_SPEC.md

Contact/Notes
- Keep changes small and focused; follow patterns in MODULE_SPEC and HOWTO_UPDATE_RSB.
- Prefer PRs that include test updates and doc tweaks where behavior changes.
