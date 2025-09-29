//! REPL (Read-Eval-Print-Loop) support for interactive command processing
//!
//! This module provides infrastructure for building interactive REPLs with:
//! - Pluggable command line parsers (quotes, tokens, lists, streams)
//! - Global argument storage (repl_arg_* pattern)
//! - Built-in commands (exit, quit, clear, history, help)
//! - Dynamic prompt configuration
//! - Integration with dispatch! system
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use rsb::repl::Repl;
//! use rsb::repl_dispatch;
//!
//! fn main() {
//!     let args = bootstrap!();
//!
//!     dispatch!(&args, {
//!         "build" => cmd_build,
//!         "test" => cmd_test,
//!         "repl" => cmd_repl,
//!     });
//! }
//!
//! fn cmd_repl(args: Args) -> i32 {
//!     let repl = Repl::new();
//!     repl_dispatch!(repl, {
//!         "status" => repl_status,
//!         "config" => repl_config,
//!     })
//! }
//! ```

// Submodules
pub mod parser;
pub mod macros;
mod helpers;

// Public API exports
pub use parser::{ReplParser, SimpleParser};
pub use helpers::store_repl_args_global;

// Core imports
use std::io::{self, Write};
use crate::global::{get_var, has_var};

/// Core REPL struct for interactive command processing
///
/// Provides a read-eval-print loop with:
/// - Dynamic prompt configuration
/// - Command history tracking
/// - Pluggable parser support (future)
///
/// # Example
/// ```rust,ignore
/// let mut repl = Repl::new();
/// repl.set_prompt("myapp> ");
///
/// loop {
///     match repl.read_line() {
///         Some(line) => println!("Got: {}", line),
///         None => break,
///     }
/// }
/// ```
pub struct Repl {
    /// Current prompt string
    prompt: String,
    /// Command history (in-memory only for v1)
    history: Vec<String>,
}

impl Repl {
    /// Create new REPL with default configuration
    ///
    /// Prompt resolution hierarchy:
    /// 1. TOML: `rsb_repl_prompt` via rsb_config!
    /// 2. Environment: `RSB_REPL_PROMPT`
    /// 3. Default: `"repl> "`
    pub fn new() -> Self {
        let prompt = if has_var("rsb_repl_prompt") {
            get_var("rsb_repl_prompt")
        } else if has_var("RSB_REPL_PROMPT") {
            get_var("RSB_REPL_PROMPT")
        } else {
            "repl> ".to_string()
        };

        Self {
            prompt,
            history: Vec::new(),
        }
    }

    /// Create REPL with explicit prompt
    ///
    /// # Arguments
    /// * `prompt` - Initial prompt string
    ///
    /// # Example
    /// ```rust,ignore
    /// let repl = Repl::with_prompt("myapp> ");
    /// ```
    pub fn with_prompt(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            history: Vec::new(),
        }
    }

    /// Update prompt dynamically
    ///
    /// Useful for context switching, subcommand REPLs, or state changes.
    ///
    /// # Arguments
    /// * `prompt` - New prompt string
    ///
    /// # Example
    /// ```rust,ignore
    /// repl.set_prompt("myapp:config> ");
    /// ```
    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string();
    }

    /// Read a line from stdin with prompt
    ///
    /// Displays the current prompt, flushes stdout, and reads until newline.
    /// Returns None on EOF or input error.
    ///
    /// # Returns
    /// * `Some(String)` - Trimmed input line
    /// * `None` - EOF or input error
    pub fn read_line(&self) -> Option<String> {
        print!("{}", self.prompt);
        io::stdout().flush().ok()?;

        let mut line = String::new();
        io::stdin().read_line(&mut line).ok()?;

        let trimmed = line.trim().to_string();
        Some(trimmed)
    }

    /// Get reference to command history
    pub fn history(&self) -> &[String] {
        &self.history
    }

    /// Add command to history
    pub fn add_to_history(&mut self, line: String) {
        self.history.push(line);
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}