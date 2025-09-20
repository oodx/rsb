# Contributing to RSB

Thanks for contributing! This guide captures practical expectations so changes stay predictable and easy to review.

## Principles
- Prelude policy: keep `rsb::prelude` core-only. Optional subsystems (visuals, logging, progress, etc.) require explicit imports and must not be exported via the prelude.
- MODERN + SPEC_ALIGNED: new or refactored modules follow `docs/tech/development/MODULE_SPEC.md`.
- Small, focused PRs: limit scope to the feature or bug you’re addressing.
- Tests before polish: prioritize adding/adjusting tests and examples.

## Checklist
- Code
  - [ ] Respect prelude policy (no optional exports via prelude)
  - [ ] Feature‑gate optional subsystems (Cargo.toml + module `#[cfg]`)
  - [ ] Update or add tests (sanity, features, UAT as appropriate)
  - [ ] Keep changes minimal and aligned to MODULE_SPEC
- Docs
  - [ ] Update the relevant `docs/tech/features/FEATURES_<NAME>.md`
  - [ ] Add/adjust entries in `docs/tech/INDEX.md` (table; relative links)
  - [ ] If adding new features, update the README feature table and Cargo feature map
- Behavior
  - [ ] For user‑visible output changes (e.g., progress formatting), open a brief RFC/discussion first

## Running Tests
- Core: `cargo test`
- Visuals: `cargo test --features visuals`
- Progress: `cargo test --features progress`
- Runner lanes: `./bin/test.sh list` then `./bin/test.sh run <lane>`

## Style & Linting
- Format: `cargo fmt`
- Lint: `cargo clippy -- -D warnings`

## Session Notes (Optional)
- Add/update a session file under `.session/` describing your changes and how to resume work with zero context.

## Getting Help
- See `docs/tech/INDEX.md` for feature docs, HOWTOs, and references.
