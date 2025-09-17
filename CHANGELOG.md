# Changelog

## 2025-09-17

### Changed
- Reworked the internal bookkeeping for `rsb::cli::Args` so positional access, flag
  consumption, and template expansion follow the documented contract (argv[0] is
  excluded from `$#`/`$@`, `has_pop` marks flags as consumed, and paired values are
  tracked safely).
- Updated UAT demonstrations (`tests/uat/dev.rs`, `tests/uat/gx.rs`,
  `tests/uat/parse.rs`) to respect real feature gates and API signatures so the
  suite builds cleanly without optional features.
- Migrated optional visual/prompt macros from the legacy bundle to
  `src/visual/macros.rs`, wired re-exports through `visual::mod.rs`, and removed the
  old `src/macros/visual.rs` module.

### Testing
- `cargo test --test sanity cli -- --nocapture`
- `cargo test --test uat -- --nocapture`

### Outstanding
- Investigate lingering compiler warnings (unused variables in UAT demos, legacy
  deprecated helpers) and decide whether to suppress or address them.
