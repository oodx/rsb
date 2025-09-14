//! RSB EZ Prelude — convenience imports for fast prototyping
#![allow(unused_imports)]
//!
//! Pulls in the standard prelude plus curated low-level helpers.
//! Intended for development and quick scripts. For production, prefer
//! the standard `rsb::prelude::*` and import only what you need.

pub use crate::prelude::*;          // Standard user-facing surface

// Low-level helpers (curated)
pub use crate::string::utils::*;    // String helpers/case/error/filters
pub use crate::param::utils::*;     // Param helpers

// Common module-owned macros (some are already in prelude, listed for clarity)
pub use crate::{ snake, kebab, slug, dot, space, camel,
                 snake_var, kebab_var, slug_var, dot_var, space_var, camel_var };
