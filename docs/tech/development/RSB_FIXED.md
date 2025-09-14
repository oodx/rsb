# RSB Fixes - September 2025

## Context
ProntoDB rebuild required fixing several critical RSB issues that prevented proper RSB compliance. These fixes restore Args-first consistency and complete the BashFX declarative patterns.

## Fixed Issues

### 1. Bootstrap Args Type Inconsistency
**Problem**: `bootstrap!()` macro returned `Vec<String>` instead of RSB `Args` struct
**Location**: `src/macros/core.rs:16`
**Fix**: Changed return from raw args vector to `Args::new(&args)`
**Impact**: Enables consistent Args usage throughout RSB ecosystem

### 2. Dispatch Macro Args Mismatch  
**Problem**: `dispatch!` and `pre_dispatch!` macros expected `&Vec<String>` instead of `&Args`
**Location**: `src/macros/dispatch.rs`
**Fix**: Updated both macros to accept `&Args` parameter
**Impact**: Consistent Args-first pattern across all RSB dispatch mechanisms

### 3. Missing Options Macro
**Problem**: No `options!()` macro for declarative flag processing
**Location**: `src/macros/core.rs:44-82`  
**Implementation**: Added BashFX-compliant declarative flag parser
**Features**:
- Long options: `--verbose`, `--config=value`
- Short options: `-d`, `-v` (boolean only)
- Explicit value syntax: `--flag=value` (no smart consumption)
- Global context storage via `set_var()`
- Path validation for options containing "path" or "file"
- Dash-to-underscore normalization: `--some-flag` → `opt_some_flag`

### 4. Missing App Reference Macro
**Problem**: No convenient way to get program name (arg 0)
**Location**: `src/macros/core.rs:28-32`
**Implementation**: Added `appref!()` macro returning program name

### 5. Token Stream Validation Helper
**Problem**: Need validation for token stream patterns without parsing
**Location**: `src/context.rs` (inferred from usage)
**Implementation**: `is_token_stream()` function validates comma/semicolon delimited patterns
**Formats**: 
- Comma-separated: `k1=v1,k2=v2` (shell-safe)
- Semicolon-separated: `k1=v1;k2=v2` (requires quoting)

## Architecture Principles Restored

### Args-First Consistency
All RSB macros now use consistent `Args` struct instead of mixing `Vec<String>` and `Args`

### BashFX Declarative Patterns
- No "smart" flag consumption logic
- Explicit `--flag=value` syntax enforced
- Declarative processing: "if it looks like a flag, treat it as one"
- Global context integration via `set_var()`/`get_var()`

### REBEL Philosophy Compliance
- Anti-ceremony approach maintained
- Accessible naming over academic Rust patterns
- Function ordinality preserved in command handlers

## Testing Validation
All fixes validated with ProntoDB integration tests:
- Bootstrap → Options → Dispatch flow working
- Flag processing with global context storage
- Token stream recognition and validation
- Command routing to proper handlers

## Export Updates
Updated `src/prelude.rs` to include new macros:
- `bootstrap` (fixed)
- `options` (new)
- `appref` (new)
- Dispatch macros (fixed)

---

## Visual Package Enhancements (September 12, 2025)

### Ergonomic Colors + Registry
- Added progressive, string-first color registry with runtime enablement:
  - `color_enable[_with]`, `color_mode`, `color`, `colorize`, `colored`, `get_all_colors`
- Feature-gated sets: simple, named, status; convenience features: `colors`, `colors-all`, `visuals`
- Honors NO_COLOR and `RSB_COLOR=auto|always|never`
- Removed legacy context color expander and stale refs

### Background Colors
- Added background toggle and helpers:
  - `bg`, `colorize_bg`, `{bg:name}` inline tag
- 8/16 and 256-color support via code transformation (38→48, 30–37→40–47, 90–97→100–107)

### Glyphs (Full Set)
- Lightweight glyphs package (case-insensitive string lookup) with full set ported from ref/glyphs.rs (+ box ‘■’)
- Runtime toggle: `glyph_enable()`; inline `{g:name}` tags supported
- `glyph_stderr` switched to non-emoji glyphs by level (info ◎, okay ✓, warn △, error ✕, debug ↯, trace …)

### Macros Split (Optional Visual Macros)
- Core (always): `readline!`, `stderr!`, `echo!`, `printf!`
- Visual (feature `visual`): `colored!`, `info!`, `okay!`, `warn!`, `error!`, `fatal!`, `debug!`, `trace!`
- Prelude re-exports visual macros only when visual is enabled

### Stdopts (Feature-Gated)
- Short-flag expansion behind `stdopts` feature in `options!`:
  - `-d/-q/-t/-D/-y/-s` → `opt_debug/opt_quiet/opt_trace/opt_dev_mode/opt_yes/opt_safe`

### Tests and Runner
- Added UAT tests with visible output:
  - `uat-colors`, `uat-glyphs`, `uat-visual` (bg + color + glyphs)
  - `stdopts` (feature-gated)
- `bin/test.sh` updated with new targets and feature flags

### Impact
- Cleaner API, minimal ceremony, optional footprint via features
- Backward-compatible defaults; visible demos for UAT
