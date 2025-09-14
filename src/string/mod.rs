//! String utilities orchestrator. Re-exports helpers and compiles module-owned macros.

mod helpers; // internal implementation details per MODULE_SPEC

mod case;
pub use case::*;

pub mod error;
pub mod guard;

// Keep module-owned macros compiled/included.
pub mod macros;

// Public utils namespace: curated low-level exports
pub mod utils;
// Re-export curated surface from utils at module root
pub use utils::*;
