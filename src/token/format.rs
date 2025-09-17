//! Token formatting and transformation utilities.
//!
//! Provides utilities for formatting, escaping, and transforming tokens
//! for display, serialization, and processing.

use super::types::Token;

/// Add quotes around a token value if not already quoted.
///
/// Automatically detects if the value already has quotes to avoid double-quoting.
///
/// # Examples
/// ```
/// use rsb::token::format::quote_token;
///
/// assert_eq!(quote_token("hello world"), "\"hello world\"");
/// assert_eq!(quote_token("\"already quoted\""), "\"already quoted\"");
/// assert_eq!(quote_token("single_word"), "\"single_word\"");
/// ```
pub fn quote_token(value: &str) -> String {
    // Don't double-quote if already quoted
    if (value.starts_with('"') && value.ends_with('"'))
        || (value.starts_with('\'') && value.ends_with('\''))
    {
        value.to_string()
    } else {
        format!("\"{}\"", value)
    }
}

/// Remove quotes from a token value if present.
///
/// Removes matching single or double quotes from beginning and end.
///
/// # Examples
/// ```
/// use rsb::token::format::unquote_token;
///
/// assert_eq!(unquote_token("\"hello world\""), "hello world");
/// assert_eq!(unquote_token("'single quoted'"), "single quoted");
/// assert_eq!(unquote_token("unquoted"), "unquoted");
/// ```
pub fn unquote_token(value: &str) -> String {
    let value = value.trim();
    if value.len() >= 2 {
        if (value.starts_with('"') && value.ends_with('"'))
            || (value.starts_with('\'') && value.ends_with('\''))
        {
            value[1..value.len() - 1].to_string()
        } else {
            value.to_string()
        }
    } else {
        value.to_string()
    }
}

/// Escape special characters in a token value.
///
/// Escapes common special characters that could interfere with token parsing.
///
/// # Examples
/// ```
/// use rsb::token::format::escape_token;
///
/// assert_eq!(escape_token("line1\nline2"), "line1\\nline2");
/// assert_eq!(escape_token("quote: \"hello\""), "quote: \\\"hello\\\"");
/// assert_eq!(escape_token("tab\there"), "tab\\there");
/// ```
pub fn escape_token(value: &str) -> String {
    value
        .replace('\\', "\\\\") // Escape backslashes first
        .replace('"', "\\\"") // Escape double quotes
        .replace('\'', "\\'") // Escape single quotes
        .replace('\n', "\\n") // Escape newlines
        .replace('\t', "\\t") // Escape tabs
        .replace('\r', "\\r") // Escape carriage returns
        .replace(';', "\\;") // Escape token separators
        .replace('=', "\\=") // Escape key-value separators
}

/// Unescape special characters in a token value.
///
/// Reverses the escaping done by `escape_token`.
///
/// # Examples
/// ```
/// use rsb::token::format::unescape_token;
///
/// assert_eq!(unescape_token("line1\\nline2"), "line1\nline2");
/// assert_eq!(unescape_token("quote: \\\"hello\\\""), "quote: \"hello\"");
/// ```
pub fn unescape_token(value: &str) -> String {
    value
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\r", "\r")
        .replace("\\;", ";")
        .replace("\\=", "=")
        .replace("\\'", "'")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\") // Handle escaped backslashes last
}

/// Join multiple token values with a separator.
///
/// # Examples
/// ```
/// use rsb::token::format::join_tokens;
///
/// let values = vec!["apple", "banana", "cherry"];
/// assert_eq!(join_tokens(&values, ", "), "apple, banana, cherry");
/// assert_eq!(join_tokens(&values, "; "), "apple; banana; cherry");
/// ```
pub fn join_tokens(values: &[&str], separator: &str) -> String {
    values.join(separator)
}

/// Join token values and quote each one.
///
/// Convenience function that quotes each value and then joins them.
///
/// # Examples
/// ```
/// use rsb::token::format::join_quoted_tokens;
///
/// let values = vec!["apple pie", "banana split", "cherry tart"];
/// assert_eq!(join_quoted_tokens(&values, ", "), "\"apple pie\", \"banana split\", \"cherry tart\"");
/// ```
pub fn join_quoted_tokens(values: &[&str], separator: &str) -> String {
    values
        .iter()
        .map(|v| quote_token(v))
        .collect::<Vec<_>>()
        .join(separator)
}

/// Trim whitespace from token value.
///
/// Removes leading and trailing whitespace while preserving internal spaces.
///
/// # Examples
/// ```
/// use rsb::token::format::trim_token;
///
/// assert_eq!(trim_token("  hello world  "), "hello world");
/// assert_eq!(trim_token("no_spaces"), "no_spaces");
/// ```
pub fn trim_token(value: &str) -> String {
    value.trim().to_string()
}

/// Normalize token value by trimming and unquoting.
///
/// Applies both trimming and unquoting in the correct order.
///
/// # Examples
/// ```
/// use rsb::token::format::normalize_token;
///
/// assert_eq!(normalize_token("  \"hello world\"  "), "hello world");
/// assert_eq!(normalize_token(" 'quoted' "), "quoted");
/// ```
pub fn normalize_token(value: &str) -> String {
    unquote_token(&trim_token(value))
}

/// Pad a token value to a specified width.
///
/// Useful for aligned output formatting.
///
/// # Examples
/// ```
/// use rsb::token::format::pad_token;
///
/// assert_eq!(pad_token("hello", 10, ' '), "hello     ");
/// assert_eq!(pad_token("world", 10, '.'), "world.....");
/// ```
pub fn pad_token(value: &str, width: usize, pad_char: char) -> String {
    if value.len() >= width {
        value.to_string()
    } else {
        let padding = pad_char.to_string().repeat(width - value.len());
        format!("{}{}", value, padding)
    }
}

/// Format a token as a key=value string.
///
/// # Examples
/// ```
/// use rsb::token::{Token, format::format_token};
///
/// let token = Token::simple("host", "localhost");
/// assert_eq!(format_token(&token), "host=localhost");
///
/// let quoted = Token::simple("message", "hello world");
/// assert_eq!(format_token(&quoted), "message=\"hello world\"");
/// ```
pub fn format_token(token: &Token) -> String {
    let value = if token.value.contains(' ') || token.value.contains(';') {
        quote_token(&token.value)
    } else {
        token.value.clone()
    };

    match &token.namespace {
        Some(ns) => format!("{}:{}={}", ns, token.key, value),
        None => format!("{}={}", token.key, value),
    }
}

/// Format tokens into a table-like structure.
///
/// Creates aligned columns for namespace, key, and value.
///
/// # Examples
/// ```
/// use rsb::token::{Token, Namespace, format::format_token_table};
///
/// let tokens = vec![
///     Token::simple("host", "localhost"),
///     Token::with_namespace(Namespace::from_string("db"), "user".to_string(), "admin".to_string()),
/// ];
/// let table = format_token_table(&tokens);
/// // Produces aligned output with columns
/// ```
pub fn format_token_table(tokens: &[Token]) -> String {
    if tokens.is_empty() {
        return String::new();
    }

    // Calculate column widths
    let ns_width = tokens
        .iter()
        .map(|t| match &t.namespace {
            Some(ns) => ns.to_string().len(),
            None => "[global]".len(),
        })
        .max()
        .unwrap_or(8);

    let key_width = tokens.iter().map(|t| t.key.len()).max().unwrap_or(8);

    let mut result = Vec::new();

    // Header
    result.push(format!(
        "{} | {} | {}",
        pad_token("Namespace", ns_width, ' '),
        pad_token("Key", key_width, ' '),
        "Value"
    ));

    // Separator
    result.push(format!(
        "{}-+-{}-+-{}",
        "-".repeat(ns_width),
        "-".repeat(key_width),
        "-----"
    ));

    // Data rows
    for token in tokens {
        let ns_display = match &token.namespace {
            Some(ns) => ns.to_string(),
            None => "[global]".to_string(),
        };

        result.push(format!(
            "{} | {} | {}",
            pad_token(&ns_display, ns_width, ' '),
            pad_token(&token.key, key_width, ' '),
            token.value
        ));
    }

    result.join("\n")
}

/// Truncate a token value to a maximum length with ellipsis.
///
/// # Examples
/// ```
/// use rsb::token::format::truncate_token;
///
/// assert_eq!(truncate_token("very long message", 10), "very lo...");
/// assert_eq!(truncate_token("short", 10), "short");
/// ```
pub fn truncate_token(value: &str, max_length: usize) -> String {
    if value.len() <= max_length {
        value.to_string()
    } else if max_length <= 3 {
        "...".to_string()
    } else {
        format!("{}...", &value[..max_length - 3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Namespace;

    #[test]
    fn test_quote_token() {
        assert_eq!(quote_token("hello"), "\"hello\"");
        assert_eq!(quote_token("hello world"), "\"hello world\"");
        assert_eq!(quote_token("\"already quoted\""), "\"already quoted\"");
        assert_eq!(quote_token("'already quoted'"), "'already quoted'");
    }

    #[test]
    fn test_unquote_token() {
        assert_eq!(unquote_token("\"hello\""), "hello");
        assert_eq!(unquote_token("'world'"), "world");
        assert_eq!(unquote_token("unquoted"), "unquoted");
        assert_eq!(unquote_token(" \"spaced\" "), "spaced");
    }

    #[test]
    fn test_escape_unescape_token() {
        let test_cases = vec![
            ("hello\nworld", "hello\\nworld"),
            ("tab\there", "tab\\there"),
            ("quote: \"hello\"", "quote: \\\"hello\\\""),
            ("semi;colon", "semi\\;colon"),
            ("equals=sign", "equals\\=sign"),
            ("back\\slash", "back\\\\slash"),
        ];

        for (original, escaped) in test_cases {
            assert_eq!(escape_token(original), escaped);
            assert_eq!(unescape_token(&escaped), original);
        }
    }

    #[test]
    fn test_join_tokens() {
        let values = vec!["apple", "banana", "cherry"];
        assert_eq!(join_tokens(&values, ", "), "apple, banana, cherry");
        assert_eq!(join_tokens(&values, "; "), "apple; banana; cherry");
        assert_eq!(join_tokens(&[], ", "), "");
    }

    #[test]
    fn test_join_quoted_tokens() {
        let values = vec!["apple pie", "banana split"];
        assert_eq!(
            join_quoted_tokens(&values, ", "),
            "\"apple pie\", \"banana split\""
        );
    }

    #[test]
    fn test_trim_normalize_token() {
        assert_eq!(trim_token("  hello  "), "hello");
        assert_eq!(normalize_token("  \"hello\"  "), "hello");
        assert_eq!(normalize_token(" 'world' "), "world");
    }

    #[test]
    fn test_pad_token() {
        assert_eq!(pad_token("hello", 10, ' '), "hello     ");
        assert_eq!(pad_token("verylongtext", 10, ' '), "verylongtext");
        assert_eq!(pad_token("test", 6, '.'), "test..");
    }

    #[test]
    fn test_format_token() {
        let token = Token::simple("host", "localhost");
        assert_eq!(format_token(&token), "host=localhost");

        let spaced = Token::simple("message", "hello world");
        assert_eq!(format_token(&spaced), "message=\"hello world\"");

        let ns_token = Token::with_namespace(
            Namespace::from_string("db"),
            "user".to_string(),
            "admin".to_string(),
        );
        assert_eq!(format_token(&ns_token), "db:user=admin");
    }

    #[test]
    fn test_truncate_token() {
        assert_eq!(truncate_token("hello world", 10), "hello w...");
        assert_eq!(truncate_token("short", 10), "short");
        assert_eq!(truncate_token("test", 3), "...");
        assert_eq!(truncate_token("ab", 5), "ab");
    }

    #[test]
    fn test_format_token_table() {
        let tokens = vec![
            Token::simple("host", "localhost"),
            Token::with_namespace(
                Namespace::from_string("db"),
                "user".to_string(),
                "admin".to_string(),
            ),
        ];

        let table = format_token_table(&tokens);
        assert!(table.contains("Namespace"));
        assert!(table.contains("Key"));
        assert!(table.contains("Value"));
        assert!(table.contains("localhost"));
        assert!(table.contains("admin"));
    }
}
