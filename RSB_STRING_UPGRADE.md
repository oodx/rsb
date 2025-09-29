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

## Proposed Solution: StringManager

### Core Design
```rust
// src/string/manager.rs
pub struct StringManager {
    // External strings loaded at runtime
    external: Option<HashMap<String, String>>,

    // Minimal fallback strings compiled in
    fallback: HashMap<&'static str, &'static str>,

    // Language/locale support
    locale: String,

    // Cache for frequently accessed strings
    cache: RefCell<LruCache<String, String>>,
}

impl StringManager {
    /// Load strings from standard locations
    pub fn load() -> Result<Self, StringError> {
        let paths = [
            // 1. Relative to binary (portable)
            std::env::current_exe()?.parent().join("strings.toml"),

            // 2. User config (customization)
            dirs::config_dir().join("rsb/strings.toml"),

            // 3. System-wide (package manager)
            PathBuf::from("/usr/share/rsb/strings.toml"),
        ];

        for path in paths {
            if path.exists() {
                return Self::from_file(path);
            }
        }

        // Fall back to minimal embedded strings
        Ok(Self::minimal())
    }

    /// Get a string by key with fallback
    pub fn get(&self, key: &str) -> &str {
        self.external
            .as_ref()
            .and_then(|ext| ext.get(key))
            .map(String::as_str)
            .or_else(|| self.fallback.get(key).copied())
            .unwrap_or(key) // Return key if not found
    }
}

// Global instance using once_cell
static STRINGS: Lazy<StringManager> = Lazy::new(|| {
    StringManager::load().unwrap_or_else(|_| StringManager::minimal())
});

// Convenience macro
#[macro_export]
macro_rules! s {
    ($key:expr) => {
        $crate::string::STRINGS.get($key)
    };
}
```

### Deployment Structure
```
/usr/bin/myapp                    # Clean binary (minimal strings)
/usr/share/myapp/
├── strings/
│   ├── en.toml                   # English (default)
│   ├── es.toml                   # Spanish
│   ├── de.toml                   # German
│   └── zh.toml                   # Chinese
├── help/
│   └── help.md                   # External help text
└── themes/
    └── default.yml               # UI themes
```

### String File Format (TOML)
```toml
# strings/en.toml
[errors]
file_not_found = "Error: File not found"
invalid_input = "Error: Invalid input format"
permission_denied = "Error: Permission denied"

[ui]
welcome = "Welcome to {app_name}"
version = "Version {version}"
help_header = "USAGE:"

[messages]
processing = "Processing..."
complete = "Operation completed successfully"
```

## Implementation Plan

### Phase 1: Core Infrastructure (5 SP)
- [ ] Create `string::manager` module
- [ ] Implement `StringManager` with external loading
- [ ] Add `s!()` macro for easy access
- [ ] Create minimal fallback strings

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