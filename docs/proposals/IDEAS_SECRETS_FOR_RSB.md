# 🔐 Secrets Management for RSB Framework

**Design Document for RSB secret!() Macro Integration**

This document outlines the design and implementation strategy for adding secure secret/passphrase management as a core RSB framework feature, based on lessons learned from cage's PassphraseManager implementation.

## 🎯 Goals

- **Unified Secret Handling** - Consistent secret input across all RSB applications
- **Security First** - Hidden input with no visual feedback (using rpassword)
- **Multiple Input Modes** - Interactive, stdin, environment variables, automation
- **Cross-Platform** - Unix/Windows support with RSB's existing TTY detection
- **Zero Boilerplate** - Simple macro interface for common use cases

## 🏗️ Current State Analysis

### cage's PassphraseManager Implementation

```rust
// Current cage implementation (works but app-specific)
use cage::{PassphraseManager, PassphraseMode};

let passphrase_manager = PassphraseManager::new();
let password = passphrase_manager.get_passphrase("Enter password", false)?;

// With specific modes
let password = passphrase_manager.get_passphrase_with_mode(
    "Enter API key",
    false,
    PassphraseMode::Environment("API_KEY".to_string())
)?;
```

### Key Features from cage Implementation:
- ✅ **TTY Detection** - Checks `/dev/tty` availability and stdin TTY status
- ✅ **Multiple Modes** - Interactive, Stdin, Environment, CommandLine
- ✅ **Security Validation** - Empty passphrase prevention, length limits
- ✅ **Cross-platform TTY** - Unix `libc::isatty()` with Windows planning
- ✅ **Hidden Input** - Uses `rpassword::read_password()` (no visual feedback)
- ✅ **Confirmation Support** - Double-entry for critical operations
- ✅ **Error Handling** - Comprehensive error types with user guidance

### rpassword Behavior Analysis:
- **Completely invisible input** - No stars, dots, or cursor movement
- **Most secure approach** - Zero visual feedback prevents shoulder surfing
- **Cross-platform support** - Works on Unix and Windows
- **Minimal dependencies** - Single focused crate

## 🚀 Proposed RSB Integration

### 1. RSB secret!() Macro Interface

```rust
// Basic usage - interactive with TTY detection
secret!(password, "Enter password");

// With confirmation
secret!(master_key, "Master Key", confirm: true);

// Try environment variable first, fallback to interactive
secret!(api_key, "API Key", env: "API_KEY");

// Force specific input mode
secret!(token, "Token", mode: stdin);
secret!(db_pass, "Database Password", mode: interactive);

// With validation
secret!(pin, "PIN (4 digits)", validate: |s| s.len() == 4 && s.chars().all(|c| c.is_numeric()));

// For automation (non-interactive)
secret!(automation_key, "Automation Key", required_env: "AUTOMATION_KEY");
```

### 2. RSB Integration Architecture

```rust
// In RSB framework core
pub mod secrets {
    use rpassword::read_password;
    use std::io::{self, Write};

    #[derive(Debug, Clone)]
    pub struct SecretOptions {
        pub confirm: bool,
        pub env_var: Option<String>,
        pub required_env: Option<String>,
        pub mode: Option<SecretMode>,
        pub allow_empty: bool,
        pub validator: Option<fn(&str) -> bool>,
        pub max_length: usize,
    }

    #[derive(Debug, Clone)]
    pub enum SecretMode {
        Interactive,
        Stdin,
        Environment(String),
        Auto, // RSB decides based on TTY detection
    }

    #[derive(Debug, thiserror::Error)]
    pub enum SecretError {
        #[error("No TTY available and no environment variable provided")]
        NoTtyNoEnv,

        #[error("Required environment variable '{0}' not found")]
        RequiredEnvMissing(String),

        #[error("Secret validation failed: {0}")]
        ValidationFailed(String),

        #[error("IO error during secret input: {0}")]
        IoError(String),

        #[error("Secret confirmation mismatch")]
        ConfirmationMismatch,
    }

    pub type SecretResult<T> = Result<T, SecretError>;

    // Core function that RSB apps and macro use
    pub fn get_secret(prompt: &str, options: SecretOptions) -> SecretResult<String> {
        // 1. Try required environment variable first (for automation)
        if let Some(env_name) = &options.required_env {
            return std::env::var(env_name)
                .map_err(|_| SecretError::RequiredEnvMissing(env_name.clone()));
        }

        // 2. Try optional environment variable
        if let Some(env_name) = &options.env_var {
            if let Ok(value) = std::env::var(env_name) {
                return Ok(value);
            }
        }

        // 3. Determine input mode
        let mode = options.mode.unwrap_or_else(|| {
            if rsb::tty::is_tty() { // Use RSB's existing TTY detection
                SecretMode::Interactive
            } else {
                SecretMode::Stdin
            }
        });

        // 4. Get secret based on mode
        let secret = match mode {
            SecretMode::Interactive => get_interactive_secret(prompt)?,
            SecretMode::Stdin => get_stdin_secret()?,
            SecretMode::Environment(env) => {
                std::env::var(&env).map_err(|_| SecretError::RequiredEnvMissing(env))?
            }
            SecretMode::Auto => {
                if rsb::tty::is_tty() {
                    get_interactive_secret(prompt)?
                } else {
                    get_stdin_secret()?
                }
            }
        };

        // 5. Validate secret
        if !options.allow_empty && secret.is_empty() {
            return Err(SecretError::ValidationFailed("Empty secret not allowed".to_string()));
        }

        if secret.len() > options.max_length {
            return Err(SecretError::ValidationFailed(format!("Secret too long (max: {})", options.max_length)));
        }

        if let Some(validator) = options.validator {
            if !validator(&secret) {
                return Err(SecretError::ValidationFailed("Secret validation failed".to_string()));
            }
        }

        // 6. Confirmation if requested
        if options.confirm {
            let confirmation = get_interactive_secret(&format!("Confirm {}", prompt))?;
            if secret != confirmation {
                return Err(SecretError::ConfirmationMismatch);
            }
        }

        Ok(secret)
    }

    fn get_interactive_secret(prompt: &str) -> SecretResult<String> {
        if !rsb::tty::is_tty() {
            return Err(SecretError::NoTtyNoEnv);
        }

        eprint!("🔐 {}: ", prompt);
        io::stderr().flush().map_err(|e| SecretError::IoError(e.to_string()))?;

        read_password().map_err(|e| SecretError::IoError(e.to_string()))
    }

    fn get_stdin_secret() -> SecretResult<String> {
        use std::io::BufRead;
        let stdin = io::stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line).map_err(|e| SecretError::IoError(e.to_string()))?;
        Ok(line.trim().to_string())
    }
}

// Macro implementation
#[macro_export]
macro_rules! secret {
    ($var:ident, $prompt:expr) => {
        let $var = rsb::secrets::get_secret($prompt, rsb::secrets::SecretOptions::default())?;
    };

    ($var:ident, $prompt:expr, $($key:ident : $value:expr),+ $(,)?) => {
        let mut options = rsb::secrets::SecretOptions::default();
        $(
            options = rsb::secrets::SecretOptions::$key(options, $value);
        )+
        let $var = rsb::secrets::get_secret($prompt, options)?;
    };
}
```

### 3. RSB Dependencies Update

```toml
# RSB Cargo.toml additions
[dependencies]
rpassword = "7.3"     # Secure password input
libc = "0.2"          # TTY detection (already in RSB?)
thiserror = "1.0"     # Error handling

[target.'cfg(windows)'.dependencies]
windows-sys = "0.52"  # Windows TTY detection
```

## 🔧 Implementation Strategy

### Phase 1: Core Integration
1. **Extract TTY detection** from cage and generalize for RSB
2. **Add rpassword dependency** to RSB framework
3. **Implement basic secret!() macro** with interactive mode
4. **Add comprehensive error handling**

### Phase 2: Advanced Features
1. **Environment variable support** with fallback chains
2. **Validation system** with custom validators
3. **Confirmation mode** for critical secrets
4. **Cross-platform TTY detection** improvements

### Phase 3: Integration and Testing
1. **Update cage** to use RSB secret!() macro
2. **Add comprehensive tests** for all input modes
3. **Documentation and examples**
4. **Performance optimization**

## 🎯 Migration Path for cage

### Before (current cage):
```rust
use cage::{PassphraseManager, PassphraseMode};

let passphrase_manager = PassphraseManager::new();
let passphrase = if is_true("opt_stdin_passphrase") {
    passphrase_manager.get_passphrase_with_mode(
        "Enter passphrase",
        false,
        PassphraseMode::Stdin
    )?
} else {
    passphrase_manager.get_passphrase("Enter passphrase", false)?
};
```

### After (with RSB integration):
```rust
use rsb::prelude::*;

if is_true("opt_stdin_passphrase") {
    secret!(passphrase, "Enter passphrase", mode: stdin);
} else {
    secret!(passphrase, "Enter passphrase");
}

// Or even simpler with env support:
secret!(passphrase, "Enter passphrase", env: "CAGE_PASSPHRASE");
```

## 🔍 Security Considerations

### 1. Memory Security
- **Consider zeroization** - Clear secrets from memory after use
- **Minimal lifetime** - Secrets should have short lifespans
- **No logging** - Never log actual secret values

### 2. Input Security
- **rpassword advantages** - No visual feedback, secure terminal handling
- **Environment variables** - Convenient but less secure (process lists)
- **File-based secrets** - Consider support for secret files

### 3. Cross-Platform Security
- **TTY detection** - Ensure consistent behavior across platforms
- **Terminal capabilities** - Handle various terminal types gracefully
- **Permission checks** - Verify appropriate file/process permissions

## 🧪 Testing Strategy

### Unit Tests
```rust
#[test]
fn test_secret_macro_basic() {
    // Mock TTY available
    secret!(test_secret, "Test Secret");
    assert!(!test_secret.is_empty());
}

#[test]
fn test_secret_env_fallback() {
    std::env::set_var("TEST_SECRET", "from_env");
    secret!(test_secret, "Test Secret", env: "TEST_SECRET");
    assert_eq!(test_secret, "from_env");
}

#[test]
fn test_secret_confirmation() {
    // Would need TTY mocking for full test
    secret!(test_secret, "Test Secret", confirm: true);
}
```

### Integration Tests
- **TTY availability scenarios** - Interactive vs automated environments
- **Environment variable precedence** - Various fallback chains
- **Error handling** - All error conditions and user guidance
- **Cross-platform** - Unix and Windows behavior verification

## 📋 Open Questions & Iterations Needed

### 1. API Design Questions
- **Macro vs function** - Should we support both approaches?
- **Error integration** - How to integrate with RSB's existing error system?
- **Context awareness** - Should secrets be tied to RSB's global context?

### 2. Security Enhancements
- **Memory zeroization** - Should RSB provide secure string types?
- **Secret rotation** - Support for secret rotation workflows?
- **Audit logging** - Should secret requests be audited (without values)?

### 3. User Experience
- **Prompt formatting** - Consistent styling with RSB's output patterns?
- **Help integration** - Auto-generated help for secret-using commands?
- **Configuration** - Global RSB settings for secret preferences?

### 4. Performance Considerations
- **Lazy loading** - Should rpassword be optional/feature-gated?
- **Caching** - Any secret caching for repeated operations?
- **Resource usage** - TTY detection overhead considerations?

## 🎬 Next Steps

1. **Gather feedback** on API design and security model
2. **Create prototype** implementation in separate branch
3. **Test integration** with cage as first user
4. **Iterate on API** based on real usage patterns
5. **Full RSB integration** once design is stable

## 📚 References

- **rpassword crate** - https://crates.io/crates/rpassword
- **cage PassphraseManager** - `/src/cage/passphrase.rs`
- **RSB TTY detection** - Existing RSB libc integration
- **Security best practices** - OWASP guidelines for secret handling

---

**This design represents lessons learned from cage's implementation and provides a path toward unified secret management across the RSB ecosystem.**