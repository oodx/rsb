//! Token parsing functions - ported from XStream.
//!
//! Provides robust parsing of key=value token streams with validation,
//! quote stripping, and comprehensive error handling.

use super::types::{Token, Namespace, TokenError, TokenResult, TokenStreamable};

/// Strip quotes from a value string.
///
/// Removes matching single or double quotes from the beginning and end of a string.
/// Used internally by the tokenization process.
fn strip_quotes(s: &str) -> String {
    let s = s.trim();
    if s.len() >= 2 {
        if (s.starts_with('"') && s.ends_with('"')) ||
           (s.starts_with('\'') && s.ends_with('\'')) {
            s[1..s.len()-1].to_string()
        } else {
            s.to_string()
        }
    } else {
        s.to_string()
    }
}

/// Parse a string into a vector of tokens.
///
/// This is the core tokenization function ported from XStream. It handles:
/// - Semicolon-separated token format
/// - Quote stripping from values
/// - Namespace parsing (ns:key=value)
/// - Strict validation of spacing rules
/// - Comprehensive error messages
///
/// # Format Rules
/// - Tokens are separated by semicolons (;)
/// - Format: `key=value` or `namespace:key=value`
/// - Values can be quoted with single or double quotes
/// - No spaces allowed around = or before ;
/// - Spaces allowed after ; and at start of input
/// - No spaces allowed in keys or namespaces
///
/// # Examples
/// ```
/// use rsb::token::tokenize_string;
///
/// let tokens = tokenize_string(r#"host="localhost"; db:user="admin";"#).unwrap();
/// assert_eq!(tokens.len(), 2);
/// assert_eq!(tokens[0].key, "host");
/// assert_eq!(tokens[0].value, "localhost"); // quotes stripped
/// ```
pub fn tokenize_string(input: &str) -> TokenResult<Vec<Token>> {
    if input.trim().is_empty() {
        return Err(TokenError::EmptyInput);
    }

    let mut tokens = Vec::new();

    for token_str in input.split(';') {
        // Only trim leading spaces (allow space after ;) but not trailing spaces (no space before ;)
        let token_str = token_str.trim_start();
        if token_str.is_empty() { continue; }

        // Check for trailing spaces (space before ;)
        if token_str != token_str.trim_end() {
            return Err(TokenError::MalformedToken {
                token: token_str.trim_end().to_string(),
                reason: "trailing spaces not allowed".to_string(),
            });
        }

        // Split on first '='
        let (key_part, value_part) = match token_str.split_once('=') {
            Some((k, v)) => (k, v),
            None => {
                // More specific error for malformed tokens
                if !token_str.trim().is_empty() {
                    return Err(TokenError::MalformedToken {
                        token: token_str.to_string(),
                        reason: "missing '=' separator".to_string(),
                    });
                }
                continue;
            },
        };

        // Check for spaces around '=' - key should not have trailing spaces, value should not have leading spaces
        if key_part != key_part.trim_end() {
            return Err(TokenError::MalformedToken {
                token: token_str.to_string(),
                reason: "space before '=' not allowed".to_string(),
            });
        }
        if value_part != value_part.trim_start() {
            return Err(TokenError::MalformedToken {
                token: token_str.to_string(),
                reason: "space after '=' not allowed".to_string(),
            });
        }

        let key_part = key_part.trim(); // Allow leading spaces in key for consistency
        let value = strip_quotes(value_part);

        // Check for empty key
        if key_part.is_empty() {
            return Err(TokenError::MalformedToken {
                token: token_str.to_string(),
                reason: "empty key".to_string(),
            });
        }

        // Check for namespace separator ':'
        let (namespace, key) = match key_part.split_once(':') {
            Some((ns, k)) => {
                // Validate namespace - no spaces allowed
                if ns.contains(' ') {
                    return Err(TokenError::MalformedToken {
                        token: token_str.to_string(),
                        reason: format!("spaces not allowed in namespace '{}'", ns),
                    });
                }
                // Validate key part - no spaces allowed
                if k.contains(' ') {
                    return Err(TokenError::MalformedToken {
                        token: token_str.to_string(),
                        reason: format!("spaces not allowed in key '{}'", k),
                    });
                }
                // Parse namespace with its internal delimiter
                let namespace = Namespace::from_string(ns);
                (Some(namespace), k.to_string())
            },
            None => {
                // Even non-prefixed keys shouldn't have spaces
                if key_part.contains(' ') {
                    return Err(TokenError::MalformedToken {
                        token: token_str.to_string(),
                        reason: format!("spaces not allowed in key '{}'", key_part),
                    });
                }
                (None, key_part.to_string())
            },
        };

        tokens.push(Token { namespace, key, value });
    }

    if tokens.is_empty() {
        return Err(TokenError::ParseError {
            reason: "no valid tokens found".to_string(),
        });
    }

    Ok(tokens)
}

/// Validate if a string can be successfully tokenized.
///
/// Returns true if the input can be parsed without errors, false otherwise.
/// This is more efficient than full parsing when you only need validation.
///
/// # Examples
/// ```
/// use rsb::token::is_token_streamable;
///
/// assert!(is_token_streamable(r#"host="localhost";"#));
/// assert!(!is_token_streamable("invalid token")); // missing =
/// ```
pub fn is_token_streamable(input: &str) -> bool {
    tokenize_string(input).is_ok()
}

/// Implementation of TokenStreamable trait for string slices.
impl TokenStreamable for str {
    fn tokenize(&self) -> Result<Vec<Token>, String> {
        tokenize_string(self).map_err(|e| e.to_string())
    }

    fn validate(&self) -> Result<(), String> {
        tokenize_string(self).map(|_| ()).map_err(|e| e.to_string())
    }
}

/// Implementation of TokenStreamable trait for String.
impl TokenStreamable for String {
    fn tokenize(&self) -> Result<Vec<Token>, String> {
        self.as_str().tokenize()
    }

    fn validate(&self) -> Result<(), String> {
        self.as_str().validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let tokens = tokenize_string(r#"host="localhost"; port="8080";"#).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].key, "host");
        assert_eq!(tokens[0].value, "localhost");
        assert!(tokens[0].namespace.is_none());
        assert_eq!(tokens[1].key, "port");
        assert_eq!(tokens[1].value, "8080");
    }

    #[test]
    fn test_namespace_tokenization() {
        let tokens = tokenize_string(r#"db:user="admin"; db:pass="secret";"#).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].key, "user");
        assert_eq!(tokens[0].value, "admin");
        assert!(tokens[0].namespace.is_some());
        assert_eq!(tokens[0].namespace.as_ref().unwrap().to_string(), "db");
    }

    #[test]
    fn test_quote_stripping() {
        let tokens = tokenize_string(r#"key1="value1"; key2='value2'; key3=unquoted;"#).unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].value, "value1");  // double quotes stripped
        assert_eq!(tokens[1].value, "value2");  // single quotes stripped
        assert_eq!(tokens[2].value, "unquoted"); // no quotes to strip
    }

    #[test]
    fn test_empty_input() {
        assert!(matches!(tokenize_string(""), Err(TokenError::EmptyInput)));
        assert!(matches!(tokenize_string("   "), Err(TokenError::EmptyInput)));
    }

    #[test]
    fn test_malformed_token_missing_equals() {
        let result = tokenize_string("bad_token");
        assert!(matches!(result, Err(TokenError::MalformedToken { .. })));
        if let Err(TokenError::MalformedToken { reason, .. }) = result {
            assert!(reason.contains("missing '=' separator"));
        }
    }

    #[test]
    fn test_spacing_rules() {
        // ✅ ALLOWED: Space after semicolon
        assert!(tokenize_string(r#"k1="val1"; k2="val2";"#).is_ok());

        // ❌ NOT ALLOWED: Space before semicolon
        let result = tokenize_string(r#"k1="val1" ;k2="val2";"#);
        assert!(matches!(result, Err(TokenError::MalformedToken { .. })));

        // ❌ NOT ALLOWED: Space after equals
        let result = tokenize_string(r#"k1= "val1";"#);
        assert!(matches!(result, Err(TokenError::MalformedToken { .. })));

        // ❌ NOT ALLOWED: Space before equals
        let result = tokenize_string(r#"k1 ="val1";"#);
        assert!(matches!(result, Err(TokenError::MalformedToken { .. })));
    }

    #[test]
    fn test_no_spaces_in_keys() {
        // ❌ NOT ALLOWED: Space in namespace
        let result = tokenize_string(r#"my namespace:key="value";"#);
        assert!(matches!(result, Err(TokenError::MalformedToken { .. })));

        // ❌ NOT ALLOWED: Space in key part
        let result = tokenize_string(r#"ns:my key="value";"#);
        assert!(matches!(result, Err(TokenError::MalformedToken { .. })));

        // ❌ NOT ALLOWED: Space in plain key
        let result = tokenize_string(r#"my key="value";"#);
        assert!(matches!(result, Err(TokenError::MalformedToken { .. })));
    }

    #[test]
    fn test_is_token_streamable() {
        // ✅ Valid inputs
        assert!(is_token_streamable(r#"host="localhost";"#));
        assert!(is_token_streamable(r#"k1="v1"; k2="v2";"#));
        assert!(is_token_streamable(r#"ns:key="value";"#));

        // ❌ Invalid inputs
        assert!(!is_token_streamable("bad_token"));
        assert!(!is_token_streamable("=empty_key"));
        assert!(!is_token_streamable(r#"k1= "val1";"#));
    }

    #[test]
    fn test_tokenStreamable_trait() {
        let input = r#"host="localhost"; db:user="admin";"#;
        let tokens = input.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);

        assert!(input.validate().is_ok());
        assert!("bad_token".validate().is_err());
    }
}