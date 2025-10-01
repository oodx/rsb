# RSB Documentation Inventory & Migration Plan

**Date:** 2025-09-30
**Status:** ANALYSIS COMPLETE
**Purpose:** Inventory all RSB docs and map to new rsbdoc/Brain structure

## Summary

- **Total docs found:** 41 files
- **CURRENT (actively maintained):** 40 files
- **TEMPORAL (version-specific):** 1 file (RSB_V2.md)
- **STALE:** 0 files

## Migration Strategy

### STAY in RSB Repository (24 files)
**Access via:** `rsbdoc feat <name>`
**Location:** `$RSB_HOME/docs/tech/features/FEATURES_*.md`

All feature documentation should remain in RSB repo as they are RSB-specific:

| File | Command | Notes |
|------|---------|-------|
| FEATURES_BASH.md | `rsbdoc feat bash` | Bash integration macros & utilities |
| FEATURES_CLI.md | `rsbdoc feat cli` | CLI args, dispatch, options |
| FEATURES_COLORS.md | `rsbdoc feat colors` | Color system & themes |
| FEATURES_DATE.md | `rsbdoc feat date` | Date/time utilities |
| FEATURES_DEV.md | `rsbdoc feat dev` | Developer tooling |
| FEATURES_FS.md | `rsbdoc feat fs` | Filesystem operations |
| FEATURES_GENERATORS.md | `rsbdoc feat generators` | Code generation utilities |
| FEATURES_GLOBAL.md | `rsbdoc feat global` | Global variables & config |
| FEATURES_HOST.md | `rsbdoc feat host` | Host system integration |
| FEATURES_JOBS.md | `rsbdoc feat jobs` | Background jobs & signals |
| FEATURES_MATH.md | `rsbdoc feat math` | Math utilities & comparisons |
| FEATURES_OBJECT.md | `rsbdoc feat object` | Object system |
| FEATURES_OPTIONS.md | `rsbdoc feat options` | Options parsing |
| FEATURES_PARAMS.md | `rsbdoc feat params` | Parameter processing |
| FEATURES_PARSE.md | `rsbdoc feat parse` | Parsing utilities |
| FEATURES_PROGRESS.md | `rsbdoc feat progress` | Progress tracking |
| FEATURES_PROMPTS.md | `rsbdoc feat prompts` | User prompts & interaction |
| FEATURES_REPL.md | `rsbdoc feat repl` | REPL environment |
| FEATURES_STRINGS.md | `rsbdoc feat strings` | String manipulation |
| FEATURES_TESTING.md | `rsbdoc feat testing` | test.sh framework |
| FEATURES_TOKENS.md | `rsbdoc feat tokens` | Token processing |
| FEATURES_TOML.md | `rsbdoc feat toml` | TOML configuration |
| FEATURES_TRUTH.md | `rsbdoc feat truth` | Validation & truth checking |
| FEATURES_VISUALS.md | `rsbdoc feat visuals` | Visual display & rendering |

**Additional RSB-specific docs to keep:**
- RSB_QUICK_REFERENCE.md → `rsbdoc feat quick` (quick reference guide)
- RSB_TEST_RUNNER.md → `rsbdoc feat test-runner` (test runner specifics)

### MIGRATE to Brain: Architecture (10 files)
**Access via:** `rsbdoc arch <name>`
**Target:** `$BRAIN_HOME/dev/architecture/`

| Current File | New Location | Command | Notes |
|--------------|--------------|---------|-------|
| BASHFX-v3.md | brain/dev/architecture/bashfx/ | `rsbdoc arch bashfx` | BashFX framework docs |
| RSB_ARCH.md | brain/dev/architecture/rsb/ | `rsbdoc arch rsb` | RSB architecture overview |
| RSB_BASHFX_ALIGN.md | brain/dev/architecture/bashfx/ | `rsbdoc arch bashfx-align` | BashFX alignment spec |
| AST_CONSIDERATIONS.md | brain/dev/architecture/ | `rsbdoc arch ast` | AST/IR design considerations |
| FEATURES_GATING_PLAN.md | brain/dev/architecture/ | `rsbdoc arch features` | Module feature gating |
| MODULE_SPEC.md | brain/dev/architecture/ | `rsbdoc arch modules` | Module specification |
| PRELUDE_POLICY.md | brain/dev/architecture/ | `rsbdoc arch prelude` | Prelude design policy |

### MIGRATE to Brain: Process (6 files)
**Access via:** `rsbdoc proc <name>`
**Target:** `$BRAIN_HOME/dev/proccess/`

| Current File | New Location | Command | Notes |
|--------------|--------------|---------|-------|
| HOWTO_HUB.md | brain/dev/proccess/hub/ | `rsbdoc proc hub` | Project hub usage |
| RSB_DOCS_STRATEGY.md | brain/dev/proccess/ | `rsbdoc proc docs` | Documentation strategy |
| RSB_TESTSH_INTEGRATION.md | brain/dev/proccess/testing/ | `rsbdoc proc test-integration` | test.sh integration |
| HOWTO_TEST.md | brain/dev/proccess/testing/ | `rsbdoc proc testing` | Testing procedures |
| HOWTO_UPDATE_RSB.md | brain/dev/proccess/ | `rsbdoc proc update` | Update procedures |
| TEST_ORGANIZATION.md | brain/dev/proccess/testing/ | `rsbdoc proc test-org` | Test organization |

### MIGRATE to Brain: Concepts (1 file)
**Access via:** `rsbdoc concept <name>`
**Target:** `$BRAIN_HOME/dev/concepts/`

| Current File | New Location | Command | Notes |
|--------------|--------------|---------|-------|
| REBEL.md | brain/dev/concepts/rebel/ | `rsbdoc concept rebel` | REBEL framework |

### TEMPORAL/ARCHIVE (1 file)
**Decision needed:** Archive or convert to version history

| File | Status | Recommendation |
|------|--------|----------------|
| RSB_V2.md | TEMPORAL | Move to `brain/dev/concepts/version-history/` or archive |

## Migration Steps

1. **Create Brain directories** (if not exist):
   ```bash
   mkdir -p ~/repos/docs/brain/dev/architecture/{rsb,bashfx}
   mkdir -p ~/repos/docs/brain/dev/proccess/{hub,testing}
   mkdir -p ~/repos/docs/brain/dev/concepts/{rebel,version-history}
   ```

2. **Move architecture docs:**
   ```bash
   mv docs/tech/reference/RSB_ARCH.md ~/repos/docs/brain/dev/architecture/rsb/
   mv docs/tech/reference/BASHFX-v3.md ~/repos/docs/brain/dev/architecture/bashfx/
   mv docs/tech/reference/RSB_BASHFX_ALIGN.md ~/repos/docs/brain/dev/architecture/bashfx/
   mv docs/tech/development/AST_CONSIDERATIONS.md ~/repos/docs/brain/dev/architecture/
   mv docs/tech/development/FEATURES_GATING_PLAN.md ~/repos/docs/brain/dev/architecture/
   mv docs/tech/development/MODULE_SPEC.md ~/repos/docs/brain/dev/architecture/
   mv docs/tech/development/PRELUDE_POLICY.md ~/repos/docs/brain/dev/architecture/
   ```

3. **Move process docs:**
   ```bash
   mv docs/tech/reference/HOWTO_HUB.md ~/repos/docs/brain/dev/proccess/hub/
   mv docs/tech/reference/RSB_DOCS_STRATEGY.md ~/repos/docs/brain/dev/proccess/
   mv docs/tech/reference/RSB_TESTSH_INTEGRATION.md ~/repos/docs/brain/dev/proccess/testing/
   mv docs/tech/development/HOWTO_TEST.md ~/repos/docs/brain/dev/proccess/testing/
   mv docs/tech/development/HOWTO_UPDATE_RSB.md ~/repos/docs/brain/dev/proccess/
   mv docs/tech/development/TEST_ORGANIZATION.md ~/repos/docs/brain/dev/proccess/testing/
   ```

4. **Move concept docs:**
   ```bash
   mv docs/tech/reference/REBEL.md ~/repos/docs/brain/dev/concepts/rebel/
   ```

5. **Handle temporal docs:**
   ```bash
   # Decision needed: archive or version-history
   mv docs/tech/reference/RSB_V2.md ~/repos/docs/brain/dev/concepts/version-history/
   ```

6. **Reorganize RSB repo:**
   ```bash
   # Keep only RSB-specific features
   mv docs/tech/reference/RSB_QUICK_REFERENCE.md docs/tech/features/
   mv docs/tech/reference/RSB_TEST_RUNNER.md docs/tech/features/
   ```

7. **Clean up empty directories:**
   ```bash
   rmdir docs/tech/development 2>/dev/null
   rmdir docs/tech/reference 2>/dev/null
   ```

## Final Structure

**RSB Repository (`docs/tech/features/`):**
- 26 FEATURES_*.md files (RSB-specific features)
- RSB_QUICK_REFERENCE.md
- RSB_TEST_RUNNER.md

**Brain Repository (`~/repos/docs/brain/dev/`):**
```
architecture/
├── rsb/                    # RSB_ARCH.md
├── bashfx/                 # BASHFX-v3.md, RSB_BASHFX_ALIGN.md
├── AST_CONSIDERATIONS.md
├── FEATURES_GATING_PLAN.md
├── MODULE_SPEC.md
└── PRELUDE_POLICY.md

proccess/
├── hub/                    # HOWTO_HUB.md
├── testing/                # test-related docs
├── RSB_DOCS_STRATEGY.md
└── HOWTO_UPDATE_RSB.md

concepts/
├── rebel/                  # REBEL.md
└── version-history/        # RSB_V2.md (if kept)
```

## Notes

- All dates are September 2025 (very recent, well-maintained)
- No stale documentation found
- `plans/` subdirectory in features/ not analyzed (may contain temporal planning docs)
- Migration preserves all documentation, just reorganizes for better access via rsbdoc

## Next Actions

1. ✅ Review this inventory
2. ⬜ Decide on RSB_V2.md disposition (archive vs version-history)
3. ⬜ Execute migration steps 1-7
4. ⬜ Update RSB_DOCS_STRATEGY.md with actual paths
5. ⬜ Begin rsbdoc CLI implementation
