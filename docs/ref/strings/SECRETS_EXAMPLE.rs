// EXAMPLE: Secure Token Handling for Boxy
// This file shows HOW to handle secrets. DO NOT commit actual secrets!

use std::env;
use std::fs;
use std::path::PathBuf;

/// Secure token storage patterns
pub struct TokenManager;

impl TokenManager {
    /// Method 1: Environment Variable (Best for CI/CD)
    pub fn from_env() -> Result<String, String> {
        env::var("BOXY_SECRET_TOKEN")
            .map_err(|_| "BOXY_SECRET_TOKEN not set in environment".to_string())
    }

    /// Method 2: Secure Config File
    pub fn from_config_file() -> Result<String, Box<dyn std::error::Error>> {
        let config_path = dirs::config_dir()
            .ok_or("No config directory")?
            .join("boxy/secrets.toml");

        // Check permissions (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&config_path)?;
            let mode = metadata.permissions().mode();
            if mode & 0o077 != 0 {
                return Err(format!(
                    "Insecure permissions on {}. Run: chmod 600 {}",
                    config_path.display(),
                    config_path.display()
                ).into());
            }
        }

        let content = fs::read_to_string(&config_path)?;
        // Parse your token from TOML/JSON/etc
        Ok(content.trim().to_string())
    }

    /// Method 3: System Keyring (Desktop apps)
    #[cfg(feature = "keyring")]
    pub fn from_keyring() -> Result<String, keyring::Error> {
        let entry = keyring::Entry::new("boxy", "api_token")?;
        entry.get_password()
    }

    /// Method 4: Prompt user (interactive)
    pub fn from_prompt() -> Result<String, std::io::Error> {
        use std::io::{self, Write};

        print!("Enter token (input hidden): ");
        io::stdout().flush()?;

        // Use rpassword crate for hidden input
        rpassword::read_password()
    }

    /// Method 5: Temporary in-memory only (never persisted)
    pub fn generate_session_token() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Generate random token that lives only in memory
        (0..32)
            .map(|_| {
                let idx = rng.gen_range(0..62);
                b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"[idx] as char
            })
            .collect()
    }
}

/// Example: How to use tokens securely
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Try multiple sources in order of preference
    let token = TokenManager::from_env()
        .or_else(|_| TokenManager::from_config_file())
        .or_else(|_| {
            #[cfg(feature = "keyring")]
            return TokenManager::from_keyring();
            #[cfg(not(feature = "keyring"))]
            return Err("No token source available");
        })
        .or_else(|_| {
            eprintln!("No token found in environment or config.");
            TokenManager::from_prompt()
        })?;

    // Use the token (but NEVER log it!)
    println!("Token loaded (length: {})", token.len());

    // When you need to use it:
    make_api_call(&token)?;

    // Clear from memory when done (Rust does this automatically when dropped,
    // but you can be explicit)
    drop(token);

    Ok(())
}

fn make_api_call(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Use the token but NEVER:
    // - Print it
    // - Log it
    // - Include it in error messages
    // - Store it in files without encryption

    println!("Making API call with token...");
    // actual API call here

    Ok(())
}

// ============================================================================
// WHAT NOT TO DO - These patterns will expose your secrets!
// ============================================================================

#[allow(dead_code)]
mod bad_examples {
    // ❌ NEVER: Hardcoded secrets
    const API_KEY: &str = "sk-1234567890abcdef";  // Visible with `strings binary`

    // ❌ NEVER: "Obfuscated" secrets (trivial to defeat)
    const KEY_XOR: &[u8] = &[0x73 ^ 0x42, 0x6b ^ 0x42];  // Just XOR with 0x42 to get "sk"

    // ❌ NEVER: Base64 "encoding" (not encryption!)
    const KEY_B64: &str = "c2stMTIzNDU2Nzg5MGFiY2RlZg==";  // base64 decode = "sk-1234567890abcdef"

    // ❌ NEVER: Include in error messages
    fn bad_error_handling(token: &str) {
        panic!("API call failed with token: {}", token);  // Token leaked in panic!
    }

    // ❌ NEVER: Log tokens
    fn bad_logging(token: &str) {
        log::debug!("Using token: {}", token);  // Token in logs!
    }
}

// ============================================================================
// If you MUST embed something (non-secret config only!)
// ============================================================================

/// For non-secret configuration that needs to be in the binary
mod embedded_config {
    use obfstr::obfstr;  // Only hides from casual `strings`, NOT secure!

    // This is NOT for secrets, just for hiding non-sensitive strings
    const APP_ID: &str = obfstr!("boxy-app-2025");  // Won't show in `strings` output

    // For actual configuration, use lazy_static with defaults
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    lazy_static! {
        static ref CONFIG: HashMap<&'static str, &'static str> = {
            let mut m = HashMap::new();
            m.insert("version", "0.23.0");
            m.insert("update_url", obfstr!("https://api.example.com/update"));
            m.insert("telemetry", "disabled");  // Non-sensitive defaults
            m
        };
    }
}