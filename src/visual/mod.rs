//! RSB Visual Package - Optional visual enhancements
//!
//! This module provides optional glyphs, prompts, and visual macros through
//! Cargo feature flags. Components are organized into focused packages to allow
//! selective inclusion based on application needs. The standalone color system
//! now lives under [`crate::colors`].
//!
//! ## Feature Flags:
//! - `visual`: Base feature for glyphs, prompts, and macros (enables colors-core)
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
//! use rsb::colors::colorize;
//! use rsb::visual::prompts::*;
//!
//! let colored_text = colorize("Hello", "red");
//! let response = confirm("Continue?");
//! ```

#[cfg(feature = "glyphs")]
pub mod glyphs;

#[cfg(feature = "prompts")]
pub mod prompts;

#[cfg(feature = "visual")]
pub mod utils;

#[cfg(feature = "visual")]
pub mod macros;

// Re-export commonly used items when features are enabled
#[cfg(feature = "glyphs")]
pub use glyphs::*;

#[cfg(feature = "prompts")]
pub use prompts::*;

// MODULE_SPEC: Curated low-level helpers and macro surfaces
#[cfg(feature = "visual")]
pub use utils::*;

#[cfg(feature = "visual")]
pub use crate::{colored, debug, error, fatal, info, okay, trace, warn};

#[cfg(all(feature = "visual", feature = "prompts"))]
pub use crate::{
    ask, ask_timeout, confirm, confirm_default, confirm_timeout, prompt, prompt_timeout, select,
    select_timeout,
};

#[cfg(feature = "colors-core")]
#[deprecated(note = "use `rsb::colors` directly")]
pub use crate::colors;
