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
pub mod gx;
pub mod threads;
pub mod bash;
pub mod deps;
pub mod xcls;
pub mod global;

// Dev/testing namespace (aggregated low-level helpers)
pub mod dev;

// Host discovery namespace (env/paths/script awareness)
pub mod hosts;
pub mod cli;

// EZ prelude for rapid prototyping
pub mod prelude_ez;

// Optional visual package - behind feature flags
#[cfg(feature = "visual")]
pub mod visual;

// Param helpers namespace (non-macro implementation details)
pub mod param;

// Token processing module (ported from XStream)
pub mod token;
