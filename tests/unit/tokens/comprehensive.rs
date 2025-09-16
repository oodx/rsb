//! Token module comprehensive feature tests

use rsb::token::{tokenize_string, TokenError, utils};

#[test]
fn test_comprehensive_validation_rules() {
    // Test all spacing rules comprehensively

    // ✅ ALLOWED patterns
    assert!(tokenize_string(r#"key="value";"#).is_ok());
    assert!(tokenize_string(r#"key1="value1"; key2="value2";"#).is_ok());
    assert!(tokenize_string(r#"   key="value";"#).is_ok());  // leading spaces
    assert!(tokenize_string(r#"key="value";    "#).is_ok()); // trailing spaces
    assert!(tokenize_string(r#"ns:key="value";"#).is_ok());
    assert!(tokenize_string(r#"ns.sub:key="value";"#).is_ok());

    // ❌ FORBIDDEN patterns
    assert!(tokenize_string(r#"key ="value";"#).is_err());     // space before =
    assert!(tokenize_string(r#"key= "value";"#).is_err());     // space after =
    assert!(tokenize_string(r#"key="value" ;"#).is_err());     // space before ;
    assert!(tokenize_string(r#"my key="value";"#).is_err());   // space in key
    assert!(tokenize_string(r#"my ns:key="value";"#).is_err()); // space in namespace
    assert!(tokenize_string(r#"ns:my key="value";"#).is_err()); // space in namespaced key
}

#[test]
fn test_error_message_quality() {
    // Test that error messages are informative
    let result = tokenize_string("malformed_token");
    if let Err(TokenError::MalformedToken { reason, .. }) = result {
        assert!(reason.contains("missing '=' separator"));
    } else {
        panic!("Expected MalformedToken error");
    }

    let result = tokenize_string("=empty_key");
    if let Err(TokenError::MalformedToken { reason, .. }) = result {
        assert!(reason.contains("empty key"));
    } else {
        panic!("Expected MalformedToken error");
    }

    let result = tokenize_string("key = value");
    if let Err(TokenError::MalformedToken { reason, .. }) = result {
        assert!(reason.contains("space before '=' not allowed"));
    } else {
        panic!("Expected MalformedToken error");
    }
}

#[test]
fn test_edge_cases() {
    // Test empty values
    let tokens = tokenize_string(r#"empty_quoted=""; empty_unquoted=;"#).unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].value, "");
    assert_eq!(tokens[1].value, "");

    // Test values with special characters
    let tokens = tokenize_string(r#"special="!@#$%^&*()"; unicode="🚀🦀";"#).unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].value, "!@#$%^&*()");
    assert_eq!(tokens[1].value, "🚀🦀");

    // Test very long namespace
    let long_ns = "a.very.long.hierarchical.namespace.with.many.parts";
    let input = format!(r#"{}:key="value";"#, long_ns);
    let tokens = tokenize_string(&input).unwrap();
    assert_eq!(tokens[0].namespace.as_ref().unwrap().to_string(), long_ns);
}

#[test]
fn test_namespace_utilities() {
    let tokens = tokenize_string(r#"
        global="value";
        db:host="localhost";
        db:port="5432";
        auth:enabled="true";
        config.logging:level="debug";
    "#).unwrap();

    // Test namespace extraction
    let db_tokens = utils::extract_namespace_tokens(&tokens, Some("db"));
    assert_eq!(db_tokens.len(), 2);
    assert_eq!(db_tokens[0].key, "host");
    assert_eq!(db_tokens[1].key, "port");

    let global_tokens = utils::extract_namespace_tokens(&tokens, None);
    assert_eq!(global_tokens.len(), 1);
    assert_eq!(global_tokens[0].key, "global");

    // Test namespace name extraction
    let namespaces = utils::get_namespace_names(&tokens);
    assert!(namespaces.contains(&"db".to_string()));
    assert!(namespaces.contains(&"auth".to_string()));
    assert!(namespaces.contains(&"config.logging".to_string()));
}

#[test]
fn test_token_utils() {
    // Test convenience constructors
    let token = utils::make_token("host", "localhost");
    assert_eq!(token.key, "host");
    assert_eq!(token.value, "localhost");
    assert!(token.namespace.is_none());

    let ns_token = utils::make_namespaced_token("db", "user", "admin");
    assert_eq!(ns_token.namespace.as_ref().unwrap().to_string(), "db");

    // Test tokens_to_string
    let tokens = vec![
        utils::make_token("host", "localhost"),
        utils::make_namespaced_token("db", "user", "admin"),
    ];
    let result = utils::tokens_to_string(&tokens);
    assert_eq!(result, "host=localhost; db:user=admin");
}

#[test]
fn test_complex_scenarios() {
    // Test mixed global and namespaced tokens
    let complex_input = r#"
        app_name="MyApp";
        version="1.0.0";
        db.primary:host="db1.example.com";
        db.primary:port="5432";
        db.replica:host="db2.example.com";
        db.replica:port="5432";
        cache.redis:host="cache.example.com";
        cache.redis:port="6379";
        auth.oauth:client_id="abc123";
        auth.oauth:client_secret="xyz789";
        features.experimental:enabled="false";
    "#;

    let tokens = tokenize_string(complex_input).unwrap();
    assert_eq!(tokens.len(), 11);

    // Verify global tokens
    let global_tokens = utils::extract_namespace_tokens(&tokens, None);
    assert_eq!(global_tokens.len(), 2);

    // Verify hierarchical namespaces
    let db_primary = utils::extract_namespace_tokens(&tokens, Some("db.primary"));
    assert_eq!(db_primary.len(), 2);

    let auth_oauth = utils::extract_namespace_tokens(&tokens, Some("auth.oauth"));
    assert_eq!(auth_oauth.len(), 2);

    // Get all unique namespaces
    let all_namespaces = utils::get_namespace_names(&tokens);
    assert!(all_namespaces.contains(&"db.primary".to_string()));
    assert!(all_namespaces.contains(&"cache.redis".to_string()));
    assert!(all_namespaces.contains(&"features.experimental".to_string()));
}

#[test]
fn test_performance_edge_cases() {
    // Test with many tokens
    let many_tokens: Vec<String> = (0..100)
        .map(|i| format!(r#"key{}="value{}";"#, i, i))
        .collect();
    let input = many_tokens.join(" ");

    let tokens = tokenize_string(&input).unwrap();
    assert_eq!(tokens.len(), 100);

    // Test with long values
    let long_value = "x".repeat(1000);
    let input = format!(r#"long_key="{}";"#, long_value);
    let tokens = tokenize_string(&input).unwrap();
    assert_eq!(tokens[0].value, long_value);
}

#[test]
fn test_xstream_compatibility() {
    // Test formats that should work with XStream
    let xstream_format = r#"host="localhost"; port="8080"; ns=database; user="admin"; pass="secret";"#;
    let tokens = tokenize_string(xstream_format).unwrap();
    assert_eq!(tokens.len(), 5);

    // Test namespace switching pattern (ns= tokens)
    let ns_switch = r#"item="value1"; ns=animals; dog="fido"; cat="fluffy"; ns=global; final="done";"#;
    let tokens = tokenize_string(ns_switch).unwrap();
    assert_eq!(tokens.len(), 6);

    // All should parse as individual tokens (no special ns= handling in RSB)
    assert!(tokens.iter().any(|t| t.key == "ns" && t.value == "animals"));
    assert!(tokens.iter().any(|t| t.key == "ns" && t.value == "global"));
}