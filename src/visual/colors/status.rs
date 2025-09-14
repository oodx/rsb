//! Status Color Package - Colors for logging, messaging, and process states
//!
//! This package provides status-specific colors including the missing ones
//! from bashfx (magic, trace, note, silly) plus semantic status colors.
//! Designed for logging systems, interactive prompts, and process feedback.

use std::collections::HashMap;
use super::simple::RESET;

/// Status color definitions using enum for type safety and deduplication
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusColor {
    // Missing bashfx colors
    Magic,      // \x1B[38;5;213m - Purple - magical/special operations  
    Trace,      // \x1B[38;5;240m - Dark grey - trace information
    Note,       // \x1B[38;5;244m - Medium grey - notes/annotations
    Silly,      // \x1B[38;5;141m - Light purple - silly/fun messages
    
    // Error/Alert status
    Error,      // \x1B[38;5;196m - Critical error (crimson)
    Warning,    // \x1B[38;5;220m - Warning state (amber)
    Danger,     // \x1B[38;5;160m - Dangerous operation (ruby) 
    Alert,      // \x1B[38;5;208m - Alert state (tangerine)
    Fatal,      // \x1B[38;5;124m - Fatal error (brick)
    
    // Success/Positive status
    Success,    // \x1B[38;5;46m - Success state (lime)
    Complete,   // \x1B[38;5;34m - Completion (emerald)
    Verified,   // \x1B[38;5;35m - Verification (jade)
    Approved,   // \x1B[38;5;121m - Approval (mint)
    Passed,     // \x1B[38;5;22m - Test passed (forest)
    
    // Info/Neutral status
    Info,       // \x1B[38;5;33m - Information (azure)
    Debug,      // \x1B[38;5;67m - Debug information (steel)
    Hint,       // \x1B[38;5;117m - Hint/tip (sky)
    Notice,     // \x1B[38;5;159m - Notice (ice)
    
    // Process/State status
    Pending,    // \x1B[38;5;184m - Pending state (mustard)
    Progress,   // \x1B[38;5;214m - In progress (orange)
    Blocked,    // \x1B[38;5;197m - Blocked state (red2)
    Queued,     // \x1B[38;5;143m - Queued state (khaki)
    Active,     // \x1B[38;5;51m - Active state (aqua)
    Inactive,   // \x1B[38;5;240m - Inactive state (grey2)
    Paused,     // \x1B[38;5;184m - Paused state (mustard)
    Stopped,    // \x1B[38;5;237m - Stopped state (grey3)
    
    // Additional bashfx status
    Recover,    // \x1B[38;5;141m - Recovery operations (purple2)
    Think,      // \x1B[38;5;15m - Thinking/processing (white2)
    Okay,       // \x1B[32m - Basic okay (green2)
    Warn,       // \x1B[38;5;214m - Basic warning (orange)
    UClock,     // \x1B[38;5;14m - Time-related (cyan)
}

impl StatusColor {
    /// Get the ANSI escape code for this status color
    pub const fn code(self) -> &'static str {
        match self {
            // Missing bashfx colors
            StatusColor::Magic => "\x1B[38;5;213m",
            StatusColor::Trace => "\x1B[38;5;240m",
            StatusColor::Note => "\x1B[38;5;244m", 
            StatusColor::Silly => "\x1B[38;5;141m",
            
            // Error/Alert status
            StatusColor::Error => "\x1B[38;5;196m",
            StatusColor::Warning => "\x1B[38;5;220m",
            StatusColor::Danger => "\x1B[38;5;160m",
            StatusColor::Alert => "\x1B[38;5;208m", 
            StatusColor::Fatal => "\x1B[38;5;124m",
            
            // Success/Positive status
            StatusColor::Success => "\x1B[38;5;46m",
            StatusColor::Complete => "\x1B[38;5;34m",
            StatusColor::Verified => "\x1B[38;5;35m",
            StatusColor::Approved => "\x1B[38;5;121m",
            StatusColor::Passed => "\x1B[38;5;22m",
            
            // Info/Neutral status  
            StatusColor::Info => "\x1B[38;5;33m",
            StatusColor::Debug => "\x1B[38;5;67m",
            StatusColor::Hint => "\x1B[38;5;117m",
            StatusColor::Notice => "\x1B[38;5;159m",
            
            // Process/State status
            StatusColor::Pending => "\x1B[38;5;184m",
            StatusColor::Progress => "\x1B[38;5;214m", 
            StatusColor::Blocked => "\x1B[38;5;197m",
            StatusColor::Queued => "\x1B[38;5;143m",
            StatusColor::Active => "\x1B[38;5;51m",
            StatusColor::Inactive => "\x1B[38;5;240m",
            StatusColor::Paused => "\x1B[38;5;184m",
            StatusColor::Stopped => "\x1B[38;5;237m",
            
            // Additional bashfx status
            StatusColor::Recover => "\x1B[38;5;141m",
            StatusColor::Think => "\x1B[38;5;15m",
            StatusColor::Okay => "\x1B[32m",
            StatusColor::Warn => "\x1B[38;5;214m",
            StatusColor::UClock => "\x1B[38;5;14m",
        }
    }
    
    /// Get the string name of this status color
    pub const fn name(self) -> &'static str {
        match self {
            // Missing bashfx colors
            StatusColor::Magic => "magic",
            StatusColor::Trace => "trace", 
            StatusColor::Note => "note",
            StatusColor::Silly => "silly",
            
            // Error/Alert status
            StatusColor::Error => "error",
            StatusColor::Warning => "warning",
            StatusColor::Danger => "danger",
            StatusColor::Alert => "alert",
            StatusColor::Fatal => "fatal",
            
            // Success/Positive status
            StatusColor::Success => "success",
            StatusColor::Complete => "complete",
            StatusColor::Verified => "verified", 
            StatusColor::Approved => "approved",
            StatusColor::Passed => "passed",
            
            // Info/Neutral status
            StatusColor::Info => "info",
            StatusColor::Debug => "debug",
            StatusColor::Hint => "hint",
            StatusColor::Notice => "notice",
            
            // Process/State status
            StatusColor::Pending => "pending",
            StatusColor::Progress => "progress",
            StatusColor::Blocked => "blocked",
            StatusColor::Queued => "queued",
            StatusColor::Active => "active",
            StatusColor::Inactive => "inactive",
            StatusColor::Paused => "paused",
            StatusColor::Stopped => "stopped",
            
            // Additional bashfx status
            StatusColor::Recover => "recover",
            StatusColor::Think => "think", 
            StatusColor::Okay => "okay",
            StatusColor::Warn => "warn",
            StatusColor::UClock => "uclock",
        }
    }
    
    /// Parse a status color from string name
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            // Missing bashfx colors
            "magic" => Some(StatusColor::Magic),
            "trace" => Some(StatusColor::Trace),
            "note" => Some(StatusColor::Note),
            "silly" => Some(StatusColor::Silly),
            
            // Error/Alert status
            "error" => Some(StatusColor::Error),
            "warning" => Some(StatusColor::Warning),
            "danger" => Some(StatusColor::Danger),
            "alert" => Some(StatusColor::Alert),
            "fatal" => Some(StatusColor::Fatal),
            
            // Success/Positive status
            "success" => Some(StatusColor::Success),
            "complete" => Some(StatusColor::Complete),
            "verified" => Some(StatusColor::Verified),
            "approved" => Some(StatusColor::Approved),
            "passed" => Some(StatusColor::Passed),
            
            // Info/Neutral status
            "info" => Some(StatusColor::Info),
            "debug" => Some(StatusColor::Debug),
            "hint" => Some(StatusColor::Hint), 
            "notice" => Some(StatusColor::Notice),
            
            // Process/State status
            "pending" => Some(StatusColor::Pending),
            "progress" => Some(StatusColor::Progress),
            "blocked" => Some(StatusColor::Blocked),
            "queued" => Some(StatusColor::Queued),
            "active" => Some(StatusColor::Active),
            "inactive" => Some(StatusColor::Inactive),
            "paused" => Some(StatusColor::Paused),
            "stopped" => Some(StatusColor::Stopped),
            
            // Additional bashfx status
            "recover" => Some(StatusColor::Recover),
            "think" => Some(StatusColor::Think),
            "okay" => Some(StatusColor::Okay),
            "warn" => Some(StatusColor::Warn),
            "uclock" => Some(StatusColor::UClock),
            
            // Unknown
            _ => None,
        }
    }
    
    /// Get all status colors as a vector
    pub fn all() -> Vec<Self> {
        vec![
            // Missing bashfx colors
            StatusColor::Magic, StatusColor::Trace, StatusColor::Note, StatusColor::Silly,
            // Error/Alert status
            StatusColor::Error, StatusColor::Warning, StatusColor::Danger, StatusColor::Alert, StatusColor::Fatal,
            // Success/Positive status
            StatusColor::Success, StatusColor::Complete, StatusColor::Verified, StatusColor::Approved, StatusColor::Passed,
            // Info/Neutral status
            StatusColor::Info, StatusColor::Debug, StatusColor::Hint, StatusColor::Notice,
            // Process/State status
            StatusColor::Pending, StatusColor::Progress, StatusColor::Blocked, StatusColor::Queued, 
            StatusColor::Active, StatusColor::Inactive, StatusColor::Paused, StatusColor::Stopped,
            // Additional bashfx status
            StatusColor::Recover, StatusColor::Think, StatusColor::Okay, StatusColor::Warn, StatusColor::UClock,
        ]
    }
}

/// Get status-specific colors as HashMap (for compatibility with existing systems)
pub fn get_status_colors() -> HashMap<String, String> {
    StatusColor::all()
        .iter()
        .map(|color| (color.name().to_string(), color.code().to_string()))
        .collect()
}

/// Get a status color code by name (enum-based lookup)
pub fn get_status_color(name: &str) -> &'static str {
    StatusColor::from_name(name)
        .map(|color| color.code())
        .unwrap_or("")
}

/// Colorize text with a status color
pub fn colorize_status(text: &str, color: &str) -> String {
    let color_code = get_status_color(color);
    if color_code.is_empty() {
        text.to_string()
    } else {
        format!("{}{}{}", color_code, text, RESET)
    }
}

/// Check if a color is available in the status palette
pub fn is_status_color(color: &str) -> bool {
    !get_status_color(color).is_empty()
}

/// Get colors organized by category for help/documentation
pub fn get_status_color_categories() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        ("Missing BashFX Colors", vec!["magic", "trace", "note", "silly"]),
        ("Error/Alert Status", vec!["error", "warning", "danger", "alert", "fatal"]),
        ("Success/Positive Status", vec!["success", "complete", "verified", "approved", "passed"]),
        ("Info/Neutral Status", vec!["info", "debug", "hint", "notice"]),
        ("Process/State Status", vec!["pending", "progress", "blocked", "queued", "active", "inactive", "paused", "stopped"]),
        ("Additional BashFX Status", vec!["recover", "think", "okay", "warn", "uclock"]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_missing_bashfx_colors() {
        // These were missing from boxy but essential for status messaging
        assert_eq!(get_status_color("magic"), "\x1B[38;5;213m");
        assert_eq!(get_status_color("trace"), "\x1B[38;5;240m");
        assert_eq!(get_status_color("note"), "\x1B[38;5;244m");
        assert_eq!(get_status_color("silly"), "\x1B[38;5;141m");
    }
    
    #[test] 
    fn test_semantic_status_colors() {
        assert_eq!(get_status_color("error"), "\x1B[38;5;196m");
        assert_eq!(get_status_color("success"), "\x1B[38;5;46m");
        assert_eq!(get_status_color("warning"), "\x1B[38;5;220m");
        assert_eq!(get_status_color("info"), "\x1B[38;5;33m");
    }
    
    #[test]
    fn test_process_state_colors() {
        assert_eq!(get_status_color("pending"), "\x1B[38;5;184m");
        assert_eq!(get_status_color("active"), "\x1B[38;5;51m");
        assert_eq!(get_status_color("blocked"), "\x1B[38;5;197m");
    }
    
    #[test]
    fn test_colorize_status() {
        let result = colorize_status("Hello", "magic");
        assert_eq!(result, "\x1B[38;5;213mHello\x1B[0m");
        
        let unknown = colorize_status("Hello", "unknown");
        assert_eq!(unknown, "Hello");
    }
    
    #[test]
    fn test_is_status_color() {
        assert!(is_status_color("magic"));
        assert!(is_status_color("error"));
        assert!(!is_status_color("crimson")); // That's a named color, not status
        assert!(!is_status_color("unknown"));
    }
}
