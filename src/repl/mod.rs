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

// Core types (to be implemented in REPL-01)
// pub struct Repl { ... }
// pub enum ReplResult { ... }