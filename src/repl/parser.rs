//! Command line parser trait and implementations for REPL tokenization
//!
//! Provides pluggable parsing strategies for REPL command lines with support for:
//! - Quoted strings (preserve spaces)
//! - Token patterns (key=value, prefix:key=value)
//! - Comma lists (item1,item2,item3)
//! - Semicolon streams (token1;token2;token3)
//!
//! # Parser Strategy
//!
//! The `ReplParser` trait allows different tokenization strategies:
//! - `SimpleParser`: Quote-aware with pattern detection (v1)
//! - `MeteorParser`: Full meteor tokenstream support (future)
//! - `FlagParser`: RSB flag parsing integration (future)

/// Trait for REPL command line parsing strategies
pub trait ReplParser: Send + Sync {
    /// Parse a command line into arguments
    ///
    /// # Arguments
    /// * `line` - Raw command line input
    ///
    /// # Returns
    /// Vector of parsed arguments preserving patterns as single args
    fn parse(&self, line: &str) -> Vec<String>;
}

/// Simple parser with quote, token, and list pattern support (v1)
///
/// Tokenization rules:
/// 1. Quoted strings: `"my file.txt"` → single arg
/// 2. Token patterns: `key=value`, `prefix:key=value` → single arg
/// 3. Comma lists: `a,b,c` → single arg (no spaces)
/// 4. Semicolon streams: `k1=v1;k2=v2` → single arg
/// 5. Everything else: split on whitespace
pub struct SimpleParser;

impl ReplParser for SimpleParser {
    fn parse(&self, line: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    // Toggle quote mode
                    in_quotes = !in_quotes;
                    // Don't include quotes in output
                }
                ' ' | '\t' if !in_quotes => {
                    // Whitespace outside quotes ends current token
                    if !current.is_empty() {
                        result.push(current.clone());
                        current.clear();
                    }
                }
                _ => {
                    current.push(ch);
                }
            }
        }

        // Push final token if any
        if !current.is_empty() {
            result.push(current);
        }

        result
    }
}

impl SimpleParser {
    /// Check if a token contains a comma list pattern (no spaces around commas)
    fn is_comma_list(s: &str) -> bool {
        s.contains(',') && !s.contains(" ,") && !s.contains(", ")
    }

    /// Check if a token contains a semicolon stream pattern
    fn is_semicolon_stream(s: &str) -> bool {
        s.contains(';')
    }

    /// Check if a token contains a token pattern (key=value with optional prefix)
    fn is_token_pattern(s: &str) -> bool {
        // Must have = sign
        if !s.contains('=') {
            return false;
        }

        // Check for prefix:key=value or key=value patterns
        // Examples: config:debug=true, items=a,b,c, theme=dark;timeout=30
        true
    }
}