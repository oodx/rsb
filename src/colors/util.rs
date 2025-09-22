//! Internal utilities for color enablement and gate logic

use std::sync::atomic::{AtomicBool, Ordering};

static COLOR_FORCED_OFF: AtomicBool = AtomicBool::new(false);
static BACKGROUNDS_ON: AtomicBool = AtomicBool::new(false);

/// Enable/disable color output globally at runtime.
/// Mode: "auto" | "always" | "never". Currently supports env guards, no TTY detect.
pub fn set_color_mode(mode: &str) {
    match mode.to_ascii_lowercase().as_str() {
        "never" => COLOR_FORCED_OFF.store(true, Ordering::SeqCst),
        _ => COLOR_FORCED_OFF.store(false, Ordering::SeqCst),
    }
}

/// Returns whether colors are enabled considering env and explicit mode.
pub fn colors_enabled() -> bool {
    if std::env::var("NO_COLOR").is_ok() {
        return false;
    }
    if let Ok(mode) = std::env::var("RSB_COLOR") {
        if mode.eq_ignore_ascii_case("never") {
            return false;
        }
    }
    !COLOR_FORCED_OFF.load(Ordering::SeqCst)
}

pub fn set_backgrounds_enabled(enable: bool) {
    BACKGROUNDS_ON.store(enable, Ordering::SeqCst);
}

pub fn backgrounds_enabled() -> bool {
    BACKGROUNDS_ON.load(Ordering::SeqCst)
}
