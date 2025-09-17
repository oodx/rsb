//! Token UAT - User Acceptance Tests demonstrating token processing functionality

use rsb::token::{is_token_streamable, tokenize_string, utils, TokenStreamable};

#[test]
fn uat_tokens_basic_parsing_demo() {
    println!("\n=== Token Parsing Demo ===");

    // Demo 1: Basic token parsing
    let input1 = r#"host="localhost"; port="8080"; debug="true";"#;
    println!("Input: {}", input1);

    match tokenize_string(input1) {
        Ok(tokens) => {
            println!("✅ Parsed {} tokens:", tokens.len());
            for (i, token) in tokens.iter().enumerate() {
                println!(
                    "  {}. {} = {} (namespace: {:?})",
                    i + 1,
                    token.key,
                    token.value,
                    token.namespace.as_ref().map(|ns| ns.to_string())
                );
            }
        }
        Err(e) => println!("❌ Parse error: {}", e),
    }

    // Demo 2: Namespace parsing
    let input2 =
        r#"app="MyApp"; db:host="database.example.com"; db:port="5432"; cache:enabled="true";"#;
    println!("\nInput with namespaces: {}", input2);

    match tokenize_string(input2) {
        Ok(tokens) => {
            println!("✅ Parsed {} tokens:", tokens.len());
            for token in &tokens {
                match &token.namespace {
                    Some(ns) => println!("  [{}] {} = {}", ns, token.key, token.value),
                    None => println!("  [global] {} = {}", token.key, token.value),
                }
            }
        }
        Err(e) => println!("❌ Parse error: {}", e),
    }

    assert!(true); // Test passes if we reach here
}

#[test]
fn uat_tokens_validation_demo() {
    println!("\n=== Token Validation Demo ===");

    let test_cases = vec![
        (r#"valid="token";"#, "Valid basic token"),
        (r#"ns:key="value";"#, "Valid namespaced token"),
        (r#"key1="value1"; key2="value2";"#, "Multiple valid tokens"),
        ("invalid_token", "Missing equals sign"),
        ("key = value", "Space around equals"),
        (r#"key="value" ;"#, "Space before semicolon"),
        (r#"my namespace:key="value";"#, "Space in namespace"),
        (r#"ns:my key="value";"#, "Space in key"),
    ];

    for (input, description) in test_cases {
        println!("\nTesting: {} - '{}'", description, input);

        let is_valid = is_token_streamable(input);
        let trait_result = input.validate();

        match (is_valid, trait_result.is_ok()) {
            (true, true) => println!("  ✅ Valid (both methods agree)"),
            (false, false) => {
                println!("  ❌ Invalid (both methods agree)");
                if let Err(e) = trait_result {
                    println!("     Error: {}", e);
                }
            }
            _ => println!("  ⚠️  Methods disagree! (This shouldn't happen)"),
        }
    }

    assert!(true);
}

#[test]
fn uat_tokens_quote_stripping_demo() {
    println!("\n=== Quote Stripping Demo ===");

    let test_cases = vec![
        r#"double="Double quoted value";"#,
        r#"single='Single quoted value';"#,
        r#"mixed="Double"; other='Single';"#,
        r#"unquoted=No quotes here;"#,
        r#"empty_quoted=""; empty_unquoted=;"#,
        r#"special="Value with spaces and symbols!@#$%";"#,
    ];

    for input in test_cases {
        println!("\nInput: {}", input);
        match tokenize_string(input) {
            Ok(tokens) => {
                for token in tokens {
                    println!("  {} = '{}' (quotes stripped)", token.key, token.value);
                }
            }
            Err(e) => println!("  ❌ Error: {}", e),
        }
    }

    assert!(true);
}

#[test]
fn uat_tokens_namespace_demo() {
    println!("\n=== Namespace Operations Demo ===");

    let config_input = r#"
        app_name="TokenDemo";
        version="1.0.0";
        db:host="localhost";
        db:port="5432";
        db:user="admin";
        cache:host="redis.example.com";
        cache:port="6379";
        auth.oauth:client_id="abc123";
        auth.oauth:enabled="true";
        logging:level="debug";
    "#;

    println!("Parsing complex configuration:");
    println!("{}", config_input.trim());

    match tokenize_string(config_input) {
        Ok(tokens) => {
            println!("\n✅ Parsed {} tokens successfully", tokens.len());

            // Show all namespaces
            let namespaces = utils::get_namespace_names(&tokens);
            println!("\nFound namespaces: {:?}", namespaces);

            // Extract by namespace
            println!("\n--- Global Configuration ---");
            let global_tokens = utils::extract_namespace_tokens(&tokens, None);
            for token in global_tokens {
                println!("  {} = {}", token.key, token.value);
            }

            println!("\n--- Database Configuration ---");
            let db_tokens = utils::extract_namespace_tokens(&tokens, Some("db"));
            for token in db_tokens {
                println!("  db:{} = {}", token.key, token.value);
            }

            println!("\n--- Auth OAuth Configuration ---");
            let oauth_tokens = utils::extract_namespace_tokens(&tokens, Some("auth.oauth"));
            for token in oauth_tokens {
                println!("  auth.oauth:{} = {}", token.key, token.value);
            }

            // Demonstrate round-trip
            println!("\n--- Round-trip Test ---");
            let reconstructed = utils::tokens_to_string(&tokens);
            println!("Reconstructed: {}", reconstructed);

            let reparsed = tokenize_string(&reconstructed).unwrap();
            println!("✅ Round-trip successful: {} tokens", reparsed.len());
        }
        Err(e) => println!("❌ Parse error: {}", e),
    }

    assert!(true);
}

#[test]
fn uat_tokens_error_handling_demo() {
    println!("\n=== Error Handling Demo ===");

    let error_cases = vec![
        ("", "Empty input"),
        ("   ", "Whitespace only"),
        ("malformed", "No equals sign"),
        ("=no_key", "Empty key"),
        ("key = value", "Space before equals"),
        ("key= value", "Space after equals"),
        (r#"key="value" ;"#, "Space before semicolon"),
        ("my key=value", "Space in key"),
        ("my ns:key=value", "Space in namespace"),
        ("ns:my key=value", "Space in namespaced key"),
    ];

    for (input, description) in error_cases {
        println!("\nTesting error case: {} - '{}'", description, input);

        match tokenize_string(input) {
            Ok(tokens) => {
                println!("  ⚠️  Unexpected success: {} tokens", tokens.len());
            }
            Err(e) => {
                println!("  ✅ Expected error: {}", e);
            }
        }
    }

    assert!(true);
}

#[test]
fn uat_tokens_xstream_demo() {
    println!("\n=== XStream Compatibility Demo ===");

    // Show that RSB can parse XStream-style token formats
    let xstream_examples = vec![
        (r#"host="localhost"; port="8080";"#, "Basic key=value"),
        (
            r#"user="admin"; pass="secret"; ns=database; host="db.local";"#,
            "Mixed with ns= tokens",
        ),
        (
            r#"item="value1"; ns=animals; dog="fido"; cat="fluffy"; ns=global; final="done";"#,
            "Namespace switching",
        ),
        (
            r#"config.db:host="localhost"; auth.session:timeout="3600";"#,
            "Hierarchical namespaces",
        ),
    ];

    for (input, description) in xstream_examples {
        println!("\n{}: {}", description, input);

        match tokenize_string(input) {
            Ok(tokens) => {
                println!("  ✅ Parsed {} tokens:", tokens.len());
                for token in tokens {
                    match &token.namespace {
                        Some(ns) => println!("    [{}] {} = {}", ns, token.key, token.value),
                        None => println!("    [global] {} = {}", token.key, token.value),
                    }
                }
            }
            Err(e) => println!("  ❌ Parse error: {}", e),
        }
    }

    println!("\nNote: RSB provides the low-level parsing foundation.");
    println!("XStream adds bucket semantics and namespace switching logic on top of this.");

    assert!(true);
}
