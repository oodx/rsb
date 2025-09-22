# RSB Visual System (MODERN + COMPREHENSIVE)

Updated: 2025-09-21

## Purpose
Provide a complete, feature-gated visual enhancement ecosystem for RSB applications. The visual system offers composable components—colors, glyphs, prompts, and progress indicators—through hierarchical Cargo feature flags that maintain build-time flexibility while providing runtime configurability.

## Module State (SPEC Alignment)
- **MODERN**: Complete visual ecosystem under `rsb::visual` namespace with focused subpackages
- **SPEC_ALIGNED**:
  - Curated API only (see "Public API" below); no wildcard prelude exports
  - Feature-gated modules with additive packages following dependency hierarchy
  - Tests present: sanity + UAT under feature gates (see HOWTO_TEST section)
  - Docs present: this comprehensive visual system guide

## Visual System Architecture

### Core Philosophy
The RSB visual system follows these principles:
- **Opt-in by design**: Nothing visual in prelude; explicit imports required
- **Hierarchical features**: Base `visual` feature enables focused subpackages
- **Runtime configurable**: Features compile capabilities; runtime controls behavior
- **Zero-impact when disabled**: Clean fallbacks preserve functionality
- **Composable**: Mix and match visual components as needed

### Feature Flag Hierarchy

#### Base Layer
```toml
colors-core = []                             # Standalone colors registry and runtime toggles
visual = ["colors-core"]                    # Visual macros/prompts wiring (imports colors-core)
```

#### Color System (Hierarchical)
```toml
colors-simple = ["colors-core"]             # 8/16 colors + control codes
colors-named = ["colors-core", "colors-simple"]
colors-status = ["colors-core"]
colors-all = ["colors-core", "colors-simple", "colors-named", "colors-status"]
colors = ["colors-core", "colors-simple"]  # Baseline convenience alias
```

#### Visual Components
```toml
glyphs = ["visual"]                          # Unicode glyphs for messaging
prompts = ["visual", "colors-simple"]       # Interactive prompts with colors
progress = []                                # Progress indicators (zero-dep core)
```

#### Umbrella Features
```toml
# Complete visual package
visuals = [
    "visual",
    "colors-simple", "colors-named", "colors-status",
    "glyphs", "prompts"
]

# Default includes essential visual features
default = ["visual", "colors-simple", "colors-named", "colors-status", "glyphs", "prompts"]
```

### Surface Cheat Sheet
- Colors: `SimpleColor`, `StatusColor`, `color_mode()`, `set_color_mode()`, `color_enable()`, `color_enable_with()`, `color()`, `get_color()`, `bg()`, `colorize()`, `colorize_bg()`, `colored()`, `get_all_colors()`, `colors_enabled()`, `set_backgrounds_enabled()`, `backgrounds_enabled()`, `get_named_colors()`, `get_boxy_extended_colors()`, `get_named_color()`, `colorize_named()`, `is_named_color()`, `get_simple_colors()`, `get_simple_color()`, `colorize_simple()`, `is_simple_color()`, `get_status_colors()`, `get_status_color()`, `get_status_color_categories()`, `colorize_status()`, `is_status_color()`, `glyph_enable()`, `set_glyphs_enabled()`, `glyphs_enabled()`, `get_all_glyphs()`, `from_name()`.
- Prompts: `confirm()`, `confirm_default()`, `ask()`, `select()`, `default_from()`, `confirm_with_timeout()`, `ask_with_timeout()`, `select_with_timeout()`, `confirm_default_with_timeout()`, `confirm_timeout!`, `ask_timeout!`, `select_timeout!`, `prompt_timeout!`, `confirm!`, `confirm_default!`, `ask!`, `select!`, `prompt!`.
- Visual macros & messaging: `colored!`, `info!`, `okay!`, `warn!`, `error!`, `fatal!`, `debug!`, `trace!`.
- Progress integration: see `FEATURES_PROGRESS.md` for `ProgressManager`, `ProgressStyle`, `ProgressStats`, etc.; the `visuals` feature pulls the progress package automatically.


## Component Overview

### 1. Colors System (`rsb::colors`)

**Purpose**: Runtime-configurable color system with string-first API

**Packages**:
- **colors-simple**: Basic 8/16 ANSI colors (red, green, blue, yellow, etc.)
- **colors-named**: Extended palette (crimson, azure, emerald, amber, etc.)
- **colors-status**: Status-specific colors (magic, trace, note, silly, success, warn, error)

**Runtime Model**:
```rust
use rsb::colors::{color_mode, color_enable_with, color, colorize};

// Configure color behavior
color_mode("auto");  // "auto" | "always" | "never"
color_enable_with("simple,status,named,bg");

// Use colors
println!("{}Warning{}", color("yellow"), color("reset"));
println!("{}", colorize("Success!", "green"));
```

**Environment Integration**:
- Respects `NO_COLOR` and `RSB_COLOR` environment variables
- Auto-detection based on TTY and terminal capabilities
- Runtime enable/disable without recompilation

### 2. Glyphs System (`rsb::visual::glyphs`)

**Purpose**: Unicode symbol lookup for enhanced CLI messaging

**Features**:
- 100+ predefined glyphs with semantic names
- Case-insensitive lookup (`glyph("PASS")` == `glyph("pass")`)
- Runtime enable/disable
- Categories: status, arrows, bullets, Greek letters, box drawing

**Usage**:
```rust
use rsb::visual::glyphs::{glyph_enable, glyph};

glyph_enable();
println!("{} Test passed", glyph("pass"));    // ✓ Test passed
println!("{} Loading", glyph("ellipsis"));     // … Loading
```

**Integration with Colors**:
```rust
use rsb::colored;

// Glyphs work seamlessly with color inline tags
println!("{}", colored("{green}{g:pass} Success{reset}"));
```

### 3. Prompts System (`rsb::visual::prompts`)

**Purpose**: Interactive CLI prompts with visual enhancements

**Features**:
- Confirmation prompts with customizable defaults
- Input prompts with validation
- Selection prompts (single/multiple choice)
- Timeout support for automation
- Automatic color integration when `colors-simple` enabled

**Examples**:
```rust
use rsb::visual::prompts::{confirm, prompt_input, select_option};

// Simple confirmation
if confirm("Continue with operation?") {
    println!("Proceeding...");
}

// Input with validation
let name = prompt_input("Enter name: ", |input| {
    if input.len() >= 3 { Ok(()) } else { Err("Name too short") }
});

// Selection prompt
let choice = select_option("Choose action:", &["Build", "Test", "Deploy"]);
```

### 4. Progress System (`rsb::progress`)

**Purpose**: Modular progress indicators for long-running operations

**Architecture**:
- **Zero-dependency core**: Uses only std library
- **Trait-based design**: Composable and extensible
- **Multiple styles**: Spinners, progress bars, percentage indicators
- **Framework-agnostic**: Can be extracted to standalone crate

**Usage**:
```rust
use rsb::progress::{ProgressManager, ProgressStyle};

let mut progress = ProgressManager::new();
let task = progress.start_task("Processing files", ProgressStyle::Bar { total: 100 });

for i in 0..100 {
    task.update(i + 1, &format!("Processing file {}", i + 1));
    std::thread::sleep(std::time::Duration::from_millis(50));
}

task.complete("All files processed successfully");
```

## Public API Reference

### Colors API
```rust
use rsb::colors::{
    // Configuration
    color_mode, color_enable, color_enable_with,

    // Basic color functions
    color, get_color, bg, colorize, colorize_bg,

    // Registry access
    get_all_colors, colored
};

// Macro (re-exported at crate root)
use rsb::colored;
// or explicitly opt-in via the module surface
use rsb::visual::macros::colored;
```

### Glyphs API
```rust
use rsb::visual::glyphs::{
    // Control functions
    glyph_enable, set_glyphs_enabled, glyphs_enabled,

    // Lookup functions
    glyph, get_all_glyphs
};
```

### Macro Surface (`rsb::visual::macros`)
```rust
// Opt-in macro imports (not part of the core prelude)
use rsb::visual::macros::{
    colored, info, warn, error, fatal, debug, trace, okay,
    confirm, confirm_default, confirm_timeout,
    ask, ask_timeout,
    select, select_timeout,
    prompt, prompt_timeout,
};

info!("Visual macros live under `visual::macros` now");
let answer = confirm!("Deploy to production?");
```

### Prompts API
```rust
use rsb::visual::prompts::{
    // Basic prompts
    confirm, confirm_with_default,
    prompt_input, prompt_input_with_default,

    // Selection prompts
    select_option, select_multiple,

    // Utilities
    prompt_with_timeout
};
```

### Progress API
```rust
use rsb::progress::{
    // Core types
    ProgressManager, ProgressTask, ProgressReporter,

    // Styles and configuration
    ProgressStyle, SpinnerStyle, BarStyle,
    TerminalReporter, TerminalConfig,

    // States and events
    ProgressState, ProgressEvent
};
```

## Integration Patterns

### 1. Full Visual Integration
```rust
use rsb::colors::{color_enable_with, colorize};
use rsb::visual::glyphs::{glyph_enable, glyph};
use rsb::visual::prompts::confirm;
use rsb::progress::{ProgressManager, ProgressStyle};

// Enable all visual features
color_enable_with("simple,named,status,bg,glyphs");
glyph_enable();

// Use integrated visuals
println!("{} {}", glyph("gear"), colorize("Initializing system", "blue"));

if confirm("Ready to proceed?") {
    let mut progress = ProgressManager::new();
    let task = progress.start_task("Processing", ProgressStyle::Spinner);
    // ... work ...
    task.complete(&format!("{} Complete!", glyph("pass")));
}
```

### 2. Selective Feature Usage
```rust
// Only enable colors and prompts
#[cfg(feature = "colors-simple")]
use rsb::colors::{color_enable, colorize};

#[cfg(feature = "prompts")]
use rsb::visual::prompts::confirm;

#[cfg(feature = "colors-simple")]
fn colored_output(msg: &str) -> String {
    colorize(msg, "green")
}

#[cfg(not(feature = "colors-simple"))]
fn colored_output(msg: &str) -> String {
    msg.to_string()
}
```

### 3. Runtime Configuration
```rust
use rsb::colors::{color_mode, color_enable_with};
use rsb::visual::glyphs::{set_glyphs_enabled};

// Configure based on environment or user preferences
let use_colors = std::env::var("FORCE_COLOR").is_ok() || atty::is(atty::Stream::Stdout);
let use_glyphs = std::env::var("RSB_GLYPHS").map_or(true, |v| v != "0");

if use_colors {
    color_mode("always");
    color_enable_with("simple,status");
} else {
    color_mode("never");
}

set_glyphs_enabled(use_glyphs);
```

## Testing Approach

### Feature-Gated Tests
Tests are organized to respect feature boundaries:

```rust
#[cfg(feature = "colors-simple")]
mod color_tests {
    use rsb::colors::*;

    #[test]
    fn test_basic_colors() {
        color_mode("always");
        color_enable_with("simple");
        assert!(!color("red").is_empty());
    }
}

#[cfg(feature = "glyphs")]
mod glyph_tests {
    use rsb::visual::glyphs::*;

    #[test]
    fn test_glyph_lookup() {
        glyph_enable();
        assert_eq!(glyph("pass"), "✓");
    }
}
```

### Test Runner Integration
- **Core tests**: `cargo test` (excludes visual features)
- **Visual tests**: `cargo test --features visuals`
- **Specific features**: `cargo test --features colors-simple,glyphs`
- **RSB test runner**:
  - `./bin/test.sh run colors` — color-specific tests
  - `./bin/test.sh run visuals` — all visual tests
  - `./bin/test.sh run uat` — user acceptance tests with visuals

### Mock and Fallback Testing
```rust
// Test graceful degradation when features disabled
#[test]
fn test_color_fallback() {
    use rsb::colors::colorize;

    // Should return original text when colors disabled
    let result = colorize("test", "red");
    // Behavior depends on runtime configuration
}
```

## Environment Integration

### Environment Variables
- **NO_COLOR**: Disables all color output (standard)
- **RSB_COLOR**: Override color mode ("always", "never", "auto")
- **RSB_COLORS**: Runtime color set specification
- **RSB_GLYPHS**: Enable/disable glyphs ("1"/"0")

### Terminal Detection
```rust
// Auto-detection logic
fn should_use_colors() -> bool {
    if std::env::var("NO_COLOR").is_ok() { return false; }
    if let Ok(mode) = std::env::var("RSB_COLOR") {
        return mode == "always" || (mode == "auto" && is_tty());
    }
    is_tty()  // Default to TTY detection
}
```

## Migration from FEATURES_COLORS.md

This document **replaces** the outdated FEATURES_COLORS.md. Key improvements:

### What's New
1. **Complete visual ecosystem coverage**: Beyond just colors
2. **Progress indicators**: New modular progress system
3. **Integration patterns**: How components work together
4. **Comprehensive API reference**: All visual namespaces
5. **Testing strategy**: Feature-gated testing approach
6. **Runtime configuration**: Environment and programmatic control

### Migration Path
- Replace imports: `use rsb::colors::*` (still works)
- New umbrella feature: Use `visuals` for complete visual package
- Progress indicators: New `rsb::progress` namespace
- Enhanced glyphs: More symbols and better integration

### Backward Compatibility
All existing color APIs remain unchanged. This document extends rather than breaks existing usage.

## RSB MODULE_SPEC Compliance

### ✅ Specification Requirements Met
- **Curated API**: No wildcard exports; explicit imports required
- **Feature gates**: Hierarchical feature system with clear dependencies
- **Test coverage**: Feature-gated tests with proper runner integration
- **Documentation**: Comprehensive guide following RSB patterns
- **Zero prelude**: Visual components excluded from prelude by policy

### Architecture Alignment
- **Namespace organization**: `rsb::visual::*` and `rsb::progress`
- **Runtime configurability**: Environment integration and programmatic control
- **Composable design**: Mix and match components as needed
- **Fallback behavior**: Graceful degradation when features disabled

### Development Integration
- **Test runner lanes**: Integration with `./bin/test.sh` system
- **Feature validation**: Proper feature flag hierarchy
- **Documentation linking**: Connected to INDEX.md and development guides

---

**Next Steps**: After integration, update INDEX.md to link to this comprehensive visual guide and consider deprecation notice for FEATURES_COLORS.md.

<!-- feat:visuals -->

_Generated by bin/feat.py --update-doc._

* `src/visual/colors/mod.rs`
  - pub use named::* (line 32)
  - pub use simple::* (line 38)
  - pub use {named::*, status::*} (line 41)

* `src/visual/colors/named.rs`
  - pub use super::simple::* (line 7)
  - fn get_named_colors (line 11)
  - fn get_boxy_extended_colors (line 22)
  - fn get_named_color (line 111)
  - fn colorize_named (line 208)
  - fn is_named_color (line 218)

* `src/visual/colors/registry.rs`
  - fn color_enable (line 259)
  - fn color_enable_with (line 264)
  - fn color_mode (line 269)
  - fn color (line 274)
  - fn get_color (line 288)
  - fn colorize (line 293)
  - fn bg (line 329)
  - fn colorize_bg (line 343)
  - fn colored (line 353)
  - fn get_all_colors (line 417)

* `src/visual/colors/simple.rs`
  - enum SimpleColor (line 13)
  - fn from_name (line 131)
  - fn all (line 174)
  - fn get_simple_colors (line 211)
  - fn get_simple_color (line 219)
  - fn colorize_simple (line 226)
  - fn is_simple_color (line 236)

* `src/visual/colors/status.rs`
  - enum StatusColor (line 12)
  - fn from_name (line 155)
  - fn all (line 206)
  - fn get_status_colors (line 250)
  - fn get_status_color (line 258)
  - fn colorize_status (line 265)
  - fn is_status_color (line 275)
  - fn get_status_color_categories (line 280)

* `src/visual/colors/util.rs`
  - fn set_color_mode (line 10)
  - fn colors_enabled (line 18)
  - fn set_backgrounds_enabled (line 30)
  - fn backgrounds_enabled (line 34)

* `src/visual/glyphs/mod.rs`
  - fn glyph_enable (line 121)
  - fn set_glyphs_enabled (line 124)
  - fn glyphs_enabled (line 127)
  - fn glyph (line 132)
  - fn get_all_glyphs (line 145)

* `src/visual/macros.rs`
  - macro colored! (line 11)
  - macro info! (line 24)
  - macro okay! (line 26)
  - macro warn! (line 28)
  - macro error! (line 30)
  - macro fatal! (line 32)
  - macro debug! (line 34)
  - macro trace! (line 36)
  - macro confirm! (line 41)
  - macro confirm_default! (line 49)
  - macro ask! (line 57)
  - macro select! (line 68)
  - macro prompt! (line 79)
  - macro confirm_timeout! (line 99)
  - macro ask_timeout! (line 113)
  - macro select_timeout! (line 127)
  - macro prompt_timeout! (line 146)

* `src/visual/mod.rs`
  - pub use colors::simple::* (line 49)
  - pub use colors::named::* (line 52)
  - pub use colors::status::* (line 55)
  - pub use glyphs::* (line 58)
  - pub use prompts::* (line 61)
  - pub use utils::* (line 65)
  - pub use crate::{colored, debug, error, fatal, info, okay, trace, warn} (line 68)

* `src/visual/prompts/interactive.rs`
  - fn confirm_default (line 24)
  - fn confirm (line 61)
  - fn ask (line 89)
  - fn select (line 111)
  - fn default_from (line 165)

* `src/visual/prompts/mod.rs`
  - pub use interactive::{ask, confirm, confirm_default, default_from, select} (line 40)

* `src/visual/prompts/utils.rs`
  - fn confirm_with_timeout (line 74)
  - fn confirm_default_with_timeout (line 92)
  - fn ask_with_timeout (line 112)
  - fn select_with_timeout (line 133)

<!-- /feat:visuals -->

