// General Token Processing Helpers

// FUNCTIONS TO MOVE FROM string/mod.rs:

// is_name(value: &str) -> bool
// - Validates if string is valid token name (alphanumeric + _ + -)
// - Example: is_name("valid-token_123") -> true

// str_equals(a: &str, b: &str) -> bool  
// - Simple string comparison utility
// - Example: str_equals("token", "token") -> true

// str_matches(text: &str, pattern: &str) -> bool
// - Regex pattern matching for token validation
// - Example: str_matches("token123", r"^[a-z]+\d+$") -> true

// GENERAL UTILITIES TO ADD:

// is_identifier(value: &str) -> bool
// - Checks if string is valid programming identifier
// - Example: is_identifier("_valid_id") -> true

// is_quoted(value: &str) -> bool
// - Checks if string is properly quoted
// - Example: is_quoted("\"hello\"") -> true

// trim_token(value: &str) -> String
// - Trims whitespace and normalizes token
// - Example: trim_token(" token ") -> "token"