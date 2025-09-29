# RSB Feature: OBJECT

## Overview
**Generic JavaScript-like Object for flexible string-based data structures**

The Object type provides a flexible, string-based container with phantom type parameters for shape hinting. It combines RSB's string-biased philosophy with type-level documentation, offering JavaScript-like bracket notation while maintaining Rust's type safety.

## Purpose
- Provide a universal container for configuration and dynamic data
- Enable JavaScript-like property access with bracket notation
- Support phantom types for compile-time shape documentation
- Maintain RSB's string-first philosophy (all values are `&str`)
- Integrate seamlessly with Global store and TOML snooping

## Core Design

### The Object Type
```rust
use std::collections::HashMap;
use std::marker::PhantomData;

/// Generic Object with phantom type T for shape hinting
pub struct Object<T = ()> {
    inner: HashMap<String, String>,
    namespace: String,
    _phantom: PhantomData<T>,
}
```

The type parameter `T` is never instantiated - it's purely for documentation and type hints. All operations always work with strings.

## API Reference

### Construction
```rust
// Create empty Object
let obj = Object::new("namespace");

// Load from global variables with prefix
let hub = Object::<HubShape>::from_global("hub");

// Build manually
let mut config = Object::new("app");
config.set("version", "1.0.0");
config.set("debug", "true");
```

### Access Patterns
```rust
// Bracket notation (JavaScript-like)
let value = obj["key"];
let nested = obj["section.property"];  // Converts to section_property

// Method access
let value = obj.get("key");              // Returns &str or ""
let value = obj.get_or("key", "default"); // With default
let exists = obj.has("key");             // Check existence

// Iteration
for (key, value) in obj.as_map() {
    println!("{}: {}", key, value);
}

// Get all keys
let keys: Vec<&String> = obj.keys();
```

### Type Conversion
```rust
// Change phantom type for documentation
let generic: Object = Object::from_global("config");
let typed: Object<MyAppShape> = generic.as_type();

// Type parameter doesn't affect operations
fn process_any(obj: Object) { /* ... */ }
fn process_hub(obj: Object<HubShape>) { /* ... */ }
```

## Phantom Types

### Purpose
Phantom types provide compile-time documentation without runtime overhead:

```rust
// Marker types (never instantiated)
pub struct HubShape;
pub struct InfShape;
pub struct RsbShape;
struct MyCustomShape;

// Type aliases for clarity
pub type AnyObject = Object<()>;
pub type HubConfig = Object<HubShape>;
pub type InfConfig = Object<InfShape>;
pub type RsbConfig = Object<RsbShape>;
```

### Benefits
```rust
// Function signatures document expected shape
fn configure_api(config: Object<HubShape>) {
    let url = config["api_url"];  // Reader knows hub should have api_url
}

// Vs generic object
fn configure_something(config: Object) {
    let val = config["unknown"];  // No hint about expected keys
}
```

## Integration with Global Store

### Loading from Global
Objects automatically load from global variables with namespace prefixes:

```rust
// If global has: hub_api_url="https://api.com", hub_timeout="30"
let hub = Object::<HubShape>::from_global("hub");
assert_eq!(hub["api_url"], "https://api.com");
assert_eq!(hub["timeout"], "30");
```

### Syncing to Global
```rust
impl<T> Object<T> {
    /// Write all values back to global with namespace prefix
    pub fn sync_to_global(&self) {
        for (key, value) in &self.inner {
            set_var(&format!("{}_{}", self.namespace, key), value);
        }
    }
}
```

## Macros

### Value Access Macros
```rust
// Get single value from namespace
hub_config!("api_url")    // Returns &str from hub_api_url
inf_config!("timeout")     // Returns &str from inf_timeout
rsb_config!("options_mode") // Returns &str from rsb_options_mode
```

### Object Access Macros
```rust
// Get full Object for namespace
let hub = hub_object!();     // Returns Object<HubShape>
let inf = inf_object!();     // Returns Object<InfShape>
let rsb = rsb_object!();     // Returns Object<RsbShape>
```

## TOML Integration

### Loading from Cargo.toml
```toml
[package.metadata.hub]
api_url = "https://api.example.com"
timeout = "30"
retry_count = "3"

[package.metadata.inf]
team = "RSB Core"
support_email = "support@rsb.dev"

[package.metadata.rsb]
options_mode = "remove"
global_reset = true
protected_keys = ["HOME", "PATH", "USER"]
```

### Automatic Loading
```rust
// During bootstrap, TOML sections load into global:
// hub_api_url, hub_timeout, hub_retry_count
// inf_team, inf_support_email
// rsb_options_mode, rsb_global_reset, rsb_protected_keys

// Then access via Object:
let hub = get_hub!();
println!("API: {}", hub["api_url"]);
```

## Usage Patterns

### Configuration Object
```rust
struct AppConfig;

fn load_config() -> Object<AppConfig> {
    let mut config = Object::new("app");

    // Load from file
    if let Ok(content) = std::fs::read_to_string("app.conf") {
        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                config.set(key.trim(), value.trim());
            }
        }
    }

    // Override from environment
    config.set("debug", &get_var("DEBUG_MODE"));

    config
}
```

### Dynamic Property Bag
```rust
fn build_response() -> Object {
    let mut response = Object::new("response");
    response.set("status", "200");
    response.set("content_type", "application/json");
    response.set("body", r#"{"message": "success"}"#);
    response
}
```

### Type-Safe Wrappers
```rust
// Define your shape
struct DatabaseConfig;

// Wrap with helper methods
struct DbConfig(Object<DatabaseConfig>);

impl DbConfig {
    fn host(&self) -> &str { self.0["host"] }
    fn port(&self) -> &str { self.0.get_or("port", "5432") }
    fn database(&self) -> &str { self.0["database"] }

    fn connection_string(&self) -> String {
        format!("postgres://{}:{}/{}",
            self.host(),
            self.port(),
            self.database())
    }
}
```

## Implementation Notes

### String-Only Values
All values in Object are strings. This aligns with RSB's string-biased philosophy:

```rust
let obj = Object::new("test");
obj.set("count", "42");      // Not i32
obj.set("enabled", "true");  // Not bool
obj.set("rate", "3.14");     // Not f64

// Parse when needed
let count: i32 = obj["count"].parse().unwrap_or(0);
let enabled = obj["enabled"] == "true";
```

### Case Conversion
Keys with camelCase or kebab-case are automatically converted to snake_case:

```rust
// In Cargo.toml: maxRetries = "3"
// Stored as: hub_max_retries = "3"
let hub = get_hub!();
assert_eq!(hub["max_retries"], "3");
```

### Arrays in TOML
Arrays are expanded using RSB's indexing convention:

```toml
[package.metadata.hub]
features = ["auth", "cache", "metrics"]
```

Becomes:
```rust
hub_features_LENGTH = "3"
hub_features_0 = "auth"
hub_features_1 = "cache"
hub_features_2 = "metrics"
```

### Nested Access
Dot notation provides convenient nested access:

```rust
// If stored as: api_base_url = "https://api.com"
let config = Object::from_global("api");
let url = config["base.url"];  // Looks for base_url
```

## Testing

### Unit Tests
```rust
#[test]
fn test_object_creation() {
    let mut obj = Object::<TestShape>::new("test");
    obj.set("key", "value");
    assert_eq!(obj["key"], "value");
    assert_eq!(obj.get_or("missing", "default"), "default");
}

#[test]
fn test_type_conversion() {
    let generic: Object = Object::new("test");
    let typed: Object<CustomShape> = generic.as_type();
    // Same data, different phantom type
}

#[test]
fn test_global_integration() {
    set_var("test_key", "value");
    let obj = Object::from_global("test");
    assert_eq!(obj["key"], "value");
}
```

### Property Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_object_roundtrip(
        keys in prop::collection::vec("[a-z]+", 1..10),
        values in prop::collection::vec("[a-zA-Z0-9]+", 1..10)
    ) {
        let mut obj = Object::new("test");
        for (k, v) in keys.iter().zip(values.iter()) {
            obj.set(k, v);
        }

        for (k, v) in keys.iter().zip(values.iter()) {
            assert_eq!(obj.get(k), v);
        }
    }
}
```

## Comparison with Alternatives

### vs HashMap<String, String>
- Object provides namespace association
- Phantom types for documentation
- JavaScript-like bracket notation
- Integration with global store

### vs JSON Value
- Object is always flat (string keys, string values)
- No parsing overhead
- No type confusion (everything is string)
- Simpler mental model

### vs Struct with Fields
- Object is dynamic and flexible
- No compile-time field checking
- Runtime key discovery
- Better for configuration and dynamic data

## Best Practices

### DO
- Use phantom types for documentation
- Keep values as strings
- Use Objects for configuration and dynamic data
- Leverage bracket notation for clarity
- Sync with global store when needed

### DON'T
- Don't use Object for structured domain models
- Don't store complex nested structures
- Don't parse values repeatedly (cache parsed values)
- Don't rely on phantom types for runtime behavior

## Planned Enhancements (QOL Tasks)

### Core Improvements
1. **Key Normalization Fix** (QOL-01) - Proper CamelCase → snake_case conversion
   - Current: "CamelCase" → "camelcase" (incorrect)
   - Fixed: "CamelCase" → "camel_case" (proper snake_case)

2. **Macro Exports** (QOL-02) - Export Object macros in prelude
   - `hub_config!`, `inf_config!`, `rsb_config!`
   - `hub_object!`, `inf_object!`, `rsb_object!`

3. **Additional Shape Types** (QOL-03) - More phantom types for common patterns
   - `GenericShape` - for general-purpose property bags
   - `JSONShape` - for JSON-style data structures
   - `MeteorShape` - for Meteor data type representations
   - Fix: `RsbShape` → `RSBShape` (proper capitalization)
   - Type aliases: `GenericObject`, `JSONObject`, `MeteorObject`

### API Extensions
4. **Object Merging** (QOL-04) - `Object::merge()` for combining Objects
   - Merge two Objects, with second overwriting conflicts
   - Useful for layered configuration (defaults + overrides)

5. **HashMap Constructor** (QOL-05) - `Object::from_map(HashMap)`
   - Direct construction from existing HashMaps
   - Enables easy conversion from other data structures

6. **Iteration Helpers** (QOL-07)
   - `iter()` - iterator over (key, value) pairs
   - `filter_prefix(prefix)` - get subset with key prefix
   - `to_vec()` - sorted key-value pairs for deterministic output

7. **Display/Debug Enhancements** (QOL-08)
   - Implement Display trait for pretty printing
   - Format: `Object<namespace> { key1: "value1", key2: "value2" }`
   - `dump()` method for enhanced debug output

8. **Validation Helpers** (QOL-09)
   - `require(key) -> Result<&str, Error>` - fail on missing keys
   - `require_all(&[keys]) -> Result<(), Error>` - batch validation
   - `is_empty()` and `len()` methods

### ObjectLike Trait System (QOL-11)
The most powerful enhancement - a trait system for polymorphic object patterns:

```rust
pub trait ObjectLike {
    fn get(&self, key: &str) -> &str;
    fn set(&mut self, key: impl Into<String>, value: impl Into<String>);
    fn has(&self, key: &str) -> bool;
    fn keys(&self) -> Vec<&str>;
    fn namespace(&self) -> &str;

    // Translation layer
    fn to_object(&self) -> Object;
    fn from_object(obj: &Object) -> Self;
}
```

#### Benefits
- **Universal Translation Layer** - Like `to_string()`/`from_str()` for configs
- **Polymorphic APIs** - `fn process(cfg: &impl ObjectLike)`
- **Testing** - Mock implementations for unit tests
- **Adapters** - Bridge to HashMap, BTreeMap, JSON, TOML, etc.
- **Dynamic Dispatch** - `Box<dyn ObjectLike>` for heterogeneous collections

#### Translation Patterns
```rust
// Any config → Object
let obj = external_config.to_object();

// Object → Any format
let map = HashMap::from_object(&obj);

// Chain translations
config.to_object()
    .sync_to_global()
    .to_object()
    .to_json();
```

### Naming Consistency (QOL-10)
- Decision needed: `InfShape` vs `INFShape`
- Should match `RSBShape` pattern or stay as-is?
- Document final convention

### Meteor-Object Integration (QOL-12)
**The Power Pattern: Object as frontend, Meteor as backend**

Meteor is RSB's powerful object compression and storage engine that evolved from the original token module. It provides hierarchical key-value storage with string serialization, making it a perfect complement to Object.

#### What is Meteor?
Meteor compresses objects into strings using the pattern: `context:namespace:key=value`

Example transformations:
```rust
// Object representation
let mut obj = Object::new("app");
obj.set("ui.button", "click");
obj.set("ui.theme", "dark");

// Meteor compression
"app:ui:button=click;theme=dark"

// Or with folding (stateful parsing)
"button=click;ns=ui;theme=dark"
```

#### Integration Architecture
```
Object (API) ↔ Meteor (Compression) ↔ MeteorEngine (Storage)
```

1. **Object**: User-friendly API with ergonomic methods
2. **Meteor**: String compression format for transport/serialization
3. **MeteorEngine**: Stateful storage engine with cursor navigation

#### Key Integration Points

**Object → Meteor Conversion**
```rust
impl Object {
    // Compress Object to Meteor TokenStream
    pub fn to_meteor(&self) -> String {
        // Convert Object entries to meteor format
        // "namespace:key=value;key2=value2"
    }

    // Compress with explicit addressing
    pub fn to_meteor_explicit(&self) -> String {
        // "context:namespace:key=value :;: context:namespace:key2=value2"
    }
}
```

**Meteor → Object Parsing**
```rust
impl Object {
    // Parse Meteor stream into Object
    pub fn from_meteor(stream: &str) -> Result<Self, Error> {
        // Parse "app:ui:button=click" into Object
    }
}
```

**MeteorEngine as ObjectLike Backend**
```rust
impl ObjectLike for MeteorEngine {
    fn get(&self, key: &str) -> &str {
        self.storage.get(&self.current_context, &self.current_namespace, key)
    }

    fn to_object(&self) -> Object {
        // Export current namespace as Object
    }
}
```

#### Use Cases

1. **Network Transport**: Compress Objects to Meteor strings for efficient transmission
2. **Persistent Storage**: Use MeteorEngine as database backend for Objects
3. **Document Virtualization**: Store entire documents/scripts through Object API
4. **Configuration Management**: Unified config storage across contexts

#### Document Storage Example
```rust
// Store document sections in Object
let mut doc = Object::<DocShape>::new("doc");
doc.set("guides.install[intro]", intro_content);
doc.set("guides.install[setup]", setup_content);

// Meteor handles hierarchical storage
// Internally stored as:
// doc:guides.install:sections[intro]=content
// doc:guides.install:sections[setup]=content

// Export to filesystem
// guides/
//   install/
//     intro.md
//     setup.md
```

#### Stream Processing Modes

**TokenStream (with folding)**
- Stateful parsing with control tokens
- `ns=ui` changes namespace for subsequent tokens
- `ctx=user` changes context
- More compact for related data

**MeteorStream (explicit only)**
- Every item fully qualified
- No state between tokens
- Better for heterogeneous data
- Delimiter `:;:` between meteors

#### Benefits of Integration
- **Unified Data Model**: Objects and Meteors share string-biased philosophy
- **Flexible Storage**: Memory (Object) → Compressed (Meteor) → Persistent (MeteorEngine)
- **Document Management**: Virtual filesystem in MeteorEngine, accessed via Object API
- **Cross-System Bridge**: Objects can serialize to Meteor for external systems
- **Audit Trail**: MeteorEngine tracks command history for all mutations

## Future Considerations

### Potential Features (Not Yet Planned)
1. **Serialization**: Add serde support for JSON/YAML
2. **Schema Validation**: Runtime validation against schemas
3. **Observers**: Change notification system
4. **Computed Properties**: Lazy evaluation of derived values
5. **Nested Objects**: Support for true nested structures

### Considered but Rejected
- **Generic Value Types**: Would break string-first philosophy
- **Runtime Type Checking**: Phantom types are compile-time only
- **Automatic Parsing**: User should control when parsing happens

## Examples

### Complete Configuration System
```rust
use rsb::prelude::*;

// Define configuration shapes
struct ServerConfig;
struct DatabaseConfig;
struct CacheConfig;

fn main() {
    // Load all configuration
    let args = bootstrap!(toml);

    // Access different config sections
    let server = Object::<ServerConfig>::from_global("server");
    let db = Object::<DatabaseConfig>::from_global("db");
    let cache = Object::<CacheConfig>::from_global("cache");

    // Use configuration
    start_server(&server);
    connect_database(&db);
    init_cache(&cache);
}

fn start_server(config: &Object<ServerConfig>) {
    let host = config.get_or("host", "0.0.0.0");
    let port = config.get_or("port", "8080");
    println!("Starting server on {}:{}", host, port);
}
```

### Dynamic Builder Pattern
```rust
struct EmailMessage;

fn build_email() -> Object<EmailMessage> {
    let mut email = Object::new("email");

    email.set("from", "noreply@example.com");
    email.set("subject", "Welcome!");
    email.set("template", "welcome");

    // Conditional fields
    if is_premium_user() {
        email.set("priority", "high");
        email.set("track_opens", "true");
    }

    email
}
```

### Test Fixtures
```rust
#[cfg(test)]
mod tests {
    struct TestData;

    fn test_object() -> Object<TestData> {
        let mut obj = Object::new("test");
        obj.set("user_id", "123");
        obj.set("token", "abc-def-ghi");
        obj.set("timestamp", "1234567890");
        obj
    }

    #[test]
    fn test_something() {
        let data = test_object();
        assert_eq!(data["user_id"], "123");
    }
}
```

## Related Documentation
- [FEATURES_GLOBAL.md](FEATURES_GLOBAL.md) - Global store integration
- [FEATURES_STRINGS.md](FEATURES_STRINGS.md) - String manipulation utilities
- [IDEAS_IMPLEMENTATION_PLAN.md](../proposals/IDEAS_IMPLEMENTATION_PLAN.md) - Implementation proposal

## Code Inventory

Once the Object module is implemented, run:
```bash
python3 bin/feat.py object --update-doc
```

This will populate the code inventory below with actual exports from `src/object/`.

**Note**: After implementation, update `FEATURE_MAP` in `bin/feat.py` to include:
```python
'object': ['src/object', 'src/macros/object.rs'],
```

<!-- feat:object -->

_Generated by bin/feat.py --update-doc._

* `src/object/helpers.rs`
  - fn load_globals_with_prefix (line 6)
  - fn normalize_key (line 23)

* `src/object/macros.rs`
  - macro hub_config! (line 5)
  - macro inf_config! (line 13)
  - macro rsb_config! (line 21)
  - macro hub_object! (line 29)
  - macro inf_object! (line 37)
  - macro rsb_object! (line 45)

* `src/object/mod.rs`
  - pub use utils::* (line 15)
  - struct Object (line 18)
  - fn new (line 26)
  - fn from_global (line 36)
  - fn get (line 48)
  - fn get_or (line 54)
  - fn has (line 60)
  - fn set (line 66)
  - fn as_map (line 72)
  - fn keys (line 77)
  - fn namespace (line 82)
  - fn sync_to_global (line 87)
  - fn as_type (line 94)
  - struct HubShape (line 134)
  - struct InfShape (line 135)
  - struct RsbShape (line 136)
  - type AnyObject (line 139)
  - type HubConfig (line 140)
  - type InfConfig (line 141)
  - type RsbConfig (line 142)

* `src/object/utils.rs`
  - fn get_object (line 6)
  - fn get_hub (line 11)
  - fn get_inf (line 16)
  - fn get_rsb (line 21)

<!-- /feat:object -->




---

*Generated for RSB v2.0 - Object Feature*