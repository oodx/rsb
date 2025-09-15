# RSB Summary (2025-09-12)

A compact snapshot of current state, policies, priorities, and next actions. Use this as a quick re-hydration guide.

## Current State
- Strings/Param
  - Centralized helpers in `rsb::string`: safe substrings, glob-aware prefix/suffix, replace, upper/lower.
  - Case helpers/macros: snake/kebab/slug/dot/space/camel; `param!(..., case: ...)` routes here; per-line Stream transforms exist.
  - `param!`: negative substring via `sub_rel`; `${VAR:?msg}` hard-exits; patterned first-match case transforms (${VAR^pat}/${VAR,pat}).
- ASCII Policy
  - Case helpers: ASCII-SAFE output (normalize to ASCII; non-ASCII treated as separators/stripped). Scalar-safe, not grapheme-aware (future flag optional).
  - ASCII filters: `filter_ascii_strip`, `filter_ascii_sanitize(marker)`, default marker `#INV#` (asc100-aligned).
- Errors
  - `string::error::StringError` centralizes messages (case input size guard, regex compile, invalid pattern); helpers log via `stderr!`.
- Dev/EZ Surfaces
  - `string::utils`, `param::utils` expose curated low-level helpers.
  - `rsb::dev` aggregates curated helpers; `rsb::prelude_ez` = prelude + curated helpers + case macros for prototyping.

## Policies
- Prelude: core-only; optional systems (visuals/log macros) remain explicit opt-ins.
- Param routing: keep macros thin; `param!` delegates to helper modules.
- expand_vars vs param!
  - `expand_vars(...)` = simple substitution only.
  - Use `param!` for defaults/alt/require, substring/length, replace, glob prefix/suffix, case transforms (incl. patterned first-match).

## Module Surfaces (user-facing)
- `string::utils` — helpers/case/error + ASCII filters; safety registry lists ASCII-SAFE vs UNICODE-SAFE.
- `param::utils` — curated param helpers (get/sub_rel/prefix/suffix/replace/upper/lower/len).
- `rsb::dev` — dev/test convenience: `rsb::dev::{string,param}`.
- `rsb::prelude_ez` — standard prelude + curated low-level helpers/macros (for rapid iteration).

## Priorities
- P0
  - RSB‑STREAM‑020: Stream module reorg plan (unblocks per-line filters/ops naming and additions).
- P1
  - Host/global/CLI ergonomics: options/stdopts polish; lifecycle examples/tests (bootstrap → pre_dispatch → options → dispatch).
  - Param docs: full reference + bash compatibility matrix (ISSUE‑009/008).
  - ASCII policy docs: finalize notes; optional asc100 interop evaluation captured in backlog.

## Blockers
- Stream per-line ASCII filters (RSB‑STREAM‑011) and broader streamable API sanity (RSB‑008) are deferred pending the stream reorg plan.

## Tests
- `cargo test` (default) passes; visual suites gated behind features. Minor warnings only.

## Audit Targets (expand_vars usage)
Review these for unintended bash-like semantics; prefer `param!` when needed:
- `src/macros/stderr.rs`: `echo!`, `printf!` — OK for display, confirm no defaulting expectations.
- Paths/files: `src/fs.rs`, `src/streams.rs`, `src/os.rs`, `src/args.rs`, `src/utils.rs` prompts.
- Bootstrap/context: `src/context.rs` XDG/home derivations (intentional substitution).
- Quick grep command: `rg "\bexpand_vars\(" -n`.

## Next Suggested Actions (focused)
1) Draft RSB‑STREAM‑020 (RFC): target structure, naming, method groups; mapping from current → future; no code moves.
2) Options/stdopts polish + lifecycle exemplars: finalize short-flag expansion; add concise example tests (non‑TTY/CI friendly).
3) expand_vars audit + doc note: spot-fix call sites; add 2–3 policy examples to README/FEATURES_PARAMS.

