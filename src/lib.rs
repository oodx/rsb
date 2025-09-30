pub mod prelude;
// args and context modules removed; use cli::Args and hosts bootstrap
pub mod fs;
pub mod macros;
pub mod streamable;
pub mod streams;
// Time helpers moved into the `date` module
pub mod bash;
pub mod date;
pub mod deps;
pub mod global;
pub mod math;
pub mod string;
pub mod threads;
pub mod utils;
pub mod xcls;

pub mod gx;
// Dev/testing namespace (aggregated low-level helpers)
pub mod prelude_dev;

// Host discovery namespace (env/paths/script awareness)
pub mod cli;
pub mod hosts;

// EZ prelude for rapid prototyping
pub mod prelude_ez;

// Standalone colors package - behind feature flags
#[cfg(feature = "colors-core")]
pub mod colors;

// Optional visual package - behind feature flags
#[cfg(feature = "visual")]
pub mod visual;

// Optional progress package - behind feature flags
#[cfg(feature = "progress")]
pub mod progress;

// Optional dev utilities (e.g., PTY wrapper for tests)
#[cfg(feature = "dev-pty")]
pub mod dev;

// Param helpers namespace (non-macro implementation details)
pub mod param;

// Token processing module (ported from XStream)
pub mod token;

// Common constants and helpers (REBEL booleans, conversions)
pub mod com;

// Parse (sed-like stream/string transforms)
pub mod parse;

// Object type for flexible configuration and data structures
#[cfg(feature = "object")]
pub mod object;

// TOML snooping module for extracting Cargo.toml metadata
pub mod toml;

// REPL (Read-Eval-Print-Loop) support for interactive command processing
pub mod repl;
