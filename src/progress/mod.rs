//! Modular Progress Reporting Framework
//!
//! A framework-agnostic progress reporting system designed for RSB integration.
//! Provides composable progress indicators for CLI applications with zero-dependency core.
//!
//! # Architecture
//! - **Trait-based**: Core functionality through traits
//! - **Zero-dependency**: Core types use only std library
//! - **Composable**: Mix and match different progress styles
//! - **Extractable**: Can be moved to RSB without modification
//!
//! # Usage
//! ```rust
//! use cage::progress::{ProgressManager, ProgressStyle};
//!
//! let mut progress = ProgressManager::new();
//! let task = progress.start_task("Processing files", ProgressStyle::Bar { total: 10 });
//!
//! for i in 0..10 {
//!     task.update(i + 1, &format!("Processing file {}", i + 1));
//!     std::thread::sleep(std::time::Duration::from_millis(100));
//! }
//!
//! task.complete("All files processed");
//! ```

pub mod core;
pub mod styles;
pub mod manager;
pub mod terminal;

// Re-exports for convenience
pub use core::{ProgressReporter, ProgressTask, ProgressEvent, ProgressState};
pub use styles::{ProgressStyle, SpinnerStyle, BarStyle};
pub use manager::ProgressManager;
pub use terminal::{TerminalReporter, TerminalConfig};

/// Quick-start progress creation functions
pub mod prelude {
    pub use super::{
        ProgressManager,
        ProgressStyle,
        ProgressReporter,
        ProgressTask,
        TerminalReporter,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_progress_framework_creation() {
        let manager = ProgressManager::new();
        assert!(manager.active_tasks().is_empty());
    }

    #[test]
    fn test_progress_task_lifecycle() {
        let mut manager = ProgressManager::new();
        let task = manager.start_task("Test task", ProgressStyle::Spinner);

        assert_eq!(task.current_progress(), 0);
        assert_eq!(task.state(), ProgressState::Running);

        task.update(50, "Half way");
        assert_eq!(task.current_progress(), 50);

        task.complete("Done");
        assert_eq!(task.state(), ProgressState::Complete);
    }

    #[test]
    fn test_multiple_tasks() {
        let mut manager = ProgressManager::new();

        let task1 = manager.start_task("Task 1", ProgressStyle::Bar { total: 100 });
        let task2 = manager.start_task("Task 2", ProgressStyle::Spinner);

        assert_eq!(manager.active_tasks().len(), 2);

        task1.complete("Task 1 done");
        assert_eq!(manager.active_tasks().len(), 1);

        task2.complete("Task 2 done");
        assert_eq!(manager.active_tasks().len(), 0);
    }
}