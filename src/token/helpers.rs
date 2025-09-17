//! Internal token processing helpers.
//!
//! Implementation details for the token module. These functions support
//! the public API but are not exposed directly to consumers.

use super::types::{Namespace, TokenError};

/// Internal helper for quote stripping.
pub(crate) fn strip_quotes_internal(s: &str) -> String {
    let s = s.trim();
    if s.len() >= 2 {
        if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
            s[1..s.len() - 1].to_string()
        } else {
            s.to_string()
        }
    } else {
        s.to_string()
    }
}

/// Internal helper for token format validation.
pub(crate) fn validate_token_format(
    token_str: &str,
    key_part: &str,
    value_part: &str,
) -> Result<(), TokenError> {
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

    Ok(())
}

/// Internal helper for namespace and key validation.
pub(crate) fn validate_key_parts(
    token_str: &str,
    key_part: &str,
) -> Result<(Option<Namespace>, String), TokenError> {
    // Check for empty key
    if key_part.is_empty() {
        return Err(TokenError::MalformedToken {
            token: token_str.to_string(),
            reason: "empty key".to_string(),
        });
    }

    // Check for namespace separator ':'
    match key_part.split_once(':') {
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
            Ok((Some(namespace), k.to_string()))
        }
        None => {
            // Even non-prefixed keys shouldn't have spaces
            if key_part.contains(' ') {
                return Err(TokenError::MalformedToken {
                    token: token_str.to_string(),
                    reason: format!("spaces not allowed in key '{}'", key_part),
                });
            }
            Ok((None, key_part.to_string()))
        }
    }
}

/// Internal helper for trailing space validation.
pub(crate) fn validate_no_trailing_spaces(token_str: &str) -> Result<(), TokenError> {
    // Check for trailing spaces (space before ;)
    if token_str != token_str.trim_end() {
        return Err(TokenError::MalformedToken {
            token: token_str.trim_end().to_string(),
            reason: "trailing spaces not allowed".to_string(),
        });
    }
    Ok(())
}
