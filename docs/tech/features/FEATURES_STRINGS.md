# String Utilities (FEATURES_STRINGS)

Updated: 2025-09-12

Scope
- Centralize general-purpose string helpers and macros used across RSB.
- Provide predictable, Unicode-safe behavior (at least on scalar boundaries),
  with optional guidance for grapheme-cluster correctness.

Modules
- `rsb::string` (module):
  - `str_sub(&str, offset, Option<len>) -> String` — substring by Unicode scalar index; safe from UTF-8 boundary splits.
  - `str_prefix(&str, pattern, longest: bool) -> String` — remove matching prefix; supports `*` and `?` wildcards.
  - `str_suffix(&str, pattern, longest: bool) -> String` — remove matching suffix; supports `*` and `?` wildcards.
  - `str_replace(&str, pattern, replacement, all: bool) -> String` — first or all occurrences.
  - `str_upper(&str, all: bool) -> String` — upper-case first or all.
  - `str_lower(&str, all: bool) -> String` — lower-case first or all.
  - `str_case_first_match(&str, pattern, to_upper: bool)` — toggle case on the first glob-matched token.
  - `str_matches(var, pattern)` / `str_equals(var, candidate)` — convenience guards often used with `validate!`.
  - `is_name(&str)` — quick check for ASCII identifier-ish names (letters, digits, `_`, `-`).

Case conversions (string::case)
- Helpers (line-sized by design, 64 KiB limit per input):
  - `to_snake_case(&str) -> String`
  - `to_kebab_case(&str) -> String` (alias for `slug` semantics)
  - `to_dot_case(&str) -> String`
  - `to_space_case(&str) -> String`
  - `to_camel_case(&str) -> String`
  - ASCII-SAFE (default): these helpers normalize to ASCII-only output by stripping non-ASCII and treating them as separators
  - UNICODE-SAFE: parsing uses Unicode scalars; output normalization targets ASCII use cases
- Tokenization rules:
  - Split on delimiters: space, `_`, `-`, `.`, `/`.
  - Split at lower→Upper boundaries: `userName` → `user` | `Name`.
  - Acronym break before final upper when next is lower: `HTTPSever` → `HTTP` | `Sever`.
  - Split between letters and digits: `v2Build` → `v` | `2` | `Build`.
- Line limit behavior:
  - Inputs > 64 KiB: logged once via `StringError::CaseInputTooLarge` and returned unchanged. For large content, use line-wise streams.
- Lower-level utilities:
  - `split_words(&str)` — canonical tokeniser used by the case helpers.
  - `to_lower(&str)` / `to_upper(&str)` — ASCII-safe case normalisers used internally and available for direct use.

Case macros (value + var forms)
- Value: `snake!(s)`, `kebab!(s)`, `slug!(s)`, `dot!(s)`, `space!(s)`, `camel!(s)`
- Context var: `snake_var!("NAME")`, `kebab_var!`, `slug_var!`, `dot_var!`, `space_var!`, `camel_var!`
- Numeric helper: `to_number!(expr, default: ..?)` — parse integers directly in macro form (0 or provided default on failure).

Streams (per-line transforms)
- `Stream` adds: `.snake()`, `.kebab()`, `.slug()`, `.dot()`, `.space()`, `.camel()`, `.lower()`, `.upper()`
- Example:
  ```rust
  use rsb::prelude::*;
  Stream::from_file("names.txt").snake().to_file("names_snake.txt");
  ```

ASCII Filtering (utilities)
- `string::utils::filter_ascii_strip(&str)` — removes non-ASCII characters
- `string::utils::filter_ascii_sanitize(&str, marker)` — replaces non-ASCII with `marker` (default `#INV#`)
- `string::utils::ascii_safe(&str)` / `unicode_safe(&str)` — explicit toggles when you need to couple case helpers with broader parsing contexts.
- Example:
  ```rust
  use rsb::string::utils::{filter_ascii_strip, filter_ascii_sanitize_default};
  assert_eq!(filter_ascii_strip("Hello🌍World"), "HelloWorld");
  assert_eq!(filter_ascii_sanitize_default("Crème brûlée"), "Cr#INV#me br#INV#l#INV#e");
  ```

Related
- asc100 (adjacent toolkit): ../asc100/README.md
  - Invalid Character Handling Strategies (Strict/Strip/Sanitize)
  - Extension Markers (#SSX#, #ESX#, #EOF#, #NL#, and #INV#)
  - Charset variants (STANDARD/NUMBERS/LOWERCASE/URL)
  - Consider asc100 for advanced pipelines or optional interop

Macros (exported at crate root; re-exported via prelude)
- `str_in!(needle, in: haystack)` — substring containment.
- `str_explode!(string, on: delim, into: "ARR")` — splits into global-context array keys.
- `str_trim!("VAR")` — trims value fetched from context.
- `str_len!("VAR")` — length of value fetched from context (bytes count of resulting `String`).
- `str_line!(ch, n)` — string of `n` repeated characters.

Unicode behavior
- Scalar-safety: `str_sub` iterates with `chars()`, so it won’t split inside a code point.
- Prefix/Suffix safety: uses indices at char boundaries to avoid panics; wildcard matching is regex-based.
- Grapheme clusters: a “visual character” can be multiple scalars (e.g., emoji sequences, combining marks).
  - Current functions operate on Unicode scalars, not grapheme clusters. This is acceptable for most usages but may split grapheme clusters.
  - If grapheme-accurate operations are needed, consider adding an optional `string-graphemes` feature using `unicode-segmentation` and document the trade-offs.

Case mapping notes
- Uses Rust’s standard Unicode case conversions. Edge cases (e.g., Turkish dotted/dotless I, `ß` uppercasing) follow standard library semantics.

Testing
- Suite: `tests/features_string.rs` → `tests/features/string/string_test.rs`.
- Coverage includes:
  - ASCII and Unicode substrings
  - Literal and wildcard prefix/suffix removal
  - Replace first/all
  - Case transforms: helpers, macros, param!(case: ...), and stream per-line transforms
  - Add edge cases as needed (combining marks, emoji sequences) to document behavior; ensure no panics at char boundaries.

Migration notes
- Helpers were previously under `utils` and partially duplicated in `param::basic`.
- Now centralized in `rsb::string`; `param::basic` delegates to these helpers.
- Keep `str_*` prefixes to make call sites easy to locate via grep.

Errors
- `rsb::string::error::StringError` centralizes messaging across helpers.
  - Fail-fast (RS policy): default helpers log a fatal message and exit with status 1. No panics; immediate process exit.
  - `try_*` variants return `Result<String, StringError>` without exiting (for tests/diagnostics).
  - Common errors:
    - `SizeLimitExceeded { limit, length }` — case helpers guard at 64 KiB.
    - `RegexCompile { pattern }` — invalid glob→regex pattern (prefix/suffix/case-first-match).
    - `IndexOutOfBounds { index, len }` — substring guards in `try_*` forms.
- `rsb::string::error::log_string_error(op, &err)` — shared logging helper invoked by the fail-fast paths before exiting.

Try variants
- Patterns:
  - `try_str_prefix(&str, pattern, longest) -> Result<String, StringError>`
  - `try_str_suffix(&str, pattern, longest) -> Result<String, StringError>`
  - `try_str_case_first_match(&str, pattern, to_upper) -> Result<String, StringError>`
- Substrings:
  - `try_str_sub_abs(&str, offset, Option<len>) -> Result<String, StringError>`
  - `try_str_sub_rel(&str, start:isize, Option<len:isize>) -> Result<String, StringError>`
- Case conversions:
  - `try_to_snake_case`, `try_to_kebab_case`, `try_to_dot_case`, `try_to_space_case`, `try_to_camel_case`

Testing hints
- Guard helpers (`guard_size`, `guard_index`) are exported for advanced callers but typically only needed when you are building custom operations on top of the primitives.

Logging policy
- Fail-fast path uses `stderrx("fatal", ...)` then exits(1).
- Example: `[string::prefix] Regex compilation failed for pattern: '['` then exit.

Shell helpers
- `string::utils::shell_single_quote(&str) -> String` — POSIX-safe single-quoting (wraps in single quotes and escapes embedded `'`). Useful for constructing shell commands safely.

Specifications
- See `docs/tech/development/MODULE_SPEC.md` for module structure and exposure conventions.

<!-- feat:strings -->

_Generated by bin/feat.py --update-doc._

* `src/string/case.rs`
  - fn split_words (line 39)
  - fn to_lower (line 113)
  - fn to_upper (line 116)
  - fn to_snake_case (line 121)
  - fn to_kebab_case (line 134)
  - fn to_dot_case (line 147)
  - fn to_space_case (line 160)
  - fn to_camel_case (line 173)
  - fn try_to_snake_case (line 197)
  - fn try_to_kebab_case (line 204)
  - fn try_to_dot_case (line 211)
  - fn try_to_space_case (line 218)
  - fn try_to_camel_case (line 225)

* `src/string/error.rs`
  - enum StringError (line 6)
  - fn log_string_error (line 61)

* `src/string/guard.rs`
  - fn guard_size (line 6)
  - fn guard_index (line 17)

* `src/string/helpers.rs`
  - fn str_sub (line 5)
  - fn try_str_sub_abs (line 13)
  - fn try_str_sub_rel (line 32)
  - fn try_str_prefix (line 55)
  - fn try_str_suffix (line 94)
  - fn try_str_case_first_match (line 136)
  - fn str_prefix (line 174)
  - fn str_suffix (line 183)
  - fn str_replace (line 191)
  - fn str_upper (line 200)
  - fn str_lower (line 213)
  - fn str_case_first_match (line 227)
  - fn is_name (line 237)
  - fn str_equals (line 244)
  - fn str_matches (line 248)

* `src/string/macros.rs`
  - macro to_number! (line 6)
  - macro str_in! (line 16)
  - macro str_explode! (line 23)
  - macro str_trim! (line 31)
  - macro str_len! (line 38)
  - macro str_line! (line 45)
  - macro snake! (line 54)
  - macro kebab! (line 60)
  - macro slug! (line 66)
  - macro dot! (line 72)
  - macro space! (line 78)
  - macro camel! (line 84)
  - macro snake_var! (line 92)
  - macro kebab_var! (line 98)
  - macro slug_var! (line 104)
  - macro dot_var! (line 110)
  - macro space_var! (line 116)
  - macro camel_var! (line 122)

* `src/string/mod.rs`
  - pub use case::* (line 6)
  - pub use utils::* (line 17)

* `src/string/utils.rs`
  - pub use super::case::* (line 10)
  - pub use super::error::* (line 11)
  - pub use super::helpers::* (line 12)
  - fn ascii_safe (line 17)
  - fn unicode_safe (line 27)
  - fn filter_ascii_strip (line 42)
  - fn filter_ascii_sanitize (line 47)
  - fn filter_ascii_sanitize_default (line 62)
  - fn shell_single_quote (line 72)

<!-- /feat:strings -->

