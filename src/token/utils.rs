//! Curated token processing utilities.
//!
//! This module provides stable, public token processing functions for consumers
//! who prefer an explicit `token::utils` namespace.
//!
//! For general usage, prefer importing from the main `token` module.

pub use super::parse::{is_token_streamable, tokenize_string};
pub use super::types::*;

/// Convenience function for creating a simple token without namespace.
///
/// # Examples
/// ```
/// use rsb::token::utils::make_token;
///
/// let token = make_token("host", "localhost");
/// assert_eq!(token.key, "host");
/// assert_eq!(token.value, "localhost");
/// assert!(token.namespace.is_none());
/// ```
pub fn make_token(key: &str, value: &str) -> Token {
    Token::simple(key, value)
}

/// Convenience function for creating a token with namespace.
///
/// # Examples
/// ```
/// use rsb::token::utils::make_namespaced_token;
///
/// let token = make_namespaced_token("db", "host", "localhost");
/// assert_eq!(token.key, "host");
/// assert_eq!(token.value, "localhost");
/// assert_eq!(token.namespace.as_ref().unwrap().to_string(), "db");
/// ```
pub fn make_namespaced_token(namespace: &str, key: &str, value: &str) -> Token {
    let ns = Namespace::from_string(namespace);
    Token::with_namespace(ns, key.to_string(), value.to_string())
}

/// Extract all tokens from a given namespace.
///
/// Returns tokens that match the specified namespace, or tokens with no namespace
/// if namespace_name is None.
///
/// # Examples
/// ```
/// use rsb::token::utils::{tokenize_string, extract_namespace_tokens};
///
/// let tokens = tokenize_string(r#"host="localhost"; db:user="admin"; db:pass="secret";"#).unwrap();
/// let db_tokens = extract_namespace_tokens(&tokens, Some("db"));
/// assert_eq!(db_tokens.len(), 2);
/// ```
pub fn extract_namespace_tokens<'a>(
    tokens: &'a [Token],
    namespace_name: Option<&str>,
) -> Vec<&'a Token> {
    tokens
        .iter()
        .filter(|token| match (&token.namespace, namespace_name) {
            (None, None) => true,
            (Some(ns), Some(name)) => ns.to_string() == name,
            _ => false,
        })
        .collect()
}

/// Get all unique namespace names from a token collection.
///
/// # Examples
/// ```
/// use rsb::token::utils::{tokenize_string, get_namespace_names};
///
/// let tokens = tokenize_string(r#"host="localhost"; db:user="admin"; auth:token="xyz";"#).unwrap();
/// let namespaces = get_namespace_names(&tokens);
/// assert!(namespaces.contains(&"db".to_string()));
/// assert!(namespaces.contains(&"auth".to_string()));
/// ```
pub fn get_namespace_names(tokens: &[Token]) -> Vec<String> {
    let mut namespaces: Vec<String> = tokens
        .iter()
        .filter_map(|token| token.namespace.as_ref().map(|ns| ns.to_string()))
        .collect();
    namespaces.sort();
    namespaces.dedup();
    namespaces
}

/// Convert a token back to its string representation.
///
/// This is equivalent to calling `token.to_string()` but provides a consistent
/// function-based API.
///
/// # Examples
/// ```
/// use rsb::token::utils::{make_token, token_to_string};
///
/// let token = make_token("host", "localhost");
/// assert_eq!(token_to_string(&token), "host=localhost");
/// ```
pub fn token_to_string(token: &Token) -> String {
    token.to_string()
}

/// Convert a collection of tokens to a semicolon-separated string.
///
/// # Examples
/// ```
/// use rsb::token::utils::{make_token, tokens_to_string};
///
/// let tokens = vec![
///     make_token("host", "localhost"),
///     make_token("port", "8080"),
/// ];
/// let result = tokens_to_string(&tokens);
/// assert_eq!(result, "host=localhost; port=8080");
/// ```
pub fn tokens_to_string(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(|token| token.to_string())
        .collect::<Vec<String>>()
        .join("; ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_token() {
        let token = make_token("test", "value");
        assert_eq!(token.key, "test");
        assert_eq!(token.value, "value");
        assert!(token.namespace.is_none());
    }

    #[test]
    fn test_make_namespaced_token() {
        let token = make_namespaced_token("db", "host", "localhost");
        assert_eq!(token.key, "host");
        assert_eq!(token.value, "localhost");
        assert_eq!(token.namespace.as_ref().unwrap().to_string(), "db");
    }

    #[test]
    fn test_extract_namespace_tokens() {
        let tokens =
            tokenize_string(r#"host="localhost"; db:user="admin"; db:pass="secret";"#).unwrap();

        let db_tokens = extract_namespace_tokens(&tokens, Some("db"));
        assert_eq!(db_tokens.len(), 2);

        let global_tokens = extract_namespace_tokens(&tokens, None);
        assert_eq!(global_tokens.len(), 1);
        assert_eq!(global_tokens[0].key, "host");
    }

    #[test]
    fn test_get_namespace_names() {
        let tokens = tokenize_string(
            r#"host="localhost"; db:user="admin"; auth:token="xyz"; db:pass="secret";"#,
        )
        .unwrap();
        let namespaces = get_namespace_names(&tokens);
        assert_eq!(namespaces, vec!["auth", "db"]);
    }

    #[test]
    fn test_tokens_to_string() {
        let tokens = vec![
            make_token("host", "localhost"),
            make_namespaced_token("db", "user", "admin"),
        ];
        let result = tokens_to_string(&tokens);
        assert_eq!(result, "host=localhost; db:user=admin");
    }
}
