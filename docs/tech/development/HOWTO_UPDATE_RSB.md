# HOWTO: Update and Refactor RSB (Read Me First)

**WARNING**: Verify repository folder name before making changes.
- As of 2025-09, `rsb/` is the canonical path.
- If you see `.../rsb.old/`, that was a temporary workspace; prefer `rsb/` when present.

**Purpose**: Give a zero-context engineer or agent a fast, opinionated guide to make safe, consistent changes to RSB without rediscovering patterns. Summarize key paradigms (prelude policy, progressive enhancement, macros, tests, features) and where to find deeper docs.


IMPORTANT! RSB uses UNIX/POSIX convention for return values; 0=true; 1=false; any code that has this backwards is WRONG. We've also added TRUE and FALSE constants, and is_true and is_false helpers to assist in maintaining this standard. 

# Chapter 1: Quick Orientation
- Code root: `src/`
  - `lib.rs`: module exports; do not casually add to prelude; keep optional packages behind features.
  - `prelude.rs`: exports only core APIs/macros; OPTIONAL packages (visuals, etc.) must be explicit imports.
  - `macros/`: legacy grouped macros (core, text, streams_exec, fs_data, time_math, visual, etc.). New work should prefer module‑owned macros (see Chapter 2).
  - `param/`: progressive helpers for `param!` macro. Owns `param!` in `src/param/macros.rs`.
  - `string/`: general string helpers and macros. `mod.rs` orchestrates `helpers.rs` and `macros.rs`.
  - `date/`: date/time helpers and macros. `mod.rs` orchestrates `helpers.rs` and `macros.rs`. Replaces legacy `time` module.
  - `visual/`: optional color/glyph/prompt packages behind feature flags.
  - `xcls/`, `streams.rs`, `streamable/`: stream utilities and helpers.

## 1.1: Tests Structure
- Tests root: `tests/` following **strict, enforced test organization system**
- See `docs/tech/development/HOWTO_TEST.md` for complete requirements
- Use `./bin/test.sh docs` for quick access to testing documentation
- **No direct `cargo test`** - all testing flows through `test.sh` runner

## 1.2: Essential Documentation
- `README.md` (top-level orientation + Visuals section)
- **Quick access via test.sh**: `./bin/test.sh docs` (documentation hub)
- Feature guides (under `docs/tech/features/`):
  - `FEATURES_COLORS.md` (visual colors quick stub)
  - `FEATURES_PARAMS.md` (param progressive enhancement plan)
  - `FEATURES_STRINGS.md` (string helpers, macros, Unicode behavior)
  - `FEATURES_GLOBAL.md` (global store/expansion/config/introspection)
  - `FEATURES_DATE.md` (date macros and helpers)
  - Access via: `./bin/test.sh docs features` or `./bin/test.sh docs <feature-name>`
- Module spec: `docs/tech/development/MODULE_SPEC.md` (helper/macro/prelude exposure spec)
  - Access via: `./bin/test.sh docs modules`
- Testing: `docs/tech/development/HOWTO_TEST.md` (test organization and runner)
  - Access via: `./bin/test.sh docs howto`
- Test organization: `docs/tech/development/TEST_ORGANIZATION.md` (structure requirements)
  - Access via: `./bin/test.sh docs org`
- EZ prelude: `rsb::prelude_ez` (curated helpers for prototyping)
- Session notes: `docs/tech/development/SESSION.md` and `.session/SESSION_CURRENT.md`
- References: `docs/tech/reference/RSB_ARCH.md`, `docs/tech/reference/REBEL.md`
  - Access via: `./bin/test.sh docs rsb`

EVERY NEW MAJOR FEATURE NEEDS a `FEATURES_<NAME>.md` under `docs/tech/features/`. Create it alongside code changes.

# Chapter 2: Core Paradigms and Policies

## 2.1: Prelude Policy (Critical)
- The prelude is core-only. Do NOT export optional subsystems (visual/log macros, colors) via prelude.
- Optional components require explicit imports by callers, even when features are enabled.

## 2.2: Progressive Enhancement Pattern
- Prefer a small, stable macro/API surface with internal helpers organized for staged evolution.
- Example: `param!` macro uses `rsb::param::basic` helpers under the hood; `param::advanced` holds future tracing/features.
- Example: `visual::colors` exposes a runtime registry; named colors stay in a global HashMap (no enum) by stakeholder direction.

## 2.3: Module‑Owned Macros and Orchestrators
- **New policy**: each domain module owns its macros under `<module>/macros.rs` and exposes functions via `<module>/helpers.rs`.
- `<module>/mod.rs` acts as an orchestrator that re‑exports its helpers (`pub use helpers::*;`) and includes its macros module.
- We retain a unified import path via `prelude::macros` which re‑exports legacy grouped macros and module‑owned macros (e.g., `param`, `str_*`).
  - Example: `date!` moved from `macros/time_math.rs` to `date/macros.rs` and is re‑exported via the prelude alias.

## 2.4: String Helpers Consolidation
- String helper functions were moved from `utils` to the dedicated `string` module.
- Wildcard prefix/suffix removal uses anchored regex patterns and iterates on Unicode char boundaries to avoid UTF‑8 slicing panics.
- See `FEATURES_STRINGS.md` for details, Unicode notes, and tests. Grapheme‑aware behavior may be added behind a feature flag.

## 2.5: Features and Optional Packages
- Base feature is minimal. Optional packages are behind flags:
  - `visual` base + `colors-simple`, `colors-status`, `colors-named`, `glyphs`, `prompts`.
  - `visuals` umbrella aggregates color sets + glyphs + prompts.
- Ensure callers opt in explicitly; do not make visuals a transitive surprise.

## 2.6: Dependency Re-exports (`rsb::deps`)
- RSB re-exports selected third‑party crates under `rsb::deps` for convenience.
- Use per‑dependency feature flags to keep builds lean:
  - `deps-chrono`, `deps-rand`, `deps-regex`, `deps-serde`, etc.
  - Umbrella alias `deps` (and `deps-all`) enables the full set.
- Example usage patterns:
  - Minimal: `cargo test --features deps-chrono` → `use rsb::deps::chrono;`
  - All: `cargo test --features deps` → `use rsb::deps::*;`
- Implementation pattern:
  - Cargo.toml: define `deps-<name>` features and a `deps-all` group; alias `deps = ["deps-all"]`.
  - src/deps.rs: gate each `pub use <crate>;` behind `#[cfg(any(feature = "deps", feature = "deps-all", feature = "deps-<name>"))]`.

## 2.6: Legacy Macro Organization
- All legacy macros live under `src/macros/` and export at crate root via `#[macro_export]`.
- Group macros logically (core, control_validation, text, time_math, fs_data, streams_exec, visual, etc.).
- Visual/log macros (e.g., `colored!`, `info!`) depend on `utils::expand_colors_unified` and should be considered optional.
- For inline tag macros like `colored!`: support a single-arg form to avoid format! brace conflicts.

## 2.7: Visual Colors and Registry (Optional)
- String-first runtime registry. Case-insensitive lookups.
- Backgrounds are off until explicitly enabled via `color_enable_with("...,bg")`.
- `colored("{...}")` expands inline tags; unknown tags pass through verbatim; glyph tags only render when glyphs are enabled.
- Named colors MUST remain in the global HashMap (not enums).

# Chapter 3: Testing Framework

## 3.1: Test Structure (ENFORCED)
RSB uses a **strict, enforced test organization system** following BASHFX Visual Friendliness Principles:

**Test Categories (all enforced by `test.sh`):**
- **smoke** - Minimal CI tests (<10s total runtime)
- **sanity** - Core functionality validation (**REQUIRED** for every module)
- **unit** - Fast, isolated module tests (<1s each)
- **integration** - Cross-module interaction tests
- **e2e** - End-to-end user workflow tests
- **uat** - User Acceptance Tests with visual ceremony (**REQUIRED** for every module)
- **chaos** - Edge cases, stress tests, property tests
- **bench** - Performance benchmarks

**Directory Structure:**
```
tests/
├── unit/                    # Fast, isolated module tests
├── sanity/                  # Core functionality validation (REQUIRED)
├── smoke/                   # Minimal CI tests
├── integration/             # Cross-module interaction tests
├── e2e/                     # End-to-end user workflow tests
├── uat/                     # User Acceptance Tests (visual ceremony)
├── chaos/                   # Edge cases, stress tests
├── bench/                   # Performance benchmarks
├── _adhoc/                  # Experimental tests (outside enforcement)
└── sh/                      # Shell scripts for test ceremony
```

**Wrapper Pattern (REQUIRED):** All tests need wrapper files in `tests/` root:
- `sanity_strings.rs` → includes `tests/sanity/strings.rs`
- `uat_strings.rs` → includes `tests/uat/strings.rs`

**CRITICAL:** Every module MUST have sanity and UAT tests or tests will be blocked.

## 3.2: Test Runner and Ceremony
The test runner: `bin/test.sh` - **ALL testing flows through this runner**

**Core Commands:**
- `./bin/test.sh` - Show test status and help
- `./bin/test.sh run sanity` - Run sanity tests
- `./bin/test.sh run uat` - Run UAT with visual ceremony
- `./bin/test.sh lint` - Check test organization compliance
- `./bin/test.sh list` - List available tests
- `./bin/test.sh adhoc` - Work with experimental tests

**Visual Test Ceremonies:**
- `./tests/sh/ceremony.sh sanity` - Sanity tests with ceremony
- `./tests/sh/ceremony.sh uat` - UAT with visual demonstrations
- `./tests/sh/ceremony.sh all` - Complete test ceremony

**Documentation Access:**
- `./bin/test.sh docs` - Documentation hub
- `./bin/test.sh docs howto` - Testing HOWTO guide
- `./bin/test.sh docs org` - Test organization requirements

## 3.3: Adding New Test Suites
Follow the enforced test organization pattern:

**Steps:**
1. Create the actual test: `tests/sanity/new_module.rs`
2. Create the wrapper: `tests/sanity_new_module.rs`
3. Verify compliance: `./bin/test.sh lint`
4. Run your test: `./bin/test.sh run sanity_new_module`

**Required Tests for Every Module:**
1. **Sanity tests** - `tests/sanity/module_name.rs`
2. **UAT tests** - `tests/uat/module_name.rs`

**Test Patterns:**
- **Sanity tests** - Simple, demonstrative functionality checks
- **UAT tests** - Visual ceremony with `println!` statements showing commands and outputs

Missing either sanity or UAT tests will block all tests until created.

# Chapter 4: Development Patterns
## 4.1: Pre-Refactoring Checklist
Before refactoring:
- Confirm prelude policy impact (avoid adding exports there).
- Identify feature flags needed.
- Ensure tests follow enforced organization (sanity + UAT required).
- Check test compliance: `./bin/test.sh lint`
- Test via runner: `./bin/test.sh run sanity` and `./bin/test.sh run uat`

## 4.2: Module Creation/Refactoring
- Keep code in `src/<module>` directories when it grows beyond a single file.
- Follow MODULE_SPEC.md patterns (access via `./bin/test.sh docs modules`):
  - `<module>/mod.rs` - Orchestrator and re-exports, curated public surface
  - `<module>/utils.rs` - Curated low-level helpers ("utils" namespace)
  - `<module>/helpers.rs` - Internal implementations (optional)
  - `<module>/macros.rs` - Module-owned macros (value + var forms)
  - `<module>/error.rs` - Typed error enums
- For progressive enhancement, expose a small public API and delegate to `::<module>::basic` internally, reserving `::<module>::advanced` for richer features later.
- **REQUIRED:** Create sanity and UAT tests for new modules or tests will be blocked.

## 4.3: Visual Component Additions
- Add color names to the named palette map (registry), not enums. Update runtime registry if needed.
- **REQUIRED:** Add UATs under `tests/uat/` following enforced test organization.
- Follow wrapper pattern: create `tests/uat_colors.rs` wrapper that includes `tests/uat/colors.rs`
- Test via: `./bin/test.sh run uat_colors`

# Chapter 5: Change Validation

## 5.1: Minimal Checklist for Changes
- [ ] Respect prelude policy (no optional exports in prelude)
- [ ] Use progressive helpers (`::<module>::basic`) for complex macros
- [ ] Gate optional features; keep default lean
- [ ] Follow MODULE_SPEC.md patterns for new modules
- [ ] **REQUIRED:** Create sanity and UAT tests for new modules
- [ ] Use enforced test organization pattern (wrapper + actual test files)
- [ ] Check test compliance: `./bin/test.sh lint`
- [ ] Test via runner: `./bin/test.sh run sanity` and `./bin/test.sh run uat`
- [ ] **NO direct `cargo test`** - use test.sh runner only
- [ ] Update `prelude::macros` if adding new module‑owned macros
- [ ] Update feature guides (`FEATURES_<NAME>.md`) if behavior changes
- [ ] Update documentation via `./bin/test.sh docs` as needed

# Chapter 6: Common Commands

## 6.1: Test Commands (**ONLY use test.sh runner**)
- `./bin/test.sh` — show test status and help
- `./bin/test.sh lint` — check test organization compliance
- `./bin/test.sh run sanity` — core functionality validation
- `./bin/test.sh run uat` — user acceptance tests with visual ceremony
- `./bin/test.sh run smoke` — minimal CI tests (<10s)
- `./bin/test.sh list` — list all available tests
- `./bin/test.sh adhoc` — work with experimental tests

## 6.2: Test Ceremony Commands
- `./tests/sh/ceremony.sh sanity` — sanity tests with ceremony
- `./tests/sh/ceremony.sh uat` — UAT with visual demonstrations
- `./tests/sh/ceremony.sh all` — complete test ceremony

## 6.3: Documentation Commands
- `./bin/test.sh docs` — documentation hub
- `./bin/test.sh docs howto` — testing HOWTO guide
- `./bin/test.sh docs org` — test organization requirements
- `./bin/test.sh docs modules` — module specification patterns
- `./bin/test.sh docs rsb` — RSB architecture documentation
- `./bin/test.sh docs features` — list all feature documentation
- `./bin/test.sh docs <feature-name>` — show specific feature docs

## 6.4: **DEPRECATED:** Direct Cargo Commands
❌ **DO NOT USE:** `cargo test` directly - all testing flows through `test.sh`


# Chapter 7: Additional Resources

## 7.1: Architecture and Context
- For architectural intent: `./bin/test.sh docs rsb` (RSB_ARCH.md + REBEL.md)
- For current session context, read `.session/SESSION_CURRENT.md` (recent) and `docs/tech/development/SESSION.md` (history).

## 7.2: Documentation Quick Access
- **Documentation Hub:** `./bin/test.sh docs` - central access to all docs
- **Test Organization:** `./bin/test.sh docs org` - structure requirements
- **Testing Guide:** `./bin/test.sh docs howto` - complete testing documentation
- **Module Patterns:** `./bin/test.sh docs modules` - MODULE_SPEC.md reference
- **Features:** `./bin/test.sh docs features` - list all features
- **Specific Feature:** `./bin/test.sh docs <feature-name>` - detailed feature docs

## 7.3: Module-Specific Quick Reference
- `param!` lives at `src/param/macros.rs`; helpers at `src/param/basic.rs`.
- Strings live at `src/string/` with `helpers.rs` and `macros.rs`. See `FEATURES_STRINGS.md` for Unicode and wildcard semantics.
- Module exposure pattern and naming conventions: `./bin/test.sh docs modules`
- For dev/testing convenience, `rsb::prelude_dev` aggregates curated low-level helpers:
  - `rsb::prelude_dev::string` → `string::utils` (helpers, case, error, safety registry)
  - `rsb::prelude_dev::param` → `param::utils`
  - Note: stream items are intentionally deferred until the stream module reorg is complete.

## 7.4: Test Organization Enforcement
- **CRITICAL:** RSB uses strict test organization - tests will be blocked if not compliant
- Check compliance: `./bin/test.sh lint`
- Every module MUST have: sanity tests + UAT tests
- All testing flows through `test.sh` runner - **NO direct `cargo test`**
- For complete requirements: `./bin/test.sh docs org`
