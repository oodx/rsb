pub mod prelude;
// args and context modules removed; use cli::Args and hosts bootstrap
pub mod fs;
pub mod macros;
pub mod os;
pub mod streamable;
pub mod streams;
// Time helpers moved into the `date` module
pub mod date;
pub mod utils;
pub mod string;
pub mod random;
pub mod math;
pub mod threads;
pub mod bash;
pub mod deps;
pub mod xcls;
pub mod global;

pub mod gx;
// Dev/testing namespace (aggregated low-level helpers)
pub mod prelude_dev;

// Host discovery namespace (env/paths/script awareness)
pub mod hosts;
pub mod cli;

// EZ prelude for rapid prototyping
pub mod prelude_ez;

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

// Parse (sed-like stream/string transforms)
pub mod parse;
