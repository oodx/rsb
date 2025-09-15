//! GX Package - Generators and Extensions
//!
//! Specialized generators for strings, IDs, and collection utilities following
//! MODULE_SPEC patterns. Provides curated generation capabilities for RSB.

// Submodule declarations following MODULE_SPEC
pub mod string;
pub mod id;
pub mod collection;

mod utils;
pub use utils::*;

// Re-export commonly used items for convenience
pub use string::{get_rand_alnum, get_rand_alpha, get_rand_hex, get_rand_string};
pub use id::get_rand_uuid;
pub use collection::get_rand_from_slice;