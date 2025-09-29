# RSB Feature: TOML SNOOPING

## Overview
**Automatic extraction of Cargo.toml metadata sections into RSB's global store**

The TOML Snooping module provides automatic extraction of configuration from `[package.metadata.*]` sections in Cargo.toml files. It seamlessly integrates with RSB's global store and Object<T> system, enabling declarative configuration management without boilerplate.

## Purpose
- Extract custom metadata from Cargo.toml into global variables
- Provide namespace-based configuration management (rsb, hub, inf)
- Auto-convert keys to snake_case for RSB consistency
- Handle arrays using RSB's indexed convention
- Enable zero-config bootstrap with automatic metadata loading

## Core Concepts

### Metadata Sections
Cargo.toml supports custom metadata via `[package.metadata.*]` sections. RSB snoops these sections and stores them as global variables:

```toml
[package.metadata.hub]
api_url = "https://api.hub.example.com"
timeout = "30"
features = ["auth", "cache", "metrics"]

[package.metadata.rsb]
options_mode = "remove"
debug = true
```

After snooping, these become global variables:
```rust
// hub_api_url = "https://api.hub.example.com"
// hub_timeout = "30"
// hub_features_LENGTH = "3"
// hub_features_0 = "auth"
// hub_features_1 = "cache"
// hub_features_2 = "metrics"
// rsb_options_mode = "remove"
// rsb_debug = "true"
```

### Default Namespaces
RSB snoops three namespaces by default:
- **`rsb`** - RSB framework configuration
- **`hub`** - Application/service configuration
- **`inf`** - Infrastructure/deployment configuration

Custom namespaces can be added via `snoop_namespace()`.

## API Reference

### Enable Snooping

```rust
use rsb::toml::enable_toml_snooping;

// Enable snooping (extracts all default namespaces)
enable_toml_snooping();

// Access snooped values
let api_url = rsb::global::get_var("hub_api_url");
```

### Custom Namespaces

```rust
use rsb::toml::{snoop_namespace, enable_toml_snooping};

// Add custom namespace before enabling
snoop_namespace("myapp");
snoop_namespace("config");
enable_toml_snooping();

// Now myapp_* and config_* variables are available
let value = rsb::global::get_var("myapp_setting");
```

### Bootstrap Integration

The most common usage is through the enhanced `bootstrap!` macro:

```rust
// Default bootstrap (no TOML snooping)
let args = bootstrap!();

// Bootstrap with TOML snooping (rsb, hub, inf)
let args = bootstrap!(toml);

// Bootstrap with custom namespaces
let args = bootstrap!(toml: "myapp", "config");
```

### Query Functions

```rust
use rsb::toml::{is_enabled, has_namespace};

// Check if snooping is enabled
if is_enabled() {
    println!("TOML snooping active");
}

// Check if namespace is registered
if has_namespace("custom") {
    println!("Custom namespace available");
}
```

## Key Normalization

All keys are automatically converted to snake_case:

```toml
[package.metadata.hub]
apiUrl = "test"         # → hub_api_url
maxRetries = "5"        # → hub_max_retries
connectTimeout = "10"   # → hub_connect_timeout
team-name = "RSB Core"  # → hub_team_name
```

This ensures consistency with RSB's naming conventions.

## Array Handling

Arrays are stored using RSB's indexed convention:

```toml
[package.metadata.hub]
features = ["auth", "cache", "metrics"]
```

Becomes:
```rust
// hub_features_LENGTH = "3"
// hub_features_0 = "auth"
// hub_features_1 = "cache"
// hub_features_2 = "metrics"
```

Access via loops:
```rust
let len = get_var("hub_features_LENGTH").parse::<usize>().unwrap_or(0);
for i in 0..len {
    let feature = get_var(&format!("hub_features_{}", i));
    println!("Feature {}: {}", i, feature);
}
```

## Value Type Handling

All TOML values are converted to strings (RSB string-biased philosophy):

```toml
[package.metadata.rsb]
name = "My App"    # String → "My App"
port = 8080        # Integer → "8080"
enabled = true     # Boolean → "true"
threshold = 3.14   # Float → "3.14"
```

Parse when needed:
```rust
let port: u16 = get_var("rsb_port").parse().unwrap_or(8080);
let enabled: bool = get_var("rsb_enabled") == "true";
```

## Integration with Object<T>

TOML snooping integrates seamlessly with the Object<T> system:

```rust
use rsb::toml::enable_toml_snooping;
use rsb::object::*;

// Enable snooping
enable_toml_snooping();

// Load namespace as Object
let hub_config = Object::<HubShape>::from_global("hub");

// Access via Object API
let api_url = &hub_config["api_url"];
let timeout = hub_config.get_or("timeout", "30");

// Iterate
for (key, value) in hub_config.as_map() {
    println!("{}: {}", key, value);
}
```

## Common Patterns

### Framework Configuration

Store RSB-specific configuration in the `rsb` namespace:

```toml
[package.metadata.rsb]
options_mode = "remove"      # Options cleanup strategy
global_reset = true          # Enable global store clearing
debug = false                # Debug mode
```

Access automatically during bootstrap:
```rust
let args = bootstrap!(toml);  // Automatically loads rsb_* settings
```

### Service Configuration

Store application config in `hub` namespace:

```toml
[package.metadata.hub]
api_url = "https://api.example.com"
api_key = "secret"           # Note: For dev only! Use env vars in production
timeout = "30"
retries = "3"
endpoints = ["users", "posts", "comments"]
```

Access via Object:
```rust
let hub = get_hub!();  // Helper macro returns Object<HubShape>
let url = &hub["api_url"];
```

### Infrastructure Configuration

Store deployment/infra config in `inf` namespace:

```toml
[package.metadata.inf]
environment = "staging"
region = "us-west-2"
team = "Platform"
support_email = "support@example.com"
```

## Error Handling

TOML snooping gracefully handles errors:

- **Missing Cargo.toml**: Silently skips (no error)
- **Invalid TOML**: Silently skips (no error)
- **Missing namespaces**: Silently skips (no error)
- **Complex nested types**: Skips unsupported values

This ensures snooping doesn't break applications that don't use it.

## Performance

- **Lazy initialization**: Snooping only happens when `enable_toml_snooping()` is called
- **One-time parse**: Cargo.toml is parsed once during initialization
- **Zero runtime cost**: After initialization, all values are in global store
- **Target overhead**: <1ms for typical Cargo.toml files

## Best Practices

### ✅ DO

- Use for compile-time known configuration
- Store developer defaults and presets
- Use namespace prefixes to organize config
- Parse string values when needed
- Combine with Object<T> for structured access

### ❌ DON'T

- Store secrets or sensitive data (use environment variables)
- Use for runtime-dynamic configuration
- Rely on snooping in libraries (it's app-level)
- Nest complex structures (keep it flat)
- Use in hot paths (load once at startup)

## Examples

### Basic Usage

```rust
use rsb::prelude::*;
use rsb::toml::enable_toml_snooping;

fn main() {
    let args = bootstrap!(toml);  // Automatic TOML snooping

    // Access configuration
    let api_url = get_var("hub_api_url");
    let debug = get_var("rsb_debug") == "true";

    println!("API URL: {}", api_url);
    println!("Debug mode: {}", debug);
}
```

### Custom Namespace

```rust
use rsb::prelude::*;
use rsb::toml::{snoop_namespace, enable_toml_snooping};

fn main() {
    // Add custom namespace
    snoop_namespace("myapp");
    enable_toml_snooping();

    // Access custom config
    let setting = get_var("myapp_custom_setting");
}
```

### With Object<T>

```rust
use rsb::prelude::*;
use rsb::object::*;

fn main() {
    let args = bootstrap!(toml);

    // Load as Object
    let config = get_hub!();

    // Structured access
    let api = ApiClient::new(
        &config["api_url"],
        &config["api_key"],
    );
}
```

## Module Location
- **Source**: `src/toml/mod.rs`
- **Tests**: `tests/sanity/toml_snooping.rs`, `tests/uat/toml_snooping.rs`
- **Dependencies**: `toml = "0.8"`

## Version
- **Introduced**: RSB v2.0 (Phase 2)
- **Status**: Stable
- **Breaking Changes**: None (fully backward compatible)

## Related Features
- **[FEATURES_GLOBAL.md](FEATURES_GLOBAL.md)** - Global store integration
- **[FEATURES_OBJECT.md](FEATURES_OBJECT.md)** - Object<T> system
- **[FEATURES_CLI.md](FEATURES_CLI.md)** - Bootstrap macro
- **[FEATURES_OPTIONS.md](FEATURES_OPTIONS.md)** - Options strategy from TOML

## Technical Notes

### Implementation Details

The TomlSnooper struct manages extraction:
```rust
pub struct TomlSnooper {
    enabled: bool,
    namespaces: Vec<String>,
}
```

Static instance via lazy_static:
```rust
static SNOOPER: Lazy<Mutex<TomlSnooper>> = Lazy::new(|| {
    Mutex::new(TomlSnooper::new())
});
```

### Cargo.toml Discovery

Walks up directory tree to find Cargo.toml:
```rust
let mut path = std::env::current_dir()?;
loop {
    let cargo_path = path.join("Cargo.toml");
    if cargo_path.exists() {
        return Ok(cargo_path);
    }
    if !path.pop() {
        return Err(/* Not found */);
    }
}
```

### Thread Safety

The global SNOOPER is protected by a Mutex, ensuring thread-safe access. However, snooping should typically be done once during application bootstrap before spawning threads.

## Migration Guide

### From Manual Global Variables

**Before:**
```rust
fn main() {
    set_var("hub_api_url", "https://api.example.com");
    set_var("hub_timeout", "30");
    // ... many more
}
```

**After:**
```toml
[package.metadata.hub]
api_url = "https://api.example.com"
timeout = "30"
```

```rust
fn main() {
    let args = bootstrap!(toml);  // Automatic!
}
```

### From Config Files

**Before:**
```rust
let config_file = fs::read_to_string("config.toml")?;
let config: MyConfig = toml::from_str(&config_file)?;
```

**After:**
```toml
# Move config to Cargo.toml metadata
[package.metadata.hub]
# your config here
```

```rust
let args = bootstrap!(toml);
let config = get_hub!();  // Already loaded!
```

## Future Enhancements

Potential future improvements:
- Support for nested table flattening
- Merge multiple TOML sources
- Hot-reload on Cargo.toml changes (dev mode)
- Export to JSON/YAML formats
- Schema validation support
- Environment variable overrides

## See Also
- [RSB v2.0 Enhancement Plan](../../proposals/IDEAS_IMPLEMENTATION_PLAN.md)
- [Phase 2 Roadmap](../../procs/ROADMAP.txt)
- [TOML Snooping Tests](../../../tests/sanity/toml_snooping.rs)