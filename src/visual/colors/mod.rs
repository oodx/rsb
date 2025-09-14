//! RSB Color Package Coordinator
//!
//! Organizes color systems into focused packages for selective inclusion.
//! Each package builds on the previous one to avoid duplication while
//! maintaining clear dependency relationships.
//!
//! ## Package Structure:
//! - **simple**: Basic 8-16 colors sufficient for prompts and basic styling
//! - **named**: Extended named colors from boxy (includes simple colors)  
//! - **status**: Status-specific colors for logging and messaging
//!
//! ## Dependencies:
//! - colors-named includes colors-simple automatically
//! - colors-all includes all packages
//! - prompts only requires colors-simple

#[cfg(feature = "colors-simple")]
pub mod simple;

#[cfg(feature = "colors-named")]
pub mod named;

#[cfg(feature = "colors-status")]
pub mod status;

// Progressive enhancement runtime registry and ergonomic API
mod registry;
mod util;

// Conditional re-exports to avoid duplication when multiple packages are enabled
#[cfg(all(feature = "colors-named", not(feature = "colors-all")))]
pub use named::*;

#[cfg(all(feature = "colors-simple", not(any(feature = "colors-named", feature = "colors-all"))))]
pub use simple::*;

#[cfg(feature = "colors-all")]
pub use {named::*, status::*};

// Ergonomic, string-first API surface
pub use registry::{
    color, get_color, bg, colorize, colorize_bg,
    color_enable, color_enable_with, color_mode, colored, get_all_colors,
};
