// RSB Sanity Tests - Token Module Core Functionality Verification
// Tests verify the token module functions work as documented in FEATURES_TOKENS

use rsb::prelude::*;
use rsb::token::{is_token_streamable, tokenize_string, Token, TokenStreamable};

#[test]
fn test_basic_token_parsing() {
    // Test basic token parsing functionality

    let input = r#"key1="value1"; key2="value2";"#;
    let tokens = tokenize_string(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].key, "key1");
    assert_eq!(tokens[0].value, "value1"); // Quotes should be stripped
    assert_eq!(tokens[1].key, "key2");
    assert_eq!(tokens[1].value, "value2");
    assert!(tokens[0].namespace.is_none());
}

#[test]
fn test_token_with_namespaces() {
    // Test token parsing with namespaces

    let input = r#"db:host="localhost"; db:port="5432"; app:name="myapp";"#;
    let tokens = tokenize_string(input).unwrap();

    assert_eq!(tokens.len(), 3);

    // Check first token (db:host)
    assert!(tokens[0].namespace.is_some());
    let ns0 = tokens[0].namespace.as_ref().unwrap();
    assert_eq!(ns0.to_string(), "db");
    assert_eq!(tokens[0].key, "host");
    assert_eq!(tokens[0].value, "localhost");

    // Check second token (db:port)
    assert_eq!(tokens[1].namespace.as_ref().unwrap().to_string(), "db");
    assert_eq!(tokens[1].key, "port");
    assert_eq!(tokens[1].value, "5432");

    // Check third token (app:name)
    assert_eq!(tokens[2].namespace.as_ref().unwrap().to_string(), "app");
    assert_eq!(tokens[2].key, "name");
    assert_eq!(tokens[2].value, "myapp");
}

#[test]
fn test_hierarchical_namespaces() {
    // Test tokens with hierarchical (dotted) namespaces

    let input = r#"config.db:host="localhost"; auth.session:timeout="3600";"#;
    let tokens = tokenize_string(input).unwrap();

    assert_eq!(tokens.len(), 2);

    // First token should have "config.db" namespace
    let ns0 = tokens[0].namespace.as_ref().unwrap();
    assert_eq!(ns0.to_string(), "config.db");
    assert_eq!(tokens[0].key, "host");

    // Second token should have "auth.session" namespace
    let ns1 = tokens[1].namespace.as_ref().unwrap();
    assert_eq!(ns1.to_string(), "auth.session");
    assert_eq!(tokens[1].key, "timeout");
}

#[test]
fn test_quote_stripping() {
    // Test that quotes are properly stripped from values

    let input = r#"single='value'; double="value"; mixed="val'ue"; unquoted=value;"#;
    let tokens = tokenize_string(input).unwrap();

    assert_eq!(tokens[0].value, "value"); // Single quotes stripped
    assert_eq!(tokens[1].value, "value"); // Double quotes stripped
    assert_eq!(tokens[2].value, "val'ue"); // Inner quotes preserved
    assert_eq!(tokens[3].value, "value"); // No quotes to strip
}

#[test]
fn test_empty_values() {
    // Test tokens with empty values

    let input = r#"empty1=; empty2=""; empty3='';"#;
    let tokens = tokenize_string(input).unwrap();

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].value, "");
    assert_eq!(tokens[1].value, "");
    assert_eq!(tokens[2].value, "");
}

#[test]
fn test_is_token_streamable() {
    // Test token validation function

    // Valid formats
    assert!(is_token_streamable(r#"key="value";"#));
    assert!(is_token_streamable(r#"key1="val1"; key2="val2";"#));
    assert!(is_token_streamable(r#"ns:key="value";"#));
    assert!(is_token_streamable(r#"a.b.c:key="value";"#));

    // Invalid formats
    assert!(!is_token_streamable("invalid")); // No equals sign
    assert!(!is_token_streamable("key = value")); // Spaces around equals
    assert!(!is_token_streamable("my key=value")); // Space in key
    assert!(!is_token_streamable("my ns:key=value")); // Space in namespace
}

#[test]
fn test_token_creation() {
    // Test Token struct creation methods

    // Simple token
    let token1 = Token::simple("key", "value");
    assert_eq!(token1.key, "key");
    assert_eq!(token1.value, "value");
    assert!(token1.namespace.is_none());

    // Token with namespace
    use rsb::token::Namespace;
    let ns = Namespace::from_string("db.config");
    let token2 = Token::with_namespace(ns.clone(), "host".to_string(), "localhost".to_string());
    assert_eq!(token2.key, "host");
    assert_eq!(token2.value, "localhost");
    assert!(token2.namespace.is_some());
    assert_eq!(token2.namespace.unwrap().to_string(), "db.config");
}

#[test]
fn test_token_to_string() {
    // Test converting Token back to string format

    let token1 = Token::simple("key", "value");
    let str1 = token1.to_string();
    assert!(str1.contains("key"));
    assert!(str1.contains("value"));

    // Token with namespace
    use rsb::token::Namespace;
    let ns = Namespace::from_string("app");
    let token2 = Token::with_namespace(ns, "name".to_string(), "myapp".to_string());
    let str2 = token2.to_string();
    assert!(str2.contains("app"));
    assert!(str2.contains("name"));
    assert!(str2.contains("myapp"));
}

#[test]
fn test_token_streamable_trait() {
    // Test TokenStreamable trait implementation

    let input = r#"key1="value1"; key2="value2";"#;

    // Test validate method
    let validation = input.validate();
    assert!(validation.is_ok());

    // Test tokenize method
    let tokens = input.tokenize();
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 2);

    // Test with invalid input
    let invalid = "bad token";
    assert!(invalid.validate().is_err());
    assert!(invalid.tokenize().is_err());
}

#[test]
fn test_spaces_in_values() {
    // Test that spaces are allowed in quoted values

    let input = r#"message="Hello World"; path="/my path/to/file";"#;
    let tokens = tokenize_string(input).unwrap();

    assert_eq!(tokens[0].value, "Hello World");
    assert_eq!(tokens[1].value, "/my path/to/file");
}

#[test]
fn test_special_characters_in_values() {
    // Test special characters in values

    let input = r#"special="!@#$%^&*()"; unicode="你好世界"; emoji="🚀";"#;
    let tokens = tokenize_string(input).unwrap();

    assert_eq!(tokens[0].value, "!@#$%^&*()");
    assert_eq!(tokens[1].value, "你好世界");
    assert_eq!(tokens[2].value, "🚀");
}

#[test]
fn test_validation_errors() {
    // Test various validation error cases

    // Missing equals sign
    let result1 = tokenize_string("no_equals_here");
    assert!(result1.is_err());

    // Spaces around equals
    let result2 = tokenize_string("key = value");
    assert!(result2.is_err());

    // Spaces in key
    let result3 = tokenize_string("my key=value");
    assert!(result3.is_err());

    // Spaces in namespace
    let result4 = tokenize_string("my ns:key=value");
    assert!(result4.is_err());

    // Empty input
    let result5 = tokenize_string("");
    assert!(result5.is_err());

    // Whitespace only
    let result6 = tokenize_string("   ");
    assert!(result6.is_err());
}

#[test]
fn test_semicolon_handling() {
    // Test semicolon separator handling

    // Trailing semicolon is optional
    let tokens1 = tokenize_string(r#"key="value""#).unwrap();
    assert_eq!(tokens1.len(), 1);

    // Multiple semicolons
    let _tokens2 = tokenize_string(r#"key1="val1";; key2="val2";"#);
    // Behavior depends on implementation - test that it doesn't panic

    // Spaces after semicolon are allowed
    let tokens3 = tokenize_string(r#"key1="val1"; key2="val2";"#).unwrap();
    assert_eq!(tokens3.len(), 2);
}

#[test]
fn test_mixed_namespace_tokens() {
    // Test mixing namespaced and non-namespaced tokens

    let input = r#"global="value"; db:host="localhost"; another="value"; app:name="test";"#;
    let tokens = tokenize_string(input).unwrap();

    assert_eq!(tokens.len(), 4);
    assert!(tokens[0].namespace.is_none()); // global
    assert!(tokens[1].namespace.is_some()); // db:host
    assert!(tokens[2].namespace.is_none()); // another
    assert!(tokens[3].namespace.is_some()); // app:name
}

#[test]
fn test_edge_cases() {
    // Test edge cases and boundary conditions

    // Very long key
    let long_key = format!("{}=value", "k".repeat(1000));
    let result1 = tokenize_string(&long_key);
    assert!(result1.is_ok());

    // Very long value
    let long_value = format!(r#"key="{}""#, "v".repeat(1000));
    let result2 = tokenize_string(&long_value);
    assert!(result2.is_ok());

    // Many tokens
    let many_tokens = (0..100)
        .map(|i| format!(r#"key{}="value{}""#, i, i))
        .collect::<Vec<_>>()
        .join("; ");
    let result3 = tokenize_string(&many_tokens);
    assert!(result3.is_ok());
    assert_eq!(result3.unwrap().len(), 100);

    // Deeply nested namespace
    let deep_ns = r#"a.b.c.d.e.f.g:key="value";"#;
    let result4 = tokenize_string(deep_ns);
    assert!(result4.is_ok());
}

#[test]
fn test_token_utilities() {
    // Test utility functions if available

    use rsb::token::utils::{make_namespaced_token, make_token};

    // Test make_token
    let token1 = make_token("key", "value");
    assert_eq!(token1.key, "key");
    assert_eq!(token1.value, "value");
    assert!(token1.namespace.is_none());

    // Test make_namespaced_token
    let token2 = make_namespaced_token("app", "name", "myapp");
    assert_eq!(token2.key, "name");
    assert_eq!(token2.value, "myapp");
    assert!(token2.namespace.is_some());
}

#[test]
fn test_token_extraction() {
    // Test namespace-based token extraction

    use rsb::token::utils::extract_namespace_tokens;

    let input = r#"global="val"; db:host="localhost"; db:port="5432"; app:name="test";"#;
    let all_tokens = tokenize_string(input).unwrap();

    // Extract db namespace tokens
    let db_tokens = extract_namespace_tokens(&all_tokens, Some("db"));
    assert_eq!(db_tokens.len(), 2);

    // Extract global (no namespace) tokens
    let global_tokens = extract_namespace_tokens(&all_tokens, None);
    assert_eq!(global_tokens.len(), 1);
    assert_eq!(global_tokens[0].key, "global");
}
