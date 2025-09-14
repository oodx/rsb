# RSB Issues & Defects Tracker

## Critical Issues (Blocking)
None currently tracked.

## Enhancement Requests

### ISSUE-003: Standard Options Expansion (stdopts)
Status: ✅ Completed (feature-gated). Moved to RSB_FIXED.

### ISSUE-004: Add Missing Bash Parameter Expansion Patterns
**Priority**: 🟢 **LOW**  
**Component**: `src/param/macros.rs` - `param!` macro  
**Status**: ✅ Completed  
**Impact**: Bash parity for covered patterns

**Implemented Patterns**:
- **`${VAR:+alt_value}`** → `param!("VAR", alt: "alt_value")` (returns alt when set and non-empty)
- **`${VAR:?error_message}`** → `param!("VAR", require: "error_message")` (stderr + returns empty)
- **`${VAR:offset:length}`** → `param!("VAR", sub: offset[, length])` (✅ supports negative indices)
- **`${#VAR}`** → `param!("VAR", len)`
- **`${VAR^pattern}`** → `param!("VAR", upper: "pattern")` (uppercase first char of first substring match)
- **`${VAR,pattern}`** → `param!("VAR", lower: "pattern")` (lowercase first char of first substring match)

**Current Available Patterns** (✅ Working or Partially):
- `${VAR}` → `param!("VAR")`
- `${VAR:-default}` → `param!("VAR", default: "default")`
- `${VAR:start:len}` → `param!("VAR", sub: start, len)` (✅ supports negative start/len)
- `${VAR/pattern/replacement}` → `param!("VAR", replace: "pattern" => "replacement")`
- `${VAR//pattern/replacement}` → `param!("VAR", replace: "pattern" => "replacement", all)`
- `${VAR^^}` → `param!("VAR", upper)`
- `${VAR,,}` → `param!("VAR", lower)`

Additional prefix/suffix patterns (glob-aware):
- `${VAR#pat}` / `${VAR##pat}` → `param!("VAR", prefix: pat[, longest])`
- `${VAR%pat}` / `${VAR%%pat}` → `param!("VAR", suffix: pat[, longest])`

---

### ISSUE-005: Add Wildcard Pattern Support for Prefix/Suffix Operations
**Priority**: 🟢 **LOW**  
**Component**: `src/string/helpers.rs` (used via `param::basic::{prefix,suffix}`)  
**Status**: ✅ Completed  
**Impact**: Enhanced bash compatibility

**Behavior**: Wildcards in prefix/suffix are supported, with shortest/longest selection.
```rust
param!("file.tar.gz", suffix: "*.gz")     // → "file.tar"
param!("src/main.rs", prefix: "*/")       // → "main.rs" (shortest), longest → "rs"
```

Test coverage:
- `tests/features/string/string_test.rs` (helpers; literals + wildcards)
- `tests/sanity.rs::test_prefix_suffix_wildcard_patterns` (param!)

---

## Infrastructure Issues

### ISSUE-006: XSed Integration for Advanced Text Processing  
**Priority**: 🟢 **LOW**  
**Component**: `src/macros/text.rs`  
**Status**: Open  
**Impact**: Enhanced text manipulation capabilities

**Enhancement**: Integrate XSed chainable transformations with param! macro
```rust
// Potential advanced syntax
param!("VAR", xsed: chain().replace("old", "new").upper().trim())
```

---

### ISSUE-007: Math Expression Evaluation in param!
**Priority**: 🟢 **LOW**  
**Component**: `src/macros/text.rs`, `src/math.rs`  
**Status**: Open  
**Impact**: Bash arithmetic expansion equivalent

**Enhancement**: Support arithmetic evaluation within param!
```rust
// Bash: ${#VAR} + 5
param!("VAR", len_plus: 5)
// Or more complex expressions
param!("NUM", math: "* 2 + 1")
```

---

## Testing Requirements

### ISSUE-008: Comprehensive param! Test Suite
**Priority**: 🟡 **MODERATE**  
**Component**: `tests/`  
**Status**: In Progress  
**Impact**: Quality assurance

**Required Tests**:
1. All bash parameter expansion patterns
2. Edge cases (empty strings, unicode, special characters)
3. Performance benchmarks for large strings
4. Wildcard pattern matching
5. Error handling (invalid patterns, etc.)
6. New case DSL: `param!(..., case: ...)`

---

## Documentation Issues

### ISSUE-009: param! Reference Documentation
**Priority**: 🟡 **MODERATE**  
**Component**: Documentation  
**Status**: Open  
**Impact**: Developer experience

**Missing Documentation**:
1. Complete param! pattern reference
2. Bash compatibility matrix
3. Performance characteristics
4. Migration guide from manual string operations
5. Integration examples with RSB ecosystem

---

*Last Updated: 2025-09-12*  
*RSB Version: 0.6.1*  
*Reporter: Automated param! comprehensive testing*

---

### ISSUE-010: Centralize String Errors Across Helpers
**Priority**: 🟢 **LOW**
**Component**: `src/string/`  
**Status**: Open  
**Impact**: Consistent user-facing messaging

**Enhancement**: Use `StringError` for string helper failures (e.g., regex compile) instead of ad-hoc messages. Ensure prefix/suffix wildcard paths surface RegexCompile as needed.

**Notes**: Case helpers already use `StringError::CaseInputTooLarge`.
