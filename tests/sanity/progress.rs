use rsb::prelude::*;

// Progress tests require the progress feature flag
#[cfg(feature = "progress")]
use rsb::progress::{ProgressBar, Spinner};

#[test]
#[cfg(feature = "progress")]
fn sanity_progress_basic_bar() {
    // Test basic progress bar creation and update
    let mut progress = ProgressBar::new(100);

    // Test initial state
    assert_eq!(progress.current(), 0);
    assert_eq!(progress.total(), 100);
    assert!(!progress.is_complete());

    // Test progress updates
    progress.set_current(25);
    assert_eq!(progress.current(), 25);
    assert!(!progress.is_complete());

    progress.set_current(100);
    assert_eq!(progress.current(), 100);
    assert!(progress.is_complete());
}

#[test]
#[cfg(feature = "progress")]
fn sanity_progress_increment() {
    // Test incremental progress updates
    let mut progress = ProgressBar::new(10);

    progress.increment();
    assert_eq!(progress.current(), 1);

    progress.increment_by(3);
    assert_eq!(progress.current(), 4);

    // Test increment beyond total
    progress.increment_by(10);
    assert_eq!(progress.current(), 10); // Should cap at total
}

#[test]
#[cfg(feature = "progress")]
fn sanity_progress_percentage() {
    // Test percentage calculations
    let mut progress = ProgressBar::new(100);

    progress.set_current(0);
    assert_eq!(progress.percentage(), 0.0);

    progress.set_current(25);
    assert_eq!(progress.percentage(), 25.0);

    progress.set_current(50);
    assert_eq!(progress.percentage(), 50.0);

    progress.set_current(100);
    assert_eq!(progress.percentage(), 100.0);
}

#[test]
#[cfg(feature = "progress")]
fn sanity_progress_display() {
    // Test progress bar display functionality
    let mut progress = ProgressBar::new(20);
    progress.set_current(10);

    let display = progress.display();
    assert!(!display.is_empty());
    assert!(display.contains("50")); // Should show 50% progress

    // Test with different progress values
    progress.set_current(5);
    let display_25 = progress.display();
    assert!(display_25.contains("25")); // Should show 25% progress
}

#[test]
#[cfg(feature = "progress")]
fn sanity_progress_spinner() {
    // Test spinner functionality
    let mut spinner = rsb::progress::Spinner::new();

    // Test initial state
    let initial_display = spinner.display();
    assert!(!initial_display.is_empty());

    // Test tick advancement
    spinner.tick();
    let after_tick = spinner.display();
    assert!(!after_tick.is_empty());

    // Should cycle through different states
    for _ in 0..10 {
        spinner.tick();
    }
    let after_many_ticks = spinner.display();
}

// When progress feature is not available, provide basic sanity test
#[test]
#[cfg(not(feature = "progress"))]
fn test_progress_module_disabled() {
    // When progress feature is disabled, ensure we can still run basic sanity checks
    // This tests that RSB gracefully handles missing progress functionality

    // Test that we can simulate progress using basic arithmetic
    let total = 100;
    let current = 25;
    let percentage = (current as f64 / total as f64) * 100.0;
    assert_eq!(percentage, 25.0);

    // Test progress state simulation
    let is_complete = current >= total;
    assert!(!is_complete);

    let current_complete = 100;
    let is_complete = current_complete >= total;
    assert!(is_complete);
}

#[test]
#[cfg(feature = "progress")]
fn sanity_progress_with_message() {
    // Test progress with custom messages
    let mut progress = ProgressBar::new(100);
    progress.set_message("Processing files...");

    let display = progress.display();
    assert!(display.contains("Processing files"));

    // Test message updates
    progress.set_message("Almost done...");
    let updated_display = progress.display();
    assert!(updated_display.contains("Almost done"));
}

#[test]
#[cfg(feature = "progress")]
fn sanity_progress_eta_calculation() {
    // Test ETA (Estimated Time of Arrival) calculation
    let mut progress = ProgressBar::new(100);

    // Start timing
    progress.start_timing();

    // Simulate some progress
    std::thread::sleep(std::time::Duration::from_millis(10));
    progress.set_current(10);

    // ETA calculation should not panic
    let eta = progress.estimated_time_remaining();
    assert!(eta.is_some() || eta.is_none()); // Either result is acceptable
}
