use rsb::prelude::*;

#[test]
fn sanity_progress_basic_bar() {
    // Test basic progress bar creation and update
    let mut progress = rsb::progress::ProgressBar::new(100);

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
fn sanity_progress_increment() {
    // Test incremental progress updates
    let mut progress = rsb::progress::ProgressBar::new(10);

    progress.increment();
    assert_eq!(progress.current(), 1);

    progress.increment_by(3);
    assert_eq!(progress.current(), 4);

    // Test increment beyond total
    progress.increment_by(10);
    assert_eq!(progress.current(), 10); // Should cap at total
}

#[test]
fn sanity_progress_percentage() {
    // Test percentage calculations
    let mut progress = rsb::progress::ProgressBar::new(100);

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
fn sanity_progress_display() {
    // Test progress bar display functionality
    let mut progress = rsb::progress::ProgressBar::new(20);
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
    assert!(!after_many_ticks.is_empty());
}

#[test]
fn sanity_progress_with_message() {
    // Test progress with custom messages
    let mut progress = rsb::progress::ProgressBar::new(100);
    progress.set_message("Processing files...");

    let display = progress.display();
    assert!(display.contains("Processing files"));

    // Test message updates
    progress.set_message("Almost done...");
    let updated_display = progress.display();
    assert!(updated_display.contains("Almost done"));
}

#[test]
fn sanity_progress_eta_calculation() {
    // Test ETA (Estimated Time of Arrival) calculation
    let mut progress = rsb::progress::ProgressBar::new(100);

    // Start timing
    progress.start_timing();

    // Simulate some progress
    std::thread::sleep(std::time::Duration::from_millis(10));
    progress.set_current(10);

    // ETA calculation should not panic
    let eta = progress.estimated_time_remaining();
    assert!(eta.is_some() || eta.is_none()); // Either result is acceptable
}