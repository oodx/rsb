//! Signal Handling
//!
//! Install trap handlers for common termination signals.

use std::sync::atomic::{AtomicBool, Ordering};

static TRAP_INSTALLED: AtomicBool = AtomicBool::new(false);

/// The actual C-style signal handler.
extern "C" fn signal_handler(signal: i32) {
    let event_name = match signal {
        libc::SIGINT => "SIGINT",
        libc::SIGTERM => "SIGTERM",
        _ => "UNKNOWN_SIGNAL",
    };
    eprintln!("\nrsb-trap: Caught signal {}, exiting.", event_name);
    std::process::exit(128 + signal);
}

/// Installs the signal handlers for common termination signals.
pub fn install_signal_handlers() {
    if TRAP_INSTALLED
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        unsafe {
            libc::signal(libc::SIGINT, signal_handler as usize);
            libc::signal(libc::SIGTERM, signal_handler as usize);
        }
    }
}
