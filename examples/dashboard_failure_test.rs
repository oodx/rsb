use rsb::progress::{ProgressManager, ProgressStyle, TerminalReporter};
use std::sync::Arc;

fn main() {
    println!("🎨 Dashboard Failure & Title Test\n");

    // Initialize colors
    #[cfg(feature = "colors-core")]
    {
        rsb::colors::color_mode("always");
        rsb::colors::color_enable_with("simple,status");
    }

    let manager = ProgressManager::new();
    manager.add_reporter(Arc::new(TerminalReporter::new()));

    // Test 1: Dashboard with custom title
    println!("[Test 1] Dashboard with custom title\n");
    let task1 = manager.start_task(
        "config-backup.tar.gz",
        ProgressStyle::Dashboard {
            total_chunks: 5,
            chunk_size: 100,
            title: Some("📦 Backing up configuration files".to_string()),
        },
    );

    for chunk in 0..5 {
        for byte in 0..=100 {
            let current = chunk * 100 + byte;
            task1.update(current, &format!("config-backup.tar.gz"));
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    task1.complete("config-backup.tar.gz");

    // Wait a bit before next test
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Test 2: Dashboard that fails at chunk 3
    println!("\n\n[Test 2] Dashboard that fails at chunk 3\n");
    let task2 = manager.start_task(
        "database-migration.sql",
        ProgressStyle::Dashboard {
            total_chunks: 8,
            chunk_size: 50,
            title: Some("🗄️  Running database migration".to_string()),
        },
    );

    let mut failed = false;
    for chunk in 0..8 {
        for byte in 0..=50 {
            // Simulate failure at chunk 3, byte 25
            if chunk == 2 && byte == 25 {
                task2.fail("Migration failed: constraint violation at table 'users'");
                failed = true;
                break;
            }

            let current = chunk * 50 + byte;
            task2.update(current, &format!("database-migration.sql"));
            std::thread::sleep(std::time::Duration::from_millis(8));
        }
        if failed {
            break;
        }
    }

    // Wait a bit before next test
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Test 3: Dashboard without title (for comparison)
    println!("\n\n[Test 3] Dashboard without title (no failure)\n");
    let task3 = manager.start_task(
        "log-archive.zip",
        ProgressStyle::Dashboard {
            total_chunks: 4,
            chunk_size: 75,
            title: None,
        },
    );

    for chunk in 0..4 {
        for byte in 0..=75 {
            let current = chunk * 75 + byte;
            task3.update(current, &format!("log-archive.zip"));
            std::thread::sleep(std::time::Duration::from_millis(7));
        }
    }
    task3.complete("log-archive.zip");

    println!("\n\n✅ Dashboard tests complete!");
    println!("\nYou should have seen:");
    println!("  • Test 1: Dashboard with custom title on first line");
    println!("  • Test 2: Dashboard that fails at chunk 3 (all chunks turn red)");
    println!("  • Test 3: Dashboard without title (normal 4-row display)");
}
