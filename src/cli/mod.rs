//! CLI building utilities orchestrator. Re-exports helpers and compiles module-owned macros.
#![allow(unused_imports)]

mod helpers; // internal per MODULE_SPEC
             // Curated low-level surface
pub mod utils;
pub use utils::*;

mod dispatch;
pub use dispatch::*;

mod args;
pub use args::*;

mod help;
pub use help::*;

mod bootstrap;
pub use bootstrap::*;

mod options;
pub use options::*;

mod flags;
pub use flags::*;

// Keep module-owned macros compiled/included.
pub mod macros;
