# String Security Patterns for Rust

## ⚠️ CRITICAL: Never Embed Secrets in Binaries!

**Tokens/keys in binaries are NOT secure:**
```rust
// ❌ NEVER DO THIS
const API_KEY: &str = "sk-1234567890abcdef";  // Visible with `strings`!

// Even obfuscated strings are easily defeated:
const KEY: &str = obfstr!("sk-1234567890");   // Just delays attackers by seconds
```

## 1. String Externalization Pattern (i18n/l10n Ready)

Instead of inline strings, use a centralized string module:

### Basic Pattern:
```rust
// src/strings.rs
pub mod strings {
    // Group by feature
    pub mod errors {
        pub const FILE_NOT_FOUND: &str = "Error: File not found";
        pub const INVALID_INPUT: &str = "Error: Invalid input format";
    }

    pub mod ui {
        pub const HELP_HEADER: &str = "OVERVIEW:";
        pub const HELP_USAGE: &str = "USAGE:";
    }
}

// src/main.rs
use crate::strings::{errors, ui};

fn main() {
    println!("{} {}", ui::HELP_HEADER, "Boxy v1.0");
    eprintln!("{}", errors::FILE_NOT_FOUND);
}
```

### Advanced: Lazy Static String Tables
```rust
// Cargo.toml
[dependencies]
lazy_static = "1.4"
phf = { version = "0.11", features = ["macros"] }

// src/strings.rs
use lazy_static::lazy_static;
use phf::phf_map;

lazy_static! {
    pub static ref STRINGS: phf::Map<&'static str, &'static str> = phf_map! {
        "error.file_not_found" => "File not found",
        "error.invalid_input" => "Invalid input",
        "ui.help_header" => "OVERVIEW:",
    };
}

// Usage
println!("{}", STRINGS["ui.help_header"]);
```

## 2. Compile-Time String Loading (include_str!)

Load strings at compile time from external files:

```rust
// strings/en/help.txt
Boxy - A powerful CLI tool for boxing text

// src/main.rs
const HELP_TEXT: &str = include_str!("../strings/en/help.txt");
const ERROR_MSGS: &str = include_str!("../strings/en/errors.txt");

// Parse at runtime if needed
lazy_static! {
    static ref ERRORS: HashMap<&'static str, &'static str> = {
        ERROR_MSGS.lines()
            .filter_map(|line| {
                let parts: Vec<_> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    Some((parts[0], parts[1]))
                } else {
                    None
                }
            })
            .collect()
    };
}
```

## 3. Runtime String Loading (Dynamic, Smaller Binary)

Load strings at runtime - they're NOT in the binary:

```rust
// src/strings.rs
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct StringTable {
    pub errors: HashMap<String, String>,
    pub ui: HashMap<String, String>,
}

impl StringTable {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Try multiple locations
        let paths = [
            "./strings.toml",
            "~/.config/boxy/strings.toml",
            "/usr/share/boxy/strings.toml",
        ];

        for path in &paths {
            if let Ok(content) = std::fs::read_to_string(path) {
                return Ok(toml::from_str(&content)?);
            }
        }

        // Fall back to embedded defaults
        Ok(Self::default())
    }
}
```

## 4. String Obfuscation (Weak Security, But Hides from `strings`)

Use `obfstr` crate for compile-time XOR encryption:

```rust
// Cargo.toml
[dependencies]
obfstr = "0.4"

// src/main.rs
use obfstr::{obfstr, obfstr_impl};

// Strings are XOR'd at compile time
const HIDDEN: &str = obfstr!("This won't show in strings output");

fn main() {
    println!("{}", HIDDEN);  // Decrypted at runtime
}
```

**Note**: This only stops casual `strings` inspection, NOT real attackers!

## 5. Secure Token/Secret Patterns

### NEVER embed secrets! Use these patterns instead:

### Pattern 1: Environment Variables
```rust
use std::env;

fn get_api_key() -> Result<String, String> {
    env::var("BOXY_API_KEY")
        .map_err(|_| "BOXY_API_KEY not set".to_string())
}
```

### Pattern 2: Config Files (with permissions)
```rust
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn load_secret() -> Result<String, Box<dyn std::error::Error>> {
    let path = dirs::config_dir()
        .ok_or("No config dir")?
        .join("boxy/secrets.toml");

    // Check file permissions (Unix)
    let metadata = fs::metadata(&path)?;
    let permissions = metadata.permissions();
    if permissions.mode() & 0o077 != 0 {
        return Err("Secret file has insecure permissions (should be 0600)".into());
    }

    Ok(fs::read_to_string(path)?)
}
```

### Pattern 3: System Keyring
```rust
// Cargo.toml
[dependencies]
keyring = "2.0"

// src/main.rs
use keyring::Entry;

fn get_token() -> Result<String, keyring::Error> {
    let entry = Entry::new("boxy", "api_token")?;
    entry.get_password()
}

fn store_token(token: &str) -> Result<(), keyring::Error> {
    let entry = Entry::new("boxy", "api_token")?;
    entry.set_password(token)
}
```

### Pattern 4: Encrypted at Rest
```rust
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce, Key
};

fn decrypt_token(encrypted: &[u8], key: &[u8; 32]) -> Result<String, Box<dyn std::error::Error>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(&encrypted[..12]);
    let ciphertext = &encrypted[12..];

    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))?;

    Ok(String::from_utf8(plaintext)?)
}
```

## 6. Build-Time String Reduction Patterns

### Pattern 1: Feature Flags for Strings
```rust
// Cargo.toml
[features]
minimal = []
full = ["help", "themes", "colors"]
help = []

// src/strings.rs
#[cfg(feature = "help")]
pub const HELP_TEXT: &str = include_str!("../help.txt");

#[cfg(not(feature = "help"))]
pub const HELP_TEXT: &str = "Help not included in minimal build";
```

### Pattern 2: Const Generics for String Tables
```rust
// Compile-time string lookup without HashMap overhead
const STRINGS: &[(&str, &str)] = &[
    ("error.file", "File not found"),
    ("error.input", "Invalid input"),
];

const fn lookup(key: &str) -> &'static str {
    let mut i = 0;
    while i < STRINGS.len() {
        if const_str_eq(STRINGS[i].0, key) {
            return STRINGS[i].1;
        }
        i += 1;
    }
    "Unknown string"
}
```

## 7. String Compression Pattern

Store compressed strings, decompress on demand:

```rust
use flate2::read::GzDecoder;
use std::io::Read;

// Compressed at build time
const HELP_GZ: &[u8] = include_bytes!("../help.txt.gz");

lazy_static! {
    static ref HELP_TEXT: String = {
        let mut decoder = GzDecoder::new(HELP_GZ);
        let mut s = String::new();
        decoder.read_to_string(&mut s).unwrap();
        s
    };
}
```

## Summary: Best Practices

### For Regular Strings:
1. **Centralize strings** in a module
2. **Use const/static** for compile-time strings
3. **Consider i18n** early with string tables
4. **Feature-flag** optional strings

### For Sensitive Data (Tokens/Keys):
1. **NEVER embed in binary** - it's not secure!
2. **Use environment variables** for CI/CD
3. **Use config files** with proper permissions (0600)
4. **Use system keyring** for desktop apps
5. **Encrypt at rest** if you must store them

### For Distribution:
1. **External string files** for i18n
2. **Compressed strings** for large text
3. **Runtime loading** for optional features
4. **Obfuscation** only for casual hiding (not security!)

### What NOT to do:
```rust
// ❌ NEVER: Tokens in source
const TOKEN: &str = "secret-key-123";

// ❌ NEVER: Obfuscation as "security"
const KEY: &str = obfstr!("secret");  // Still extractable!

// ❌ NEVER: Base64 "encoding" as security
const KEY_B64: &str = "c2VjcmV0LWtleS0xMjM=";  // Trivial to decode

// ❌ NEVER: XOR "encryption" with hardcoded key
const ENCRYPTED: &[u8] = &[0x12, 0x34, ...];  // Easy to break
```

Remember: If it's compiled into the binary, assume it's public!