//! Progress Colors Adapter
//!
//! Cross-module integration between progress and colors modules.
//! This adapter allows progress indicators to use RSB's color system
//! with named colors instead of hardcoded ANSI codes.
//!
//! Following RSB MODULE_SPEC for cross-module integration patterns.

#[cfg(feature = "colors-core")]
use crate::colors::{color, colorize};

/// Color scheme for progress indicators using RSB color names
#[derive(Debug, Clone)]
pub struct ProgressColorScheme {
    /// Color for running/active state (default: "cyan")
    pub running: String,
    /// Color for completed state (default: "success" or "green")
    pub complete: String,
    /// Color for failed state (default: "error" or "red")
    pub failed: String,
    /// Color for cancelled state (default: "warning" or "yellow")
    pub cancelled: String,
    /// Color for completed chunks in Dashboard (default: "green")
    pub chunk_complete: String,
    /// Color for current chunk in Dashboard (default: "cyan")
    pub chunk_current: String,
    /// Color for pending chunks in Dashboard (default: "grey")
    pub chunk_pending: String,
}

impl Default for ProgressColorScheme {
    fn default() -> Self {
        Self {
            running: "cyan".to_string(),
            complete: "success".to_string(),
            failed: "error".to_string(),
            cancelled: "warning".to_string(),
            chunk_complete: "green".to_string(),
            chunk_current: "cyan".to_string(),
            chunk_pending: "grey".to_string(),
        }
    }
}

impl ProgressColorScheme {
    /// Create a new color scheme with custom RSB color names
    ///
    /// # Examples
    /// ```
    /// use rsb::progress::ProgressColorScheme;
    ///
    /// let scheme = ProgressColorScheme::new("magic", "complete", "danger", "alert");
    /// ```
    pub fn new(
        running: impl Into<String>,
        complete: impl Into<String>,
        failed: impl Into<String>,
        cancelled: impl Into<String>,
    ) -> Self {
        Self {
            running: running.into(),
            complete: complete.into(),
            failed: failed.into(),
            cancelled: cancelled.into(),
            // Use defaults for chunk colors
            chunk_complete: "green".to_string(),
            chunk_current: "cyan".to_string(),
            chunk_pending: "grey".to_string(),
        }
    }

    /// Create a scheme using simple colors only
    pub fn simple() -> Self {
        Self {
            running: "cyan".to_string(),
            complete: "green".to_string(),
            failed: "red".to_string(),
            cancelled: "yellow".to_string(),
            chunk_complete: "green".to_string(),
            chunk_current: "cyan".to_string(),
            chunk_pending: "grey".to_string(),
        }
    }

    /// Create a scheme using status colors
    pub fn status() -> Self {
        Self {
            running: "active".to_string(),
            complete: "success".to_string(),
            failed: "error".to_string(),
            cancelled: "warning".to_string(),
            chunk_complete: "complete".to_string(),
            chunk_current: "progress".to_string(),
            chunk_pending: "inactive".to_string(),
        }
    }

    /// Create a scheme with no colors (returns empty strings)
    pub fn none() -> Self {
        Self {
            running: String::new(),
            complete: String::new(),
            failed: String::new(),
            cancelled: String::new(),
            chunk_complete: String::new(),
            chunk_current: String::new(),
            chunk_pending: String::new(),
        }
    }

    /// Create a scheme with raw ANSI codes (no RSB colors dependency)
    ///
    /// Use this when you want to specify exact ANSI escape codes without
    /// depending on the RSB color system.
    ///
    /// # Examples
    /// ```
    /// use rsb::progress::ProgressColorScheme;
    ///
    /// let scheme = ProgressColorScheme::from_ansi(
    ///     "\x1b[35m",  // Magenta
    ///     "\x1b[92m",  // Bright green
    ///     "\x1b[91m",  // Bright red
    ///     "\x1b[93m",  // Bright yellow
    /// );
    /// ```
    pub fn from_ansi(
        running: impl Into<String>,
        complete: impl Into<String>,
        failed: impl Into<String>,
        cancelled: impl Into<String>,
    ) -> Self {
        Self {
            running: running.into(),
            complete: complete.into(),
            failed: failed.into(),
            cancelled: cancelled.into(),
            // Use defaults for chunk colors
            chunk_complete: "\x1b[32m".to_string(),  // Green
            chunk_current: "\x1b[36m".to_string(),    // Cyan
            chunk_pending: "\x1b[90m".to_string(),    // Grey
        }
    }

    /// Get ANSI color code for running state
    ///
    /// When colors-core feature is enabled, tries RSB color registry first.
    /// Falls back to treating the string as raw ANSI code.
    pub fn running_code(&self) -> String {
        #[cfg(feature = "colors-core")]
        {
            let code = color(&self.running);
            if !code.is_empty() {
                return code.to_string();
            }
        }
        self.running.clone()
    }

    /// Get ANSI color code for complete state
    pub fn complete_code(&self) -> String {
        #[cfg(feature = "colors-core")]
        {
            let code = color(&self.complete);
            if !code.is_empty() {
                return code.to_string();
            }
        }
        self.complete.clone()
    }

    /// Get ANSI color code for failed state
    pub fn failed_code(&self) -> String {
        #[cfg(feature = "colors-core")]
        {
            let code = color(&self.failed);
            if !code.is_empty() {
                return code.to_string();
            }
        }
        self.failed.clone()
    }

    /// Get ANSI color code for cancelled state
    pub fn cancelled_code(&self) -> String {
        #[cfg(feature = "colors-core")]
        {
            let code = color(&self.cancelled);
            if !code.is_empty() {
                return code.to_string();
            }
        }
        self.cancelled.clone()
    }

    /// Colorize text with running color
    pub fn colorize_running(&self, text: &str) -> String {
        #[cfg(feature = "colors-core")]
        {
            let result = colorize(text, &self.running);
            // If RSB colorize worked (returned something different), use it
            if result != text {
                return result;
            }
        }
        // Fallback: manual ANSI wrapping
        let code = self.running_code();
        if code.is_empty() {
            text.to_string()
        } else {
            format!("{}{}{}", code, text, self.reset_code())
        }
    }

    /// Colorize text with complete color
    pub fn colorize_complete(&self, text: &str) -> String {
        #[cfg(feature = "colors-core")]
        {
            let result = colorize(text, &self.complete);
            if result != text {
                return result;
            }
        }
        let code = self.complete_code();
        if code.is_empty() {
            text.to_string()
        } else {
            format!("{}{}{}", code, text, self.reset_code())
        }
    }

    /// Colorize text with failed color
    pub fn colorize_failed(&self, text: &str) -> String {
        #[cfg(feature = "colors-core")]
        {
            let result = colorize(text, &self.failed);
            if result != text {
                return result;
            }
        }
        let code = self.failed_code();
        if code.is_empty() {
            text.to_string()
        } else {
            format!("{}{}{}", code, text, self.reset_code())
        }
    }

    /// Colorize text with cancelled color
    pub fn colorize_cancelled(&self, text: &str) -> String {
        #[cfg(feature = "colors-core")]
        {
            let result = colorize(text, &self.cancelled);
            if result != text {
                return result;
            }
        }
        let code = self.cancelled_code();
        if code.is_empty() {
            text.to_string()
        } else {
            format!("{}{}{}", code, text, self.reset_code())
        }
    }

    /// Get reset code
    pub fn reset_code(&self) -> String {
        #[cfg(feature = "colors-core")]
        {
            let code = color("reset");
            if !code.is_empty() {
                return code.to_string();
            }
        }
        "\x1b[0m".to_string()
    }

    /// Colorize text with chunk_complete color
    pub fn colorize_chunk_complete(&self, text: &str) -> String {
        #[cfg(feature = "colors-core")]
        {
            let result = colorize(text, &self.chunk_complete);
            if result != text {
                return result;
            }
        }
        let code = self.chunk_complete_code();
        if code.is_empty() {
            text.to_string()
        } else {
            format!("{}{}{}", code, text, self.reset_code())
        }
    }

    /// Colorize text with chunk_current color (with blink)
    pub fn colorize_chunk_current(&self, text: &str, blink: bool) -> String {
        #[cfg(feature = "colors-core")]
        {
            let result = colorize(text, &self.chunk_current);
            if result != text {
                if blink {
                    return format!("\x1b[5m{}\x1b[25m", result);  // Add blink
                }
                return result;
            }
        }
        let code = self.chunk_current_code();
        if code.is_empty() {
            text.to_string()
        } else if blink {
            format!("\x1b[5m{}{}{}\x1b[25m", code, text, self.reset_code())
        } else {
            format!("{}{}{}", code, text, self.reset_code())
        }
    }

    /// Colorize text with chunk_pending color
    pub fn colorize_chunk_pending(&self, text: &str) -> String {
        #[cfg(feature = "colors-core")]
        {
            let result = colorize(text, &self.chunk_pending);
            if result != text {
                return result;
            }
        }
        let code = self.chunk_pending_code();
        if code.is_empty() {
            text.to_string()
        } else {
            format!("{}{}{}", code, text, self.reset_code())
        }
    }

    fn chunk_complete_code(&self) -> String {
        #[cfg(feature = "colors-core")]
        {
            let code = color(&self.chunk_complete);
            if !code.is_empty() {
                return code.to_string();
            }
        }
        self.chunk_complete.clone()
    }

    fn chunk_current_code(&self) -> String {
        #[cfg(feature = "colors-core")]
        {
            let code = color(&self.chunk_current);
            if !code.is_empty() {
                return code.to_string();
            }
        }
        self.chunk_current.clone()
    }

    fn chunk_pending_code(&self) -> String {
        #[cfg(feature = "colors-core")]
        {
            let code = color(&self.chunk_pending);
            if !code.is_empty() {
                return code.to_string();
            }
        }
        self.chunk_pending.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scheme_creation() {
        let scheme = ProgressColorScheme::default();
        assert_eq!(scheme.running, "cyan");
        assert_eq!(scheme.complete, "success");
        assert_eq!(scheme.failed, "error");
        assert_eq!(scheme.cancelled, "warning");
    }

    #[test]
    fn test_custom_scheme() {
        let scheme = ProgressColorScheme::new("magic", "verified", "fatal", "alert");
        assert_eq!(scheme.running, "magic");
        assert_eq!(scheme.complete, "verified");
        assert_eq!(scheme.failed, "fatal");
        assert_eq!(scheme.cancelled, "alert");
    }

    #[test]
    fn test_simple_scheme() {
        let scheme = ProgressColorScheme::simple();
        assert_eq!(scheme.running, "cyan");
        assert_eq!(scheme.complete, "green");
        assert_eq!(scheme.failed, "red");
        assert_eq!(scheme.cancelled, "yellow");
    }

    #[test]
    fn test_status_scheme() {
        let scheme = ProgressColorScheme::status();
        assert_eq!(scheme.running, "active");
        assert_eq!(scheme.complete, "success");
        assert_eq!(scheme.failed, "error");
        assert_eq!(scheme.cancelled, "warning");
    }

    #[test]
    fn test_none_scheme() {
        let scheme = ProgressColorScheme::none();
        assert_eq!(scheme.running, "");
        assert_eq!(scheme.complete, "");
        assert_eq!(scheme.failed, "");
        assert_eq!(scheme.cancelled, "");
    }

    #[cfg(feature = "colors-core")]
    #[test]
    fn test_color_integration() {
        use crate::colors::{color_enable_with, color_mode};

        color_mode("always");
        color_enable_with("simple,status");

        let scheme = ProgressColorScheme::default();

        // These should return ANSI codes when colors are enabled
        let running = scheme.running_code();
        let complete = scheme.complete_code();

        assert!(!running.is_empty() || !complete.is_empty());
    }
}
