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
        // TODO: Implement in REPL-02
        // For now, simple whitespace split
        line.split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }
}