// RSB XCls - Closure-compatible extensions to RSB
//
// This module provides closure-supporting versions of RSB functions
// that enable complex transformations beyond simple string replacement.

pub mod xfilter;
pub mod xgrep;
pub mod xsed;

// Re-export main items
pub use xfilter::{xfilter, XFilter};
pub use xgrep::{xgrep, XGrep};
pub use xsed::{xsed, ToXSed, XSed};
