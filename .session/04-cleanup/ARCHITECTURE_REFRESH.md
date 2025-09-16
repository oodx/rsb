# Architecture Refresh for Next Tasks

## KEY MODULE_SPEC PATTERNS (Essential for RSB-004 & RSB-007)

### Module Structure:
- `<module>/mod.rs` - orchestrator, curated public surface
- `<module>/utils.rs` - low-level helpers (explicit opt-in)
- `<module>/helpers.rs` - internal implementations
- `<module>/macros.rs` - thin macros delegating to functions
- `<module>/error.rs` - typed errors

### CRITICAL: Thin Macro Pattern
```rust
// ❌ BAD: Business logic in macro
macro_rules! confirm { /* lots of logic */ }

// ✅ GOOD: Thin macro delegates to function
macro_rules! confirm {
    ($msg:expr) => { $crate::visual::prompts::confirm($msg) }
}
```

## VISUAL FEATURE ARCHITECTURE (for RSB-004 Prompts)

### Feature Flag Hierarchy:
- `visual` - Base feature for all visual components
- `colors-simple` - Basic 8-16 colors (required for prompts)
- `prompts` - Interactive prompts (depends on colors-simple)

### Usage Pattern:
```rust
// NOT in prelude - explicit import required
use rsb::visual::prompts::{confirm, ask, select};

#[cfg(feature = "prompts")]
fn interactive_mode() {
    let answer = confirm("Continue?").unwrap();
}
```

### Prompts Must Handle:
1. **opt_yes/opt_quiet** - Global flags override interaction
2. **Non-TTY fallback** - CI/automation compatibility
3. **Deterministic behavior** - Predictable in testing

## MACRO AUDIT TARGETS (for RSB-007)

### Common Macro Categories:
- **fs/file operations**: `cat!`, `backup!`, `chmod!`
- **text processing**: `sed_*!`, `str_*!`
- **time/sleep**: `sleep!`, date formatting
- **random**: `rand_*!` series
- **validation**: `require!`, `validate!`, `test!`
- **control flow**: `case!`, `with_lock!`
- **metadata**: `meta_keys!`, path operations

### Validation Needed:
- All macros delegate to helper functions
- Error handling is consistent
- Global context integration works
- Test coverage exists

## PRELUDE POLICY (Recently Established)

### IN PRELUDE:
- Core Args, global functions, common macros
- Essential streamables and utilities
- Module utils: `string_utils`, `date_utils`

### NOT IN PRELUDE:
- Visual macros (`colored!`, `info!`, etc.)
- Optional features behind flags
- Advanced/specialized functions

## OPTIONS PROCESSING (Context from Session)

### Flag Processing:
```rust
--i-am-sure → opt_i_am_sure = "1"  // dash to underscore + opt_ prefix
--config=val → opt_config = "val"  // equals syntax
--not-verbose → opt_verbose = "0"  // negation pattern
```

This architecture refresh captures the essential patterns needed for the next tasks.