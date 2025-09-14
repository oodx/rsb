// Token Validation Functions

// FUNCTIONS TO MOVE FROM string/mod.rs:

// is_name(value: &str) -> bool
// - Validates token names (alphanumeric + underscore + hyphen)
// - Used for variable names, identifiers, etc.

// str_matches(text: &str, pattern: &str) -> bool
// - Regex pattern matching for token validation
// - Safe regex compilation with error handling

// VALIDATION FUNCTIONS TO ADD:

// is_valid_token(value: &str, rules: &TokenRules) -> bool
// - Validates token against configurable rules
// - Rules: min/max length, allowed chars, patterns

// is_identifier(value: &str) -> bool
// - Programming language identifier validation
// - Follows common identifier rules (starts with letter/underscore)

// is_literal(value: &str) -> bool
// - Checks if string is a valid literal (quoted string, number, etc.)

// is_keyword(value: &str, keywords: &[&str]) -> bool
// - Checks if token matches reserved keywords

// validate_token_format(value: &str, format: TokenFormat) -> Result<(), ValidationError>
// - Comprehensive token format validation with detailed errors