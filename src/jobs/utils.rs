//! Job Control Utilities
//!
//! Helper functions for sleeping and benchmarking.

use std::time::{Duration, Instant};

/// Sleep for the specified number of milliseconds.
pub fn sleep_ms(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}

/// Benchmark a closure and return the elapsed time.
pub fn bench<F: FnOnce()>(label: &str, f: F) -> Duration {
    let start = Instant::now();
    f();
    let dur = start.elapsed();
    crate::utils::stderrx("info", &format!("[bench] {} took: {:?}", label, dur));
    dur
}
