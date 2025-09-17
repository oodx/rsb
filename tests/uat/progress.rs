use rsb::prelude::*;

#[cfg(feature = "progress")]
use rsb::progress::{Progress, ProgressConfig, ProgressStyle};

#[test]
#[cfg(feature = "progress")]
fn uat_progress_spinner_demo() {
    println!("\n=== UAT: Progress Spinner Demo ===");

    let config = ProgressConfig::spinner()
        .with_message("Processing files...")
        .with_style(ProgressStyle::default());

    let mut progress = Progress::new(config);
    progress.start();

    // Simulate work with updates
    for i in 0..5 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        progress.update_message(&format!("Processing file {}...", i + 1));
    }

    progress.finish_with_message("✓ Processing complete!");
    println!("Spinner demo completed");
}

#[test]
#[cfg(feature = "progress")]
fn uat_progress_bar_demo() {
    println!("\n=== UAT: Progress Bar Demo ===");

    let total = 100;
    let config = ProgressConfig::bar(total)
        .with_message("Downloading...")
        .with_style(ProgressStyle::default());

    let mut progress = Progress::new(config);
    progress.start();

    // Simulate download progress
    for i in 0..=total {
        std::thread::sleep(std::time::Duration::from_millis(10));
        progress.set(i);
        if i % 20 == 0 {
            progress.update_message(&format!("Downloading... {}%", i));
        }
    }

    progress.finish_with_message("✓ Download complete!");
    println!("Progress bar demo completed");
}

#[test]
#[cfg(feature = "progress")]
fn uat_progress_bytes_demo() {
    println!("\n=== UAT: Progress Bytes Demo ===");

    let total_bytes = 1024 * 1024 * 50; // 50 MB
    let config = ProgressConfig::bytes(total_bytes)
        .with_message("Transferring data...")
        .with_style(ProgressStyle::default());

    let mut progress = Progress::new(config);
    progress.start();

    // Simulate data transfer
    let chunk_size = 1024 * 512; // 512 KB chunks
    let mut transferred = 0u64;

    while transferred < total_bytes {
        std::thread::sleep(std::time::Duration::from_millis(50));
        transferred = (transferred + chunk_size).min(total_bytes);
        progress.set(transferred);
    }

    progress.finish_with_message("✓ Transfer complete!");
    println!("Bytes progress demo completed");
}

#[test]
#[cfg(not(feature = "progress"))]
fn uat_progress_feature_disabled() {
    println!("\n=== UAT: Progress Feature Disabled ===");
    println!("Progress feature is not enabled. Enable with:");
    println!("  cargo test --features progress");
    println!("Progress module provides spinner, bar, and bytes indicators.");
}