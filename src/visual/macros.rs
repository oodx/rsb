//! Visual output macros (feature gated) — migrated from legacy `src/macros/visual.rs`.
//!
//! These macros provide styled output logging (info!, warn!, error!, etc.).
//! Callers must import them explicitly (`use rsb::visual::macros::*;`) or via
//! selective re-exports in `visual::mod.rs`. They are intentionally not part
//! of the core prelude per `PRELUDE_POLICY`.
//!
//! Note: Color macros moved to `src/colors/macros.rs`,
//!       Prompt macros moved to `src/visual/prompts/macros.rs`

/// Emit visual log lines with severity tags routed through stderr.
#[macro_export]
macro_rules! info { ($($arg:tt)*) => { $crate::utils::stderrx("info", &format!($($arg)*)); }; }
#[macro_export]
macro_rules! okay { ($($arg:tt)*) => { $crate::utils::stderrx("okay", &format!($($arg)*)); }; }
#[macro_export]
macro_rules! warn { ($($arg:tt)*) => { $crate::utils::stderrx("warn", &format!($($arg)*)); }; }
#[macro_export]
macro_rules! error { ($($arg:tt)*) => { $crate::utils::stderrx("error", &format!($($arg)*)); }; }
#[macro_export]
macro_rules! fatal { ($($arg:tt)*) => { $crate::utils::stderrx("fatal", &format!($($arg)*)); }; }
#[macro_export]
macro_rules! debug { ($($arg:tt)*) => { $crate::utils::stderrx("debug", &format!($($arg)*)); }; }
#[macro_export]
macro_rules! trace { ($($arg:tt)*) => { $crate::utils::stderrx("trace", &format!($($arg)*)); }; }
