//! Visual Utils - Curated low-level functions for advanced usage
//!
//! This module exposes the underlying implementation functions for users who want
//! explicit control over visual functionality. Most users should prefer the ergonomic
//! macros, but these functions are available for advanced use cases, library
//! integration, and custom workflows.
//!
//! ## MODULE_SPEC Pattern
//! Functions here are "curated" - they represent the intentionally exposed
//! low-level API surface that users may explicitly opt into via `visual::utils::*`
//!
//! ## Usage
//! ```rust
//! use rsb::visual::utils::*;
//!
//! // Explicit control over timeout behavior
//! let result = confirm_with_timeout("Deploy?", Some(30), false);
//! let input = ask_with_timeout("API Key", Some("default"), Some(10));
//! ```

// MODULE_SPEC: Re-export curated functions from implementation modules

#[cfg(feature = "prompts")]
pub use crate::visual::prompts::utils::{
    confirm_with_timeout,
    ask_with_timeout,
    select_with_timeout,
};

// Future: Other visual utils can be added here
// #[cfg(feature = "colors")]
// pub use crate::visual::colors::utils::*;

// #[cfg(feature = "glyphs")]
// pub use crate::visual::glyphs::utils::*;