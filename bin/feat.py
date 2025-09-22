#!/usr/bin/env python3
"""Feature surface inspection helper for the RSB docs workflow.

Generates an inventory of public-facing items (functions, types, re-exports,
and exported macros) for a given feature so we can compare code surfaces with
the corresponding feature documentation.

Usage examples:
    bin/feat.py list
    bin/feat.py global
    bin/feat.py global strings cli
    bin/feat.py --paths src/custom/mod.rs another.rs

The script intentionally uses lightweight line-based parsing so it does not
depend on rustc or external crates. It is best-effort and meant to assist the
manual documentation pass rather than produce a formal API manifest.
"""

from __future__ import annotations

import argparse
import dataclasses
import pathlib
import re
import sys
from typing import Dict, Iterable, Iterator, List, Sequence, Tuple, Optional


ROOT = pathlib.Path(__file__).resolve().parents[1]


FEATURE_MAP: Dict[str, Sequence[str]] = {
    "global": [
        "src/global",
        "src/macros/control_validation.rs",  # export!/src! live here
    ],
    "bash": [
        "src/bash",
    ],
    "colors": [
        "src/colors",
        "src/utils.rs",
        "src/visual/macros.rs",
    ],
    "date": [
        "src/date",
    ],
    "dev": [
        "src/dev",
        "src/dev_ns",
    ],
    "fs": [
        "src/fs",
    ],
    "generators": [
        "src/gx",
    ],
    "host": [
        "src/hosts",
    ],
    "math": [
        "src/math",
    ],
    "options": [
        "src/cli/options.rs",
        "src/cli/macros.rs",
    ],
    "params": [
        "src/param",
    ],
    "parse": [
        "src/parse",
    ],
    "progress": [
        "src/progress",
    ],
    "prompts": [
        "src/visual/prompts",
        "src/visual/macros.rs",
        "src/visual/utils.rs",
    ],
    "strings": [
        "src/string",
    ],
    "testing": [
        "bin/test.sh",
        "src/macros/test_helpers.rs",
    ],
    "threads": [
        "src/threads",
    ],
    "tokens": [
        "src/token",
    ],
    "truth": [
        "src/com",
    ],
    "visuals": [
        "src/visual",
    ],
    "cli": [
        "src/cli",
        "src/cli/macros.rs",
    ],
}


PUB_FN_RE = re.compile(r"^\s*pub\s+(?:async\s+)?fn\s+([A-Za-z0-9_]+)")
PUB_STRUCT_RE = re.compile(r"^\s*pub\s+struct\s+([A-Za-z0-9_]+)")
PUB_ENUM_RE = re.compile(r"^\s*pub\s+enum\s+([A-Za-z0-9_]+)")
PUB_TRAIT_RE = re.compile(r"^\s*pub\s+trait\s+([A-Za-z0-9_]+)")
PUB_TYPE_RE = re.compile(r"^\s*pub\s+type\s+([A-Za-z0-9_]+)")
PUB_USE_RE = re.compile(r"^\s*pub\s+use\s+(.+);\s*$")
MACRO_RULES_RE = re.compile(r"^\s*macro_rules!\s+([A-Za-z0-9_]+)")


@dataclasses.dataclass(slots=True)
class Item:
    kind: str
    name: str
    location: pathlib.Path
    line: int
    extra: str | None = None

    def render(self, root: pathlib.Path) -> str:
        rel = self.location.relative_to(root)
        suffix = f" [{self.extra}]" if self.extra else ""
        return f"- {self.name} ({rel}:{self.line}){suffix}"


def iter_files(paths: Sequence[str]) -> Iterator[pathlib.Path]:
    for entry in paths:
        p = (ROOT / entry).resolve()
        if not p.exists():
            continue
        if p.is_dir():
            yield from p.rglob("*.rs")
        else:
            yield p


def collect_items(files: Iterable[pathlib.Path]) -> List[Item]:
    items: List[Item] = []
    for file_path in sorted(set(files)):
        rel = file_path.relative_to(ROOT)
        try:
            text = file_path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            print(f"warning: failed to read {rel} (encoding)", file=sys.stderr)
            continue

        macro_export_pending = False
        for lineno, line in enumerate(text.splitlines(), start=1):
            stripped = line.strip()
            if stripped.startswith("#[macro_export]"):
                macro_export_pending = True
                continue

            matchers: Tuple[Tuple[str, re.Pattern[str]], ...] = (
                ("fn", PUB_FN_RE),
                ("struct", PUB_STRUCT_RE),
                ("enum", PUB_ENUM_RE),
                ("trait", PUB_TRAIT_RE),
                ("type", PUB_TYPE_RE),
                ("use", PUB_USE_RE),
            )

            matched = False
            for kind, regex in matchers:
                m = regex.match(line)
                if m:
                    matched = True
                    name = m.group(1).strip()
                    extra = None
                    if kind == "use":
                        extra = name
                        name = "pub use"
                    items.append(
                        Item(kind=kind, name=name, location=file_path, line=lineno, extra=extra)
                    )
                    break

            if matched:
                macro_export_pending = False
                continue

            if macro_export_pending:
                m = MACRO_RULES_RE.match(line)
                if m:
                    items.append(
                        Item(kind="macro", name=f"{m.group(1)}!", location=file_path, line=lineno)
                    )
                    macro_export_pending = False
                continue

            # Reset pending flag if we encounter a non-attribute line to avoid carrying too far.
            if stripped and not stripped.startswith("#"):
                macro_export_pending = False

    return items


def render_feature(name: str, paths: Sequence[str]) -> None:
    files = list(iter_files(paths))
    if not files:
        print(f"== {name.upper()} ==\n(no matching files)\n")
        return

    items = collect_items(files)
    if not items:
        print(f"== {name.upper()} ==\n(no public items found)\n")
        return

    categories = {
        "fn": "Functions",
        "struct": "Structs",
        "enum": "Enums",
        "trait": "Traits",
        "type": "Type Aliases",
        "use": "Re-exports",
        "macro": "Exported Macros",
    }

    print(f"== {name.upper()} ==")
    display_paths = [p if p.startswith("/") else p for p in paths]
    print(f"paths: {', '.join(display_paths)}")

    grouped: Dict[str, List[Item]] = {key: [] for key in categories}
    for item in items:
        grouped.setdefault(item.kind, []).append(item)

    for key, title in categories.items():
        bucket = grouped.get(key) or []
        if not bucket:
            continue
        print(f"\n{title}:")
        for entry in sorted(bucket, key=lambda it: (str(it.location), it.line, it.name)):
            print(entry.render(ROOT))

    print()


def available_features() -> List[str]:
    return sorted(FEATURE_MAP)


def parse_args(argv: Sequence[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "items",
        nargs="*",
        help="Feature names to inspect (see 'list') or file paths via --paths",
    )
    parser.add_argument(
        "--paths",
        nargs="+",
        help="Direct file or directory paths (relative to repo root) to inspect",
    )
    parser.add_argument(
        "--list",
        action="store_true",
        help="List known features and exit",
    )
    parser.add_argument(
        "--update-doc",
        nargs="?",
        const="auto",
        help="Rewrite sentinel block in the feature doc (default auto path)",
    )
    return parser.parse_args(argv)


def make_doc_block(feature: str, items: List[Item]) -> str:
    label = feature.lower()
    header = f"<!-- feat:{label} -->"
    footer = f"<!-- /feat:{label} -->"

    grouped: Dict[pathlib.Path, List[Item]] = {}
    for item in items:
        grouped.setdefault(item.location, []).append(item)

    lines: List[str] = [header, "", "_Generated by bin/feat.py --update-doc._", ""]

    for file_path in sorted(grouped):
        rel = file_path.relative_to(ROOT)
        lines.append(f"* `{rel}`")
        for entry in sorted(grouped[file_path], key=lambda it: it.line):
            if entry.kind == "use" and entry.extra:
                desc = f"pub use {entry.extra}"
            elif entry.kind == "macro":
                desc = f"macro {entry.name}"
            else:
                desc = f"{entry.kind} {entry.name}"
            lines.append(f"  - {desc} (line {entry.line})")
        lines.append("")

    if lines[-1] != "":
        lines.append("")
    lines.append(footer)
    lines.append("")
    return "\n".join(lines)


def update_doc(feature: str, paths: Sequence[str], doc_spec: Optional[str]) -> None:
    doc_path = doc_spec
    if doc_path in (None, "auto"):
        candidate = ROOT / "docs" / "tech" / "features" / f"FEATURES_{feature.upper()}.md"
        doc_path = str(candidate)

    doc_file = pathlib.Path(doc_path)
    if not doc_file.exists():
        print(f"warning: doc file not found for {feature}: {doc_file}")
        return

    items = collect_items(iter_files(paths))
    block = make_doc_block(feature, items)

    text = doc_file.read_text(encoding="utf-8")
    start_marker = f"<!-- feat:{feature.lower()} -->"
    end_marker = f"<!-- /feat:{feature.lower()} -->"

    if start_marker not in text or end_marker not in text:
        # Append block to the end (ensuring newline separation)
        new_text = text.rstrip() + "\n\n" + block + "\n"
        doc_file.write_text(new_text, encoding="utf-8")
        print(f"appended sentinel block to {doc_file} for feature '{feature}'")
        return

    pattern = re.compile(
        rf"(<!-- feat:{feature.lower()} -->).*?(<!-- /feat:{feature.lower()} -->)",
        re.DOTALL,
    )
    new_text, count = pattern.subn(lambda _: block, text)
    if count == 0:
        print(f"warning: failed to replace sentinel block for {feature} in {doc_file}")
        return

    doc_file.write_text(new_text, encoding="utf-8")
    print(f"updated {doc_file} with feat.py surface for '{feature}'")


def main(argv: Sequence[str]) -> int:
    args = parse_args(argv)

    if args.list or (not args.items and not args.paths):
        for name in available_features():
            mapped = ", ".join(FEATURE_MAP[name])
            print(f"{name}: {mapped}")
        return 0

    update_requested = args.update_doc is not None

    if args.paths:
        render_feature("custom", args.paths)

    for raw_name in args.items:
        name = raw_name.lower()
        if name not in FEATURE_MAP:
            print(f"warning: unknown feature '{raw_name}'. use --list to see options.")
            continue
        if update_requested:
            update_doc(name, FEATURE_MAP[name], args.update_doc)
        else:
            render_feature(name, FEATURE_MAP[name])

    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
