//! Token types for RSB - ported from XStream.
//!
//! Provides generic key=value token processing with optional namespace support.
//! This is the low-level, namespace-agnostic foundation that XStream builds upon.

use std::fmt;
use std::str::FromStr;

/// Hierarchical namespace for organizing tokens.
///
/// Namespaces use dot notation (e.g., "db.config", "auth.session") and support
/// hierarchical organization without the bucket/stream semantics from XStream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Namespace {
    pub parts: Vec<String>,
    pub delimiter: char,
}

impl Namespace {
    /// Default delimiter for namespace parts.
    pub const DELIMITER: char = '.';

    /// Create a new namespace from parts.
    pub fn new(parts: Vec<String>) -> Self {
        Namespace {
            parts,
            delimiter: Self::DELIMITER,
        }
    }

    /// Create namespace from string using default delimiter.
    pub fn from_string(s: &str) -> Self {
        Self::from_str_with_delimiter(s, Self::DELIMITER)
    }

    /// Create namespace from string with custom delimiter.
    pub fn from_str_with_delimiter(s: &str, delimiter: char) -> Self {
        Namespace {
            parts: s.split(delimiter).map(|s| s.to_string()).collect(),
            delimiter,
        }
    }
}

impl FromStr for Namespace {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Namespace::from_string(s))
    }
}

impl fmt::Display for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.parts.join(&self.delimiter.to_string()))
    }
}

/// A token representing a key=value pair with optional namespace.
///
/// Examples:
/// - `host=localhost` (no namespace)
/// - `db:host=localhost` (with namespace)
/// - `config.db:host=localhost` (hierarchical namespace)
#[derive(Debug, Clone)]
pub struct Token {
    /// Optional namespace for the token
    pub namespace: Option<Namespace>,
    /// The key part of the key=value pair
    pub key: String,
    /// The value part of the key=value pair
    pub value: String,
}

impl Token {
    /// Create a new token.
    pub fn new(key: String, value: String) -> Self {
        Self {
            namespace: None,
            key,
            value,
        }
    }

    /// Create a new token with namespace.
    pub fn with_namespace(namespace: Namespace, key: String, value: String) -> Self {
        Self {
            namespace: Some(namespace),
            key,
            value,
        }
    }

    /// Create a simple token from key and value strings.
    pub fn simple(key: &str, value: &str) -> Self {
        Self::new(key.to_string(), value.to_string())
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.namespace {
            Some(ns) => write!(f, "{}:{}={}", ns, self.key, self.value),
            None => write!(f, "{}={}", self.key, self.value),
        }
    }
}

impl FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key_part, value) = s
            .split_once('=')
            .ok_or_else(|| "Token must contain '='".to_string())?;

        let (namespace, key) = match key_part.split_once(':') {
            Some((ns, k)) => (Some(Namespace::from_string(ns)), k.to_string()),
            None => (None, key_part.to_string()),
        };

        Ok(Token {
            namespace,
            key,
            value: value.to_string(),
        })
    }
}

/// Trait for types that can be converted to/from token streams.
pub trait TokenStreamable {
    /// Parse the input into a vector of tokens.
    fn tokenize(&self) -> Result<Vec<Token>, String>;
    /// Validate that the input can be tokenized successfully.
    fn validate(&self) -> Result<(), String>;
}

/// Errors that can occur during token processing.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenError {
    /// Input string is empty or contains only whitespace
    EmptyInput,
    /// Token is malformed (e.g., missing '=', invalid format)
    MalformedToken { token: String, reason: String },
    /// Parsing failed for other reasons
    ParseError { reason: String },
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenError::EmptyInput => write!(f, "Empty or whitespace-only input"),
            TokenError::MalformedToken { token, reason } => {
                write!(f, "Malformed token '{}': {}", token, reason)
            }
            TokenError::ParseError { reason } => write!(f, "Parse error: {}", reason),
        }
    }
}

impl std::error::Error for TokenError {}

/// Result type for token operations.
pub type TokenResult<T> = Result<T, TokenError>;

// Re-export bucket error types for convenience
pub use super::error::{TokenBucketError, TokenBucketResult};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_creation() {
        let ns = Namespace::from_string("db.config");
        assert_eq!(ns.parts, vec!["db", "config"]);
        assert_eq!(ns.delimiter, '.');
        assert_eq!(ns.to_string(), "db.config");
    }

    #[test]
    fn test_token_creation() {
        let token = Token::simple("host", "localhost");
        assert_eq!(token.key, "host");
        assert_eq!(token.value, "localhost");
        assert!(token.namespace.is_none());
        assert_eq!(token.to_string(), "host=localhost");
    }

    #[test]
    fn test_token_with_namespace() {
        let ns = Namespace::from_string("db");
        let token = Token::with_namespace(ns, "host".to_string(), "localhost".to_string());
        assert_eq!(token.to_string(), "db:host=localhost");
    }

    #[test]
    fn test_token_from_str() {
        let token = Token::from_str("host=localhost").unwrap();
        assert_eq!(token.key, "host");
        assert_eq!(token.value, "localhost");
        assert!(token.namespace.is_none());

        let token = Token::from_str("db:host=localhost").unwrap();
        assert_eq!(token.key, "host");
        assert_eq!(token.value, "localhost");
        assert!(token.namespace.is_some());
        assert_eq!(token.namespace.unwrap().to_string(), "db");
    }

    #[test]
    fn test_token_parse_error() {
        let result = Token::from_str("invalid_token");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must contain '='"));
    }
}
