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
use crate::global::{get_var, has_var, clear_prefix};
use crate::cli::Args;

/// Result of processing a REPL command
///
/// Determines control flow in the REPL loop
#[derive(Debug, Clone)]
pub enum ReplResult {
    /// Exit the REPL loop
    Exit,
    /// Built-in command handled, continue loop
    Continue,
    /// User command to dispatch to handler
    Command(Args),
    /// Error occurred, print and continue
    Error(String),
}

/// Core REPL struct for interactive command processing
///
/// Provides a read-eval-print loop with:
/// - Dynamic prompt configuration
/// - Command history tracking
/// - Pluggable parser support
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
    /// Pluggable parser for command line tokenization
    parser: Box<dyn ReplParser>,
}

impl Repl {
    /// Create new REPL with default configuration
    ///
    /// Prompt resolution hierarchy:
    /// 1. TOML: `rsb_repl_prompt` via rsb_config!
    /// 2. Environment: `RSB_REPL_PROMPT`
    /// 3. Default: `"repl> "`
    ///
    /// Uses SimpleParser by default.
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
            parser: Box::new(SimpleParser),
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
            parser: Box::new(SimpleParser),
        }
    }

    /// Create REPL with custom parser
    ///
    /// Allows pluggable parsing strategies for different tokenization needs.
    ///
    /// # Arguments
    /// * `parser` - Custom parser implementation
    ///
    /// # Example
    /// ```rust,ignore
    /// let parser = Box::new(MyCustomParser);
    /// let repl = Repl::with_parser(parser);
    /// ```
    pub fn with_parser(parser: Box<dyn ReplParser>) -> Self {
        Self {
            prompt: "repl> ".to_string(),
            history: Vec::new(),
            parser,
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

    /// Dispatch built-in REPL commands
    ///
    /// Handles standard commands like exit, quit, clear, history, help.
    /// Returns ReplResult to control REPL flow.
    ///
    /// # Arguments
    /// * `args` - Parsed command arguments
    ///
    /// # Returns
    /// * `ReplResult::Exit` - for exit/quit commands
    /// * `ReplResult::Continue` - for handled built-ins
    /// * `ReplResult::Command(args)` - for user commands to dispatch
    pub fn dispatch_builtin(&self, args: &Args) -> ReplResult {
        let cmd = args.all().get(0).map(|s| s.as_str()).unwrap_or("");

        match cmd {
            "exit" | "quit" => ReplResult::Exit,

            "clear" => {
                // Temporarily enable RSB_GLOBAL_RESET for clear operation
                crate::global::set_var("RSB_GLOBAL_RESET", "1");
                let result = clear_prefix("repl_");
                crate::global::set_var("RSB_GLOBAL_RESET", "0");

                match result {
                    Ok(count) => {
                        if count > 0 {
                            println!("REPL context cleared ({} variables)", count);
                        } else {
                            println!("REPL context already clear");
                        }
                    }
                    Err(e) => println!("Error clearing context: {}", e),
                }
                ReplResult::Continue
            }

            "history" => {
                self.show_history();
                ReplResult::Continue
            }

            "help" => {
                self.show_repl_help();
                ReplResult::Continue
            }

            "" => ReplResult::Continue, // Empty line

            _ => ReplResult::Command(args.clone()),
        }
    }

    /// Show command history
    fn show_history(&self) {
        if self.history.is_empty() {
            println!("No command history");
            return;
        }

        println!("\nCommand History:");
        for (i, cmd) in self.history.iter().enumerate() {
            println!("  {}: {}", i + 1, cmd);
        }
        println!();
    }

    /// Show REPL help message
    fn show_repl_help(&self) {
        println!("\nREPL Built-in Commands:");
        println!("  exit, quit  - Exit REPL mode");
        println!("  clear       - Clear REPL context variables");
        println!("  history     - Show command history");
        println!("  help        - Show this help message");
        println!();
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}