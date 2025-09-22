# `bin/feat.py` Quick Reference

This repository includes `bin/feat.py`, a lightweight surface-inspection tool that helps keep feature documentation aligned with the code.

## What it does

- Walks the module tree for a given feature (e.g. `src/global`, `src/string`).
- Collects exported items (`pub fn`, `struct`, `enum`, `type`, `pub use`) and `#[macro_export]` macros.
- Prints a grouped report to stdout, or updates the corresponding feature doc inside the `<!-- feat:* -->` sentinel block (`--update-doc`).

The output serves as an authoritative checklist when editing `docs/tech/features/FEATURES_*.md` files.

## Usage

> The script currently ships without the executable bit set in most checkouts.
> Use `python3 bin/feat.py …` (or `python bin/feat.py …` if `python` maps to
> Python 3) unless you deliberately flip the permissions.

```bash
# List known features → source paths
python3 bin/feat.py --list

# Inspect a feature (stdout only)
python3 bin/feat.py global

# Refresh sentinel section in FEATURE file (auto-detects path)
python3 bin/feat.py colors --update-doc

# Provide explicit paths (useful for experiments)
python3 bin/feat.py --paths src/custom/mod.rs
```

## Sentinel Workflow

Feature docs include markers such as:

```markdown
<!-- feat:global -->
... auto-generated content ...
<!-- /feat:global -->
```

Running `--update-doc` replaces (or appends, if missing) the block with the latest inventory. The surrounding prose must describe each listed item to avoid drift.

## Extending the tool

- Update `FEATURE_MAP` in `bin/feat.py` when new feature directories are added.
- For multi-file modules, include every relevant path so the report covers helper modules, adapters, and macro companions.
- The script uses simple regex parsing—adjust or extend the regexes if new item kinds need to be tracked.

### When modules move (e.g. `visual::colors` → `rsb::colors`)

1. Update the `FEATURE_MAP` entry to point at the new source directories.
2. Run `python3 bin/feat.py <feature> --update-doc` to rebuild the sentinel block.
3. Double-check that any other feature docs referencing the old path are updated.

## Best Practices

1. Run `bin/feat.py <feature> --update-doc` before editing a doc so you start from a fresh surface list.
2. After editing the prose, run the same command again to ensure the sentinel block still matches the code.
3. When the script flags items that do not appear in the doc body, update the prose accordingly (or decide whether the item should remain internal).

This workflow keeps the documentation honest and drastically reduces manual auditing effort. If `--update-doc` ever fails because a sentinel block is missing, add the matching `<!-- feat:<name> --> … <!-- /feat:<name> -->` wrapper to the relevant doc and re-run the command.
