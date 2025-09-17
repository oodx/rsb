//! Token processing macros.
//!
//! Module-owned macros for token validation, parsing, and transformation.
//! Follows RSB's pattern of keeping macros thin and delegating to helpers.

// TODO: Implement token processing macros
//
// Planned macros:
// - token_validate!(value, rules...) -> Result<(), ValidationError>
// - is_valid_token!(value, rules...) -> bool
// - tokenize!(text, delim: $delim) -> Vec<String>
// - parse_tokens!(text, rules...) -> Result<Vec<Token>, ParseError>
// - transform_tokens!(tokens, transform) -> Vec<String>
// - normalize_tokens!(tokens, style: $style) -> Vec<String>
// - assert_token_type!(token, expected_type)
// - test_token_parsing!(name, input, expected)
// - format_tokens!(tokens, style: $style, separator: $sep) -> String
// - color_tokens!(tokens, color: $color) -> Vec<String>

// For now, provide basic placeholder macros

/// Basic token validation macro (placeholder).
#[macro_export]
macro_rules! token_validate {
    ($value:expr, min_length: $min:expr) => {{
        use $crate::token::{validate_token_format, TokenRules};
        let rules = TokenRules::new().min_length($min);
        validate_token_format($value, &rules)
    }};
    ($value:expr, pattern: $pattern:expr) => {{
        use $crate::token::{validate_token_format, TokenRules};
        let rules = TokenRules::new().pattern($pattern.to_string());
        validate_token_format($value, &rules)
    }};
}

/// Simple tokenization macro (placeholder).
#[macro_export]
macro_rules! tokenize {
    ($text:expr, delim: $delim:expr) => {{
        use $crate::token::tokenize;
        tokenize($text, &[$delim])
    }};
}
