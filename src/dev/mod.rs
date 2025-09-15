//! Development utilities (feature-gated)
//!
//! This namespace hosts utilities useful for tests and local development
//! that should not be part of the default surface.
//!
//! Modules are individually feature-gated. Nothing here is re-exported
//! via the core prelude.

#[cfg(feature = "dev-pty")]
pub mod pty;

#[cfg(feature = "dev-pty")]
pub use pty::*;

