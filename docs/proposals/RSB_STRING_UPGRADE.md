# RSB String Module Security & Performance Upgrade Plan

## Executive Summary

Critical security issue discovered: **All strings in Rust binaries are exposed via the `strings` command**, including sensitive paths, error messages, and internal structure. This affects every RSB-based application and requires immediate attention.

## The Security Problem

### Binary String Exposure
Running `strings` on any Rust binary reveals:
```bash
$ strings my_rust_app | head -20
/home/xnull/repos/code/rust/oodx/projects/rsb
/home/xnull/.cargo/registry/src/index.crates.io
Error: Invalid API key format
Failed to connect to database at
Internal assertion failed at line
SECRET_TOKEN_PREFIX_
```

### Why This Matters
1. **Information Leakage** - Exposes internal paths, developer usernames, project structure
2. **Security Hints** - Error messages reveal system behavior to attackers
3. **Compliance Issues** - May violate security standards (GDPR, SOC2, etc.)
4. **Professional Concerns** - Unprofessional to ship binaries with developer paths

## Research Findings

### String Storage in Binaries (from docs/ref/strings/)

#### 1. Performance Analysis (STRING_LOADING_PERFORMANCE.md)
- **Compiled-in strings**: Zero runtime cost, 100% binary bloat
- **Runtime loading**: 1-3ms for 100KB file, then cached (0ms)
- **Memory-mapped**: 0.1-1ms initial, zero after
- **Conclusion**: Runtime loading performance is negligible

#### 2. Security Patterns (STRING_SECURITY_PATTERNS.md)
- **Never embed secrets** - They're visible with `strings`
- **Obfuscation doesn't work** - Delays attackers by seconds
- **External files are secure** - Not in binary, can't be extracted
- **Professional deployment** - Industry standard is external strings

#### 3. Strategy Recommendations (STRINGS_STRAT.md)
- **20-30% binary size reduction** with external strings
- **Zero information leakage** when done correctly
- **Post-deployment updates** without recompilation
- **Clean `strings` output** - No sensitive information

## Current RSB String Module Analysis

### Strengths
- ✅ Unicode-safe operations (char boundaries respected)
- ✅ Comprehensive case conversions (snake, kebab, camel, etc.)
- ✅ ASCII filtering utilities (strip/sanitize)
- ✅ Try variants for error handling
- ✅ Stream integration for large files
- ✅ Shell-safe quoting

### Critical Gaps
1. **No string externalization support**
2. **No security-conscious defaults**
3. **CamelCase bug** (same as Object module): `CamelCase` → `camelcase` instead of `camel_case`
4. **No compile-time string stripping**
5. **No integration with configuration systems**

## Proposed Solution: Two-Path Architecture

### Design Philosophy
RSB provides **two distinct paths** for string management:

1. **Path 1: `string` module** - Simple, embedded strings (current behavior)
2. **Path 2: `lang` module** - External, localized strings (new capability)

This gives developers **choice** - start simple, graduate to external when needed.

### Path 1: string Module (Simple Use)
```rust
// src/string/ - Current API unchanged
use rsb::string::*;

// Direct embedding (development/simple tools)
let result = str_replace("hello world", "world", "RSB", false);
println!("Error: File not found");  // Embedded string
```

**Characteristics**:
- ✅ Zero configuration
- ✅ Self-contained binaries
- ✅ Immediate availability
- ✅ Perfect for CLI tools, utilities, development

### Path 2: lang Module (Robust + i18n)
```rust
// src/lang/ - New external string management
pub struct LangManager {
    // External strings loaded at runtime
    external: HashMap<String, HashMap<String, String>>,  // locale -> key -> value

    // Minimal fallback strings compiled in
    fallback: HashMap<&'static str, &'static str>,

    // Current language/locale
    locale: String,

    // Cache for performance
    cache: RefCell<LruCache<String, String>>,
}

impl LangManager {
    /// Load strings from standard locations with locale detection
    pub fn load() -> Result<Self, LangError> {
        let locale = Self::detect_locale();

        let lang_dirs = [
            // 1. Relative to binary (portable)
            std::env::current_exe()?.parent().join("lang"),

            // 2. User config (customization)
            dirs::config_dir().join("rsb/lang"),

            // 3. System-wide (package manager)
            PathBuf::from("/usr/share/rsb/lang"),
        ];

        for dir in lang_dirs {
            if dir.exists() {
                return Self::from_directory(dir, &locale);
            }
        }

        // Fall back to minimal embedded strings
        Ok(Self::minimal(&locale))
    }

    /// Get localized string with fallback chain
    pub fn get(&self, key: &str) -> &str {
        // 1. Try current locale
        self.external.get(&self.locale)
            .and_then(|lang| lang.get(key))
            .map(String::as_str)
        // 2. Try 'en' fallback
        .or_else(|| self.external.get("en")
            .and_then(|lang| lang.get(key))
            .map(String::as_str))
        // 3. Try embedded fallback
        .or_else(|| self.fallback.get(key).copied())
        // 4. Return key itself
        .unwrap_or(key)
    }

    /// Switch language at runtime
    pub fn set_locale(&mut self, locale: &str) -> Result<(), LangError> {
        self.locale = locale.to_string();
        // Trigger reload if needed
        Ok(())
    }
}

// Global instance using once_cell
static LANG: Lazy<RwLock<LangManager>> = Lazy::new(|| {
    RwLock::new(LangManager::load().unwrap_or_else(|_| LangManager::minimal("en")))
});

    /// Format string with positional arguments using RSB's Args pattern
    ///
    /// **CAVEAT**: This implementation uses &[&str] for simplicity, but RSB has an internal
    /// `Arg` type that may be more appropriate for cross-module compatibility. The `Arg` type
    /// should be evaluated and potentially updated to support lang, cli, and incoming REPL
    /// use cases generically. This may require:
    ///
    /// 1. Making `Arg` more generic to handle different context types
    /// 2. Adding conversion traits between string slices and `Arg` instances
    /// 3. Ensuring `Arg` can represent both positional values and key-value pairs
    /// 4. Validating that `Arg` works consistently across lang/cli/REPL modules
    ///
    /// Future implementation may use:
    /// ```rust
    /// pub fn format(&self, key: &str, args: &[Arg]) -> String
    /// ```
    pub fn format(&self, key: &str, args: &[&str]) -> String {
        let template = self.get(key);
        self.substitute_positional(template, args)
    }

    /// Internal: Substitute %1, %2, %3... patterns (bash-style)
    fn substitute_positional(&self, template: &str, args: &[&str]) -> String {
        let mut result = template.to_string();

        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("%{}", i + 1);  // %1, %2, %3...
            result = result.replace(&placeholder, arg);
        }

        result
    }
}

// Convenience macros
#[macro_export]
macro_rules! lang {
    ($key:expr) => {
        $crate::lang::LANG.read().unwrap().get($key)
    };
}

#[macro_export]
macro_rules! lang_fmt {
    // Simple case - just get the string
    ($key:expr) => {
        $crate::lang::LANG.read().unwrap().get($key)
    };
    // With positional arguments - bash-style %1, %2, %3...
    ($key:expr, $($args:expr),+) => {
        $crate::lang::LANG.read().unwrap().format($key, &[$($args),+])
    };
}
```

### Deployment Structure
```
/usr/bin/myapp                    # Clean binary (minimal strings)
/usr/share/myapp/
├── lang/                         # Language files (Path 2)
│   ├── en.toml                   # English (default)
│   ├── es.toml                   # Spanish
│   ├── de.toml                   # German
│   ├── zh.toml                   # Chinese
│   └── fr.toml                   # French
├── help/
│   └── help.md                   # External help text
└── themes/
    └── default.yml               # UI themes
```

### Language File Format (TOML)
```toml
# lang/en.toml
[meta]
name = "English"
code = "en"
direction = "ltr"

[errors]
file_not_found = "Error: File not found"
invalid_input = "Error: Invalid input format"
permission_denied = "Error: Permission denied"

[ui]
welcome = "Welcome to {app_name}"
version = "Version {version}"
help_header = "USAGE:"
language_changed = "Language switched to English"

[messages]
processing = "Processing..."
complete = "Operation completed successfully"

# Positional argument templates (bash-style %1, %2, %3...)
[templates]
syntax_error = "Syntax Error: %1 occurred at line %2"
file_error = "Cannot read file '%1': %2"
connection_failed = "Failed to connect to %1:%2 (timeout: %3s)"
file_processed = "Processed %1 (%2 bytes) in %3ms"

# Pluralization support
[plurals]
items_count = [
    "No items",           # 0
    "One item",           # 1
    "%1 items"            # 2+ (with positional arg)
]
```

### Usage Examples

#### Path 1: Simple (string module)
```rust
use rsb::string::*;

fn main() {
    // Direct embedding - perfect for simple tools
    println!("Starting application...");

    let input = str_replace(user_input, "bad", "good", true);

    if error_occurred {
        eprintln!("Error: Invalid input");  // Embedded
    }
}
```

#### Path 2: Localized (lang module)
```rust
use rsb::prelude::*;

fn main() {
    // External, localized - perfect for user apps
    println!("{}", lang!("messages.starting"));

    let input = str_replace(user_input, "bad", "good", true);

    if error_occurred {
        // Simple message
        eprintln!("{}", lang!("errors.invalid_input"));

        // With positional arguments (bash-style %1, %2...)
        eprintln!("{}", lang_fmt!("templates.syntax_error", "Missing semicolon", "42"));
        // → "Syntax Error: Missing semicolon occurred at line 42"

        eprintln!("{}", lang_fmt!("templates.file_error", filename, error_msg));
        // → "Cannot read file 'config.toml': Permission denied"
    }

    // Runtime language switching
    lang::set_locale("es")?;
    println!("{}", lang!("ui.language_changed"));
}
```

#### Hybrid Approach
```rust
use rsb::prelude::*;

fn main() {
    // Try external first, fall back to embedded
    let msg = lang!("welcome.user")
        .unwrap_or("Welcome!");  // Embedded fallback

    println!("{}", msg);
}

## Implementation Plan

### Phase 0: Arg Type Investigation (2 SP)
- [ ] Analyze RSB's current `Arg` type implementation
- [ ] Evaluate compatibility with lang/cli/REPL use cases
- [ ] Design generic `Arg` interface if updates needed
- [ ] Add conversion traits between `&str` and `Arg` types
- [ ] Document cross-module Arg usage patterns

### Phase 1: Core Infrastructure (5 SP)
- [ ] Create `lang::manager` module (using lang instead of string)
- [ ] Implement `LangManager` with external loading
- [ ] Add `lang!()` and `lang_fmt!()` macros for easy access
- [ ] Create minimal fallback strings
- [ ] Integrate with updated `Arg` type from Phase 0

### Phase 2: Security Features (3 SP)
- [ ] Add `string::security` module
- [ ] Implement binary string scanner
- [ ] Create string extraction tool
- [ ] Add obfuscation for fallbacks

### Phase 3: Integration (4 SP)
- [ ] Integrate with Object for configuration
- [ ] Add Meteor support for string compression
- [ ] Update bootstrap to load strings
- [ ] Add i18n/l10n support

### Phase 4: Tooling (3 SP)
- [ ] Create `rsb-strings` CLI tool
- [ ] Add string extraction from source
- [ ] Build compile-time stripping
- [ ] Add string usage analyzer

### Phase 5: Migration (2 SP)
- [ ] Update all RSB error messages
- [ ] Convert help text to external
- [ ] Document migration path
- [ ] Add examples

## Build Features

```toml
[features]
default = ["embedded-strings"]

# String loading strategies
embedded-strings = []        # Traditional (current)
external-strings = []        # Load from files
minimal-strings = []         # Strip all but critical
secure-strings = []          # Obfuscate + external

# Development
string-debug = []            # Debug string loading
```

## Case Conversion Bug Fix

The `split_words` function needs fixing:
```rust
// Current (WRONG)
"CamelCase" → ["camel", "case"] → "camelcase"

// Fixed
"CamelCase" → ["camel", "case"] → "camel_case"
```

This affects both `string::case` and `object::helpers::normalize_key`.

## Security Best Practices

### DO:
- ✅ Use external string files for production
- ✅ Minimize embedded strings to critical only
- ✅ Strip debug strings in release builds
- ✅ Use compile-time features to control inclusion
- ✅ Sanitize error messages (no paths/internals)

### DON'T:
- ❌ Embed file paths in binaries
- ❌ Include detailed error messages
- ❌ Use `format!()` with sensitive data
- ❌ Rely on obfuscation for security
- ❌ Ship with debug strings

## Performance Optimization

### Lazy Loading Pattern
```rust
// Load once, use everywhere
static STRINGS: Lazy<StringManager> = Lazy::new(|| {
    let start = Instant::now();
    let mgr = StringManager::load().expect("Failed to load strings");
    debug!("Loaded strings in {:?}", start.elapsed());
    mgr
});
```

### Memory-Mapped Option
```rust
// For very large string files
pub struct MmapStringManager {
    mmap: Mmap,
    index: HashMap<&'static str, (usize, usize)>,
}
```

## Integration with RSB Ecosystem

### Object Integration
```rust
// Load strings as Object
let strings = Object::<StringsShape>::from_external("strings.toml");
let error_msg = strings["errors.file_not_found"];
```

### Meteor Compression
```rust
// Compress strings for transport
let compressed = strings.to_meteor();
// "errors:file_not_found=Error: File not found"
```

### Hub Distribution
- Add `string-manager` to Hub for ecosystem use
- Standardize string file locations
- Share common error messages

## Testing Strategy

### Unit Tests
- Test external loading with missing files
- Verify fallback behavior
- Check locale switching
- Validate cache performance

### Security Tests
- Run `strings` on test binaries
- Verify no sensitive data exposed
- Check obfuscation effectiveness
- Test string extraction tools

### Integration Tests
- Test with Object/Meteor
- Verify bootstrap integration
- Check i18n switching
- Test deployment scenarios

## Migration Guide

### For Existing RSB Apps

1. **Audit current strings**:
```bash
strings my_app | grep -E "(home|path|error|secret)" > exposed.txt
```

2. **Extract strings to file**:
```bash
rsb-strings extract --source src/ --output strings.toml
```

3. **Update code to use StringManager**:
```rust
// Before
println!("Error: File not found");

// After
println!("{}", s!("errors.file_not_found"));
```

4. **Configure build**:
```toml
[features]
default = ["external-strings"]
```

5. **Test thoroughly**:
```bash
cargo test --features external-strings
strings target/release/my_app | wc -l  # Should be minimal
```

## Benefits Summary

### Security
- **Zero path leakage** - No internal paths in binary
- **Clean strings output** - Professional appearance
- **Reduced attack surface** - Less information for attackers

### Performance
- **20-30% smaller binaries** - Significant size reduction
- **Faster compilation** - Less data to embed
- **Better caching** - Strings separate from code

### Maintenance
- **Post-deployment updates** - Change strings without recompiling
- **Easy localization** - Add languages without code changes
- **A/B testing** - Test different messages easily

### Professional
- **Industry standard** - How production software works
- **Clean deployment** - Proper file structure
- **Compliance ready** - Meets security standards

## Conclusion

The discovery of string exposure in Rust binaries is a **critical security issue** that affects every RSB application. The proposed StringManager solution addresses this while maintaining RSB's string-first philosophy and adding powerful features like i18n, post-deployment updates, and binary size reduction.

This upgrade transforms RSB from embedding everything to a professional, secure, external string management system that aligns with industry best practices while maintaining excellent performance.

## References

- `/docs/ref/strings/STRING_SECURITY_PATTERNS.md` - Security patterns and examples
- `/docs/ref/strings/STRINGS_STRAT.md` - Strategic recommendations
- `/docs/ref/strings/STRING_LOADING_PERFORMANCE.md` - Performance analysis
- `/docs/tech/features/FEATURES_STRINGS.md` - Current string module documentation

---
*Critical security upgrade required - String exposure affects all RSB applications*