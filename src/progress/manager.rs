//! Progress Manager
//!
//! Central coordinator for multiple progress tasks.
//! Framework-agnostic and extractable to RSB.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicU64, Ordering}};

use super::core::{ProgressReporter, ProgressTask, TaskId, TaskBuilder};
use super::styles::ProgressStyle;

/// Central manager for progress tasks
pub struct ProgressManager {
    next_task_id: AtomicU64,
    active_tasks: Arc<Mutex<HashMap<TaskId, Arc<ProgressTask>>>>,
    reporters: Arc<Mutex<Vec<Arc<dyn ProgressReporter>>>>,
    enabled: bool,
}

impl ProgressManager {
    /// Create a new progress manager
    pub fn new() -> Self {
        Self {
            next_task_id: AtomicU64::new(1),
            active_tasks: Arc::new(Mutex::new(HashMap::new())),
            reporters: Arc::new(Mutex::new(Vec::new())),
            enabled: true,
        }
    }

    /// Create a disabled progress manager (no-op)
    pub fn disabled() -> Self {
        let mut manager = Self::new();
        manager.enabled = false;
        manager
    }

    /// Add a progress reporter
    pub fn add_reporter(&self, reporter: Arc<dyn ProgressReporter>) {
        if self.enabled {
            self.reporters.lock().unwrap().push(reporter);
        }
    }

    /// Remove all reporters
    pub fn clear_reporters(&self) {
        self.reporters.lock().unwrap().clear();
    }

    /// Enable or disable progress reporting
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.clear_reporters();
        }
    }

    /// Check if progress reporting is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Start a new progress task
    pub fn start_task(&self, title: impl Into<String>, style: ProgressStyle) -> Arc<ProgressTask> {
        if !self.enabled {
            // Return a no-op task for disabled manager
            return Arc::new(self.create_noop_task(title.into()));
        }

        let task_id = self.next_task_id.fetch_add(1, Ordering::Relaxed);
        let reporters = self.reporters.lock().unwrap().clone();

        let mut builder = TaskBuilder::new(title);
        if let Some(total) = style.total() {
            builder = builder.with_total(total);
        }

        let task = Arc::new(builder.build(task_id, reporters));

        // Add to active tasks
        self.active_tasks.lock().unwrap().insert(task_id, task.clone());

        task
    }

    /// Start a task with custom builder
    pub fn start_task_with_builder(&self, builder: TaskBuilder, style: ProgressStyle) -> Arc<ProgressTask> {
        if !self.enabled {
            return Arc::new(self.create_noop_task("disabled".to_string()));
        }

        let task_id = self.next_task_id.fetch_add(1, Ordering::Relaxed);
        let reporters = self.reporters.lock().unwrap().clone();

        let mut builder = builder;
        if let Some(total) = style.total() {
            builder = builder.with_total(total);
        }

        let task = Arc::new(builder.build(task_id, reporters));

        self.active_tasks.lock().unwrap().insert(task_id, task.clone());
        task
    }

    /// Get a task by ID
    pub fn get_task(&self, task_id: TaskId) -> Option<Arc<ProgressTask>> {
        self.active_tasks.lock().unwrap().get(&task_id).cloned()
    }

    /// Get all active (non-finished) tasks
    pub fn active_tasks(&self) -> Vec<Arc<ProgressTask>> {
        self
            .active_tasks
            .lock()
            .unwrap()
            .values()
            .filter(|t| !t.is_finished())
            .cloned()
            .collect()
    }

    /// Remove a completed task
    pub fn remove_task(&self, task_id: TaskId) -> Option<Arc<ProgressTask>> {
        self.active_tasks.lock().unwrap().remove(&task_id)
    }

    /// Clean up finished tasks
    pub fn cleanup_finished(&self) -> usize {
        let mut active_tasks = self.active_tasks.lock().unwrap();
        let initial_count = active_tasks.len();

        active_tasks.retain(|_, task| !task.is_finished());

        initial_count - active_tasks.len()
    }

    /// Cancel all active tasks
    pub fn cancel_all(&self, reason: &str) {
        let tasks = self.active_tasks();
        for task in tasks {
            task.cancel(reason);
        }
        self.active_tasks.lock().unwrap().clear();
    }

    /// Wait for all tasks to complete
    pub fn wait_all(&self, timeout_ms: Option<u64>) -> bool {
        let start = std::time::Instant::now();
        let timeout = timeout_ms.map(std::time::Duration::from_millis);

        loop {
            let active_count = self.active_tasks.lock().unwrap().len();
            if active_count == 0 {
                return true;
            }

            if let Some(timeout) = timeout {
                if start.elapsed() > timeout {
                    return false;
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    /// Get progress statistics
    pub fn stats(&self) -> ProgressStats {
        let active_tasks = self.active_tasks.lock().unwrap();
        let mut stats = ProgressStats::default();

        for task in active_tasks.values() {
            stats.total_tasks += 1;

            match task.state() {
                super::core::ProgressState::Running => stats.running_tasks += 1,
                super::core::ProgressState::Complete => stats.completed_tasks += 1,
                super::core::ProgressState::Failed => stats.failed_tasks += 1,
                super::core::ProgressState::Cancelled => stats.cancelled_tasks += 1,
            }

            if let (Some(percentage), Some(total)) = (task.percentage(), task.total_progress()) {
                stats.total_progress += (percentage / 100.0) * total as f64;
                stats.total_items += total;
            }
        }

        stats
    }

    /// Create a no-op task for disabled manager
    fn create_noop_task(&self, title: String) -> ProgressTask {
        TaskBuilder::new(title).build(0, vec![])
    }
}

impl Default for ProgressManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Progress statistics
#[derive(Debug, Default, Clone)]
pub struct ProgressStats {
    pub total_tasks: usize,
    pub running_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
    pub cancelled_tasks: usize,
    pub total_progress: f64,
    pub total_items: u64,
}

impl ProgressStats {
    /// Get overall completion percentage
    pub fn overall_percentage(&self) -> f64 {
        if self.total_items == 0 {
            0.0
        } else {
            (self.total_progress / self.total_items as f64) * 100.0
        }
    }

    /// Check if all tasks are completed
    pub fn all_complete(&self) -> bool {
        self.total_tasks > 0 && self.running_tasks == 0 && self.failed_tasks == 0 && self.cancelled_tasks == 0
    }

    /// Check if any tasks failed
    pub fn has_failures(&self) -> bool {
        self.failed_tasks > 0
    }
}

/// Builder for progress manager with configuration
#[derive(Debug, Default)]
pub struct ManagerBuilder {
    enabled: bool,
    auto_cleanup: bool,
    cleanup_interval_ms: u64,
}

impl ManagerBuilder {
    /// Create a new manager builder
    pub fn new() -> Self {
        Self {
            enabled: true,
            auto_cleanup: false,
            cleanup_interval_ms: 1000,
        }
    }

    /// Enable or disable progress reporting
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Enable automatic cleanup of finished tasks
    pub fn with_auto_cleanup(mut self, interval_ms: u64) -> Self {
        self.auto_cleanup = true;
        self.cleanup_interval_ms = interval_ms;
        self
    }

    /// Build the progress manager
    pub fn build(self) -> ProgressManager {
        if self.enabled {
            let manager = ProgressManager::new();

            // Start auto-cleanup thread if requested
            if self.auto_cleanup {
                let active_tasks = manager.active_tasks.clone();
                let interval = std::time::Duration::from_millis(self.cleanup_interval_ms);

                std::thread::spawn(move || {
                    loop {
                        std::thread::sleep(interval);
                        let mut tasks = active_tasks.lock().unwrap();
                        tasks.retain(|_, task| !task.is_finished());
                    }
                });
            }

            manager
        } else {
            ProgressManager::disabled()
        }
    }
}

/// Multi-step progress tracker for complex operations
pub struct MultiStepProgress {
    manager: Arc<ProgressManager>,
    main_task: Arc<ProgressTask>,
    steps: Vec<StepInfo>,
    current_step: usize,
}

struct StepInfo {
    name: String,
    weight: f64, // Relative weight for progress calculation
    task: Option<Arc<ProgressTask>>,
}

impl MultiStepProgress {
    /// Create a new multi-step progress tracker
    pub fn new(manager: Arc<ProgressManager>, title: String, steps: Vec<(String, f64)>) -> Self {
        let total_weight: f64 = steps.iter().map(|(_, weight)| weight).sum();
        let main_task = manager.start_task(&title, ProgressStyle::Percentage { total: 100 });

        let step_infos = steps
            .into_iter()
            .map(|(name, weight)| StepInfo {
                name,
                weight: weight / total_weight, // Normalize weights
                task: None,
            })
            .collect();

        Self {
            manager,
            main_task,
            steps: step_infos,
            current_step: 0,
        }
    }

    /// Start the next step
    pub fn next_step(&mut self, style: ProgressStyle) -> Option<Arc<ProgressTask>> {
        if self.current_step >= self.steps.len() {
            return None;
        }

        let step = &mut self.steps[self.current_step];
        let task = self.manager.start_task(&step.name, style);
        step.task = Some(task.clone());

        self.current_step += 1;
        Some(task)
    }

    /// Update overall progress based on step completion
    pub fn update_overall(&self) {
        let mut total_progress = 0.0;

        for (i, step) in self.steps.iter().enumerate() {
            if i < self.current_step {
                // Completed steps contribute full weight
                total_progress += step.weight;
            } else if i == self.current_step - 1 {
                // Current step contributes partial weight based on progress
                if let Some(ref task) = step.task {
                    if let Some(percentage) = task.percentage() {
                        total_progress += step.weight * (percentage / 100.0);
                    }
                }
            }
        }

        let overall_percentage = (total_progress * 100.0) as u64;
        self.main_task.update(overall_percentage, &format!("Step {}/{}", self.current_step, self.steps.len()));
    }

    /// Complete the multi-step operation
    pub fn complete(&self, message: &str) {
        self.main_task.complete(message);
    }

    /// Get the main task handle
    pub fn main_task(&self) -> &Arc<ProgressTask> {
        &self.main_task
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::progress::ProgressState;

    #[test]
    fn test_progress_manager_creation() {
        let manager = ProgressManager::new();
        assert!(manager.is_enabled());
        assert_eq!(manager.active_tasks().len(), 0);

        let disabled_manager = ProgressManager::disabled();
        assert!(!disabled_manager.is_enabled());
    }

    #[test]
    fn test_task_creation_and_management() {
        let manager = ProgressManager::new();

        let task1 = manager.start_task("Task 1", ProgressStyle::Spinner);
        let task2 = manager.start_task("Task 2", ProgressStyle::Bar { total: 100 });

        assert_eq!(manager.active_tasks().len(), 2);

        task1.complete("Done");
        assert_eq!(manager.cleanup_finished(), 1);
        assert_eq!(manager.active_tasks().len(), 1);

        task2.fail("Error");
        assert_eq!(manager.cleanup_finished(), 1);
        assert_eq!(manager.active_tasks().len(), 0);
    }

    #[test]
    fn test_disabled_manager() {
        let manager = ProgressManager::disabled();
        let task = manager.start_task("Test", ProgressStyle::Spinner);

        // Should create a no-op task
        assert_eq!(task.id(), 0);
        assert_eq!(manager.active_tasks().len(), 0);
    }

    #[test]
    fn test_progress_stats() {
        let manager = ProgressManager::new();

        let task1 = manager.start_task("Task 1", ProgressStyle::Bar { total: 100 });
        let task2 = manager.start_task("Task 2", ProgressStyle::Bar { total: 200 });

        task1.update(50, "Half way");
        task2.update(100, "Half way");

        let stats = manager.stats();
        assert_eq!(stats.total_tasks, 2);
        assert_eq!(stats.running_tasks, 2);
        assert_eq!(stats.total_items, 300);

        task1.complete("Done");
        let stats = manager.stats();
        assert_eq!(stats.completed_tasks, 1);
        assert_eq!(stats.running_tasks, 1);
    }

    #[test]
    fn test_multi_step_progress() {
        let manager = Arc::new(ProgressManager::new());
        let steps = vec![
            ("Backup".to_string(), 1.0),
            ("Process".to_string(), 3.0),
            ("Cleanup".to_string(), 1.0),
        ];

        let mut multi = MultiStepProgress::new(manager, "Multi-step operation".to_string(), steps);

        // Start first step
        let step1 = multi.next_step(ProgressStyle::Spinner).unwrap();
        step1.complete("Backup done");
        multi.update_overall();

        // Start second step
        let step2 = multi.next_step(ProgressStyle::Bar { total: 100 }).unwrap();
        step2.update(50, "Processing");
        multi.update_overall();

        step2.complete("Processing done");
        multi.update_overall();

        // Start third step
        let step3 = multi.next_step(ProgressStyle::Spinner).unwrap();
        step3.complete("Cleanup done");
        multi.update_overall();

        multi.complete("All steps completed");

        assert_eq!(multi.main_task().state(), ProgressState::Complete);
    }

    #[test]
    fn test_manager_builder() {
        let manager = ManagerBuilder::new()
            .enabled(true)
            .with_auto_cleanup(500)
            .build();

        assert!(manager.is_enabled());

        let disabled_manager = ManagerBuilder::new()
            .enabled(false)
            .build();

        assert!(!disabled_manager.is_enabled());
    }
}
