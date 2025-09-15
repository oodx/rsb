# RSB Tests: Structure and Conventions

Goals
- Make tests self‑documenting and easy to run via `bin/test.sh`.
- Avoid editing `test.sh` every time by using predictable wrappers and folders.
- Support suites that matter to humans (sanity, uat) and to CI/AI (smoke, integration).

Directory Layout

tests/
- sanity/
  - baseline.rs            # Visible demos (user‑friendly outputs)
- features/
  - colors/
    - sanity.rs           # Functional coverage
    - runtime.rs          # Env toggles, backgrounds, glyph behavior
  - param/
    - param_test.rs       # Comprehensive param! behaviors
    - helpers.rs          # Helper layer tests
- uat/
  - colors.rs, colors_macros.rs, glyphs.rs, prompts.rs, visual.rs, param_uat.rs
- sh/
  - <name>.sh             # Shell‑based tests (executed directly)
- old/                    # Legacy tests kept for reference

Wrappers (Cargo integration tests)
- Create a top‑level wrapper file for each module/suite you want to run from `test.sh`:
  - `tests/<module>_<suite>.rs` (preferred) or named wrapper like `features_colors.rs`.
  - Inside the wrapper, include submodules using `#[path = "<folder>/<file>.rs"]`.
  - Examples:
    - `tests/features_colors.rs` includes `features/colors/{sanity.rs,runtime.rs}`
    - `tests/features_param.rs` includes `features/param/{param_test.rs,helpers.rs}`
    - `tests/uat_main.rs` includes all files under `tests/uat/`
    - `tests/sanity_main.rs` includes `sanity.rs` (core) and `sanity/baseline.rs`

Suites (per module)
- `sanity`: quick but meaningful functional checks, with visible output where helpful
- `smoke`: extremely fast checks (subset of sanity) for CI/AI fast lanes
- `uat`: demos of user experience (color/glyph/prompt/visual/param demos)
- `integration`: end‑to‑end or multi‑module flows

Naming Convention (recommended)
- Wrapper filename: `<module>_<suite>.rs`
- Folder: `tests/<module>/<suite>/*.rs` (optional if you want more granularity)
- `test.sh` auto‑discovers wrappers in `tests/*.rs` and runs them by wrapper base name.
- Example: add `tests/pronto_sanity.rs`, then run `./bin/test.sh run pronto_sanity`.

Feature Flags & Visuals
- Visual tests require `--features visuals`. `test.sh` passes the right features for UAT and color suites.
- Keep UAT and visuals out of default builds to avoid noisy output in CI unless explicitly requested.

IMPORTANT! all tests must implement a sanity test (check core assumptions), and a visual uat test (show the commands being called and show the outputs)

Shell‑Based Tests
- Place scripts under `tests/sh/`. They are runnable via `test.sh` by name:
  - `./bin/test.sh run ceremony_runner` → runs `tests/sh/ceremony_runner.sh`.
  - `./bin/test.sh run cli` → runs `tests/sh/cli_macros_e2e.sh` (CLI macros end‑to‑end)

How to add a new test suite (example: tokens module)
1) Create folder: `tests/tokens/sanity/*.rs` (optional: or just a single file).
2) Add wrapper: `tests/tokens_sanity.rs` with:
   ```rust
   #[path = "tokens/sanity/basic.rs"]
   mod tokens_sanity_basic;
   ```
3) Run: `./bin/test.sh list` (wrapper auto‑appears) then `./bin/test.sh run tokens_sanity`.

Notes
- Keep tests small and focused; baseline and UAT should demonstrate outputs clearly.
- Prefer `#[cfg(feature = "...")]` guards for feature‑gated areas to keep default profile clean.
- When reorganizing, wrappers let you change folder contents without updating `test.sh`.
 - For macro E2E coverage, see `tests/sh/cli_macros_e2e.sh` which drives `examples/cli_e2e.rs`.


# HOWTO: Run and Add Tests in RSB

Updated: 2025-09-15

Purpose
- Provide a quick, reliable way to run core and feature-gated tests.
- Explain the test layout and how to add new suites without editing scripts.

Test Layout
- Wrappers live at `tests/*.rs` and organize real tests under subfolders:
  - Sanity: `tests/sanity_main.rs` includes `tests/sanity/*.rs`
  - Features: `tests/features_<module>.rs` includes `tests/features/<module>/*.rs`
  - UAT: `tests/uat_main.rs` includes `tests/uat/*.rs`
  - Shell tests: `tests/sh/*.sh`
- Keep tests small and visible. Prefer helper functions in modules over heavy test logic.

Runner Usage (`bin/test.sh`)
- `./bin/test.sh list` — list mapped and auto-discovered wrappers
- `./bin/test.sh run smoke` — fast checks (core; skips visuals)
- `./bin/test.sh run all` — full checks (enables visuals where needed)
- Targeted runs (examples):
  - `./bin/test.sh run sanity`
  - `./bin/test.sh run param`
  - `./bin/test.sh run colors`
  - `./bin/test.sh run uat-colors`
  - `./bin/test.sh run uat-visual`
- Flags:
  - `--verbose` — pass `-- --nocapture` to cargo where applicable
  - `--comprehensive` — hint for broader runs (informational)
  
Timeouts and Non‑TTY
- The runner uses a best‑effort timeout wrapper when `timeout`/`gtimeout` is available.
  - Set `RSB_TEST_TIMEOUT=<seconds>` to override the default (600 seconds).
- Interactive prompts are TTY‑aware and tests use non‑TTY fallbacks.
  - In CI/non‑TTY, prompts return defaults immediately.
  - You can also force non‑interactive behavior in tests by setting the global
    context variable `opt_quiet` to `1`.

Cargo Equivalents
- Default core tests (no visuals):
  - `cargo test`
- With visuals umbrella (colors + glyphs + prompts):
  - `cargo test --features visuals`
- Individual wrappers:
  - `cargo test --test sanity_main`
  - `cargo test --features visuals --test features_colors`
  - `cargo test --features visuals --test uat_main -- --nocapture`

Visual Tests and Env
- Visual suites require feature flags. The runner enables them; with cargo, add `--features visuals`.
- Useful env vars (set before running):
  - `RSB_COLOR=always|auto|never` — color policy
  - `RSB_COLORS=simple,status,named[,bg]` — enable color sets and optional backgrounds
- Glyphs and prompts are part of `visuals`. Use the umbrella unless optimizing footprint.

Adding New Suites
- Create a wrapper: `tests/<module>_<suite>.rs` (e.g., `tests/features_string.rs`).
- Place tests in `tests/<module>/<suite>/*.rs` (e.g., `tests/features/string/*.rs`).
- The runner will auto-discover wrappers; no changes to `bin/test.sh` needed.

Conventions
- Always add a minimal sanity test for each module.
- For visual components, add a UAT that prints sample output; gate via features.
- Keep module docs in `docs/tech/features/FEATURES_<NAME>.md` in sync with tests.
