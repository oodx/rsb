//! GX Package - Generators and Extensions
//!
//! Specialized generators for strings, IDs, and collection utilities following
//! MODULE_SPEC patterns. Provides curated generation capabilities for RSB.

// Submodule declarations following MODULE_SPEC
pub mod collection;
pub mod id;
pub mod macros;
pub mod rand;
pub mod string;
// Cross-module adapters owned by gx (consumer):
pub mod gx_math_adapter;
pub use gx_math_adapter::rand_usize_inclusive;
pub mod gx_fs_adapter;
pub use gx_fs_adapter::{load_dict as load_dict_file, rand_from_dict_file};

mod utils;
pub use utils::*;

// Re-export commonly used items for convenience
pub use collection::get_rand_from_slice;
pub use id::get_rand_uuid;
pub use rand::*;  // Random number generation
pub use string::{get_rand_alnum, get_rand_alpha, get_rand_hex, get_rand_string};
