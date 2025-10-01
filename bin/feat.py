#!/usr/bin/env python3
"""feat2.py - Generalized feature surface inspection tool.

A repository-agnostic documentation helper that discovers, inspects, and
documents public API surfaces across codebases. Works with Rust, Python,
and other languages through a pluggable parser system.

Convention over configuration: Works zero-config by scanning source trees,
with optional `.feat.toml` for customization.

Usage examples:
    feat2.py init                          # Generate .feat.toml config
    feat2.py list                          # Show discovered features
    feat2.py list --verbose                # Include paths and counts
    feat2.py scan global                   # Inspect feature surface
    feat2.py scan global --format json     # JSON output
    feat2.py update global                 # Update feature documentation
    feat2.py sync                          # Update all feature docs
    feat2.py check                         # Validate configuration
    feat2.py --paths src/custom.rs         # Direct file inspection

Configuration:
    Place `.feat.toml` in repository root to customize behavior.
    See FEAT2_STRATEGY.md for full configuration schema.

Links:
    Strategy: docs/tech/FEAT2_STRATEGY.md
    Tasks: TASKS_FEAT2.txt
"""

from __future__ import annotations

import argparse
import dataclasses
import pathlib
import re
import sys
from abc import ABC, abstractmethod
from typing import Dict, Iterable, Iterator, List, Optional, Sequence, Tuple

# TOML parsing: Python 3.11+ has tomllib, older needs tomli
try:
    import tomllib
except ImportError:
    try:
        import tomli as tomllib  # type: ignore
    except ImportError:
        tomllib = None  # type: ignore


# ═══════════════════════════════════════════════════════════════════════════
# DATA MODELS
# ═══════════════════════════════════════════════════════════════════════════


@dataclasses.dataclass(slots=True)
class Item:
    """Represents a public API item (function, struct, class, etc.)."""

    kind: str              # fn, struct, class, export, etc.
    name: str              # Identifier
    location: pathlib.Path # Source file
    line: int              # Line number
    extra: Optional[str] = None  # Context (e.g., "pub use foo::bar")
    language: str = "rust" # Language (rust, python, typescript)

    def render(self, root: pathlib.Path) -> str:
        """Render item as markdown list entry with relative path."""
        rel = self.location.relative_to(root)
        suffix = f" [{self.extra}]" if self.extra else ""
        return f"- {self.name} ({rel}:{self.line}){suffix}"


@dataclasses.dataclass(slots=True)
class Feature:
    """Represents a feature with its source paths and documentation."""

    name: str              # Feature identifier
    paths: List[str]       # Source paths (relative to repo root)
    doc_path: Optional[pathlib.Path] = None  # Documentation file
    language: str = "rust" # Primary language


@dataclasses.dataclass(slots=True)
class Config:
    """Configuration loaded from .feat.toml or defaults."""

    features_root: str = "src"
    docs_root: str = "docs/features"
    doc_pattern: str = "FEATURES_{FEATURE}.md"
    languages: List[str] = dataclasses.field(default_factory=lambda: ["rust"])
    auto_discover: bool = True
    exclude: List[str] = dataclasses.field(default_factory=list)
    features: Dict[str, List[str]] = dataclasses.field(default_factory=dict)

    @staticmethod
    def load(path: pathlib.Path) -> Config:
        """Load configuration from TOML file, return defaults if not found."""
        if not path.exists():
            return Config()  # Return defaults

        if tomllib is None:
            print("warning: tomllib/tomli not available, using defaults", file=sys.stderr)
            return Config()

        try:
            with open(path, "rb") as f:
                data = tomllib.load(f)
            return Config.from_dict(data)
        except Exception as e:
            print(f"error: failed to load config from {path}: {e}", file=sys.stderr)
            sys.exit(1)

    @staticmethod
    def from_dict(data: dict) -> Config:
        """Create Config from dictionary (parsed TOML)."""
        return Config(
            features_root=data.get("features_root", "src"),
            docs_root=data.get("docs_root", "docs/features"),
            doc_pattern=data.get("doc_pattern", "FEATURES_{FEATURE}.md"),
            languages=data.get("languages", ["rust"]),
            auto_discover=data.get("auto_discover", True),
            exclude=data.get("exclude", []),
            features=data.get("features", {}),
        )

    def validate(self) -> List[str]:
        """Validate configuration, return list of error messages."""
        errors = []
        if not self.features_root:
            errors.append("features_root cannot be empty")
        if not self.docs_root:
            errors.append("docs_root cannot be empty")
        if not self.languages:
            errors.append("languages list cannot be empty")
        return errors


# ═══════════════════════════════════════════════════════════════════════════
# REPOSITORY CONTEXT
# ═══════════════════════════════════════════════════════════════════════════


class RepoContext:
    """Detects and caches repository root location."""

    MARKERS = [".git", "Cargo.toml", "package.json", "pyproject.toml"]

    def __init__(self, root: Optional[pathlib.Path] = None):
        """Initialize with explicit root or auto-detect."""
        if root:
            self.root = root.resolve()
        else:
            detected = self.detect_root(pathlib.Path.cwd())
            if detected is None:
                print("warning: not in a repository (no .git, Cargo.toml, etc.)", file=sys.stderr)
                self.root = pathlib.Path.cwd().resolve()
            else:
                self.root = detected

    @staticmethod
    def detect_root(start_path: pathlib.Path) -> Optional[pathlib.Path]:
        """Walk up directory tree looking for repository markers."""
        current = start_path.resolve()
        while current != current.parent:
            for marker in RepoContext.MARKERS:
                if (current / marker).exists():
                    return current
            current = current.parent
        return None


# ═══════════════════════════════════════════════════════════════════════════
# PARSER ABSTRACTION
# ═══════════════════════════════════════════════════════════════════════════


class Parser(ABC):
    """Abstract base class for language-specific parsers."""

    @abstractmethod
    def parse_file(self, path: pathlib.Path) -> List[Item]:
        """Extract public API items from source file."""
        pass

    @abstractmethod
    def supported_extensions(self) -> List[str]:
        """Return file extensions this parser handles."""
        pass


class RustParser(Parser):
    """Parser for Rust source files (pub items, macros)."""

    # Regex patterns ported from feat.py
    PUB_FN_RE = re.compile(r"^\s*pub\s+(?:async\s+)?fn\s+([A-Za-z0-9_]+)")
    PUB_STRUCT_RE = re.compile(r"^\s*pub\s+struct\s+([A-Za-z0-9_]+)")
    PUB_ENUM_RE = re.compile(r"^\s*pub\s+enum\s+([A-Za-z0-9_]+)")
    PUB_TRAIT_RE = re.compile(r"^\s*pub\s+trait\s+([A-Za-z0-9_]+)")
    PUB_TYPE_RE = re.compile(r"^\s*pub\s+type\s+([A-Za-z0-9_]+)")
    PUB_USE_RE = re.compile(r"^\s*pub\s+use\s+(.+);\s*$")
    MACRO_RULES_RE = re.compile(r"^\s*macro_rules!\s+([A-Za-z0-9_]+)")

    def parse_file(self, path: pathlib.Path) -> List[Item]:
        """Parse Rust file for public items."""
        items: List[Item] = []

        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            print(f"warning: failed to read {path} (encoding)", file=sys.stderr)
            return items

        macro_export_pending = False

        for lineno, line in enumerate(text.splitlines(), start=1):
            stripped = line.strip()

            # Track #[macro_export] attribute
            if stripped.startswith("#[macro_export]"):
                macro_export_pending = True
                continue

            # Try to match public items
            matchers: Tuple[Tuple[str, re.Pattern[str]], ...] = (
                ("fn", self.PUB_FN_RE),
                ("struct", self.PUB_STRUCT_RE),
                ("enum", self.PUB_ENUM_RE),
                ("trait", self.PUB_TRAIT_RE),
                ("type", self.PUB_TYPE_RE),
                ("use", self.PUB_USE_RE),
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
                        Item(kind=kind, name=name, location=path, line=lineno,
                             extra=extra, language="rust")
                    )
                    break

            if matched:
                macro_export_pending = False
                continue

            # Check for macro_rules! if we saw #[macro_export]
            if macro_export_pending:
                m = self.MACRO_RULES_RE.match(line)
                if m:
                    items.append(
                        Item(kind="macro", name=f"{m.group(1)}!", location=path,
                             line=lineno, language="rust")
                    )
                    macro_export_pending = False
                continue

            # Reset pending flag if non-attribute line
            if stripped and not stripped.startswith("#"):
                macro_export_pending = False

        return items

    def supported_extensions(self) -> List[str]:
        return [".rs"]


class PythonParser(Parser):
    """Parser for Python source files (classes, functions, __all__)."""

    CLASS_RE = re.compile(r'^\s*class\s+([A-Za-z0-9_]+)')
    DEF_RE = re.compile(r'^\s*def\s+([A-Za-z0-9_]+)')
    ASYNC_DEF_RE = re.compile(r'^\s*async\s+def\s+([A-Za-z0-9_]+)')

    def parse_file(self, path: pathlib.Path) -> List[Item]:
        """Parse Python file for classes and functions."""
        items: List[Item] = []

        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            print(f"warning: failed to read {path} (encoding)", file=sys.stderr)
            return items

        for lineno, line in enumerate(text.splitlines(), start=1):
            # Match classes
            m = self.CLASS_RE.match(line)
            if m:
                name = m.group(1)
                # Skip private classes (leading underscore)
                if not name.startswith("_"):
                    items.append(
                        Item(kind="class", name=name, location=path,
                             line=lineno, language="python")
                    )
                continue

            # Match async functions
            m = self.ASYNC_DEF_RE.match(line)
            if m:
                name = m.group(1)
                if not name.startswith("_"):
                    items.append(
                        Item(kind="async_fn", name=name, location=path,
                             line=lineno, language="python")
                    )
                continue

            # Match regular functions
            m = self.DEF_RE.match(line)
            if m:
                name = m.group(1)
                if not name.startswith("_"):
                    items.append(
                        Item(kind="fn", name=name, location=path,
                             line=lineno, language="python")
                    )
                continue

        return items

    def supported_extensions(self) -> List[str]:
        return [".py"]


class TypeScriptParser(Parser):
    """Parser stub for TypeScript/JavaScript (NOT IMPLEMENTED)."""

    def parse_file(self, path: pathlib.Path) -> List[Item]:
        raise NotImplementedError("NOT IMPLEMENTED lol")

    def supported_extensions(self) -> List[str]:
        return [".ts", ".tsx", ".js", ".jsx"]


# Parser registry
PARSERS: Dict[str, Parser] = {
    "rust": RustParser(),
    "python": PythonParser(),
    "typescript": TypeScriptParser(),
}


# ═══════════════════════════════════════════════════════════════════════════
# DISCOVERY & COLLECTION
# ═══════════════════════════════════════════════════════════════════════════


class Discovery:
    """Auto-discovers features by scanning source tree."""

    def __init__(self, config: Config, repo: RepoContext):
        self.config = config
        self.repo = repo

    def discover(self) -> Dict[str, Feature]:
        """Build feature map from config or auto-discovery."""
        features = {}

        # Start with auto-discovered features if enabled
        if self.config.auto_discover:
            features = self._auto_discover()

        # Merge/override with explicit features from config
        if self.config.features:
            explicit = self._features_from_config()

            # Build map of paths used by explicit features
            explicit_paths = set()
            for feature in explicit.values():
                explicit_paths.update(feature.paths)

            # Remove auto-discovered features that conflict with explicit mappings
            to_remove = []
            for auto_name, auto_feature in features.items():
                # Check if any of this auto-discovered feature's paths are used by explicit mappings
                for auto_path in auto_feature.paths:
                    if auto_path in explicit_paths:
                        to_remove.append(auto_name)
                        break

            for name in to_remove:
                del features[name]

            # Add explicit mappings
            features.update(explicit)

        return features

    def _features_from_config(self) -> Dict[str, Feature]:
        """Build features from explicit config."""
        features = {}
        for name, paths in self.config.features.items():
            features[name] = Feature(name=name, paths=paths)
        return features

    def _auto_discover(self) -> Dict[str, Feature]:
        """Scan source tree and build feature map."""
        features_root = self.repo.root / self.config.features_root

        if not features_root.exists():
            print(f"warning: features_root not found: {features_root}", file=sys.stderr)
            return {}

        features = {}

        # Scan top-level directories as potential features
        for entry in sorted(features_root.iterdir()):
            if not entry.is_dir():
                continue

            # Skip excluded patterns
            if self._is_excluded(entry):
                continue

            feature_name = entry.name
            relative_path = str(entry.relative_to(self.repo.root))

            features[feature_name] = Feature(
                name=feature_name,
                paths=[relative_path]
            )

        return features

    def _is_excluded(self, path: pathlib.Path) -> bool:
        """Check if path matches exclusion patterns."""
        # Simple exclusion: check if path name matches any exclude pattern
        path_str = str(path)
        for pattern in self.config.exclude:
            # Basic glob-style matching (simplified)
            if pattern.replace("**", "").replace("*", "") in path_str:
                return True
        return False


class Collector:
    """Collects files and items for features."""

    def __init__(self, config: Config, repo: RepoContext):
        self.config = config
        self.repo = repo

    def collect_files(self, feature: Feature) -> List[pathlib.Path]:
        """Gather source files for a feature."""
        files = []
        for path_str in feature.paths:
            path = (self.repo.root / path_str).resolve()
            if not path.exists():
                print(f"warning: path not found: {path}", file=sys.stderr)
                continue

            if path.is_dir():
                files.extend(self._iter_files_in_dir(path))
            else:
                files.append(path)

        return sorted(set(files))

    def _iter_files_in_dir(self, directory: pathlib.Path) -> Iterator[pathlib.Path]:
        """Recursively find source files in directory."""
        # Get all supported extensions from parsers
        extensions = []
        for lang in self.config.languages:
            parser = PARSERS.get(lang)
            if parser:
                extensions.extend(parser.supported_extensions())

        for ext in extensions:
            yield from directory.rglob(f"*{ext}")

    def detect_language(self, files: List[pathlib.Path]) -> str:
        """Detect primary language by counting file extensions."""
        counts: Dict[str, int] = {}

        for file_path in files:
            ext = file_path.suffix
            for lang, parser in PARSERS.items():
                if ext in parser.supported_extensions():
                    counts[lang] = counts.get(lang, 0) + 1
                    break

        if not counts:
            return "rust"  # Default

        # Return language with most files
        return max(counts.items(), key=lambda x: x[1])[0]

    def collect_items(self, feature: Feature) -> List[Item]:
        """Parse files and collect all public items."""
        files = self.collect_files(feature)

        if not files:
            return []

        # Detect primary language
        language = self.detect_language(files)
        parser = PARSERS.get(language)

        if not parser:
            print(f"warning: no parser for language '{language}'", file=sys.stderr)
            return []

        items: List[Item] = []
        for file_path in files:
            try:
                file_items = parser.parse_file(file_path)
                items.extend(file_items)
            except NotImplementedError as e:
                print(f"warning: {e} for {file_path}", file=sys.stderr)
            except Exception as e:
                print(f"error: failed to parse {file_path}: {e}", file=sys.stderr)

        # Sort by location, line, name
        return sorted(items, key=lambda it: (str(it.location), it.line, it.name))


# ═══════════════════════════════════════════════════════════════════════════
# DOCUMENTATION
# ═══════════════════════════════════════════════════════════════════════════


class DocResolver:
    """Resolves documentation file paths for features."""

    def __init__(self, config: Config, repo: RepoContext):
        self.config = config
        self.repo = repo

    def resolve(self, feature: Feature, include_stubs: bool = True) -> Optional[pathlib.Path]:
        """Find documentation file for feature (including .stub.md if include_stubs=True)."""
        # Apply pattern substitutions
        pattern = self.config.doc_pattern
        pattern = pattern.replace("{FEATURE}", feature.name.upper())
        pattern = pattern.replace("{feature}", feature.name.lower())
        pattern = pattern.replace("{feat}", feature.name.lower())

        doc_path = self.repo.root / self.config.docs_root / pattern

        if doc_path.exists():
            return doc_path

        # Try alternate locations
        alternates = [
            self.repo.root / "docs" / "features" / pattern,
            self.repo.root / "docs" / "tech" / "features" / pattern,
            self.repo.root / "docs" / pattern,
        ]

        for alt in alternates:
            if alt.exists():
                return alt

        # Try .stub.md variants if requested
        if include_stubs:
            stub_pattern = pattern.replace(".md", ".stub.md")
            stub_paths = [
                self.repo.root / self.config.docs_root / stub_pattern,
                self.repo.root / "docs" / "features" / stub_pattern,
                self.repo.root / "docs" / "tech" / "features" / stub_pattern,
                self.repo.root / "docs" / stub_pattern,
            ]
            for stub in stub_paths:
                if stub.exists():
                    return stub

        return None

    def is_stub(self, doc_path: pathlib.Path) -> bool:
        """Check if documentation file is a stub."""
        return doc_path.suffix == ".md" and doc_path.stem.endswith(".stub")


class DocUpdater:
    """Manages markdown documentation updates."""

    def __init__(self, repo: RepoContext):
        self.repo = repo

    def make_doc_block(self, feature: Feature, items: List[Item]) -> str:
        """Generate markdown sentinel block from items."""
        label = feature.name.lower()
        header = f"<!-- feat:{label} -->"
        footer = f"<!-- /feat:{label} -->"

        # Group items by file
        grouped: Dict[pathlib.Path, List[Item]] = {}
        for item in items:
            grouped.setdefault(item.location, []).append(item)

        lines: List[str] = [header, "", "_Generated by bin/feat2.py --update-doc._", ""]

        for file_path in sorted(grouped):
            rel = file_path.relative_to(self.repo.root)
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

    def create_stub_doc(self, feature: Feature, doc_path: pathlib.Path, items: List[Item]) -> bool:
        """Create a stub documentation file with basic structure."""
        # Ensure parent directory exists
        doc_path.parent.mkdir(parents=True, exist_ok=True)

        # Generate stub content
        feature_title = feature.name.replace("_", " ").title()
        block = self.make_doc_block(feature, items)

        stub_content = f"""# {feature_title} Feature

> **Note**: This is a stub documentation file. Rename to `.md` when ready to finalize.

## Overview

TODO: Describe the purpose and scope of this feature.

## Usage

TODO: Provide usage examples.

## API Surface

{block}
"""

        doc_path.write_text(stub_content, encoding="utf-8")
        print(f"created stub documentation at {doc_path}")
        # Show correct .md filename (remove .stub from stem)
        final_name = doc_path.name.replace(".stub.md", ".md")
        print(f"  → rename to {final_name} when ready to finalize")
        return True

    def update_doc(self, feature: Feature, items: List[Item], doc_path: pathlib.Path) -> bool:
        """Update or append documentation block."""
        if not doc_path.exists():
            print(f"warning: doc file not found: {doc_path}", file=sys.stderr)
            return False

        block = self.make_doc_block(feature, items)
        text = doc_path.read_text(encoding="utf-8")

        label = feature.name.lower()
        start_marker = f"<!-- feat:{label} -->"
        end_marker = f"<!-- /feat:{label} -->"

        # Check if markers exist
        if start_marker not in text or end_marker not in text:
            # Append to end
            new_text = text.rstrip() + "\n\n" + block + "\n"
            doc_path.write_text(new_text, encoding="utf-8")
            print(f"appended sentinel block to {doc_path}")
            return True

        # Replace existing block
        pattern = re.compile(
            rf"(<!-- feat:{label} -->).*?(<!-- /feat:{label} -->)",
            re.DOTALL
        )
        new_text, count = pattern.subn(lambda _: block.rstrip(), text)

        if count == 0:
            print(f"warning: failed to replace block in {doc_path}", file=sys.stderr)
            return False

        doc_path.write_text(new_text, encoding="utf-8")
        print(f"updated {doc_path}")
        return True


# ═══════════════════════════════════════════════════════════════════════════
# CLI COMMANDS
# ═══════════════════════════════════════════════════════════════════════════


def cmd_init(args: argparse.Namespace, repo: RepoContext) -> int:
    """Initialize .feat.toml configuration file."""
    config_path = repo.root / ".feat.toml"

    if config_path.exists() and not args.force:
        print(f"error: {config_path} already exists (use --force to overwrite)", file=sys.stderr)
        return 1

    # Detect primary language
    src_dir = repo.root / "src"
    if not src_dir.exists():
        src_dir = repo.root / "lib"

    rust_files = len(list(repo.root.rglob("*.rs"))) if src_dir.exists() else 0
    python_files = len(list(repo.root.rglob("*.py"))) if src_dir.exists() else 0

    if rust_files > python_files:
        primary_lang = "rust"
    elif python_files > 0:
        primary_lang = "python"
    else:
        primary_lang = "rust"  # Default

    # Generate config
    config_content = f'''# feat2.py configuration
# Auto-generated on {pathlib.Path.cwd()}

features_root = "src"
docs_root = "docs/features"
doc_pattern = "FEATURES_{{FEATURE}}.md"
languages = ["{primary_lang}"]
auto_discover = true

exclude = [
    "**/tests/**",
    "**/target/**",
    "**/__pycache__/**",
]

# Explicit feature mappings (optional, overrides auto-discovery)
[features]
# example = ["src/example"]
'''

    config_path.write_text(config_content, encoding="utf-8")
    print(f"created {config_path}")
    print(f"detected primary language: {primary_lang}")
    print(f"run 'feat2.py list' to see discovered features")

    return 0


def cmd_list(args: argparse.Namespace, config: Config, repo: RepoContext) -> int:
    """List discovered features."""
    discovery = Discovery(config, repo)
    features = discovery.discover()

    if not features:
        print("no features found")
        return 0

    print(f"Found {len(features)} feature(s):\n")

    for name in sorted(features.keys()):
        feature = features[name]
        paths_str = ", ".join(feature.paths)
        print(f"  {name}: {paths_str}")

        if args.verbose:
            collector = Collector(config, repo)
            files = collector.collect_files(feature)
            items = collector.collect_items(feature)
            print(f"    files: {len(files)}, items: {len(items)}")

    return 0


def cmd_scan(args: argparse.Namespace, config: Config, repo: RepoContext) -> int:
    """Inspect feature surface."""
    discovery = Discovery(config, repo)
    features = discovery.discover()

    feature_name = args.feature
    if feature_name not in features:
        print(f"error: unknown feature '{feature_name}'", file=sys.stderr)
        print(f"run 'feat2.py list' to see available features", file=sys.stderr)
        return 1

    feature = features[feature_name]
    collector = Collector(config, repo)
    items = collector.collect_items(feature)

    if not items:
        print(f"no public items found for feature '{feature_name}'")
        return 0

    # Render output
    if args.format == "json":
        import json
        items_data = [
            {
                "kind": it.kind,
                "name": it.name,
                "location": str(it.location.relative_to(repo.root)),
                "line": it.line,
                "extra": it.extra,
                "language": it.language,
            }
            for it in items
        ]
        print(json.dumps(items_data, indent=2))
    else:
        # Text format (like feat.py)
        print(f"== {feature_name.upper()} ==")
        print(f"paths: {', '.join(feature.paths)}")

        categories = {
            "fn": "Functions",
            "async_fn": "Async Functions",
            "struct": "Structs",
            "enum": "Enums",
            "trait": "Traits",
            "type": "Type Aliases",
            "use": "Re-exports",
            "macro": "Exported Macros",
            "class": "Classes",
        }

        grouped: Dict[str, List[Item]] = {key: [] for key in categories}
        for item in items:
            grouped.setdefault(item.kind, []).append(item)

        for key, title in categories.items():
            bucket = grouped.get(key) or []
            if not bucket:
                continue
            print(f"\n{title}:")
            for entry in bucket:
                print(entry.render(repo.root))

        print()

    return 0


def cmd_update(args: argparse.Namespace, config: Config, repo: RepoContext) -> int:
    """Update documentation for a feature."""
    discovery = Discovery(config, repo)
    features = discovery.discover()

    feature_name = args.feature
    if feature_name not in features:
        print(f"error: unknown feature '{feature_name}'", file=sys.stderr)
        return 1

    feature = features[feature_name]
    collector = Collector(config, repo)
    items = collector.collect_items(feature)

    updater = DocUpdater(repo)
    resolver = DocResolver(config, repo)

    # Resolve doc path
    if args.doc:
        doc_path = pathlib.Path(args.doc)
        if not doc_path.exists():
            print(f"error: specified doc file not found: {doc_path}", file=sys.stderr)
            return 1
    else:
        doc_path = resolver.resolve(feature, include_stubs=True)

        if not doc_path:
            # Create stub documentation
            pattern = config.doc_pattern.replace("{FEATURE}", feature_name.upper())
            pattern = pattern.replace("{feature}", feature_name.lower())
            pattern = pattern.replace("{feat}", feature_name.lower())
            stub_pattern = pattern.replace(".md", ".stub.md")

            stub_path = repo.root / config.docs_root / stub_pattern
            success = updater.create_stub_doc(feature, stub_path, items)
            return 0 if success else 1

    # Warn if using stub
    if resolver.is_stub(doc_path):
        print(f"warning: updating stub documentation at {doc_path}", file=sys.stderr)
        final_name = doc_path.name.replace(".stub.md", ".md")
        print(f"  → rename to {final_name} when ready to finalize", file=sys.stderr)

    success = updater.update_doc(feature, items, doc_path)
    return 0 if success else 1


def cmd_sync(args: argparse.Namespace, config: Config, repo: RepoContext) -> int:
    """Update all feature documentation."""
    discovery = Discovery(config, repo)
    features = discovery.discover()

    resolver = DocResolver(config, repo)
    updater = DocUpdater(repo)
    collector = Collector(config, repo)

    updated = 0
    stubs = 0
    skipped = 0
    failed = 0

    for name in sorted(features.keys()):
        feature = features[name]
        doc_path = resolver.resolve(feature, include_stubs=True)

        if not doc_path:
            print(f"skip: {name} (no doc file found)")
            skipped += 1
            continue

        items = collector.collect_items(feature)

        # Check if stub
        is_stub = resolver.is_stub(doc_path)

        if args.dry_run:
            stub_marker = " [stub]" if is_stub else ""
            print(f"would update: {doc_path}{stub_marker}")
            if is_stub:
                stubs += 1
            continue

        if is_stub:
            print(f"warning: updating stub at {doc_path}", file=sys.stderr)
            stubs += 1

        success = updater.update_doc(feature, items, doc_path)
        if success:
            updated += 1
        else:
            failed += 1

    if args.dry_run:
        print(f"\nSummary: would update {updated + stubs} ({stubs} stubs), {skipped} skipped, {failed} failed")
    else:
        summary_parts = [f"{updated} updated"]
        if stubs > 0:
            summary_parts.append(f"{stubs} stubs")
        if skipped > 0:
            summary_parts.append(f"{skipped} skipped")
        if failed > 0:
            summary_parts.append(f"{failed} failed")
        print(f"\nSummary: {', '.join(summary_parts)}")

    return 0 if failed == 0 else 1


def cmd_check(args: argparse.Namespace, config: Config, repo: RepoContext) -> int:
    """Validate configuration and features."""
    errors = config.validate()

    if errors:
        print("Configuration errors:")
        for err in errors:
            print(f"  - {err}")
        return 1

    discovery = Discovery(config, repo)
    features = discovery.discover()

    issues = 0

    # Check feature paths exist
    for name, feature in features.items():
        for path_str in feature.paths:
            path = repo.root / path_str
            if not path.exists():
                print(f"error: path not found for '{name}': {path}")
                issues += 1

    # Check for missing docs
    if args.missing_docs:
        resolver = DocResolver(config, repo)
        stub_count = 0
        missing_count = 0

        for name, feature in features.items():
            doc_path = resolver.resolve(feature, include_stubs=True)
            if not doc_path:
                print(f"warning: no doc file for '{name}'")
                missing_count += 1
                issues += 1
            elif resolver.is_stub(doc_path):
                print(f"stub: {name} has stub documentation at {doc_path}")
                stub_count += 1

        if stub_count > 0 or missing_count > 0:
            print(f"\nDocumentation status: {stub_count} stub(s), {missing_count} missing")

    if issues == 0:
        print("configuration OK")
        return 0
    else:
        print(f"\n{issues} issue(s) found")
        return 1


# ═══════════════════════════════════════════════════════════════════════════
# MAIN ENTRY POINT
# ═══════════════════════════════════════════════════════════════════════════


def parse_args(argv: Sequence[str]) -> argparse.Namespace:
    """Parse command-line arguments."""
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter
    )

    parser.add_argument(
        "--config",
        type=pathlib.Path,
        help="Path to .feat.toml config file (default: auto-detect)"
    )
    parser.add_argument(
        "--root",
        type=pathlib.Path,
        help="Repository root directory (default: auto-detect)"
    )

    subparsers = parser.add_subparsers(dest="command", help="Commands")

    # init
    parser_init = subparsers.add_parser("init", help="Generate .feat.toml config")
    parser_init.add_argument("--force", action="store_true", help="Overwrite existing config")

    # list
    parser_list = subparsers.add_parser("list", help="List discovered features")
    parser_list.add_argument("--verbose", "-v", action="store_true", help="Show details")

    # scan
    parser_scan = subparsers.add_parser("scan", help="Inspect feature surface")
    parser_scan.add_argument("feature", help="Feature name")
    parser_scan.add_argument("--format", choices=["text", "json"], default="text", help="Output format")

    # update
    parser_update = subparsers.add_parser("update", help="Update feature documentation")
    parser_update.add_argument("feature", help="Feature name")
    parser_update.add_argument("--doc", help="Documentation file path")

    # sync
    parser_sync = subparsers.add_parser("sync", help="Update all feature docs")
    parser_sync.add_argument("--dry-run", action="store_true", help="Preview changes")

    # check
    parser_check = subparsers.add_parser("check", help="Validate configuration")
    parser_check.add_argument("--missing-docs", action="store_true", help="Report missing docs")

    return parser.parse_args(argv)


def main(argv: List[str]) -> int:
    """Main entry point."""
    args = parse_args(argv)

    # Detect repository root
    repo = RepoContext(root=args.root if hasattr(args, 'root') else None)

    # Load configuration
    if hasattr(args, 'config') and args.config:
        config_path = args.config
    else:
        config_path = repo.root / ".feat.toml"

    config = Config.load(config_path)

    # Dispatch to command handler
    if args.command == "init":
        return cmd_init(args, repo)
    elif args.command == "list":
        return cmd_list(args, config, repo)
    elif args.command == "scan":
        return cmd_scan(args, config, repo)
    elif args.command == "update":
        return cmd_update(args, config, repo)
    elif args.command == "sync":
        return cmd_sync(args, config, repo)
    elif args.command == "check":
        return cmd_check(args, config, repo)
    else:
        print("error: no command specified", file=sys.stderr)
        print("run 'feat2.py --help' for usage", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
