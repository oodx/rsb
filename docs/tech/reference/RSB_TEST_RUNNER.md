# RSB Test Runner (BashFX Legendary Profile)

**Status**: Draft
**Profile**: `rsb`
**Scope**: Documents how the `fx-testsh` legendary runner maps to the legacy RSB `bin/test.sh` surface.

## Lane Surface

| Command                | Runner Target                      | Notes                                                   |
|------------------------|------------------------------------|---------------------------------------------------------|
| `run sanity`           | `tests/sanity.rs`                  | Core contract aggregator (wraps `tests/sanity/*.rs`)    |
| `run smoke`            | `tests/smoke.rs`                   | CI-friendly subset                                      |
| `run unit`             | `tests/unit.rs`                    | Fast module-focused coverage                            |
| `run integration`      | `tests/integration.rs`             | Cross-module & host adapters                            |
| `run uat`              | `tests/uat.rs`                     | Visual ceremonies; pipes through Boxy when available    |
| `run regression`       | `tests/regression.rs`              | Long-running historical suites                          |
| `run param`            | Alias to sanity lane               | Use `run sanity param` for the parameter module slice   |
| `run cli`              | `tests/sh/cli_macros_e2e.sh`       | Shell-driven CLI macros UAT                            |
| `run adhoc <name>`     | `tests/_adhoc/<name>.rs|.sh`       | Experimental lanes                                      |
| `run all`              | Composite of key RSB lanes         | Chains sanity + visual specialization helpers           |
| `docs org`             | `docs/tech/development/TEST_ORGANIZATION.md` | Enforced structure & lint policy            |
| `docs modules`         | `docs/tech/development/MODULE_SPEC.md`        | Module naming and layout guidance           |
| `docs prelude`         | `docs/tech/development/PRELUDE_POLICY.md`     | Prelude guardrails                           |
| `docs runner`          | `docs/tech/reference/RSB_TEST_RUNNER.md`      | This reference document                       |
| `list`                 | Aggregates wrappers + dynamic lanes | Includes generated aliases & discovery map             |

_For the full lane matrix see `TESTSH_TEST_MAP` in `bin/test.sh` when `--profile=rsb` is active._

## Ceremony & Boxy

- `_run_header` now emits a Boxy banner with lane, profile, mode, strict/override state.
- Successful runs close with a `🏁 Run Summary` banner; failures flip the theme to `error`, overrides to `warning`.
- `_emit_override_warning` surfaces emergency mode guidance before violating lanes execute.
- Boxy remains optional—stderr fallback preserves legacy behaviour when the binary is unavailable.

## Enforcement Hooks

- `_org_validate` is the extracted `validate_test_structure` legacy logic with BashFX naming.
- `--override`, `--violations`, and `--skip-enforcement` map 1:1 with the original runner flags.

## Profiles & Environment

- Activate with `test.sh --profile=rsb <command>` or the shortcut `--rsb`.
- Exported env vars retain the historical names (`RSB_TEST_MODE`, `RSB_VERBOSE`, etc.) so downstream Rust tests continue to read the same signals.
- The companion integration playbook lives at `docs/tech/reference/RSB_TESTSH_INTEGRATION.md`.

## TODO

- Document module-filter behaviour (`run <lane> <module>`) after snapshots land.
- Capture the `parts/` build workflow for teams adopting the BashFX assembly pattern.
