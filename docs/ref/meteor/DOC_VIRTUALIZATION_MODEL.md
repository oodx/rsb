# Document & Script Virtualization Model

This note captures how MeteorEngine can host large documents, scripts, and code libraries in-memory while still materialising to and from a conventional filesystem layout. The goal is to make "doc/kb/shell" style contexts first-class, so teams and agents share canonical content without juggling per-repo copies.

## Core Ideas

1. **Context Isolation** – dedicate contexts to the content class (`Context::doc()`, `Context::kb()`, `Context::shell()`, future `Context::python()`/`Context::rust()`), keeping each corpus independent.
2. **Namespace as Folder** – treat namespaces as directory paths (`guides.install`, `scripts.setup.env`, `lib.utils.hashing`).
3. **Bracket Keys as Files** – use bracket notation to represent file-like entries (`sections[intro]`, `parts[env_check]`, `snippets[derive_display]`). Internally this flattens to canonical keys but the tree index preserves the structure.
4. **Canonical Blobs + Segments** – retain a "full" value (`full`, `raw`, `packed`) for cheap round-trips, and mirror it with structured slices for ergonomic editing (`sections[...]`, `parts[...]`, `snippets[...]`).

## Addressing Patterns

| Context | Namespace Example | Keys | Description |
|---------|-------------------|------|-------------|
| `doc`   | `guides.install`  | `full`, `sections[intro]`, `sections[10_setup]`, `sections[troubleshooting.assets]` | Markdown broken into sections; ordering enforced via key names (numeric prefix or metadata). |
| `kb`    | `patterns.git_hooks` | `summary`, `snippets[pre_commit]`, `snippets[enforcement.rules]` | Knowledge-base entries with reusable fragments. |
| `shell` | `setup.env`       | `full`, `parts[env_check]`, `parts[install_pkg]`, `metadata[permissions]` | Shell scripts with modular blocks plus metadata hints. |
| `python` (future) | `lib.utils` | `modules[hashing.py]`, `snippets[derive_display]` | Reusable modules/snippets for agents or tooling. |

## Export / Import Workflow

1. **Export Namespace → Filesystem**
   - Resolve `(context, namespace)` to a tree of entries.
   - Create directories mirroring namespace segments.
   - Map `sections[intro]` → `intro.md`, `parts[install_pkg]` → `20-install_pkg.sh` (ordering via prefix or metadata).
   - Write the canonical `full` value to `_full.md` (or `_full.sh`) when round-trip fidelity is needed.
   - Persist metadata keys (e.g., `metadata[permissions] = "755"`) into sidecar files (`.meta`), or as extended attributes.

2. **Import Filesystem → Namespace**
   - Walk a root directory, infer namespace from folder structure.
   - Generate bracket keys from filenames (`intro.md` → `sections[intro]`, numeric prefixes preserved).
   - Update `full` by concatenating ordered sections (or leave for later assembly).
   - Capture file metadata (permissions, modified time, language) into `metadata[...]` keys.

3. **Round-trip Confirmation**
   - Provide hashing or checksums to ensure exported files match engine content.
   - Optionally inject commit IDs or version tags into `metadata[version]` for downstream syncing.

## Ordering & Metadata

- Adopt numeric prefixes or explicit `metadata[order.<slot>]` tokens to keep deterministic section ordering.
- Store MIME or language hints (`metadata[mime] = "text/markdown"`, `metadata[lang] = "bash"`).
- Track authorship or provenance for audit (`metadata[last_editor]`).

## Engine Feature Requirements

To support this model ergonomically, MeteorEngine should add:

- **Namespace Views:** `engine.namespace_view(context, namespace)` returning structured slices (sections/parts/snippets + metadata + canonical `full`).
- **Ordered Iteration:** guarantee stable iteration of bracket keys (respect numeric prefixes; expose comparator hook).
- **Import / Export Helpers:**
  - `engine.export_namespace(path, context, namespace, ExportOptions)`
  - `engine.import_namespace(path, context, namespace, ImportOptions)`
- **Cursor Guards & Transactions:** allow temporary cursor overrides while importing/exporting.
- **Metadata Helpers:** `engine.set_metadata(context, namespace, key, value)` to isolate metadata management.
- **Integrity Checks:** support checksums or manifests (`metadata[sha256.full]`).

## CLI / REPL Extensions

- `meteor export --context doc --namespace guides.install --dest ./docs/install`.
- `meteor import --context shell --namespace setup.env --src ./scripts/setup`.
- `meteor doc` subcommands: `list`, `show`, `sections`, `diff`.
- REPL commands mirroring export/import plus `meteor meteor <ctx> <ns>` to materialise aggregated view.

## Future Directions

- **Language-specific Contexts:** treat `python`, `rust`, `bash` contexts with tailored validators (lint hooks before import/export).
- **Live Sync:** watch filesystem changes and push updates back into engine (optional watch mode for docs editing).
- **Version Snapshots:** extend command history to tag exports/imports, enabling rollback or multi-version docs.
- **Agent Integration:** expose gRPC/HTTP endpoints so automated tools fetch the latest canonical snippets without direct filesystem access.

## References

- Link back to `ENGINE_ENHANCEMENT.md` for API roadmap.
- Align with `docs/ref/guides/TOKEN_NAMESPACE_CONCEPT.md` (namespacing rules) and the hybrid storage architecture docs.
