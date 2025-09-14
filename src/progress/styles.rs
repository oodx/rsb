//! Progress Display Styles
//!
//! Different visual styles for progress indicators.
//! Framework-agnostic and extractable to RSB.

use std::fmt;

/// Visual style for progress indicators
#[derive(Debug, Clone, PartialEq)]
pub enum ProgressStyle {
    /// Simple spinner for indeterminate progress
    Spinner,
    /// Progress bar with known total
    Bar { total: u64 },
    /// Counter showing current/total
    Counter { total: u64 },
    /// Percentage display
    Percentage { total: u64 },
    /// Size-based progress (for file operations)
    Bytes { total_bytes: u64 },
    /// Silent mode - no visual output
    Silent,
    /// Custom style with user-defined format
    Custom(String),
}

impl ProgressStyle {
    /// Get the total value for styles that support it
    pub fn total(&self) -> Option<u64> {
        match self {
            ProgressStyle::Bar { total } => Some(*total),
            ProgressStyle::Counter { total } => Some(*total),
            ProgressStyle::Percentage { total } => Some(*total),
            ProgressStyle::Bytes { total_bytes } => Some(*total_bytes),
            _ => None,
        }
    }

    /// Check if this style supports real-time updates
    pub fn supports_updates(&self) -> bool {
        !matches!(self, ProgressStyle::Silent)
    }

    /// Check if this style needs a known total
    pub fn needs_total(&self) -> bool {
        matches!(
            self,
            ProgressStyle::Bar { .. }
                | ProgressStyle::Counter { .. }
                | ProgressStyle::Percentage { .. }
                | ProgressStyle::Bytes { .. }
        )
    }
}

impl fmt::Display for ProgressStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgressStyle::Spinner => write!(f, "spinner"),
            ProgressStyle::Bar { total } => write!(f, "bar({})", total),
            ProgressStyle::Counter { total } => write!(f, "counter({})", total),
            ProgressStyle::Percentage { total } => write!(f, "percentage({})", total),
            ProgressStyle::Bytes { total_bytes } => write!(f, "bytes({})", total_bytes),
            ProgressStyle::Silent => write!(f, "silent"),
            ProgressStyle::Custom(format) => write!(f, "custom({})", format),
        }
    }
}

/// Configuration for spinner-style progress
#[derive(Debug, Clone)]
pub struct SpinnerStyle {
    /// Characters to cycle through
    pub chars: Vec<char>,
    /// Speed of rotation (milliseconds per frame)
    pub speed_ms: u64,
    /// Prefix to show before spinner
    pub prefix: String,
    /// Suffix to show after spinner
    pub suffix: String,
}

impl Default for SpinnerStyle {
    fn default() -> Self {
        Self {
            chars: vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            speed_ms: 80,
            prefix: String::new(),
            suffix: String::new(),
        }
    }
}

impl SpinnerStyle {
    /// Create a simple spinning style
    pub fn simple() -> Self {
        Self {
            chars: vec!['|', '/', '-', '\\'],
            speed_ms: 100,
            ..Default::default()
        }
    }

    /// Create a dots style
    pub fn dots() -> Self {
        Self {
            chars: vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            speed_ms: 80,
            ..Default::default()
        }
    }

    /// Create an arrow style
    pub fn arrow() -> Self {
        Self {
            chars: vec!['←', '↖', '↑', '↗', '→', '↘', '↓', '↙'],
            speed_ms: 120,
            ..Default::default()
        }
    }

    /// Set custom prefix
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    /// Set custom suffix
    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = suffix.into();
        self
    }

    /// Get current spinner character based on time
    pub fn current_char(&self, frame: usize) -> char {
        self.chars[frame % self.chars.len()]
    }
}

/// Configuration for bar-style progress
#[derive(Debug, Clone)]
pub struct BarStyle {
    /// Width of the progress bar in characters
    pub width: usize,
    /// Character for completed portion
    pub filled_char: char,
    /// Character for remaining portion
    pub empty_char: char,
    /// Characters for the progress edge
    pub edge_chars: Option<(char, char)>, // (left, right)
    /// Whether to show percentage
    pub show_percentage: bool,
    /// Whether to show current/total counters
    pub show_counters: bool,
    /// Whether to show ETA
    pub show_eta: bool,
    /// Whether to show rate
    pub show_rate: bool,
    /// Custom message position
    pub message_position: MessagePosition,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessagePosition {
    /// Show message before the bar
    Before,
    /// Show message after the bar
    After,
    /// Show message above the bar
    Above,
    /// Show message below the bar
    Below,
    /// Don't show message
    None,
}

impl Default for BarStyle {
    fn default() -> Self {
        Self {
            width: 40,
            filled_char: '█',
            empty_char: '░',
            edge_chars: Some(('[', ']')),
            show_percentage: true,
            show_counters: true,
            show_eta: true,
            show_rate: false,
            message_position: MessagePosition::After,
        }
    }
}

impl BarStyle {
    /// Create a simple bar style
    pub fn simple() -> Self {
        Self {
            width: 20,
            filled_char: '=',
            empty_char: '-',
            edge_chars: Some(('[', ']')),
            show_percentage: true,
            show_counters: false,
            show_eta: false,
            show_rate: false,
            message_position: MessagePosition::After,
        }
    }

    /// Create a detailed bar style with all info
    pub fn detailed() -> Self {
        Self {
            width: 50,
            show_percentage: true,
            show_counters: true,
            show_eta: true,
            show_rate: true,
            ..Default::default()
        }
    }

    /// Create a minimal bar style
    pub fn minimal() -> Self {
        Self {
            width: 20,
            filled_char: '■',
            empty_char: ' ',
            edge_chars: None,
            show_percentage: false,
            show_counters: false,
            show_eta: false,
            show_rate: false,
            message_position: MessagePosition::Before,
        }
    }

    /// Set bar width
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Set fill characters
    pub fn with_chars(mut self, filled: char, empty: char) -> Self {
        self.filled_char = filled;
        self.empty_char = empty;
        self
    }

    /// Set edge characters
    pub fn with_edges(mut self, left: char, right: char) -> Self {
        self.edge_chars = Some((left, right));
        self
    }

    /// Remove edge characters
    pub fn without_edges(mut self) -> Self {
        self.edge_chars = None;
        self
    }

    /// Set what information to display
    pub fn with_info(mut self, percentage: bool, counters: bool, eta: bool, rate: bool) -> Self {
        self.show_percentage = percentage;
        self.show_counters = counters;
        self.show_eta = eta;
        self.show_rate = rate;
        self
    }

    /// Set message position
    pub fn with_message_position(mut self, position: MessagePosition) -> Self {
        self.message_position = position;
        self
    }

    /// Generate the progress bar string
    pub fn render(&self, current: u64, total: u64, message: Option<&str>) -> String {
        if total == 0 {
            return "Error: Division by zero".to_string();
        }

        let percentage = (current as f64 / total as f64) * 100.0;
        let filled_width = ((current as f64 / total as f64) * self.width as f64) as usize;
        let empty_width = self.width.saturating_sub(filled_width);

        // Build the bar
        let mut bar = String::new();

        if let Some((left, _)) = self.edge_chars {
            bar.push(left);
        }

        bar.extend(std::iter::repeat(self.filled_char).take(filled_width));
        bar.extend(std::iter::repeat(self.empty_char).take(empty_width));

        if let Some((_, right)) = self.edge_chars {
            bar.push(right);
        }

        // Build info string
        let mut info_parts = Vec::new();

        if self.show_percentage {
            info_parts.push(format!("{:5.1}%", percentage));
        }

        if self.show_counters {
            info_parts.push(format!("{}/{}", current, total));
        }

        let info_str = if info_parts.is_empty() {
            String::new()
        } else {
            format!(" {}", info_parts.join(" "))
        };

        // Combine bar and info
        let progress_line = format!("{}{}", bar, info_str);

        // Add message based on position
        match (&self.message_position, message) {
            (MessagePosition::Before, Some(msg)) => format!("{} {}", msg, progress_line),
            (MessagePosition::After, Some(msg)) => format!("{} {}", progress_line, msg),
            (MessagePosition::Above, Some(msg)) => format!("{}\n{}", msg, progress_line),
            (MessagePosition::Below, Some(msg)) => format!("{}\n{}", progress_line, msg),
            _ => progress_line,
        }
    }
}

/// Preset styles for common use cases
pub mod presets {
    use super::*;

    /// File encryption progress style
    pub fn encryption_progress(total_bytes: u64) -> ProgressStyle {
        ProgressStyle::Bytes { total_bytes }
    }

    /// Multi-file operation style
    pub fn multi_file_progress(file_count: u64) -> ProgressStyle {
        ProgressStyle::Bar { total: file_count }
    }

    /// Key rotation progress style
    pub fn key_rotation_progress(file_count: u64) -> ProgressStyle {
        ProgressStyle::Counter { total: file_count }
    }

    /// Simple operation spinner
    pub fn simple_spinner() -> ProgressStyle {
        ProgressStyle::Spinner
    }

    /// Get appropriate style based on operation characteristics
    pub fn auto_select(has_total: bool, total_value: Option<u64>, operation_type: &str) -> ProgressStyle {
        match (has_total, total_value, operation_type) {
            (true, Some(total), "files") if total > 1 => ProgressStyle::Bar { total },
            (true, Some(total), "bytes") => ProgressStyle::Bytes { total_bytes: total },
            (true, Some(total), "rotation") => ProgressStyle::Counter { total },
            (true, Some(total), _) => ProgressStyle::Percentage { total },
            _ => ProgressStyle::Spinner,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_style_properties() {
        let bar_style = ProgressStyle::Bar { total: 100 };
        assert_eq!(bar_style.total(), Some(100));
        assert!(bar_style.supports_updates());
        assert!(bar_style.needs_total());

        let spinner_style = ProgressStyle::Spinner;
        assert_eq!(spinner_style.total(), None);
        assert!(spinner_style.supports_updates());
        assert!(!spinner_style.needs_total());

        let silent_style = ProgressStyle::Silent;
        assert!(!silent_style.supports_updates());
    }

    #[test]
    fn test_spinner_style() {
        let spinner = SpinnerStyle::simple();
        assert_eq!(spinner.chars, vec!['|', '/', '-', '\\']);
        assert_eq!(spinner.current_char(0), '|');
        assert_eq!(spinner.current_char(1), '/');
        assert_eq!(spinner.current_char(4), '|'); // wraps around

        let custom_spinner = SpinnerStyle::dots()
            .with_prefix("Loading: ")
            .with_suffix("...");
        assert_eq!(custom_spinner.prefix, "Loading: ");
        assert_eq!(custom_spinner.suffix, "...");
    }

    #[test]
    fn test_bar_style_rendering() {
        let bar = BarStyle::simple();

        // Test 0%
        let result = bar.render(0, 100, Some("Starting"));
        assert!(result.contains("--------------------"));
        assert!(result.contains("0.0%"));

        // Test 50%
        let result = bar.render(50, 100, Some("Half way"));
        assert!(result.contains("==========----------"));
        assert!(result.contains("50.0%"));

        // Test 100%
        let result = bar.render(100, 100, Some("Complete"));
        assert!(result.contains("===================="));
        assert!(result.contains("100.0%"));
    }

    #[test]
    fn test_bar_style_customization() {
        let bar = BarStyle::minimal()
            .with_width(10)
            .with_chars('●', '○')
            .without_edges();

        let result = bar.render(3, 10, None);
        assert!(result.contains("●●●○○○○○○○"));
    }

    #[test]
    fn test_message_positions() {
        let bar = BarStyle::simple().with_message_position(MessagePosition::Before);
        let result = bar.render(50, 100, Some("Progress"));
        assert!(result.starts_with("Progress"));

        let bar = BarStyle::simple().with_message_position(MessagePosition::After);
        let result = bar.render(50, 100, Some("Progress"));
        assert!(result.ends_with("Progress"));

        let bar = BarStyle::simple().with_message_position(MessagePosition::Above);
        let result = bar.render(50, 100, Some("Progress"));
        assert!(result.contains("Progress\n["));
    }

    #[test]
    fn test_preset_styles() {
        use presets::*;

        let file_style = multi_file_progress(10);
        assert_eq!(file_style.total(), Some(10));

        let byte_style = encryption_progress(1024);
        assert_eq!(byte_style.total(), Some(1024));

        let auto_style = auto_select(true, Some(5), "files");
        assert!(matches!(auto_style, ProgressStyle::Bar { total: 5 }));

        let auto_spinner = auto_select(false, None, "unknown");
        assert!(matches!(auto_spinner, ProgressStyle::Spinner));
    }
}