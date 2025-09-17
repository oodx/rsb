//! Core Progress Framework Types
//!
//! Framework-agnostic core types for progress reporting.
//! Zero dependencies - only uses std library.
//! Designed for RSB extraction.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Unique identifier for a progress task
pub type TaskId = u64;

/// Current state of a progress task
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressState {
    /// Task is currently running
    Running,
    /// Task completed successfully
    Complete,
    /// Task failed with an error
    Failed,
    /// Task was cancelled
    Cancelled,
}

/// Event emitted during progress updates
#[derive(Debug, Clone)]
pub struct ProgressEvent {
    pub task_id: TaskId,
    pub timestamp: Instant,
    pub current: u64,
    pub total: Option<u64>,
    pub message: Option<String>,
    pub state: ProgressState,
}

/// Core trait for progress reporting implementations
pub trait ProgressReporter: Send + Sync {
    /// Handle a progress event
    fn report(&self, event: &ProgressEvent);

    /// Configure the reporter (optional)
    fn configure(&mut self, config: HashMap<String, String>) {
        let _ = config; // Default: ignore configuration
    }

    /// Cleanup resources when reporter is no longer needed
    fn cleanup(&mut self) {
        // Default: no cleanup needed
    }

    /// Check if this reporter supports real-time updates
    fn supports_realtime(&self) -> bool {
        true
    }

    /// Get reporter name/type for debugging
    fn name(&self) -> &'static str {
        "unknown"
    }
}

/// Handle to a specific progress task
pub struct ProgressTask {
    id: TaskId,
    state: Arc<Mutex<TaskState>>,
    reporters: Vec<Arc<dyn ProgressReporter>>,
}

#[derive(Debug)]
struct TaskState {
    current: u64,
    total: Option<u64>,
    message: String,
    state: ProgressState,
    created_at: Instant,
    updated_at: Instant,
}

impl ProgressTask {
    /// Create a new progress task
    pub(crate) fn new(
        id: TaskId,
        title: String,
        total: Option<u64>,
        reporters: Vec<Arc<dyn ProgressReporter>>,
    ) -> Self {
        let now = Instant::now();
        let state = Arc::new(Mutex::new(TaskState {
            current: 0,
            total,
            message: title,
            state: ProgressState::Running,
            created_at: now,
            updated_at: now,
        }));

        let task = Self {
            id,
            state,
            reporters,
        };

        // Send initial event
        task.emit_event();
        task
    }

    /// Update progress with current value and optional message
    pub fn update(&self, current: u64, message: &str) {
        {
            let mut state = self.state.lock().unwrap();
            state.current = current;
            state.message = message.to_string();
            state.updated_at = Instant::now();
        }
        self.emit_event();
    }

    /// Update progress with just a message
    pub fn update_message(&self, message: &str) {
        {
            let mut state = self.state.lock().unwrap();
            state.message = message.to_string();
            state.updated_at = Instant::now();
        }
        self.emit_event();
    }

    /// Increment progress by 1 with optional message
    pub fn increment(&self, message: Option<&str>) {
        {
            let mut state = self.state.lock().unwrap();
            state.current += 1;
            if let Some(msg) = message {
                state.message = msg.to_string();
            }
            state.updated_at = Instant::now();
        }
        self.emit_event();
    }

    /// Complete the task successfully
    pub fn complete(&self, message: &str) {
        {
            let mut state = self.state.lock().unwrap();
            state.message = message.to_string();
            state.state = ProgressState::Complete;
            state.updated_at = Instant::now();

            // Set current to total if we have a total
            if let Some(total) = state.total {
                state.current = total;
            }
        }
        self.emit_event();
    }

    /// Mark task as failed
    pub fn fail(&self, error_message: &str) {
        {
            let mut state = self.state.lock().unwrap();
            state.message = error_message.to_string();
            state.state = ProgressState::Failed;
            state.updated_at = Instant::now();
        }
        self.emit_event();
    }

    /// Cancel the task
    pub fn cancel(&self, reason: &str) {
        {
            let mut state = self.state.lock().unwrap();
            state.message = reason.to_string();
            state.state = ProgressState::Cancelled;
            state.updated_at = Instant::now();
        }
        self.emit_event();
    }

    /// Get current progress value
    pub fn current_progress(&self) -> u64 {
        self.state.lock().unwrap().current
    }

    /// Get total progress value (if known)
    pub fn total_progress(&self) -> Option<u64> {
        self.state.lock().unwrap().total
    }

    /// Get current state
    pub fn state(&self) -> ProgressState {
        self.state.lock().unwrap().state
    }

    /// Get current message
    pub fn message(&self) -> String {
        self.state.lock().unwrap().message.clone()
    }

    /// Get task ID
    pub fn id(&self) -> TaskId {
        self.id
    }

    /// Get elapsed time since task creation
    pub fn elapsed(&self) -> Duration {
        let state = self.state.lock().unwrap();
        state.updated_at.duration_since(state.created_at)
    }

    /// Calculate progress percentage (0-100)
    pub fn percentage(&self) -> Option<f64> {
        let state = self.state.lock().unwrap();
        state.total.map(|total| {
            if total == 0 {
                100.0
            } else {
                (state.current as f64 / total as f64) * 100.0
            }
        })
    }

    /// Check if task is finished (complete, failed, or cancelled)
    pub fn is_finished(&self) -> bool {
        let state = self.state.lock().unwrap().state;
        matches!(
            state,
            ProgressState::Complete | ProgressState::Failed | ProgressState::Cancelled
        )
    }

    /// Emit progress event to all reporters
    fn emit_event(&self) {
        let state = self.state.lock().unwrap();
        let event = ProgressEvent {
            task_id: self.id,
            timestamp: state.updated_at,
            current: state.current,
            total: state.total,
            message: Some(state.message.clone()),
            state: state.state,
        };
        drop(state); // Release lock before calling reporters

        for reporter in &self.reporters {
            reporter.report(&event);
        }
    }
}

/// Builder for progress tasks with fluent API
#[derive(Debug)]
pub struct TaskBuilder {
    title: String,
    total: Option<u64>,
    metadata: HashMap<String, String>,
}

impl TaskBuilder {
    /// Create a new task builder
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            total: None,
            metadata: HashMap::new(),
        }
    }

    /// Set total progress value for bar-style indicators
    pub fn with_total(mut self, total: u64) -> Self {
        self.total = Some(total);
        self
    }

    /// Add metadata to the task
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Build the task (internal use)
    pub(crate) fn build(
        self,
        id: TaskId,
        reporters: Vec<Arc<dyn ProgressReporter>>,
    ) -> ProgressTask {
        ProgressTask::new(id, self.title, self.total, reporters)
    }
}

/// Utility functions for progress calculations
pub mod utils {
    use super::*;

    /// Calculate ETA based on current progress and elapsed time
    pub fn calculate_eta(current: u64, total: u64, elapsed: Duration) -> Option<Duration> {
        if current == 0 || current >= total {
            return None;
        }

        let progress_ratio = current as f64 / total as f64;
        let total_estimated = elapsed.as_secs_f64() / progress_ratio;
        let remaining = total_estimated - elapsed.as_secs_f64();

        if remaining > 0.0 {
            Some(Duration::from_secs_f64(remaining))
        } else {
            None
        }
    }

    /// Format duration for display
    pub fn format_duration(duration: Duration) -> String {
        let secs = duration.as_secs();
        if secs < 60 {
            format!("{}s", secs)
        } else if secs < 3600 {
            format!("{}m {}s", secs / 60, secs % 60)
        } else {
            format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
        }
    }

    /// Format bytes for display
    pub fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }

    /// Format rate (items/operations per second)
    pub fn format_rate(items: u64, duration: Duration) -> String {
        if duration.as_secs() == 0 {
            return "∞ items/s".to_string();
        }

        let rate = items as f64 / duration.as_secs_f64();
        if rate >= 1.0 {
            format!("{:.1} items/s", rate)
        } else {
            format!("{:.2} items/s", rate)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockReporter {
        events: Arc<Mutex<Vec<ProgressEvent>>>,
    }

    impl MockReporter {
        fn new() -> Self {
            Self {
                events: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn events(&self) -> Vec<ProgressEvent> {
            self.events.lock().unwrap().clone()
        }
    }

    impl ProgressReporter for MockReporter {
        fn report(&self, event: &ProgressEvent) {
            self.events.lock().unwrap().push(event.clone());
        }

        fn name(&self) -> &'static str {
            "mock"
        }
    }

    #[test]
    fn test_progress_task_lifecycle() {
        let reporter = Arc::new(MockReporter::new());
        let task = ProgressTask::new(1, "Test".to_string(), Some(100), vec![reporter.clone()]);

        assert_eq!(task.current_progress(), 0);
        assert_eq!(task.total_progress(), Some(100));
        assert_eq!(task.state(), ProgressState::Running);

        task.update(50, "Half way");
        assert_eq!(task.current_progress(), 50);
        assert_eq!(task.percentage(), Some(50.0));

        task.complete("Done");
        assert_eq!(task.state(), ProgressState::Complete);
        assert_eq!(task.current_progress(), 100);

        // Verify events were sent
        let events = reporter.events();
        assert_eq!(events.len(), 3); // Initial, update, complete
        assert_eq!(events[0].current, 0);
        assert_eq!(events[1].current, 50);
        assert_eq!(events[2].current, 100);
        assert_eq!(events[2].state, ProgressState::Complete);
    }

    #[test]
    fn test_task_builder() {
        let builder = TaskBuilder::new("Test task")
            .with_total(200)
            .with_metadata("type", "file");

        assert_eq!(builder.title, "Test task");
        assert_eq!(builder.total, Some(200));
        assert_eq!(builder.metadata.get("type"), Some(&"file".to_string()));
    }

    #[test]
    fn test_progress_calculations() {
        use utils::*;

        // Test ETA calculation
        let eta = calculate_eta(25, 100, Duration::from_secs(10));
        assert!(eta.is_some());
        assert_eq!(eta.unwrap().as_secs(), 30); // 10s for 25%, so 40s total, 30s remaining

        // Test duration formatting
        assert_eq!(format_duration(Duration::from_secs(30)), "30s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m");

        // Test byte formatting
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");

        // Test rate formatting
        assert_eq!(format_rate(100, Duration::from_secs(10)), "10.0 items/s");
        assert_eq!(format_rate(5, Duration::from_secs(10)), "0.50 items/s");
    }

    #[test]
    fn test_multiple_reporters() {
        let reporter1 = Arc::new(MockReporter::new());
        let reporter2 = Arc::new(MockReporter::new());

        let task = ProgressTask::new(
            1,
            "Test".to_string(),
            None,
            vec![reporter1.clone(), reporter2.clone()],
        );

        task.update(10, "Progress");

        assert_eq!(reporter1.events().len(), 2); // Initial + update
        assert_eq!(reporter2.events().len(), 2); // Initial + update
    }
}
