# RSB Options (FEATURES_OPTIONS)

Updated: 2025-09-12

Purpose
- Document the `options!` macro behavior and the stdopts feature.
- Set clear expectations: simple, declarative, string‑first options without “smart” consumption.

Design Principles
- Declarative parsing: treat tokens that look like options as options; no hidden cursor moves.
- Explicit value binding: use `--long=value` for values; short flags are boolean.
- String‑first: parsed flags and values are written into the global context as `opt_*` keys.
- Minimalism: favor predictable behavior over CLI bells/whistles.
- MODULE_SPEC compliance: `options!` macro is thin DSL that delegates to `options()` function.

Supported Patterns
- Long boolean: `--quiet` → `opt_quiet = "true"`
- Long with value: `--config=path.conf` → `opt_config = "path.conf"`
  - Path/File heuristic: if option name contains "path" or "file", the given value must exist on disk or the program exits with error.
- Short boolean (single): `-q` → `opt_q = "true"`
- Negated long boolean: `--not-quiet` → sets `opt_quiet = "false"` (same key; no extra vars)
- Stdopts mapping (feature‑gated): when compiled with Cargo feature `stdopts`
  - `-d`→`opt_debug`, `-q`→`opt_quiet`, `-t`→`opt_trace`, `-D`→`opt_dev_mode`, `-y`→`opt_yes`, `-s`→`opt_safe` (all set to "true").
- Comma lists in values: `--features=a,b,c` keeps value intact as `opt_features = "a,b,c"` (caller may split as needed).
- Multi-short flags in a single option value (generic):
  - Comma-delimited: `--multi=d,q,t,s` → sets `opt_d,opt_q,opt_t,opt_s` to `true`.
  - Inline (no commas): `--multi=dq!ts` → sets `opt_d,opt_q=true`; `!` toggles negation so `opt_t,opt_s="false"`.
  - Negation: prefix `!` toggles negation mode; negated letters set their own key to `"false"`.
  - Note: `--multi` is generic and does not apply stdopts descriptive mappings; controllers decide how to interpret letters.

Not Supported (by design)
- Combined short flags: `-dq` (use `-d -q`).
- Space‑bound long values: `--long value` is not consumed as a value by `options!`.

Negation Semantics
- `--not-<name>` always wins over earlier `--<name>` occurrences.
- Implementation sets `opt_<name> = "false"` on the same key (no extra variables).

Token Streams (when you want `--long <token_stream>`)
- RSB provides `global::is_token_stream(&str)` to validate token streams like `k1=v1,k2=v2` or `k1=v1;k2=v2`.
- Pattern: pass a token stream as the next positional arg and handle it explicitly in your command logic.
  - Example usage sketch:
    ```rust
    let args = bootstrap!();
    options!(&args); // sets opt_features=true if `--features` present
    // If the next positional looks like a token stream, process it
    if let Some(next) = args.get(1) { // example index; adjust to your layout
        if rsb::prelude::is_token_stream(next) {
            set_var("opt_features", next); // store the full token stream
        }
    }
    ```
- Alternatively, prefer `--long=value` with comma lists for simpler flows: `--features=a,b,c`.

Global Context Keys
- Boolean/flags: `opt_<name>` (dashes normalized to underscores).
- Values: `opt_<name>` stores the exact right‑hand side of `--name=value`.
- Short flags always get `opt_<char>` and may map to descriptive names via `stdopts`.

Examples
- Minimal lifecycle pattern:
  ```rust
  use rsb::prelude::*;

  fn main() {
      let args = bootstrap!();
      options!(&args);
      if has_var("opt_quiet") { /* ... */ }
      let config = get_var("opt_config"); // from --config=...
      // Handle comma lists
      for f in get_var("opt_features").split(',').filter(|s| !s.is_empty()) {
          // enable feature f
      }
  }
  ```

- Stdopts demo (feature‑gated):
  ```bash
  cargo test --features stdopts --test stdopts
  ```

Guidance
- Prefer `--long=value` for values; choose comma lists for multi‑values.
- Use short flags for booleans; enable `stdopts` to get conventional names in context.
- If you truly want `--long <token_stream>`, treat the next positional token as data and validate with `is_token_stream()` explicitly in your handler.

Trade‑offs (explicit, intentional)
- No auto‑consumption of following tokens keeps parsing predictable and testable.
- No `-dq` bundling avoids ambiguity and hidden state; write flags explicitly.
- Path/file existence check is a convenience guard; broaden or disable in your own code if needed.

Related
- `src/cli/macros.rs` → `options!` macro (thin DSL)
- `src/cli/options.rs` → `options()` function implementation (business logic)
- `src/global/mod.rs` → `is_token_stream()` helper, global context operations
- Tests: `tests/stdopts.rs`, `tests/uat_stdopts.rs`, `tests/options.rs`
