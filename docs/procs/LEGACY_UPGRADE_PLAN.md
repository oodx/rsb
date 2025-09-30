# RSB Legacy Code Upgrade Plan

**Date**: 2025-09-30
**Spec Version**: MODULE_SPEC v3
**Reference**: `docs/tech/development/MODULE_SPEC.md`
**Status**: Analysis Complete - Ticketed in TASKS.txt as LEG-XX

## Executive Summary

**Compliance Rate**: 77% (24/31 modules compliant)
**Legacy Files Found**: 7 root-level .rs files (1,207 lines total)
**Strategy**: Audit for duplicates/superseded code, migrate unique functionality

### Key MODULE_SPEC Violation
> *IMPORTANT* no other top level files or modules are allowed under `src` legacy files need to be refactored or reorgd into a proper submodule space.

---

## 🎯 Prioritized Upgrade Tickets

All tickets tracked in `docs/procs/TASKS.txt` with LEG-XX identifiers.

### HIGH PRIORITY - Immediate Investigation Required

#### LEG-01: Audit src/os.rs for Duplicates/Migration [5 SP]
**File**: `src/os.rs` (401 lines, 39 exports)
**Status**: Partially migrated to src/os/ directory (missing mod.rs)
**Concern**: May contain duplicate functionality from bash/ or other modules

**Investigation Required**:
- Compare with `src/bash/` - command execution overlap?
- Check if CmdResult duplicates existing types
- Verify exec! and cmd! macros aren't superseded
- Determine if src/os/ directory has incomplete migration

**Action Path**:
1. Audit exports vs bash/, threads/, hosts/ modules
2. Document unique vs duplicate functionality
3. Create src/os/mod.rs if keeping unique code
4. Delete src/os.rs after migration OR deprecate if superseded
5. Run: `./bin/test.sh run sanity bash`

**Estimated**: 3-5 hours (includes audit + migration/deletion)

---

#### LEG-02: Audit src/utils.rs for Duplicates/Distribution [4 SP]
**File**: `src/utils.rs` (290 lines, 17 exports)
**Status**: Miscellaneous utilities, possibly superseded
**Concern**: May duplicate string/, fs/, cli/, visual/prompts/ functionality

**Investigation Required**:
- StringExt trait - compare with `src/string/` exports
- Array operations - check math/ or param/ for equivalents
- User interaction functions - compare with visual/prompts/

**Action Path**:
1. List all public exports from utils.rs
2. Cross-reference with string/, fs/, cli/, visual/ modules
3. Mark duplicates for deletion, unique for migration
4. Create distribution plan for unique code
5. Update imports across codebase
6. Delete src/utils.rs after distribution

**Estimated**: 2-4 hours (audit-heavy, low migration risk)

---

#### LEG-03: Audit src/random.rs for Superseded Code [2 SP]
**File**: `src/random.rs` (78 lines)
**Status**: Contains comment "BACKWARD COMPATIBILITY RE-EXPORTS"
**Concern**: May be fully superseded by gx/ (generators) module

**Investigation Required**:
- Compare exports with `src/gx/` module
- Check if any callers still use rsb::random::*
- Verify gx/ provides equivalent or better functionality

**Action Path**:
1. List public re-exports
2. Check gx/ module for equivalents
3. Search codebase for import references
4. If superseded: deprecate with warnings pointing to gx/
5. If unique: migrate to src/gx/ as submodule
6. Delete src/random.rs after deprecation period

**Estimated**: 1-2 hours (small file, clear deprecation path)

---

### MEDIUM PRIORITY - Cleanup & Organization

#### LEG-04: Review src/_todo/ Directory [1 SP]
**Directory**: `src/_todo/` (no mod.rs, underscore prefix)
**Status**: Unknown purpose, non-standard naming
**Concern**: May contain incomplete MODULE_SPEC v3 migration attempts

**Investigation Required**:
- List contents of src/_todo/
- Determine original purpose
- Check for references in src/lib.rs or other modules

**Action Path**:
1. Inventory directory contents
2. Review any README or comments
3. Delete if scratch/abandoned work
4. Migrate if contains useful code
5. Document decision in this plan

**Estimated**: 30 minutes (review + decision)

---

### LOW PRIORITY - Optional Refactoring

#### LEG-05: Optionally Consolidate Preludes [1 SP]
**Files**: `src/prelude_dev.rs` (15 lines), `src/prelude_ez.rs` (18 lines)
**Status**: Working, small files, low priority
**Benefit**: Cleaner organization, not critical

**Options**:
- **Option A**: Keep as-is (low impact, already working)
- **Option B**: Move to `src/prelude/dev.rs` and `src/prelude/ez.rs`
- **Option C**: Collapse into main prelude.rs with conditional exports

**Recommendation**: DEFER - working code, minimal benefit

**Estimated**: 30 minutes if pursued

---

### DEFERRED - Complex Migration Requiring Deep Analysis

#### LEG-06: Audit & Consolidate streams/streamable [DEFERRED]
**Files**: `src/streams.rs` (405 lines, 43 exports) + `src/streams/` directory
**Conflict**: Both `src/streams/` AND `src/streamable/` modules exist
**Status**: **DEFERRED** - too complex, working code, needs architectural review

**Why Deferred**:
- Significant export count (43+ public items)
- Dual directory structure suggests incomplete migration
- Unknown dependency graph across codebase
- Risk of breaking existing functionality
- Requires architectural decision on consolidation strategy

**Future Investigation Required**:
- Document purpose of streams vs streamable
- Map all callers and dependencies
- Design consolidation strategy (which module wins?)
- Create comprehensive test plan
- Estimate 8-12 hours for safe migration

**Recommendation**: Leave for dedicated refactor sprint, not quick cleanup

---

## 🍎 Low-Hanging Fruit - Immediate Action Items

### Quick Win #1: LEG-04 (_todo/ directory)
**Effort**: 30 minutes
**Risk**: Minimal (likely scratch directory)
**Action**: Review and delete if abandoned

### Quick Win #2: LEG-03 (random.rs audit)
**Effort**: 1-2 hours
**Risk**: Low (backward compat re-exports)
**Action**: Check gx/ equivalents, deprecate if superseded

### Quick Win #3: Documentation of LEG-05 Decision
**Effort**: 5 minutes
**Risk**: None
**Action**: Document decision to keep prelude_dev/ez as-is

---

## 📋 Testing Strategy

**Problem Identified**: test.sh may not properly wire all feature tests

**Investigation Needed** (separate ticket):
- Verify test.sh can run all sanity/uat tests per module
- Document missing test lanes
- Add feature-specific test discovery

**Blocked By**: LEG-XX test discovery audit (not yet created)

---

## 🔍 Audit Methodology

### For Each Legacy File:

1. **List Exports**
   ```bash
   grep "pub fn\|pub struct\|pub enum\|pub macro" src/<file>.rs
   ```

2. **Find References**
   ```bash
   rg "use.*<module>::" --type rust
   rg "<function_name>" --type rust
   ```

3. **Compare with Modern Modules**
   - Check feature docs: `docs/tech/features/FEATURES_*.md`
   - Check module code: `src/<module>/mod.rs`
   - Use feat.py: `python3 bin/feat.py <module>`

4. **Classify Each Export**
   - ✅ **UNIQUE**: Migrate to proper module
   - ❌ **DUPLICATE**: Delete, update imports
   - ⚠️ **SUPERSEDED**: Deprecate, guide users to new API
   - ❓ **UNKNOWN**: Needs deeper investigation

5. **Create Migration Plan**
   - Document target module
   - List breaking changes
   - Write deprecation warnings
   - Update documentation

---

## 📊 Compliance Tracking

### Current State (2025-09-30)
- ✅ Compliant: 24 modules with proper structure
- ❌ Non-compliant: 7 root files + 4 directories
- 📊 Compliance: 77%

### Target State (After LEG-01 to LEG-05)
- ✅ Compliant: 27+ modules
- ❌ Non-compliant: 1-2 files (if any unique code)
- 📊 Compliance: 90%+

### Full Compliance (After LEG-06)
- ✅ Compliant: 100%
- ❌ Non-compliant: 0
- 📊 Compliance: 100%

---

## 🔗 Reference Documents

- **MODULE_SPEC**: `docs/tech/development/MODULE_SPEC.md`
- **TASKS**: `docs/procs/TASKS.txt` (LEG-XX tickets)
- **Features Index**: `docs/tech/INDEX.md`
- **Test Organization**: `docs/tech/development/TEST_ORGANIZATION.md`
- **How to Update**: `docs/tech/development/HOWTO_UPDATE_RSB.md`

---

## 📝 Decision Log

**2025-09-30**: Created LEGACY_UPGRADE_PLAN.md
- Identified 7 legacy files requiring audit
- Prioritized by risk and effort
- Deferred streams/streamable consolidation (too complex)
- Tagged low-hanging fruit (LEG-03, LEG-04)

**Next Review**: After LEG-01 and LEG-02 audits complete

---

## ⚠️ Critical Notes

1. **DO NOT BLINDLY DELETE** - Audit for duplicates first
2. **Test discovery issue** - test.sh may not run all feature tests properly
3. **Streams/streamable** - Deferred, needs architectural review
4. **Prelude files** - Working, low priority, can stay as-is
5. **_todo directory** - Unknown purpose, investigate before deleting

---

**Status**: Plan complete, tickets created, ready for execution
**Owner**: TBD (assign tickets in TASKS.txt)
**Estimated Total**: 11-15 SP (excluding LEG-06 deferred work)
