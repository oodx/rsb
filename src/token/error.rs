//! Token processing errors - ported from XStream.

use std::fmt;

/// Errors that can occur during token bucket operations.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenBucketError {
    /// Failed to parse the input string into tokens
    ParseError(String),
    /// A token was missing required parts (like no '=' separator)
    MalformedToken(String),
    /// Empty input string
    EmptyInput,
    /// Invalid namespace format
    InvalidNamespace(String),
    /// Generic error with context
    Generic(String),
}

impl fmt::Display for TokenBucketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenBucketError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            TokenBucketError::MalformedToken(token) => write!(f, "Malformed token: '{}'", token),
            TokenBucketError::EmptyInput => write!(f, "Input string is empty"),
            TokenBucketError::InvalidNamespace(ns) => write!(f, "Invalid namespace: '{}'", ns),
            TokenBucketError::Generic(msg) => write!(f, "TokenBucket error: {}", msg),
        }
    }
}

impl std::error::Error for TokenBucketError {}

/// Convenience conversion from string parse errors.
impl From<String> for TokenBucketError {
    fn from(msg: String) -> Self {
        TokenBucketError::ParseError(msg)
    }
}

impl From<&str> for TokenBucketError {
    fn from(msg: &str) -> Self {
        TokenBucketError::ParseError(msg.to_string())
    }
}

/// Result type for token bucket operations.
pub type TokenBucketResult<T> = Result<T, TokenBucketError>;
