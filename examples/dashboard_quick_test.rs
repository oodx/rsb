use rsb::progress::{ProgressManager, ProgressStyle, TerminalReporter};
use std::sync::Arc;

fn main() {
    println!("🎨 Quick Dashboard Test\n");

    // Initialize colors
    #[cfg(feature = "colors-core")]
    {
        rsb::colors::color_mode("always");
        rsb::colors::color_enable_with("simple,status");
    }

    let manager = ProgressManager::new();
    manager.add_reporter(Arc::new(TerminalReporter::new()));

    // Test 1: Simple 5-chunk dashboard
    println!("[Test 1] 5 chunks x 100 bytes\n");
    let task1 = manager.start_task(
        "file-backup.dat",
        ProgressStyle::Dashboard {
            total_chunks: 5,
            chunk_size: 100,
            title: None,
        },
    );

    for chunk in 0..5 {
        for byte in 0..=100 {
            let current = chunk * 100 + byte;
            task1.update(current, &format!("file-backup.dat"));
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    task1.complete("file-backup.dat");

    println!("\n\n[Test 2] 8 chunks x 50 bytes (faster)\n");
    let task2 = manager.start_task(
        "database-export.sql",
        ProgressStyle::Dashboard {
            total_chunks: 8,
            chunk_size: 50,
            title: None,
        },
    );

    for chunk in 0..8 {
        for byte in 0..=50 {
            let current = chunk * 50 + byte;
            task2.update(current, &format!("database-export.sql"));
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }
    task2.complete("database-export.sql");

    println!("\n\n✅ Dashboard tests complete!");
    println!("\nYou should have seen:");
    println!("  • Status indicator (▶ Running)");
    println!("  • Size, elapsed time, ETA, chunk count");
    println!("  • Chunk blocks: ■ complete, █ current (blinking), □ pending");
    println!("  • Progress bar showing current chunk progress");
}
