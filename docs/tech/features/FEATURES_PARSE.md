# RSB Parse (MODERN + SPEC_ALIGNED)

Updated: 2025-09-15

Purpose
- Provide string/stream “sed-like” transforms in one place.
- Offer a clean adapter for file-based sed helpers without coupling parse to FS.
- Keep macros thin; delegate heavy lifting to `streams` and FS utils per MODULE_SPEC.

Module Layout (SPEC)
- `src/parse/mod.rs` — Orchestrator and curated re-exports.
- `src/parse/macros.rs` — String/stream sed-like macros.
- `src/parse/sed_file.rs` — Adapter over FS for file-based sed helpers.
- `src/parse/utils.rs` — Reserved for future helpers.

Public API (curated)
- String/stream macros:
  - `sed_lines!(content, start, end) -> String`
  - `sed_around!(content, pattern, context) -> String`
  - `sed_insert!(content, sentinel, source) -> String` (errors if sentinel not unique)
  - `sed_template!(content, sentinel, source) -> String`
  - `sed_replace!(source, from, to)`
- File-based macros (adapter):
  - `sed_lines_file!(path, start, end) -> String`
  - `sed_around_file!(path, pattern, context) -> String`
  - `sed_insert_file!(path, content, sentinel)`
  - `sed_template_file!(path, content, sentinel)`
Note: wc-like counters (lines/words/chars for strings and files) are provided by the FS module.

Design Notes
- String/stream macros wrap `rsb::streams::Stream` methods to remain line-wise and memory-aware.
- File-based macros are defined under `parse::macros` and call `parse::sed_file::*`, which in turn wraps `fs::utils`.
- This follows the Cross‑Module Integration pattern: consumer module (parse) owns the adapter to avoid circular deps.
- Behavior is unchanged from legacy macros; only the owning module has changed.

Testing
- Covered by existing stream and FS tests.
- Run:
  - `cargo test`

Status
- MODERN: Yes — macros thin, logic in streams/fs.
- SPEC_ALIGNED: Yes — orchestrator-only `mod.rs`, module-owned macros, adapter in consumer.
