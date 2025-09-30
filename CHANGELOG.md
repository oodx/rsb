# Changelog

## 2025-09-30

### Added
- **REPL Module**: Complete v1 implementation with interactive command processing
  - Quote-aware command tokenization with SimpleParser
  - Built-in commands: exit, quit, clear, history, help
  - REPL macros: repl_arg!, repl_argc!, repl_args!, repl_argv!, repl_dispatch!
  - Pluggable parser architecture via ReplParser trait
  - Dynamic prompt configuration (TOML → env → default)
  - 42 comprehensive tests (35 sanity + 7 UAT)
  - Documentation: FEATURES_REPL.md
  - Integration example: examples/repl_demo.rs

### Fixed
- **Test Isolation (BUG-01)**: Resolved global state contamination in REPL and CLI tests
  - Added cleanup helpers with RSB_GLOBAL_RESET pattern
  - Serial execution markers for tests sharing globals
  - All tests now pass in parallel (380 sanity + 94 UAT)

### Changed
- **Module Alignment**: REPL module updated to MODULE_SPEC v3 compliance
- **Test Discovery**: Added REPL to bin/test.sh (sanity-repl, uat-repl)
- **Documentation**: Added feat.py integration for REPL API surface tracking

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
