use rsb::progress::{ProgressManager, ProgressStyle, ProgressColorScheme, TerminalConfig, TerminalReporter};
use std::sync::Arc;

fn main() {
    // Initialize RSB colors if available
    #[cfg(feature = "colors-core")]
    {
        rsb::colors::color_mode("always");
        rsb::colors::color_enable_with("simple,status,named");
    }

    println!("🎨 RSB PROGRESS - COLOR CUSTOMIZATION TEST\n");
    println!("{}", "=".repeat(70));
    
    // Test 1: Default colors (will use RSB names if colors-core enabled)
    println!("\n[1] DEFAULT COLORS (RSB names: cyan, success, error, warning):");
    #[cfg(feature = "colors-core")]
    {
        let scheme = ProgressColorScheme::default();
        println!("    Debug: running='{}' -> code='{}'", scheme.running, scheme.running_code());
        println!("    Debug: complete='{}' -> code='{}'", scheme.complete, scheme.complete_code());
        println!("    (Watch spinner - should be cyan [38;5;33m, then green checkmark [38;5;46m)\n");
    }
    #[cfg(not(feature = "colors-core"))]
    println!("    (No colors - colors-core not enabled)\n");

    let mut manager1 = ProgressManager::new();
    manager1.add_reporter(Arc::new(TerminalReporter::new()));
    let task1 = manager1.start_task("Loading", ProgressStyle::Spinner);

    // Update more frequently to see smooth spinner animation
    for i in 0..25 {
        task1.update(i, &format!("Processing step {}...", i));
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    task1.complete("Done!");
    
    // Test 2: Custom RSB color names
    println!("\n[2] CUSTOM RSB COLORS (magic, complete, fatal, alert):");
    #[cfg(feature = "colors-core")]
    println!("    (Bar completion should be emerald green - look for [38;5;34m in raw output)\n");
    #[cfg(not(feature = "colors-core"))]
    println!("    (No colors - colors-core not enabled)\n");
    let config2 = TerminalConfig {
        color_scheme: ProgressColorScheme::new("magic", "complete", "fatal", "alert"),
        ..Default::default()
    };
    let mut manager2 = ProgressManager::new();
    manager2.add_reporter(Arc::new(TerminalReporter::with_config(config2)));
    
    let task2 = manager2.start_task("Processing", ProgressStyle::Bar { total: 5 });
    for i in 0..=5 {
        task2.update(i, &format!("Step {}/5", i));
        std::thread::sleep(std::time::Duration::from_millis(150));
    }
    task2.complete("Finished!");
    
    // Test 3: Raw ANSI codes (works WITHOUT colors-core feature)
    println!("\n[3] RAW ANSI CODES (magenta, bright green, bright red, bright yellow):");
    println!("    (Spinner magenta [35m, checkmark bright green [92m - always works!)\n");
    let config3 = TerminalConfig {
        color_scheme: ProgressColorScheme::from_ansi(
            "\x1b[35m",  // Magenta
            "\x1b[92m",  // Bright green
            "\x1b[91m",  // Bright red
            "\x1b[93m",  // Bright yellow
        ),
        ..Default::default()
    };
    let mut manager3 = ProgressManager::new();
    manager3.add_reporter(Arc::new(TerminalReporter::with_config(config3)));
    
    let task3 = manager3.start_task("Uploading", ProgressStyle::Spinner);
    for i in 0..20 {
        task3.update(i, "Uploading files...");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    task3.complete("Upload complete!");
    
    // Test 4: Custom spinner speed (fast!)
    println!("\n[4] CUSTOM SPINNER SPEED (40ms = 25 FPS - FAST!):\n");
    let config_fast = TerminalConfig {
        spinner_refresh_ms: 40,  // Fast animation - 25 FPS
        ..Default::default()
    };
    let mut manager_fast = ProgressManager::new();
    manager_fast.add_reporter(Arc::new(TerminalReporter::with_config(config_fast)));

    let task_fast = manager_fast.start_task("Fast spinner", ProgressStyle::Spinner);
    for i in 0..20 {
        task_fast.update(i, "Spinning fast...");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    task_fast.complete("Fast done!");

    // Test 5: Slow spinner
    println!("\n[5] SLOW SPINNER SPEED (200ms = 5 FPS - SLOW):\n");
    let config_slow = TerminalConfig {
        spinner_refresh_ms: 200,  // Slow animation - 5 FPS
        ..Default::default()
    };
    let mut manager_slow = ProgressManager::new();
    manager_slow.add_reporter(Arc::new(TerminalReporter::with_config(config_slow)));

    let task_slow = manager_slow.start_task("Slow spinner", ProgressStyle::Spinner);
    for i in 0..20 {
        task_slow.update(i, "Spinning slowly...");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    task_slow.complete("Slow done!");

    // Test 6: No colors
    println!("\n[6] NO COLORS:\n");
    let config4 = TerminalConfig {
        use_colors: false,
        color_scheme: ProgressColorScheme::none(),
        ..Default::default()
    };
    let mut manager4 = ProgressManager::new();
    manager4.add_reporter(Arc::new(TerminalReporter::with_config(config4)));
    
    let task4 = manager4.start_task("Plain", ProgressStyle::Bar { total: 3 });
    for i in 0..=3 {
        task4.update(i, "No colors");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    task4.complete("Done (plain)!");
    
    println!("\n{}", "=".repeat(70));
    println!("\n✅ SUMMARY:");
    println!("   • RSB color names: ✓ Works (when colors-core enabled)");
    println!("   • Raw ANSI codes: ✓ Works (always, no feature required)");
    println!("   • Custom spinner speed: ✓ Configure with spinner_refresh_ms");
    println!("   • Fallback: RSB names → raw ANSI if not in registry");
    println!("   • No colors: ✓ Fully supported");
}
