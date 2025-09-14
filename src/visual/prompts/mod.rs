//! Prompts Module - Interactive CLI helpers (feature `prompts`)
//!
//! This module provides simple, robust interactive prompts for CLI applications.
//! All prompts respect global context flags and provide TTY-aware fallbacks.
//!
//! ## Architecture (MODULE_SPEC compliant)
//! - `mod.rs` - Orchestrator and curated public surface (this file)
//! - `interactive.rs` - Core prompt function implementations
//! - `utils.rs` - Curated timeout functions (exposed via `visual::utils`)
//!
//! ## Global Context Integration
//! - `opt_yes` → All confirmations return `true`
//! - `opt_quiet` → All prompts return defaults without interaction
//! - `opt_prompt_timeout` → Timeout for enhanced functions
//! - `PROMPT_TIMEOUT` → Environment variable fallback
//!
//! ## TTY Behavior
//! - Interactive TTY: Shows prompts and waits for input
//! - Non-TTY/CI: Returns defaults immediately (non-blocking)
//!
//! ## Usage
//! ```rust
//! use rsb::visual::prompts::*;
//!
//! // Basic functions
//! let answer = confirm("Continue?");
//! let name = ask("Your name", Some("anonymous"));
//! let choice = select("Pick one", &["a", "b", "c"], Some(0));
//!
//! // Timeout-enhanced (via helpers module)
//! use rsb::{confirm_timeout, ask_timeout}; // Macros
//! let result = confirm_timeout!("Deploy?", 10); // 10s timeout
//! ```

// MODULE_SPEC: Implementation modules
pub mod interactive;
pub mod utils;  // Curated functions - exposed via visual::utils

// MODULE_SPEC: Curated public surface - re-export core functions
pub use interactive::{
    confirm,
    confirm_default,
    ask,
    select,
    default_from,
};

// Note: Timeout-enhanced functions are available via:
// 1. Ergonomic macros: confirm_timeout!, ask_timeout!, etc. (recommended)
// 2. Explicit utils: rsb::visual::utils::confirm_with_timeout (advanced)