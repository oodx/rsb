# RSB File System (MODERN + SPEC_ALIGNED)

Updated: 2025-09-15

Purpose
- Provide ergonomic, dependency-light file system helpers and macros for scripts and CLIs.
- Cover common operations: file I/O, directories, archives, path utilities, metadata extraction, and temp files.
- Keep implementation in `utils` and thin macros in `macros` per MODULE_SPEC.

Module Layout (SPEC)
- `src/fs/mod.rs` — Orchestrator only; re-exports curated surface and macros.
- `src/fs/utils.rs` — Implementation of fs helpers (read/write/mkdir/etc.).
- `src/fs/macros.rs` — Module-owned macros (thin wrappers over utils/streams/os).

Public API (curated)
- File I/O: `read_file`, `write_file`, `append_file`
- Directories: `mkdir_p`, `rm`, `rm_rf`, `cp`, `cp_r`, `mv`, `touch`
- Metadata: `extract_meta_from_file`, `backup_file`, `chmod`
- Path utils: `path_canon`, `path_split`, `parse_meta_keys`
- Predicates: `is_file`, `is_dir`, `is_entity`, `is_link`, `is_readable`, `is_writable`, `is_executable`, `is_nonempty_file`
- Dictionaries: `load_dict_from_file`
- Temp files: `create_temp_file_path`, `capture_stream_to_temp_file`, `cleanup_temp_files`
- File-based sed: `sed_lines_file`, `sed_around_file`, `sed_insert_file`, `sed_template_file`
- Counters (wc-like):
  - strings: `wc!(content) -> "lines words chars"`, `wc_lines!`, `wc_words!`, `wc_chars!`
  - files: `wc_file!(path)`, `wc_lines_file!`, `wc_words_file!`, `wc_chars_file!`

Macros (curated)
- Files/temp: `chmod!`, `backup!`, `tmp!`, `cap_stream!`, `subst!`
- Sed (string-based): `sed_lines!`, `sed_around!`, `sed_insert!`, `sed_template!`, `sed_replace!`
- Sed (file-based): `sed_lines_file!`, `sed_around_file!`, `sed_insert_file!`, `sed_template_file!`
- Archives: `tar!`, `tar_gz!`, `zip!`, `pack!`, `unpack!`
- Path/meta: `path_canon!`, `path_split!`, `meta_keys!`

Usage Examples
```rust
use rsb::fs::*;

// Write then read
write_file("/tmp/demo.txt", "hello\nworld");
let s = read_file("/tmp/demo.txt");

// Make a temp file and capture a stream into it
use rsb::streams::Stream;
let mut st = Stream::from_string("a\nb\nc");
let tmp = cap_stream!(st);

// Sed helpers (string-based)
let lines = sed_lines!("1\n2\n3\n4\n5", 2, 4); // "2\n3\n4"

// Archives
pack!("/tmp/archive.tar.gz", "/tmp/demo.txt");
unpack!("/tmp/archive.tar.gz", to: "/tmp/out");

// Path utilities
let parts = path_split("/home/me/file.txt");
```

Design Notes
- mod.rs contains only orchestration (no implementation) per MODULE_SPEC.
- Macros are thin and delegate to utils/streams/os; they are exported at crate root for compatibility.
- Error handling uses best-effort messages and `std::process::exit` where appropriate for scripting ergonomics.
- Temp files are tracked for cleanup via an internal registry; call `cleanup_temp_files()` on exit if needed.

Testing (HOWTO_TEST)
- No feature flags required. Covered by default test suite:
  - `cargo test`
  - Runner: `./bin/test.sh run smoke` (includes fs paths through sanity tests)

Status
- MODERN: Yes — utils in one place, macros are thin, curated surface.
- SPEC_ALIGNED: Yes — orchestrator-only `mod.rs`, module-owned `macros.rs`, documented and tested.
