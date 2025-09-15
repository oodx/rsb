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

Case macros (value + var forms)
- Value: `snake!(s)`, `kebab!(s)`, `slug!(s)`, `dot!(s)`, `space!(s)`, `camel!(s)`
- Context var: `snake_var!("NAME")`, `kebab_var!`, `slug_var!`, `dot_var!`, `space_var!`, `camel_var!`

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

Logging policy
- Fail-fast path uses `stderrx("fatal", ...)` then exits(1).
- Example: `[string::prefix] Regex compilation failed for pattern: '['` then exit.

Shell helpers
- `string::utils::shell_single_quote(&str) -> String` — POSIX-safe single-quoting (wraps in single quotes and escapes embedded `'`). Useful for constructing shell commands safely.

Specifications
- See `docs/tech/development/MODULE_SPEC.md` for module structure and exposure conventions.
