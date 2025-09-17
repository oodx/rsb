//! RSB Visual Package - Optional visual enhancements
//!
//! This module provides optional color systems, glyphs, and interactive prompts
//! through Cargo feature flags. Components are organized into focused packages
//! to allow selective inclusion based on application needs.
//!
//! ## Feature Flags:
//! - `visual`: Base feature required for all visual components
//! - `colors-simple`: Basic 8-16 colors (red, green, blue, etc.)
//! - `colors-named`: Extended named colors (crimson, azure, emerald) - includes simple
//! - `colors-status`: Status-specific colors (magic, trace, note, silly, error, success)
//! - `colors-all`: All color packages
//! - `glyphs`: Unicode glyphs for messaging  
//! - `prompts`: Interactive prompt functions (requires colors-simple)
//!
//! ## Usage:
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! rsb = { version = "0.5", features = ["colors-simple", "prompts"] }
//! ```
//!
//! ```rust
//! // Explicit import - not included in prelude
//! use rsb::visual::colors::colorize;
//! use rsb::visual::prompts::*;
//!
//! let colored_text = colorize("Hello", "red");
//! let response = confirm("Continue?");
//! ```

#[cfg(feature = "visual")]
pub mod colors;

#[cfg(feature = "glyphs")]
pub mod glyphs;

#[cfg(feature = "prompts")]
pub mod prompts;

// Re-export commonly used items when features are enabled
#[cfg(feature = "colors-simple")]
pub use colors::simple::*;

#[cfg(feature = "colors-named")]
pub use colors::named::*;

#[cfg(feature = "colors-status")]
pub use colors::status::*;

#[cfg(feature = "glyphs")]
pub use glyphs::*;

#[cfg(feature = "prompts")]
pub use prompts::*;

// MODULE_SPEC: Utils namespace for curated low-level functions
#[cfg(feature = "visual")]
pub mod utils;
