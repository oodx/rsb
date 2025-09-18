// RSB Sanity Tests - Parse Module Core Functionality Verification
// Tests verify the parse module functions work as documented in FEATURES_PARSE

use rsb::prelude::*;

#[test]
fn test_sed_replace_macro() {
    // Test basic string replacement
    let content = "Hello world, this is a test";

    // Single replacement
    let result1 = sed_replace!(content, "world", "universe");
    assert_eq!(result1, "Hello universe, this is a test");

    // Multiple replacements
    let result2 = sed_replace!(content, "test", "demo");
    assert_eq!(result2, "Hello world, this is a demo");

    // No match (should return original)
    let result3 = sed_replace!(content, "nonexistent", "replacement");
    assert_eq!(result3, content);
}

#[test]
fn test_sed_lines_macro() {
    // Test line range extraction
    let multiline_content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";

    // Extract lines 2-4
    let result1 = sed_lines!(multiline_content, 2, 4);
    assert_eq!(result1, "Line 2\nLine 3\nLine 4");

    // Extract from start
    let result2 = sed_lines!(multiline_content, 1, 2);
    assert_eq!(result2, "Line 1\nLine 2");

    // Extract single line
    let result3 = sed_lines!(multiline_content, 3, 3);
    assert_eq!(result3, "Line 3");
}

#[test]
fn test_sed_around_macro() {
    // Test context extraction around pattern
    let content = "Before\nTarget line\nAfter line 1\nAfter line 2";

    // Get 1 line of context around "Target"
    let result = sed_around!(content, "Target", 1);
    assert!(result.contains("Before"));
    assert!(result.contains("Target line"));
    assert!(result.contains("After line 1"));
    assert!(!result.contains("After line 2"));
}

#[test]
fn test_sed_insert_macro() {
    // Test content insertion at sentinel
    let base_content = "Header\n<!-- INSERT_POINT -->\nFooter";
    let insert_content = "Inserted content\nMultiple lines";

    let result = sed_insert!(insert_content, "<!-- INSERT_POINT -->", base_content);
    assert!(result.contains("Header"));
    assert!(result.contains("Inserted content"));
    assert!(result.contains("Multiple lines"));
    assert!(result.contains("Footer"));
    assert!(!result.contains("<!-- INSERT_POINT -->"));
}

#[test]
fn test_sed_template_macro() {
    // Test template replacement functionality
    set_var("USER_NAME", "TestUser");
    set_var("APP_VERSION", "1.0.0");

    let template_content = "Welcome $USER_NAME to version $APP_VERSION";

    let result = expand_vars(template_content);
    assert_eq!(result, "Welcome TestUser to version 1.0.0");

    // Test with missing variables
    let template_missing = "Hello $MISSING_VAR";
    let result_missing = expand_vars(template_missing);
    assert_eq!(result_missing, "Hello "); // Should replace with empty string

    unset_var("USER_NAME");
    unset_var("APP_VERSION");
}

#[test]
fn test_template_replace_with_sentinel() {
    // Test template replacement with sentinel pattern
    set_var("TITLE", "My Document");
    set_var("CONTENT", "This is the main content");

    let base = "# {{TITLE}}\n\n{{CONTENT}}\n\nEnd";
    let with_title = sed_template!(&get_var("TITLE"), "{{TITLE}}", base);
    let result = sed_template!(&get_var("CONTENT"), "{{CONTENT}}", &with_title);

    assert!(result.contains("My Document"));
    assert!(result.contains("This is the main content"));
    assert!(!result.contains("{{"));
    assert!(!result.contains("}}"));

    unset_var("TITLE");
    unset_var("CONTENT");
}

#[test]
fn test_parse_line_operations() {
    // Test line-based parsing operations
    let content = "Line 1\nLine 2\nLine 3\nLine 4";

    // Test line extraction with different ranges
    let first_two = sed_lines!(content, 1, 2);
    assert_eq!(first_two, "Line 1\nLine 2");

    let last_two = sed_lines!(content, 3, 4);
    assert_eq!(last_two, "Line 3\nLine 4");

    // Test single line extraction
    let middle = sed_lines!(content, 2, 2);
    assert_eq!(middle, "Line 2");
}

#[test]
fn test_parse_pattern_matching() {
    // Test pattern-based content extraction
    let log_content = "INFO: Starting application\nERROR: Database connection failed\nINFO: Retrying connection\nERROR: Authentication failed";

    // Extract context around ERROR messages
    let error_context = sed_around!(log_content, "ERROR", 1);
    assert!(error_context.contains("Database connection failed"));
    assert!(error_context.contains("Authentication failed"));

    // Test pattern replacement
    let cleaned_log = sed_replace!(log_content, "ERROR:", "WARN:");
    assert!(cleaned_log.contains("WARN: Database connection failed"));
    assert!(!cleaned_log.contains("ERROR:"));
}

#[test]
fn test_parse_content_insertion() {
    // Test various content insertion scenarios

    // Insert into HTML-like structure
    let html_base = "<html>\n<head>\n<!-- META_INSERT -->\n</head>\n<body></body>\n</html>";
    let meta_content = "<meta charset='utf-8'>\n<title>Test Page</title>";

    let result = sed_insert!(meta_content, "<!-- META_INSERT -->", html_base);
    assert!(result.contains("<meta charset='utf-8'>"));
    assert!(result.contains("<title>Test Page</title>"));
    assert!(!result.contains("<!-- META_INSERT -->"));

    // Insert into configuration file
    let config_base = "# Configuration\nhost=localhost\n# ADDITIONAL_CONFIG\nport=8080";
    let additional = "timeout=30\nretries=3";

    let config_result = sed_insert!(additional, "# ADDITIONAL_CONFIG", config_base);
    assert!(config_result.contains("timeout=30"));
    assert!(config_result.contains("retries=3"));
}

#[test]
fn test_parse_complex_templates() {
    // Test complex template scenarios
    set_var("SERVICE_NAME", "web-server");
    set_var("PORT", "8080");
    set_var("ENV", "production");
    set_var("WORKERS", "4");

    let docker_template = r#"
FROM alpine:latest
EXPOSE $PORT
ENV SERVICE=$SERVICE_NAME
ENV ENVIRONMENT=$ENV
CMD ["./$SERVICE_NAME", "--workers", "$WORKERS"]
"#;

    let result = expand_vars(docker_template);
    assert!(result.contains("EXPOSE 8080"));
    assert!(result.contains("ENV SERVICE=web-server"));
    assert!(result.contains("ENV ENVIRONMENT=production"));
    assert!(result.contains("--workers\", \"4"));

    unset_var("SERVICE_NAME");
    unset_var("PORT");
    unset_var("ENV");
    unset_var("WORKERS");
}

#[test]
fn test_parse_edge_cases() {
    // Test edge cases and boundary conditions

    // Empty content
    let empty_result = sed_replace!("", "pattern", "replacement");
    assert_eq!(empty_result, "");

    let empty_lines = sed_lines!("", 1, 1);
    assert_eq!(empty_lines, "");

    // Single line content
    let single_line = "Only one line";
    let single_result = sed_lines!(single_line, 1, 1);
    assert_eq!(single_result, "Only one line");

    // Out of range line numbers
    let short_content = "Line 1\nLine 2";
    let beyond_range = sed_lines!(short_content, 5, 10);
    assert_eq!(beyond_range, ""); // Should handle gracefully

    // Pattern not found
    let no_match_around = sed_around!("No match here", "nonexistent", 1);
    assert_eq!(no_match_around, ""); // Should return empty or handle gracefully
}

#[test]
fn test_parse_multiline_operations() {
    // Test operations on multi-line content
    let config_content = r#"
# Database Configuration
host=localhost
port=5432
database=myapp

# Cache Configuration
redis_host=localhost
redis_port=6379

# Application Settings
debug=true
workers=4
"#;

    // Extract database section
    let db_section = sed_around!(config_content, "Database Configuration", 3);
    assert!(db_section.contains("host=localhost"));
    assert!(db_section.contains("port=5432"));

    // Replace debug setting
    let updated_config = sed_replace!(config_content, "debug=true", "debug=false");
    assert!(updated_config.contains("debug=false"));
    assert!(!updated_config.contains("debug=true"));

    // Extract specific lines
    let middle_lines = sed_lines!(config_content, 5, 8);
    assert!(middle_lines.contains("database=myapp"));
    assert!(middle_lines.contains("# Cache Configuration"));
}

#[test]
fn test_parse_integration_scenarios() {
    // Test realistic parsing integration scenarios

    // Scenario: Log file processing
    let log_data = "2025-01-15 10:00:00 INFO Application started\n2025-01-15 10:00:05 ERROR Database connection failed\n2025-01-15 10:00:10 INFO Retrying connection\n2025-01-15 10:00:15 ERROR Authentication failed\n2025-01-15 10:00:20 INFO Application recovered";

    // Extract error entries with context
    let error_context = sed_around!(log_data, "ERROR", 1);
    assert!(error_context.contains("Database connection failed"));
    assert!(error_context.contains("Authentication failed"));

    // Clean sensitive information
    let cleaned_log = sed_replace!(log_data, "Authentication failed", "Auth issue");
    assert!(cleaned_log.contains("Auth issue"));
    assert!(!cleaned_log.contains("Authentication failed"));

    // Scenario: Configuration file generation
    set_var("DB_HOST", "prod-db.example.com");
    set_var("DB_PORT", "5432");
    set_var("APP_ENV", "production");

    let config_template = r#"
[database]
host=$DB_HOST
port=$DB_PORT

[application]
environment=$APP_ENV
"#;

    let final_config = expand_vars(config_template);
    assert!(final_config.contains("host=prod-db.example.com"));
    assert!(final_config.contains("port=5432"));
    assert!(final_config.contains("environment=production"));

    unset_var("DB_HOST");
    unset_var("DB_PORT");
    unset_var("APP_ENV");
}
