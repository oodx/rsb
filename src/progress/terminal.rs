//! Terminal Progress Reporters
//!
//! Terminal-based progress display implementations.
//! Framework-agnostic and extractable to RSB.

use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use super::core::{ProgressEvent, ProgressReporter, ProgressState};
use super::progress_colors::ProgressColorScheme;
#[allow(unused_imports)]
use super::styles::{BarStyle, MessagePosition, ProgressStyle, SpinnerStyle};

/// Configuration for terminal progress display
#[derive(Debug, Clone)]
pub struct TerminalConfig {
    /// Whether to use colors
    pub use_colors: bool,
    /// Color scheme for progress states (uses RSB color names)
    pub color_scheme: ProgressColorScheme,
    /// Whether to use Unicode characters
    pub use_unicode: bool,
    /// Whether output goes to stderr instead of stdout
    pub use_stderr: bool,
    /// Minimum interval between updates (to avoid flooding)
    pub update_interval_ms: u64,
    /// Auto-refresh rate for spinners in milliseconds (0 = disabled)
    pub spinner_refresh_ms: u64,
    /// Whether to clear completed tasks
    pub clear_on_complete: bool,
    /// Maximum width for progress display
    pub max_width: Option<usize>,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            use_colors: true,
            color_scheme: ProgressColorScheme::default(),
            use_unicode: true,
            use_stderr: false,
            update_interval_ms: 50,
            spinner_refresh_ms: 80,  // ~12 FPS for smooth spinner animation
            clear_on_complete: false,
            max_width: None,
        }
    }
}

/// Terminal-based progress reporter
pub struct TerminalReporter {
    config: TerminalConfig,
    task_states: Arc<Mutex<HashMap<u64, TaskDisplay>>>,
    last_update: Arc<Mutex<Instant>>,
    spinner_frame: Arc<Mutex<usize>>,
    cursor_hidden: Arc<Mutex<bool>>,
}

#[derive(Debug, Clone)]
struct TaskDisplay {
    style: ProgressStyle,
    last_message: String,
    start_time: Instant,
    last_current: u64,
    last_update_time: Instant,
}

impl TerminalReporter {
    /// Create a new terminal reporter with default config
    pub fn new() -> Self {
        Self::with_config(TerminalConfig::default())
    }

    /// Create a new terminal reporter with custom config
    pub fn with_config(config: TerminalConfig) -> Self {
        let use_unicode = config.use_unicode;
        let reporter = Self {
            config,
            task_states: Arc::new(Mutex::new(HashMap::new())),
            last_update: Arc::new(Mutex::new(Instant::now())),
            spinner_frame: Arc::new(Mutex::new(0)),
            cursor_hidden: Arc::new(Mutex::new(false)),
        };

        // Start background spinner animation thread if enabled
        if reporter.config.spinner_refresh_ms > 0 {
            let spinner_frame = reporter.spinner_frame.clone();
            let refresh_ms = reporter.config.spinner_refresh_ms;
            thread::spawn(move || loop {
                thread::sleep(Duration::from_millis(refresh_ms));
                let mut frame = spinner_frame.lock().unwrap();
                *frame = (*frame + 1) % 10;  // 10 frames for dots spinner
            });
        }

        reporter
    }

    /// Create a simple terminal reporter (no colors, no Unicode)
    pub fn simple() -> Self {
        Self::with_config(TerminalConfig {
            use_colors: false,
            color_scheme: ProgressColorScheme::none(),
            use_unicode: false,
            ..Default::default()
        })
    }

    /// Create a silent reporter (no output)
    pub fn silent() -> Self {
        SilentReporter::new().into()
    }

    /// Check if we should throttle updates
    fn should_update(&self) -> bool {
        let mut last_update = self.last_update.lock().unwrap();
        let now = Instant::now();
        if now.duration_since(*last_update).as_millis() >= self.config.update_interval_ms as u128 {
            *last_update = now;
            true
        } else {
            false
        }
    }

    /// Format a progress event for display
    fn format_event(&self, event: &ProgressEvent) -> String {
        let mut task_states = self.task_states.lock().unwrap();
        let task_display = task_states.get(&event.task_id).cloned();

        let display = match task_display {
            Some(mut display) => {
                display.last_message = event.message.clone().unwrap_or_default();
                display.last_current = event.current;
                display.last_update_time = event.timestamp;
                task_states.insert(event.task_id, display.clone());
                display
            }
            None => {
                // First time seeing this task - infer style from event
                let style = self.infer_style(event);
                let display = TaskDisplay {
                    style,
                    last_message: event.message.clone().unwrap_or_default(),
                    start_time: event.timestamp,
                    last_current: event.current,
                    last_update_time: event.timestamp,
                };
                task_states.insert(event.task_id, display.clone());
                display
            }
        };

        drop(task_states); // Release lock

        self.render_progress(event, &display)
    }

    /// Infer progress style from event
    fn infer_style(&self, event: &ProgressEvent) -> ProgressStyle {
        if let Some(total) = event.total {
            if total > 1000000 {
                ProgressStyle::Bytes { total_bytes: total }
            } else if total > 1 {
                ProgressStyle::Bar { total }
            } else {
                ProgressStyle::Percentage { total }
            }
        } else {
            ProgressStyle::Spinner
        }
    }

    /// Render progress based on style
    fn render_progress(&self, event: &ProgressEvent, display: &TaskDisplay) -> String {
        match &display.style {
            ProgressStyle::Spinner => self.render_spinner(event, display),
            ProgressStyle::Bar { total } => self.render_bar(event, display, *total),
            ProgressStyle::Counter { total } => self.render_counter(event, display, *total),
            ProgressStyle::Percentage { total } => self.render_percentage(event, display, *total),
            ProgressStyle::Bytes { total_bytes } => self.render_bytes(event, display, *total_bytes),
            ProgressStyle::Silent => String::new(),
            ProgressStyle::Custom(format) => self.render_custom(event, display, format),
        }
    }

    /// Render spinner-style progress
    fn render_spinner(&self, event: &ProgressEvent, display: &TaskDisplay) -> String {
        let spinner_style = if self.config.use_unicode {
            SpinnerStyle::dots()
        } else {
            SpinnerStyle::simple()
        };

        // Read current frame from background thread
        let frame = *self.spinner_frame.lock().unwrap();
        let spinner_char = spinner_style.current_char(frame);

        let prefix = match event.state {
            ProgressState::Running => {
                if self.config.use_colors {
                    self.config.color_scheme.colorize_running(&spinner_char.to_string())
                } else {
                    spinner_char.to_string()
                }
            }
            ProgressState::Complete => {
                if self.config.use_colors {
                    self.config.color_scheme.colorize_complete("✓")
                } else {
                    "✓".to_string()
                }
            }
            ProgressState::Failed => {
                if self.config.use_colors {
                    self.config.color_scheme.colorize_failed("✗")
                } else {
                    "✗".to_string()
                }
            }
            ProgressState::Cancelled => {
                if self.config.use_colors {
                    self.config.color_scheme.colorize_cancelled("◐")
                } else {
                    "◐".to_string()
                }
            }
        };

        let message = event.message.as_deref().unwrap_or("Processing...");
        let elapsed = self.format_duration(display.start_time.elapsed());

        format!("{} {} ({})", prefix, message, elapsed)
    }

    /// Render bar-style progress
    fn render_bar(&self, event: &ProgressEvent, display: &TaskDisplay, total: u64) -> String {
        let bar_style = if self.config.use_unicode {
            BarStyle::default()
        } else {
            BarStyle::simple()
        };

        let message = event.message.as_deref();
        let bar_display = bar_style.render(event.current, total, message);

        match event.state {
            ProgressState::Complete => {
                let elapsed = self.format_duration(display.start_time.elapsed());
                if self.config.use_colors {
                    format!("{} ({})", self.config.color_scheme.colorize_complete(&bar_display), elapsed)
                } else {
                    format!("{} ({})", bar_display, elapsed)
                }
            }
            ProgressState::Failed => {
                if self.config.use_colors {
                    self.config.color_scheme.colorize_failed(&bar_display)
                } else {
                    bar_display
                }
            }
            ProgressState::Cancelled => {
                if self.config.use_colors {
                    self.config.color_scheme.colorize_cancelled(&bar_display)
                } else {
                    bar_display
                }
            }
            ProgressState::Running => {
                let eta = self.calculate_eta(event.current, total, display.start_time.elapsed());
                if let Some(eta_str) = eta {
                    format!("{} ETA: {}", bar_display, eta_str)
                } else {
                    bar_display
                }
            }
        }
    }

    /// Render counter-style progress
    fn render_counter(&self, event: &ProgressEvent, display: &TaskDisplay, total: u64) -> String {
        let message = event.message.as_deref().unwrap_or("Processing");
        let counter = format!("[{}/{}]", event.current, total);

        match event.state {
            ProgressState::Complete => {
                let elapsed = self.format_duration(display.start_time.elapsed());
                if self.config.use_colors {
                    format!("{} {} {} ({})",
                        self.config.color_scheme.colorize_complete("✓"),
                        counter, message, elapsed)
                } else {
                    format!("✓ {} {} ({})", counter, message, elapsed)
                }
            }
            ProgressState::Failed => {
                if self.config.use_colors {
                    self.config.color_scheme.colorize_failed(&format!("✗ {} {}", counter, message))
                } else {
                    format!("✗ {} {}", counter, message)
                }
            }
            ProgressState::Running => {
                format!("{} {}", counter, message)
            }
            ProgressState::Cancelled => {
                if self.config.use_colors {
                    self.config.color_scheme.colorize_cancelled(&format!("◐ {} {}", counter, message))
                } else {
                    format!("◐ {} {}", counter, message)
                }
            }
        }
    }

    /// Render percentage-style progress
    fn render_percentage(
        &self,
        event: &ProgressEvent,
        display: &TaskDisplay,
        total: u64,
    ) -> String {
        let percentage = if total > 0 {
            (event.current as f64 / total as f64) * 100.0
        } else {
            100.0
        };

        let message = event.message.as_deref().unwrap_or("Processing");

        match event.state {
            ProgressState::Complete => {
                let elapsed = self.format_duration(display.start_time.elapsed());
                if self.config.use_colors {
                    format!("{} 100.0% {} ({})",
                        self.config.color_scheme.colorize_complete("✓"),
                        message, elapsed)
                } else {
                    format!("✓ 100.0% {} ({})", message, elapsed)
                }
            }
            ProgressState::Failed => {
                if self.config.use_colors {
                    self.config.color_scheme.colorize_failed(&format!("✗ {:.1}% {}", percentage, message))
                } else {
                    format!("✗ {:.1}% {}", percentage, message)
                }
            }
            ProgressState::Running => {
                format!("{:5.1}% {}", percentage, message)
            }
            ProgressState::Cancelled => {
                if self.config.use_colors {
                    self.config.color_scheme.colorize_cancelled(&format!("◐ {:.1}% {}", percentage, message))
                } else {
                    format!("◐ {:.1}% {}", percentage, message)
                }
            }
        }
    }

    /// Render byte-style progress
    fn render_bytes(
        &self,
        event: &ProgressEvent,
        display: &TaskDisplay,
        total_bytes: u64,
    ) -> String {
        let current_str = self.format_bytes(event.current);
        let total_str = self.format_bytes(total_bytes);
        let percentage = if total_bytes > 0 {
            (event.current as f64 / total_bytes as f64) * 100.0
        } else {
            100.0
        };

        let message = event.message.as_deref().unwrap_or("Processing");

        match event.state {
            ProgressState::Complete => {
                let elapsed = display.start_time.elapsed();
                let rate = self.calculate_byte_rate(total_bytes, elapsed);
                if self.config.use_colors {
                    format!(
                        "{} {} {} ({}, {})",
                        self.config.color_scheme.colorize_complete("✓"),
                        total_str,
                        message,
                        self.format_duration(elapsed),
                        rate
                    )
                } else {
                    format!(
                        "✓ {} {} ({}, {})",
                        total_str,
                        message,
                        self.format_duration(elapsed),
                        rate
                    )
                }
            }
            ProgressState::Failed => {
                if self.config.use_colors {
                    self.config.color_scheme.colorize_failed(&format!(
                        "✗ {}/{} ({:.1}%) {}",
                        current_str, total_str, percentage, message
                    ))
                } else {
                    format!(
                        "✗ {}/{} ({:.1}%) {}",
                        current_str, total_str, percentage, message
                    )
                }
            }
            ProgressState::Running => {
                let elapsed = display.start_time.elapsed();
                let rate = if event.current > 0 {
                    self.calculate_byte_rate(event.current, elapsed)
                } else {
                    "-- B/s".to_string()
                };

                let eta = self.calculate_eta(event.current, total_bytes, elapsed);
                if let Some(eta_str) = eta {
                    format!(
                        "{}/{} ({:.1}%) {} - {} ETA: {}",
                        current_str, total_str, percentage, message, rate, eta_str
                    )
                } else {
                    format!(
                        "{}/{} ({:.1}%) {} - {}",
                        current_str, total_str, percentage, message, rate
                    )
                }
            }
            ProgressState::Cancelled => {
                if self.config.use_colors {
                    self.config.color_scheme.colorize_cancelled(&format!(
                        "◐ {}/{} ({:.1}%) {}",
                        current_str, total_str, percentage, message
                    ))
                } else {
                    format!(
                        "◐ {}/{} ({:.1}%) {}",
                        current_str, total_str, percentage, message
                    )
                }
            }
        }
    }

    /// Render custom format
    fn render_custom(&self, event: &ProgressEvent, display: &TaskDisplay, format: &str) -> String {
        let mut result = format.to_string();

        // Simple template replacement
        result = result.replace("{current}", &event.current.to_string());
        if let Some(total) = event.total {
            result = result.replace("{total}", &total.to_string());
            let percentage = (event.current as f64 / total as f64) * 100.0;
            result = result.replace("{percentage}", &format!("{:.1}", percentage));
        }
        if let Some(message) = &event.message {
            result = result.replace("{message}", message);
        }

        let elapsed = display.start_time.elapsed();
        result = result.replace("{elapsed}", &self.format_duration(elapsed));

        result
    }

    /// Format duration for display
    fn format_duration(&self, duration: Duration) -> String {
        super::core::utils::format_duration(duration)
    }

    /// Format bytes for display
    fn format_bytes(&self, bytes: u64) -> String {
        super::core::utils::format_bytes(bytes)
    }

    /// Calculate ETA
    fn calculate_eta(&self, current: u64, total: u64, elapsed: Duration) -> Option<String> {
        super::core::utils::calculate_eta(current, total, elapsed)
            .map(|eta| self.format_duration(eta))
    }

    /// Calculate byte transfer rate
    fn calculate_byte_rate(&self, bytes: u64, duration: Duration) -> String {
        if duration.as_secs() == 0 {
            return "∞ B/s".to_string();
        }

        let rate = bytes as f64 / duration.as_secs_f64();
        self.format_bytes(rate as u64) + "/s"
    }

    /// Write output to appropriate stream
    #[allow(dead_code)]
    fn write_output(&self, text: &str) {
        self.write_output_with_ending(text, false);
    }

    /// Write output with control over line ending
    fn write_output_with_ending(&self, text: &str, use_newline: bool) {
        let ending = if use_newline { b"\n" } else { b"\r" };

        if self.config.use_stderr {
            let _ = io::stderr().write_all(text.as_bytes());
            let _ = io::stderr().write_all(ending);
            let _ = io::stderr().flush();
        } else {
            let _ = io::stdout().write_all(text.as_bytes());
            let _ = io::stdout().write_all(ending);
            let _ = io::stdout().flush();
        }
    }

    /// Hide cursor for cleaner progress display
    fn hide_cursor(&self) {
        let hide_seq = b"\x1b[?25l";
        if self.config.use_stderr {
            let _ = io::stderr().write_all(hide_seq);
            let _ = io::stderr().flush();
        } else {
            let _ = io::stdout().write_all(hide_seq);
            let _ = io::stdout().flush();
        }
    }

    /// Show cursor when progress is complete
    fn show_cursor(&self) {
        let show_seq = b"\x1b[?25h";
        if self.config.use_stderr {
            let _ = io::stderr().write_all(show_seq);
            let _ = io::stderr().flush();
        } else {
            let _ = io::stdout().write_all(show_seq);
            let _ = io::stdout().flush();
        }
    }
}

impl ProgressReporter for TerminalReporter {
    fn report(&self, event: &ProgressEvent) {
        // Throttle updates to avoid flooding terminal
        if !self.should_update() && event.state == ProgressState::Running {
            return;
        }

        // Handle cursor visibility based on task state
        let is_finished = matches!(
            event.state,
            ProgressState::Complete | ProgressState::Failed | ProgressState::Cancelled
        );

        if !is_finished {
            // Hide cursor when first running task starts
            let mut cursor_hidden = self.cursor_hidden.lock().unwrap();
            if !*cursor_hidden {
                self.hide_cursor();
                *cursor_hidden = true;
            }
        }

        let formatted = self.format_event(event);

        if !formatted.is_empty() {
            // Use newline for completed/failed/cancelled states, carriage return for running
            self.write_output_with_ending(&formatted, is_finished);
        }

        // Show cursor when task finishes
        if is_finished {
            // Show cursor when task completes
            let mut cursor_hidden = self.cursor_hidden.lock().unwrap();
            if *cursor_hidden {
                self.show_cursor();
                *cursor_hidden = false;
            }
        }

        // Clean up completed tasks if configured
        if self.config.clear_on_complete && is_finished {
            let mut task_states = self.task_states.lock().unwrap();
            task_states.remove(&event.task_id);
        }
    }

    fn name(&self) -> &'static str {
        "terminal"
    }

    fn supports_realtime(&self) -> bool {
        true
    }

    fn configure(&mut self, config: HashMap<String, String>) {
        if let Some(value) = config.get("use_colors") {
            self.config.use_colors = value.parse().unwrap_or(true);
        }
        if let Some(value) = config.get("use_unicode") {
            self.config.use_unicode = value.parse().unwrap_or(true);
        }
        if let Some(value) = config.get("use_stderr") {
            self.config.use_stderr = value.parse().unwrap_or(false);
        }
        if let Some(value) = config.get("update_interval_ms") {
            self.config.update_interval_ms = value.parse().unwrap_or(50);
        }
    }
}

impl Default for TerminalReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TerminalReporter {
    fn drop(&mut self) {
        // Always show cursor when reporter is dropped to avoid leaving cursor hidden
        let cursor_hidden = self.cursor_hidden.lock().unwrap();
        if *cursor_hidden {
            self.show_cursor();
        }
    }
}

/// Silent progress reporter (no output)
pub struct SilentReporter;

impl SilentReporter {
    pub fn new() -> Self {
        Self
    }
}

impl ProgressReporter for SilentReporter {
    fn report(&self, _event: &ProgressEvent) {
        // Do nothing
    }

    fn name(&self) -> &'static str {
        "silent"
    }

    fn supports_realtime(&self) -> bool {
        false
    }
}

impl From<SilentReporter> for TerminalReporter {
    fn from(_silent: SilentReporter) -> Self {
        // Return a terminal reporter that doesn't actually output anything
        Self::with_config(TerminalConfig {
            update_interval_ms: u64::MAX, // Never update
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::core::ProgressEvent;
    use super::*;

    #[test]
    fn test_terminal_reporter_creation() {
        let reporter = TerminalReporter::new();
        assert_eq!(reporter.name(), "terminal");
        assert!(reporter.supports_realtime());

        let simple_reporter = TerminalReporter::simple();
        assert!(!simple_reporter.config.use_colors);
        assert!(!simple_reporter.config.use_unicode);
    }

    #[test]
    fn test_silent_reporter() {
        let reporter = SilentReporter::new();
        assert_eq!(reporter.name(), "silent");
        assert!(!reporter.supports_realtime());

        // Should do nothing
        let event = ProgressEvent {
            task_id: 1,
            timestamp: std::time::Instant::now(),
            current: 50,
            total: Some(100),
            message: Some("Test".to_string()),
            state: ProgressState::Running,
        };
        reporter.report(&event);
    }

    #[test]
    fn test_format_helpers() {
        let reporter = TerminalReporter::new();

        // Test duration formatting
        let duration = Duration::from_secs(65);
        let formatted = reporter.format_duration(duration);
        assert!(formatted.contains("1m") && formatted.contains("5s"));

        // Test byte formatting
        let bytes = 1536; // 1.5 KB
        let formatted = reporter.format_bytes(bytes);
        assert!(formatted.contains("1.5") && formatted.contains("KB"));
    }

    #[test]
    fn test_style_inference() {
        let reporter = TerminalReporter::new();

        // Test bar style inference
        let event = ProgressEvent {
            task_id: 1,
            timestamp: std::time::Instant::now(),
            current: 10,
            total: Some(100),
            message: None,
            state: ProgressState::Running,
        };
        let style = reporter.infer_style(&event);
        assert!(matches!(style, ProgressStyle::Bar { total: 100 }));

        // Test bytes style inference
        let event = ProgressEvent {
            task_id: 2,
            timestamp: std::time::Instant::now(),
            current: 1000,
            total: Some(2000000),
            message: None,
            state: ProgressState::Running,
        };
        let style = reporter.infer_style(&event);
        assert!(matches!(
            style,
            ProgressStyle::Bytes {
                total_bytes: 2000000
            }
        ));

        // Test spinner inference
        let event = ProgressEvent {
            task_id: 3,
            timestamp: std::time::Instant::now(),
            current: 0,
            total: None,
            message: None,
            state: ProgressState::Running,
        };
        let style = reporter.infer_style(&event);
        assert!(matches!(style, ProgressStyle::Spinner));
    }
}
