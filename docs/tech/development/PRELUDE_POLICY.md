# RSB Prelude Policy

## Overview

The RSB prelude (`rsb::prelude::*`) provides a curated set of commonly-used traits, functions, and macros for easy importing. This document defines the policy for what should and should not be included in the prelude.

## Core Principles

### 1. Minimize Import Pollution
The prelude should only include items that are:
- Used in >80% of RSB applications
- Non-conflicting with common std library names
- Essential for basic RSB functionality

### 2. Feature Gating Strategy
Optional features MUST NOT leak into the prelude:
- ✅ **Core functionality**: Always available
- ❌ **Visual macros**: Require explicit import (`rsb::visual::*`)
- ❌ **Optional streamables**: Behind feature flags
- ❌ **Experimental features**: Never in prelude

### 3. Predictable Imports
Users should be able to trust that `use rsb::prelude::*` will not:
- Pull in heavy dependencies unexpectedly
- Conflict with standard library names
- Change behavior based on feature flags

## What's Included in Prelude

### Core Types & Traits
- `Args` - CLI argument handling
- `Streamable`, `StreamApply` - Core streaming traits
- `Stream` - Main streaming type

### Essential Functions
- Global state: `get_var`, `set_var`, `has_var`, etc.
- Config handling: `load_config_file`, `save_config_file`
- Variable expansion: `expand_vars`, `export_vars`
- String utilities: `str_sub`, `str_prefix`, `str_suffix`, etc.

### Common Macros
- File operations: `cat`, `backup`, `chmod`, `tmp`
- System interaction: `cmd`, `run`, `shell`, `hostname`
- Validation: `require_file`, `require_dir`, `validate`
- Data processing: `param`, `json_get`, `dict`

### Module Utilities
- Date utilities: `date_utils`
- String utilities: `string_utils`
- Math functions: `math::*`
- Random generators: `random::*`

## What's Excluded from Prelude

### Visual Components (Feature-Gated)
```rust
// ❌ NOT in prelude - requires explicit import
use rsb::visual::{colored!, info!, warn!, error!};
use rsb::visual::colors::*;
use rsb::visual::prompts::{confirm, ask, select};
```

### Advanced Streamables
```rust
// ❌ NOT in prelude - opt-in only
use rsb::streamable::{AdvancedFilter, ComplexTransform};
```

### Experimental Features
```rust
// ❌ NOT in prelude - unstable API
use rsb::experimental::*;
```

## Feature Flag Guidelines

### Required Patterns
All optional features MUST follow this pattern:

```rust
// In lib.rs
#[cfg(feature = "visual")]
pub mod visual;

// In prelude.rs - explicit exclusion comment
// Note: Visual macros (colored!, info!, etc.) are NOT re-exported in the prelude.
// Visual and other optional packages are opt-in via explicit imports.
```

### Cargo.toml Structure
```toml
[features]
default = []
# Base feature
visual = []
# Hierarchical features
colors-simple = ["visual"]
colors-named = ["visual", "colors-simple"]
colors-all = ["visual", "colors-simple", "colors-named", "colors-status"]
```

## Usage Patterns

### Recommended Basic Import
```rust
use rsb::prelude::*;
// Gets: core types, essential functions, common macros
```

### With Visual Features
```rust
use rsb::prelude::*;
use rsb::visual::*;  // Explicit opt-in to visual features
```

### Module-Specific Access
```rust
use rsb::prelude::*;
use rsb::string::utils::*;  // Access full module utilities
use rsb::date::*;           // Access full date module
```

## Maintenance Guidelines

### Adding to Prelude
Before adding any item to the prelude, verify:
1. **Usage frequency**: Used in majority of RSB applications
2. **No conflicts**: Doesn't shadow std library items
3. **No dependencies**: Doesn't require optional features
4. **Stable API**: Won't change frequently

### Removing from Prelude
Breaking changes to prelude require:
1. **Deprecation period**: One minor version with deprecation warning
2. **Migration guide**: Clear instructions for users
3. **Feature flag**: Temporary flag to enable old behavior

## Examples

### ✅ Good Prelude Candidates
```rust
pub use crate::fs::*;           // Core file operations
pub use crate::utils::*;        // Essential utilities
pub use crate::global::*;       // Global state management
```

### ❌ Bad Prelude Candidates
```rust
// Feature-gated items
#[cfg(feature = "visual")]
pub use crate::visual::*;       // Breaks feature isolation

// Experimental items
pub use crate::experimental::*; // Unstable API

// Heavy dependencies
pub use crate::database::*;     // Most apps won't use
```

## Verification

### Build Tests
The prelude policy is enforced through build tests:
```bash
# Default build should work without optional features
cargo test

# Visual features should be isolated
cargo test --no-default-features
cargo test --features visual
```

### Documentation
All prelude items must have:
- Clear documentation
- Usage examples
- Feature requirements noted

---

This policy ensures RSB's prelude remains lightweight, predictable, and user-friendly while maintaining clean feature separation.