# HOWTO: Hub Integration Guide

## Overview
Hub is a centralized dependency management system for the oodx/RSB ecosystem that uses feature flags to provide modular, conflict-free dependency management with clean namespace separation between internal and external dependencies.

## Namespace Philosophy ⚠️ Major Update in v0.3.0

Hub now enforces **clean namespace separation**:
- **Top-level namespace**: Reserved exclusively for internal oodx/rsb modules
- **External dependencies**: All third-party packages get the `-ext` suffix
- **Philosophy**: The `-ext` suffix means "we don't like these third-party packages but use them if we have to"

### Internal vs External Structure
- **Internal** (top-level): `hub::colors` - our own shared infrastructure
- **External** (with modules): `hub::text_ext`, `hub::data_ext` - third-party dependencies grouped by domain

## Quick Integration

### 1. Add Hub to Your Project

#### Primary Method: GitHub Repository (Recommended)
```toml
# Cargo.toml
[dependencies]
hub = { git = "https://github.com/oodx/hub.git", features = ["regex", "serde"] }
```

#### Secondary Method: Local Path (Emergency/Hot-fixes Only)
⚠️ **Use only when you have urgent local fixes that cannot wait for hub to publish**
```toml
# Cargo.toml - FOR EMERGENCY USE ONLY
hub = { path = "../../hub", features = ["regex", "serde"] }
```

### 2. Update Your Imports
```rust
// External dependencies (third-party)
use regex::Regex;                    // ❌ Before
use hub::regex::Regex;               // ✅ After (top-level re-export)
use hub::text_ext::regex::Regex;     // ✅ After (grouped module)

use serde::{Serialize, Deserialize};         // ❌ Before
use hub::serde::{Serialize, Deserialize};    // ✅ After (top-level re-export)
use hub::data_ext::serde::{Serialize, Deserialize}; // ✅ After (grouped module)

// Internal dependencies (our own infrastructure)
use hub::colors;                     // ✅ Internal oodx/rsb module

// Or use the prelude for common external features
use hub::prelude::*;
```

## Feature Selection Strategy

### Individual Features
Specify exactly what you need:
```toml
features = ["regex", "serde", "chrono", "uuid"]  # Individual external deps
```

### External Domain Groups (Third-party - Use If We Have To)

**⚠️ NEW: Lite/Full Variant System**
Starting with hub v0.4.0, major packages now provide both lite and full variants for optimal project control:

#### Domain Groups with Lite Defaults
- **`text-ext`** - Text processing: regex, lazy_static, unicode-width, strip-ansi-escapes
- **`data-ext`** - Serialization: serde, serde_json, base64, serde_yaml (deprecated)
- **`time-ext`** - Date/time: **chrono-lite**, uuid
- **`web-ext`** - Web utilities: urlencoding
- **`system-ext`** - System access: libc, glob
- **`terminal-ext`** - Terminal tools: portable-pty
- **`random-ext`** - Random generation: rand
- **`async-ext`** - Asynchronous programming: **tokio-lite**
- **`cli-ext`** - Command line tools: **clap-lite**, anyhow
- **`error-ext`** - Error handling: anyhow, thiserror
- **`test-ext`** - Testing utilities: criterion, tempfile

#### Major Package Variants

**Tokio (Async Runtime)**
- **`tokio`** / **`tokio-lite`** = `["rt", "macros"]` - Basic async runtime with macros
- **`tokio-full`** = `["full"]` - Complete tokio (net, fs, process, signal, sync, etc.)

**Clap (CLI Parser)**
- **`clap`** / **`clap-lite`** = `["std", "help"]` - Basic CLI argument parsing
- **`clap-full`** = `["std", "help", "derive", "env", "unicode"]` - Full-featured with derive macros

**Chrono (Date/Time)**
- **`chrono`** / **`chrono-lite`** = `["clock", "std"]` - Basic date/time functionality
- **`chrono-full`** = `["clock", "std", "serde"]` - With serialization support

### External Convenience Groups
- **`common-ext`** - Most used external: text-ext + data-ext + error-ext
- **`core-ext`** - Essential external: text-ext + data-ext + time-ext + error-ext
- **`extended-ext`** - Comprehensive external: core-ext + web-ext + system-ext + cli-ext
- **`dev-ext`** - Everything external (mega package for testing/development)

### Internal oodx/rsb Groups (Top-level Namespace Reserved)
- **`core`** - Internal core: colors (shared color system)
- **`colors`** - Shared color system from jynx architecture

### The `-ext` Philosophy
The `-ext` suffix on external features embodies our approach to third-party dependencies:
- **Namespace Separation**: Clear distinction between our code and external code
- **Reluctant Usage**: "We don't like these third-party packages but use them if we have to"
- **Controlled Integration**: External dependencies are grouped and managed, not embraced
- **Future-Proofing**: Makes it easy to replace external deps with internal alternatives

### The Lite/Full Variant Philosophy
Hub's lite/full system provides optimal balance between lean defaults and feature completeness:

#### Lite Variants (Default in Domain Groups)
- **Lean by Design**: Minimal feature sets with essential functionality only
- **Fast Compilation**: Reduced build times with fewer features to compile
- **Smaller Binaries**: Optimized for size-conscious applications
- **Minimal Dependencies**: Fewer transitive dependencies to manage
- **Safe Defaults**: Domain groups (`async-ext`, `cli-ext`, `time-ext`) use lite variants automatically

#### Full Variants (Opt-in Power)
- **Complete Functionality**: All features enabled for maximum capability
- **Cherry-picking**: Select `tokio-full`, `clap-full`, `chrono-full` individually when needed
- **Power User Features**: Advanced functionality like derive macros, networking, serialization
- **Development Convenience**: Full feature sets for prototyping and development

#### Project Control Strategy
- **Start Lite**: Domain groups provide lean, fast-compiling defaults
- **Extend Selectively**: Projects can add their own additional features on top of hub's lite defaults
- **Override When Needed**: Replace lite with full variants for specific packages requiring heavy features
- **Compose Flexibly**: Mix lite and full variants based on actual project requirements

### Migration from Legacy Features
Old feature names still work for backward compatibility:
```toml
# Legacy (still works)
features = ["text", "data", "core"]

# New structure (recommended)
features = ["text-ext", "data-ext", "core"]
```

## Hub Inclusion Criteria

### Usage-Based Inclusion
- **3+ projects using a dependency**: Eligible for hub inclusion (manual review)
- **5+ projects using a dependency**: Automatic inclusion by blade tools
- **Semantic versioning propagation**: Hub version updates reflect dependency changes

### Version Management Philosophy
Hub follows strict semantic versioning:
- **Minor version bump**: When any dependency has a minor version change
- **Major version bump**: When any dependency has a major version change
- This ensures downstream projects can trust semantic versioning for updates

## Integration Methods

### When to Use Each Method

#### GitHub Repository (Primary - Recommended)
✅ **Use for all standard development**
- Ensures you get the latest stable version
- Maintained and tested hub distribution
- Consistent with other projects in the ecosystem
- Proper semantic versioning

#### Local Path (Secondary - Emergency Only)
⚠️ **Use only when:**
- You have urgent hot-fixes that cannot wait for hub publishing
- You are actively developing hub features for testing
- You need immediate access to unpublished changes

⚠️ **Warnings for local path usage:**
- May introduce version inconsistencies across projects
- Requires manual coordination with hub updates
- Not suitable for production deployments
- Should be temporary - migrate to GitHub repo when fixes are published

## Integration Examples

### Basic Project Setup with Internal Features
```toml
[dependencies]
hub = { git = "https://github.com/oodx/hub.git", features = ["core", "core-ext"] }
# Gets you:
# - Internal: colors (shared color system)
# - External: text-ext + data-ext + time-ext + error-ext
```

### Web Service Project
```toml
[dependencies]
hub = { git = "https://github.com/oodx/hub.git", features = ["extended-ext", "random-ext"] }
# Gets you: comprehensive external capabilities + random generation
```

### Development Tools Project
```toml
[dependencies]
hub = { git = "https://github.com/oodx/hub.git", features = ["dev-ext"] }
# Gets you: ALL external packages (mega package for testing/development)
```

### Testing/Development Project (Mega Package)
```toml
[dependencies]
hub = { git = "https://github.com/oodx/hub.git", features = ["dev-ext"] }
# The dev-ext mega package includes ALL external dependencies:
# text-ext, data-ext, time-ext, web-ext, system-ext, terminal-ext,
# random-ext, async-ext, cli-ext, error-ext, test-ext
#
# Perfect for:
# - Integration testing across the ecosystem
# - Development environments where you need everything
# - Prototyping without worrying about specific feature selection
# - CI/CD pipelines that run comprehensive tests
```

### Production Service (Selective Features)
```toml
[dependencies]
hub = { git = "https://github.com/oodx/hub.git", features = ["core", "core-ext", "async-ext"] }
# Gets you:
# - Internal: colors (shared oodx/rsb infrastructure)
# - External: essential text/data/time/error handling + async capabilities
# - Clean separation between internal infrastructure and external utilities
```

### CLI Tool Development
```toml
[dependencies]
hub = { git = "https://github.com/oodx/hub.git", features = ["cli-ext", "error-ext", "terminal-ext"] }
# Focused on command-line tools:
# - clap-lite for basic argument parsing (from cli-ext)
# - anyhow/thiserror for error handling
# - portable-pty for terminal interaction
```

## Lite/Full Variant Usage Examples

### Lean Project with Lite Defaults
```toml
[dependencies]
hub = { git = "https://github.com/oodx/hub.git", features = ["async-ext", "cli-ext", "time-ext"] }
# Gets you efficient defaults:
# - tokio-lite: basic async runtime ["rt", "macros"]
# - clap-lite: simple CLI parsing ["std", "help"]
# - chrono-lite: core date/time ["clock", "std"]
# Perfect for: lightweight services, simple CLI tools, quick prototypes
```

### Power User with Selective Full Features
```toml
[dependencies]
hub = { git = "https://github.com/oodx/hub.git", features = ["async-ext", "clap-full", "time-ext"] }
# Mixed approach:
# - tokio-lite: basic async (sufficient for most use cases)
# - clap-full: advanced CLI with derive macros and env support
# - chrono-lite: basic date/time (no serialization overhead)
# Perfect for: CLI tools needing advanced argument parsing but basic async
```

### Full-Featured Development Setup
```toml
[dependencies]
hub = { git = "https://github.com/oodx/hub.git", features = ["tokio-full", "clap-full", "chrono-full", "data-ext"] }
# Maximum capability:
# - tokio-full: complete async ecosystem ["full"]
# - clap-full: all CLI features including derive macros
# - chrono-full: date/time with serde serialization
# - data-ext: serde, serde_json for data handling
# Perfect for: complex services, full-stack applications, development environments
```

### Microservice with Network Requirements
```toml
[dependencies]
hub = { git = "https://github.com/oodx/hub.git", features = ["tokio-full", "data-ext", "cli-ext"] }
# Network-focused:
# - tokio-full: includes networking, file system, process management
# - data-ext: JSON serialization capabilities
# - cli-ext: basic command line support (clap-lite)
# Perfect for: web services, network applications, API servers
```

### Project Extension Pattern
```toml
# Start with lite defaults from domain groups
[dependencies]
hub = { git = "https://github.com/oodx/hub.git", features = ["async-ext", "time-ext"] }

# Later, your project can extend specific packages with additional features
# by directly depending on them with extra features:
tokio = { version = "1.0", features = ["net", "fs"] }  # Add networking to tokio-lite base
chrono = { version = "0.4", features = ["serde"] }    # Add serialization to chrono-lite base

# Hub provides the base, your project adds the extras as needed
# This gives you control over exactly which features to enable
```

### Build Size Comparison Examples

#### Lite Setup (Fast Builds)
```toml
features = ["async-ext", "cli-ext"]
# Compilation: ~30% faster than full variants
# Binary size: ~40% smaller than full variants
# Transitive deps: ~50% fewer than full variants
# Perfect for: development iteration, CI/CD, resource-constrained environments
```

#### Full Setup (Maximum Features)
```toml
features = ["tokio-full", "clap-full"]
# Compilation: Longer but includes all functionality
# Binary size: Larger but feature-complete
# Transitive deps: More dependencies but maximum capability
# Perfect for: production services needing full feature sets
```

## Benefits

### For Your Project
✅ **No version conflicts** - All projects use same dependency versions
✅ **Cleaner Cargo.toml** - No external dependency management
✅ **Faster builds** - Cargo deduplicates dependencies efficiently + lite variants compile faster
✅ **Easy upgrades** - Hub manages all version updates centrally
✅ **Optimal performance** - Lite variants reduce binary size and compilation time
✅ **Flexible scaling** - Start lite, upgrade to full variants only when needed
✅ **Project control** - Add your own features on top of hub's lean defaults

### For the Ecosystem
✅ **Coordinated updates** - Single place to manage all dependency versions
✅ **Security scanning** - Centralized vulnerability management
✅ **Consistency** - Same behavior across all projects
✅ **Reduced bloat** - Lite variants eliminate unused features by default
✅ **Performance optimization** - Faster CI/CD with lean default builds
✅ **Resource efficiency** - Smaller memory footprint for lite deployments

## Migration Checklist

### For New Projects
1. **Add hub dependency** using GitHub repo with lite/full aware features:
   ```toml
   # Start with lite defaults for fast builds
   hub = { git = "https://github.com/oodx/hub.git", features = ["core-ext", "async-ext"] }

   # Or choose specific full variants when needed
   hub = { git = "https://github.com/oodx/hub.git", features = ["core-ext", "tokio-full"] }
   ```
2. **Choose your variant strategy**:
   ```toml
   # Lean approach (recommended for most projects)
   features = ["async-ext", "cli-ext", "time-ext"]  # Gets tokio-lite, clap-lite, chrono-lite

   # Power user approach (selective full features)
   features = ["async-ext", "clap-full", "time-ext"]  # Mix lite and full as needed

   # Full-featured approach (maximum capability)
   features = ["tokio-full", "clap-full", "chrono-full"]  # All advanced features
   ```
3. **Use grouped imports** for clarity:
   ```rust
   use hub::data_ext::serde::{Serialize, Deserialize};
   use hub::text_ext::regex::Regex;
   use hub::async_ext::tokio;  // tokio-lite by default
   ```
4. **Access internal features** directly:
   ```rust
   use hub::colors;
   ```

### For Existing Projects
1. **Remove direct dependencies** from your Cargo.toml
2. **Update feature names** to new `-ext` format and choose lite/full strategy:
   ```toml
   # Old
   features = ["text", "data", "core", "async", "cli"]

   # New with lite defaults (recommended - faster builds)
   features = ["text-ext", "data-ext", "core", "async-ext", "cli-ext"]

   # New with selective full features (power user)
   features = ["text-ext", "data-ext", "core", "tokio-full", "cli-ext"]

   # New with all full features (maximum capability)
   features = ["text-ext", "data-ext", "core", "tokio-full", "clap-full"]
   ```
3. **Assess your feature needs**:
   ```rust
   // Check if you need full features:
   // Do you use tokio's networking, file system, or process features?
   use tokio::net::TcpListener;  // Needs tokio-full

   // Do you use clap's derive macros or environment variable support?
   #[derive(Parser)]  // Needs clap-full

   // Do you serialize/deserialize chrono types?
   use serde::{Serialize, Deserialize};
   #[derive(Serialize, Deserialize)]
   struct Event { time: chrono::DateTime<Utc> }  // Needs chrono-full
   ```
4. **Choose import style** - both work:
   ```rust
   // Top-level re-exports (backward compatible)
   use hub::regex::Regex;
   use hub::tokio;  // tokio-lite or tokio-full depending on features

   // Grouped modules (clearer intent)
   use hub::text_ext::regex::Regex;
   use hub::async_ext::tokio;  // tokio-lite by default from async-ext
   ```
5. **Consider gradual migration**:
   ```toml
   # Start with domain groups (gets lite variants)
   features = ["async-ext", "cli-ext", "time-ext"]

   # Profile your build times and binary sizes
   # Upgrade to full variants only where needed:
   features = ["tokio-full", "cli-ext", "time-ext"]  # Only tokio needs full features
   ```
6. **Test compilation** with `cargo check`
7. **Profile build performance**:
   ```bash
   # Test build times with lite vs full variants
   time cargo build --features="async-ext,cli-ext"  # Lite
   time cargo build --features="tokio-full,clap-full"  # Full
   ```
8. **Run tests** to ensure compatibility
9. **Update documentation** to reflect new structure and chosen variants
10. **Avoid local paths** unless you have urgent hot-fixes that cannot wait for publishing

### Breaking Changes in v0.3.0
- Legacy feature names still work but new `-ext` naming is recommended
- New module structure available (`hub::text_ext`, etc.) alongside existing re-exports
- No breaking changes to existing import patterns

### New Features in v0.4.0 (Lite/Full Variants)
✨ **Non-breaking additions:**
- **Lite variants**: `tokio-lite`, `clap-lite`, `chrono-lite` provide lean defaults
- **Full variants**: `tokio-full`, `clap-full`, `chrono-full` provide complete functionality
- **Domain group updates**: `async-ext`, `cli-ext`, `time-ext` now use lite variants by default
- **Backward compatibility**: Existing feature names (`tokio`, `clap`, `chrono`) still work and map to lite variants
- **Optional upgrade path**: Projects can selectively upgrade to full variants when needed

⚠️ **Important changes:**
- Domain groups now provide more efficient defaults (lite variants)
- Build times and binary sizes will improve automatically for projects using domain groups
- Projects requiring advanced features may need to explicitly choose full variants
- Individual package names now default to lite variants for consistency

## Important Notes

### YAML Deprecation Warning ⚠️
The `serde_yaml` feature is **deprecated** as of hub v0.3.0:
```rust
// This will show deprecation warnings
use hub::data_ext::serde_yaml;
```

**Migration Path:**
- **Configuration files**: Use TOML instead
- **Data exchange**: Use JSON instead
- **Rust-native serialization**: Use RON instead

This feature will be removed in a future version. Update your projects to use modern alternatives.

## Common Patterns

### Internal Features (oodx/rsb Infrastructure)
```rust
// Use internal shared infrastructure
use hub::colors;

// Access through top-level namespace (reserved for our code)
fn setup_colors() {
    // Internal oodx/rsb color system
}
```

### External Features - Top-level Re-exports
```rust
// Direct access to external dependencies
use hub::thiserror::Error;
use hub::serde::{Serialize, Deserialize};
use hub::regex::Regex;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

### External Features - Grouped Module Access
```rust
// Access through domain-specific modules (preferred for clarity)
use hub::error_ext::thiserror::Error;
use hub::data_ext::serde::{Serialize, Deserialize};
use hub::text_ext::regex::Regex;
use hub::data_ext::serde_json;

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    enabled: bool,
}

fn process_data() -> Result<(), MyError> {
    let config = Config {
        name: "test".to_string(),
        enabled: true
    };
    let json = serde_json::to_string(&config)?;

    let re = Regex::new(r"\d+").unwrap();
    // ... processing logic
    Ok(())
}
```

### Lite Variant Usage Patterns
```rust
// Basic async with tokio-lite (from async-ext)
use hub::async_ext::tokio;

#[tokio::main]  // Works with tokio-lite
async fn main() {
    println!("Hello async world!");

    // Basic runtime features available
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Note: networking features require tokio-full
    // use hub::tokio::net::TcpListener; // Would need tokio-full feature
}

// Basic CLI with clap-lite (from cli-ext)
use hub::cli_ext::clap::{Arg, Command};

fn build_cli() -> Command {
    Command::new("myapp")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .help("Input file"))
        // Note: derive macros require clap-full
        // #[derive(Parser)] // Would need clap-full feature
}

// Basic date/time with chrono-lite (from time-ext)
use hub::time_ext::chrono::{DateTime, Utc};

fn current_time() -> DateTime<Utc> {
    Utc::now()
    // Note: serde serialization requires chrono-full
    // #[derive(Serialize)] // Would need chrono-full feature
}
```

### Full Variant Usage Patterns
```rust
// Advanced async with tokio-full
use hub::async_ext::tokio;  // or use tokio-full feature directly

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Full tokio capabilities available
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    // File system operations
    let contents = tokio::fs::read_to_string("config.toml").await?;

    // Process spawning
    let output = tokio::process::Command::new("ls")
        .output()
        .await?;

    Ok(())
}

// Advanced CLI with clap-full
use hub::cli_ext::clap::Parser;  // derive macros available with clap-full

#[derive(Parser)]
#[command(name = "myapp")]
#[command(about = "A CLI tool")]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

    #[arg(env = "MY_CONFIG")]  // Environment variable support
    config: Option<String>,
}

// Date/time with serialization using chrono-full
use hub::time_ext::chrono::{DateTime, Utc};
use hub::data_ext::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Event {
    name: String,
    timestamp: DateTime<Utc>,  // Serialization works with chrono-full
}
```

### Mixed Variant Strategy
```rust
// Use full variants only where needed, lite elsewhere
use hub::async_ext::tokio;      // tokio-lite from domain group
use hub::clap_full::Parser;     // clap-full for derive macros
use hub::time_ext::chrono;      // chrono-lite from domain group

// This gives you:
// - Fast compilation from tokio-lite and chrono-lite
// - Advanced CLI features from clap-full
// - Optimal balance of features vs build time
```

## Troubleshooting

### Feature Not Found
- Check if you're using the new `-ext` feature naming (e.g., `text-ext` not `text`)
- Verify the feature is available in hub's Cargo.toml
- Use domain groups instead of individual features when possible
- Check both top-level (`hub::regex`) and grouped (`hub::text_ext::regex`) import paths
- **Lite/Full variants**: Ensure you're using the correct variant (`tokio-lite` vs `tokio-full`)

### Compilation Errors
- Ensure you've updated all imports to use hub re-exports
- Try both import styles: `hub::serde` vs `hub::data_ext::serde`
- Check for version incompatibilities with other non-hub dependencies
- Verify feature flags match your usage
- **Missing features**: You might need a full variant instead of lite
  ```toml
  # If this fails:
  features = ["async-ext"]  # tokio-lite

  # Try this:
  features = ["tokio-full"]  # complete tokio
  ```

### Performance Issues
- Use specific `-ext` features instead of `dev-ext` for production
- Consider using domain groups (`text-ext`, `data-ext`) for better organization
- The `dev-ext` mega package is intended for testing/development only
- **Slow builds**: You might be using full variants unnecessarily
  ```toml
  # Slow compilation?
  features = ["tokio-full", "clap-full", "chrono-full"]

  # Try lite variants:
  features = ["async-ext", "cli-ext", "time-ext"]  # Uses lite variants
  ```

### Missing Functionality Errors
- **Tokio networking**: `tokio::net` requires `tokio-full`
- **Clap derive macros**: `#[derive(Parser)]` requires `clap-full`
- **Chrono serialization**: `#[derive(Serialize)]` with chrono types requires `chrono-full`
- **Solutions**:
  ```toml
  # Replace domain group with full variant
  features = ["tokio-full"]  # instead of async-ext

  # Or add individual full features to domain groups
  features = ["async-ext", "clap-full"]  # tokio-lite + clap-full
  ```

### Migration Issues
- Old feature names still work for backward compatibility
- Gradually migrate to new `-ext` naming when convenient
- Both import styles work: choose based on your preference for clarity
- **Lite/Full migration**: Start with domain groups (lite by default), upgrade selectively
  ```toml
  # Start here (lite variants)
  features = ["async-ext", "cli-ext"]

  # Upgrade only what you need
  features = ["tokio-full", "cli-ext"]  # Only tokio needs full features
  ```

### Path Configuration Issues
- **Always prefer GitHub repo**: Use `git = "https://github.com/oodx/hub.git"` for standard development
- **Local paths are temporary**: If using `path = "../../hub"`, plan to migrate to GitHub repo when fixes are published
- **Version conflicts**: Local paths may cause inconsistencies between projects using different hub versions

## Support

For questions or issues:
1. Check the main README.md for comprehensive documentation
2. Review hub's feature definitions in Cargo.toml
3. Use `blade` tools for ecosystem analysis
4. Follow the migration patterns used by existing oodx projects

---

## Philosophy Summary

Hub embodies a **controlled integration** approach to dependency management with intelligent defaults:

- **Internal First**: Top-level namespace reserved for oodx/rsb infrastructure
- **External Reluctance**: Third-party dependencies marked with `-ext` suffix
- **Clean Separation**: Clear boundaries between our code and external utilities
- **Ecosystem Unity**: Single source of truth for dependency versions across all projects
- **Lean by Default**: Lite variants provide fast builds and small binaries as the starting point
- **Power When Needed**: Full variants available for advanced functionality without compromise
- **Project Control**: Teams can compose exactly the feature set they need

### The Lite/Full Balance

Hub's philosophy balances efficiency with capability:

- **Start Lean**: Domain groups provide lite variants by default for optimal developer experience
- **Scale Intelligently**: Upgrade to full variants only when specific advanced features are required
- **Compose Flexibly**: Mix lite and full variants based on actual project needs, not theoretical maximums
- **Optimize Continuously**: Regular profiling of build times and binary sizes guides optimal feature selection

Hub: *One crate to rule them all, one crate to find them, one crate to bring them all, and in the ecosystem bind them - but with clear separation between internal and external, and intelligent defaults that scale from lean to powerful.* 📦✨⚡