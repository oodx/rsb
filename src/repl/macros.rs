//! REPL-specific macros for argument access and dispatch
//!
//! Provides macros for accessing REPL command arguments from global storage:
//! - `repl_arg!($n)` - Get REPL argument by position (0-indexed)
//! - `repl_argc!()` - Get total REPL argument count
//! - `repl_args!()` - Get all REPL arguments as semicolon-separated string
//! - `repl_argv!()` - Get all REPL arguments as Vec<String>
//!
//! Also provides the main REPL dispatcher macro:
//! - `repl_dispatch!` - Interactive REPL loop with command routing

// TODO: Implement in REPL-05 and REPL-06
// Macros will be defined here and exported via prelude