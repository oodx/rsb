//! Token module sanity tests - core functionality validation

use rsb::token::{is_token_streamable, tokenize_string, Namespace, Token, TokenStreamable};

#[test]
fn test_basic_token_parsing() {
    // Test basic key=value parsing
    let tokens = tokenize_string(r#"host="localhost"; port="8080";"#).unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].key, "host");
    assert_eq!(tokens[0].value, "localhost");
    assert_eq!(tokens[1].key, "port");
    assert_eq!(tokens[1].value, "8080");
}

#[test]
fn test_namespace_support() {
    // Test namespace:key=value format
    let tokens =
        tokenize_string(r#"db:user="admin"; db:pass="secret"; auth:token="xyz";"#).unwrap();
    assert_eq!(tokens.len(), 3);

    // Check db namespace tokens
    assert_eq!(tokens[0].key, "user");
    assert_eq!(tokens[0].value, "admin");
    assert!(tokens[0].namespace.is_some());
    assert_eq!(tokens[0].namespace.as_ref().unwrap().to_string(), "db");

    // Check auth namespace token
    assert_eq!(tokens[2].key, "token");
    assert_eq!(tokens[2].namespace.as_ref().unwrap().to_string(), "auth");
}

#[test]
fn test_quote_stripping() {
    // Test that quotes are properly stripped from values
    let tokens =
        tokenize_string(r#"key1="double quoted"; key2='single quoted'; key3=unquoted;"#).unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].value, "double quoted"); // Double quotes stripped
    assert_eq!(tokens[1].value, "single quoted"); // Single quotes stripped
    assert_eq!(tokens[2].value, "unquoted"); // No quotes to strip
}

#[test]
fn test_hierarchical_namespaces() {
    // Test dot-separated hierarchical namespaces
    let tokens =
        tokenize_string(r#"config.db:host="localhost"; config.auth:enabled="true";"#).unwrap();
    assert_eq!(tokens.len(), 2);

    assert_eq!(
        tokens[0].namespace.as_ref().unwrap().to_string(),
        "config.db"
    );
    assert_eq!(
        tokens[1].namespace.as_ref().unwrap().to_string(),
        "config.auth"
    );
}

#[test]
fn test_validation_functions() {
    // Test validation helpers
    assert!(is_token_streamable(r#"valid="token";"#));
    assert!(is_token_streamable(r#"key1="value1"; key2="value2";"#));
    assert!(is_token_streamable(r#"ns:key="value";"#));

    // Test invalid formats
    assert!(!is_token_streamable("missing_equals"));
    assert!(!is_token_streamable("key = value")); // spaces around =
    assert!(!is_token_streamable("key=value ")); // trailing space
}

#[test]
fn test_token_streamable_trait() {
    // Test trait implementation
    let input = r#"host="localhost"; db:user="admin";"#;
    let tokens = input.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);

    // Test validation method
    assert!(input.validate().is_ok());
    assert!("invalid".validate().is_err());
}

#[test]
fn test_error_handling() {
    use rsb::token::TokenError;

    // Test specific error types
    let result = tokenize_string("");
    assert!(matches!(result, Err(TokenError::EmptyInput)));

    let result = tokenize_string("bad_token");
    assert!(matches!(result, Err(TokenError::MalformedToken { .. })));

    let result = tokenize_string("key = value"); // space before =
    assert!(matches!(result, Err(TokenError::MalformedToken { .. })));
}

#[test]
fn test_token_construction() {
    // Test Token creation methods
    let token = Token::simple("host", "localhost");
    assert_eq!(token.key, "host");
    assert_eq!(token.value, "localhost");
    assert!(token.namespace.is_none());
    assert_eq!(token.to_string(), "host=localhost");

    // Test namespaced token
    let ns = Namespace::from_string("db");
    let token = Token::with_namespace(ns, "user".to_string(), "admin".to_string());
    assert_eq!(token.to_string(), "db:user=admin");
}

#[test]
fn test_round_trip_processing() {
    // Test that we can parse and reconstruct tokens
    let original = r#"host="localhost"; db:user="admin"; config.auth:enabled="true";"#;
    let tokens = tokenize_string(original).unwrap();

    // Reconstruct
    let reconstructed: Vec<String> = tokens.iter().map(|t| t.to_string()).collect();
    let joined = reconstructed.join("; ");

    // Parse again
    let reparsed = tokenize_string(&joined).unwrap();
    assert_eq!(tokens.len(), reparsed.len());

    for (orig, reparsed) in tokens.iter().zip(reparsed.iter()) {
        assert_eq!(orig.key, reparsed.key);
        assert_eq!(orig.value, reparsed.value);
        assert_eq!(orig.namespace, reparsed.namespace);
    }
}
