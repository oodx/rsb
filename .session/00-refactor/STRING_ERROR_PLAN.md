# STRING_ERROR_PLAN

Purpose
- Centralize string error handling in RSB to ensure consistent behavior for pattern/regex operations, substring bounds, and size guards, without sacrificing string‑first ergonomics.

Goals
- Standardize how string helpers report issues (invalid pattern, regex compile, size/index guards).
- Keep default public APIs ergonomic (return String), avoiding panics.
- Provide explicit try_* variants that return Result for callers that need error detail.

Design
- Error type: `string::errors::StringError`
  - InvalidPattern { pattern: String, reason: &'static str }
  - RegexCompile { pattern: String }
  - RegexReplace { pattern: String }
  - SizeLimitExceeded { limit: usize, length: usize }
  - IndexOutOfBounds { index: isize, len: usize }
  - Utf8
  - Other(&'static str)

- Guard + logging helpers
  - `string::guard::guard_size(s: &str, limit: usize) -> Result<(), StringError>`
  - `string::guard::guard_index(len: usize, idx: isize) -> Result<usize, StringError>` (normalizes negative indices; bounds check)
  - `string::errors::log_string_error(op: &str, err: &StringError)`
    - Emits one standardized message via `stderr!` (gated by QUIET/DEBUG/TRACE per RSB rules).
    - Example: `[string::replace] invalid pattern '[' (regex compile) — returning input unchanged`

- API shape
  - Public helpers remain String‑returning. Internals operate as `Result<String, StringError>`.
  - On Err: log via `log_string_error` and return the original input unchanged.
  - Add parallel `try_*` variants that return `Result<String, StringError>`.

Scope (first pass)
- String helpers that use patterns/regex
  - Wildcard prefix/suffix conversion paths used by `param!` (glob → regex).
  - Regex replace and related functions in `string`.
- Guards
  - Case transforms (snake/kebab/slug/dot/space/camel/lower/upper): wire to `guard_size` (currently a 64 KiB guard exists; standardize message).
  - Substring helpers and `param!` sub/sub_rel: normalize via `guard_index` and errors on OOB.

Behavior policy
- Default helpers (non‑try):
  - Do not error out; return input unchanged on failure; emit one standardized log line.
  - Respect `QUIET_MODE` (no logs). Show detail when `DEBUG_MODE`/`TRACE_MODE`/`opt_debug` is set.
- Try variants:
  - Return `Result` with `StringError` for explicit caller handling (no internal logging unless caller requests).

Tests
- Invalid regex compile → `try_replace` returns Err; `replace` logs + returns input.
- Size guard tripped on large input for case helper → unchanged input + standardized log; try_ variant returns Err.
- Index OOB in substring → unchanged input + log; try_ variant returns Err.

File layout
- `src/string/errors.rs` — `StringError` + `log_string_error`.
- `src/string/guard.rs` — size/index guards.
- `src/string/helpers.rs` (and related): adopt guards; add `try_*` variants.
- `src/param/...` — propagate pattern compile errors as `StringError` to string layer handling.

Docs
- Add “Error Handling: StringError” to `FEATURES_STRINGS.md`:
  - Explain default behavior (non‑try returns input unchanged + log) and try_* variants.
  - List common error cases with short examples.

Acceptance
- New modules compile; string + param tests extended to cover failure cases.
- Default ergonomics preserved; callers see consistent logs and unchanged outputs when not using try_*.

Pointers
- Code: `src/string/*`, `src/param/*`
- Related docs: `FEATURES_STRINGS.md`, `FEATURES_PARAMS.md`, `docs/development/MODULE_SPECIFICATION.md`

