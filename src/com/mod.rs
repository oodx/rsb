//! Common RSB constants and helpers (COM)
//! Orchestrator: re-exports boolean utilities, exit codes, and macros for REBEL semantics.

// Boolean semantics and parsing
pub mod bool;
pub use bool::*;

// Exit code modeling
pub mod exit;
pub use exit::*;

// Truthiness detection macros
pub mod macros;
