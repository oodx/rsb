//! Thread Utilities (forwarding to hosts::jobs)
//!
//! This module forwards to hosts::jobs for backward compatibility.

// Re-export everything from hosts::jobs
pub use crate::hosts::jobs::{bench, list_jobs, sleep_ms, start_background, wait};
