// RSB XCls - Closure-compatible extensions to RSB
//
// This module provides closure-supporting versions of RSB functions
// that enable complex transformations beyond simple string replacement.

pub mod xsed;
pub mod xgrep;
pub mod xfilter;

// Re-export main items
pub use xsed::{xsed, XSed, ToXSed};
pub use xgrep::{xgrep, XGrep};
pub use xfilter::{xfilter, XFilter};