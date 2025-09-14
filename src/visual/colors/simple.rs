//! Simple Color Package - Basic 8-16 colors for prompts and basic styling
//!
//! This package provides the essential colors needed for interactive prompts
//! and basic text styling. Uses enum pattern for type safety and performance.
//! Focuses on the core colors that work reliably across different terminals.

use std::collections::HashMap;

pub const RESET: &str = "\x1B[0m";

/// Simple color definitions using enum for type safety and deduplication
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleColor {
    // Basic 8 colors - widely supported
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Black,
    
    // Bright variants (16-color support)
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    
    // Grey scale for UI elements
    Grey,         // Same as bright_black
    BrightBlack,  // Alias for grey
    
    // Control codes
    Reset,
    Bold,
    Dim,
    Underline,
    
    // Semantic aliases using simple colors
    Error,    // bright_red
    Success,  // bright_green
    Warning,  // bright_yellow
    Info,     // bright_blue
}

impl SimpleColor {
    /// Get the ANSI escape code for this simple color
    pub const fn code(self) -> &'static str {
        match self {
            // Basic colors
            SimpleColor::Red => "\x1B[31m",
            SimpleColor::Green => "\x1B[32m",
            SimpleColor::Yellow => "\x1B[33m",
            SimpleColor::Blue => "\x1B[34m",
            SimpleColor::Magenta => "\x1B[35m",
            SimpleColor::Cyan => "\x1B[36m",
            SimpleColor::White => "\x1B[37m",
            SimpleColor::Black => "\x1B[30m",
            
            // Bright colors
            SimpleColor::BrightRed => "\x1B[91m",
            SimpleColor::BrightGreen => "\x1B[92m",
            SimpleColor::BrightYellow => "\x1B[93m",
            SimpleColor::BrightBlue => "\x1B[94m",
            SimpleColor::BrightMagenta => "\x1B[95m",
            SimpleColor::BrightCyan => "\x1B[96m",
            SimpleColor::BrightWhite => "\x1B[97m",
            
            // Grey
            SimpleColor::Grey | SimpleColor::BrightBlack => "\x1B[90m",
            
            // Control
            SimpleColor::Reset => "\x1B[0m",
            SimpleColor::Bold => "\x1B[1m",
            SimpleColor::Dim => "\x1B[2m",
            SimpleColor::Underline => "\x1B[4m",
            
            // Semantic aliases
            SimpleColor::Error => "\x1B[91m",    // bright_red
            SimpleColor::Success => "\x1B[92m",  // bright_green
            SimpleColor::Warning => "\x1B[93m",  // bright_yellow
            SimpleColor::Info => "\x1B[94m",     // bright_blue
        }
    }
    
    /// Get the string name of this simple color
    pub const fn name(self) -> &'static str {
        match self {
            // Basic colors
            SimpleColor::Red => "red",
            SimpleColor::Green => "green",
            SimpleColor::Yellow => "yellow",
            SimpleColor::Blue => "blue",
            SimpleColor::Magenta => "magenta",
            SimpleColor::Cyan => "cyan",
            SimpleColor::White => "white",
            SimpleColor::Black => "black",
            
            // Bright colors  
            SimpleColor::BrightRed => "bright_red",
            SimpleColor::BrightGreen => "bright_green",
            SimpleColor::BrightYellow => "bright_yellow",
            SimpleColor::BrightBlue => "bright_blue",
            SimpleColor::BrightMagenta => "bright_magenta",
            SimpleColor::BrightCyan => "bright_cyan",
            SimpleColor::BrightWhite => "bright_white",
            
            // Grey
            SimpleColor::Grey => "grey",
            SimpleColor::BrightBlack => "bright_black",
            
            // Control
            SimpleColor::Reset => "reset",
            SimpleColor::Bold => "bold",
            SimpleColor::Dim => "dim",
            SimpleColor::Underline => "underline",
            
            // Semantic aliases
            SimpleColor::Error => "error",
            SimpleColor::Success => "success", 
            SimpleColor::Warning => "warning",
            SimpleColor::Info => "info",
        }
    }
    
    /// Parse a simple color from string name
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            // Basic colors
            "red" => Some(SimpleColor::Red),
            "green" => Some(SimpleColor::Green),
            "yellow" => Some(SimpleColor::Yellow),
            "blue" => Some(SimpleColor::Blue),
            "magenta" => Some(SimpleColor::Magenta),
            "cyan" => Some(SimpleColor::Cyan),
            "white" => Some(SimpleColor::White),
            "black" => Some(SimpleColor::Black),
            
            // Bright colors
            "bright_red" => Some(SimpleColor::BrightRed),
            "bright_green" => Some(SimpleColor::BrightGreen),
            "bright_yellow" => Some(SimpleColor::BrightYellow),
            "bright_blue" => Some(SimpleColor::BrightBlue),
            "bright_magenta" => Some(SimpleColor::BrightMagenta),
            "bright_cyan" => Some(SimpleColor::BrightCyan),
            "bright_white" => Some(SimpleColor::BrightWhite),
            
            // Grey (both names map to same enum)
            "grey" => Some(SimpleColor::Grey),
            "bright_black" => Some(SimpleColor::BrightBlack),
            
            // Control
            "reset" => Some(SimpleColor::Reset),
            "bold" => Some(SimpleColor::Bold),
            "dim" => Some(SimpleColor::Dim),
            "underline" => Some(SimpleColor::Underline),
            
            // Semantic aliases
            "error" => Some(SimpleColor::Error),
            "success" => Some(SimpleColor::Success),
            "warning" => Some(SimpleColor::Warning),
            "info" => Some(SimpleColor::Info),
            
            // Unknown
            _ => None,
        }
    }
    
    /// Get all simple colors as a vector
    pub fn all() -> Vec<Self> {
        vec![
            // Basic colors
            SimpleColor::Red, SimpleColor::Green, SimpleColor::Yellow, SimpleColor::Blue,
            SimpleColor::Magenta, SimpleColor::Cyan, SimpleColor::White, SimpleColor::Black,
            // Bright colors
            SimpleColor::BrightRed, SimpleColor::BrightGreen, SimpleColor::BrightYellow, SimpleColor::BrightBlue,
            SimpleColor::BrightMagenta, SimpleColor::BrightCyan, SimpleColor::BrightWhite,
            // Grey scale
            SimpleColor::Grey, SimpleColor::BrightBlack,
            // Control
            SimpleColor::Reset, SimpleColor::Bold, SimpleColor::Dim, SimpleColor::Underline,
            // Semantic
            SimpleColor::Error, SimpleColor::Success, SimpleColor::Warning, SimpleColor::Info,
        ]
    }
}

/// Get simple colors as HashMap (for compatibility with existing systems)
pub fn get_simple_colors() -> HashMap<String, String> {
    SimpleColor::all()
        .iter()
        .map(|color| (color.name().to_string(), color.code().to_string()))
        .collect()
}

/// Get a simple color code by name (enum-based lookup)
pub fn get_simple_color(name: &str) -> &'static str {
    SimpleColor::from_name(name)
        .map(|color| color.code())
        .unwrap_or("")
}

/// Colorize text with a simple color
pub fn colorize_simple(text: &str, color: &str) -> String {
    let color_code = get_simple_color(color);
    if color_code.is_empty() {
        text.to_string()
    } else {
        format!("{}{}{}", color_code, text, RESET)
    }
}

/// Check if a color is available in the simple palette
pub fn is_simple_color(color: &str) -> bool {
    !get_simple_color(color).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_colors() {
        assert_eq!(get_simple_color("red"), "\x1B[31m");
        assert_eq!(get_simple_color("green"), "\x1B[32m");
        assert_eq!(get_simple_color("blue"), "\x1B[34m");
    }
    
    #[test]
    fn test_semantic_colors() {
        assert_eq!(get_simple_color("error"), "\x1B[91m");
        assert_eq!(get_simple_color("success"), "\x1B[92m");
        assert_eq!(get_simple_color("warning"), "\x1B[93m");
        assert_eq!(get_simple_color("info"), "\x1B[94m");
    }
    
    #[test]
    fn test_colorize_simple() {
        let result = colorize_simple("Hello", "red");
        assert_eq!(result, "\x1B[31mHello\x1B[0m");
        
        let unknown = colorize_simple("Hello", "unknown");
        assert_eq!(unknown, "Hello");
    }
    
    #[test]
    fn test_is_simple_color() {
        assert!(is_simple_color("red"));
        assert!(is_simple_color("success")); 
        assert!(!is_simple_color("crimson"));
        assert!(!is_simple_color("unknown"));
    }
}