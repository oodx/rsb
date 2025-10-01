use rsb::progress::{ProgressManager, ProgressStyle, TerminalReporter};
use std::sync::Arc;

fn main() {
    // Initialize RSB colors if available
    #[cfg(feature = "colors-core")]
    {
        rsb::colors::color_mode("always");
        rsb::colors::color_enable_with("simple,status,named");
    }

    println!("🎨 RSB PROGRESS - DASHBOARD STYLE DEMO\n");
    println!("{}", "=".repeat(70));

    // Test 1: Dashboard with 8 chunks, 100 bytes each
    println!("\n[1] DASHBOARD STYLE - Batch File Processing (8 chunks x 100 bytes):\n");

    let mut manager = ProgressManager::new();
    manager.add_reporter(Arc::new(TerminalReporter::new()));

    let total_chunks = 8;
    let chunk_size = 100u64;
    let total_bytes = total_chunks as u64 * chunk_size;

    let task = manager.start_task(
        "batch-encryption.dat",
        ProgressStyle::Dashboard {
            total_chunks,
            chunk_size,
        },
    );

    // Simulate processing 8 chunks
    for chunk in 0..total_chunks {
        // Process each chunk byte by byte (simulate)
        for byte in 0..chunk_size {
            let current_byte = chunk as u64 * chunk_size + byte;
            task.update(
                current_byte,
                &format!("batch-encryption.dat [chunk {}/{}]", chunk + 1, total_chunks),
            );
            std::thread::sleep(std::time::Duration::from_millis(20)); // Slow down for visibility
        }
    }

    task.complete("batch-encryption.dat");

    println!("\n{}", "=".repeat(70));

    // Test 2: Dashboard with 5 chunks (different size)
    println!("\n[2] DASHBOARD STYLE - Large File Processing (5 chunks x 1024 bytes):\n");

    let mut manager2 = ProgressManager::new();
    manager2.add_reporter(Arc::new(TerminalReporter::new()));

    let total_chunks2 = 5;
    let chunk_size2 = 1024u64;
    let total_bytes2 = total_chunks2 as u64 * chunk_size2;

    let task2 = manager2.start_task(
        "large-document.pdf",
        ProgressStyle::Dashboard {
            total_chunks: total_chunks2,
            chunk_size: chunk_size2,
        },
    );

    // Simulate processing with larger steps
    for i in 0..=50 {
        let current = (total_bytes2 * i) / 50;
        let current_chunk = current / chunk_size2;
        task2.update(
            current,
            &format!("large-document.pdf [chunk {}/{}]", current_chunk.min(total_chunks2 as u64 - 1) + 1, total_chunks2),
        );
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    task2.complete("large-document.pdf");

    println!("\n{}", "=".repeat(70));

    // Test 3: Dashboard with 12 chunks (many chunks)
    println!("\n[3] DASHBOARD STYLE - Multi-Step Process (12 steps):\n");

    let mut manager3 = ProgressManager::new();
    manager3.add_reporter(Arc::new(TerminalReporter::new()));

    let total_chunks3 = 12;
    let chunk_size3 = 50u64;
    let total_bytes3 = total_chunks3 as u64 * chunk_size3;

    let task3 = manager3.start_task(
        "pipeline",
        ProgressStyle::Dashboard {
            total_chunks: total_chunks3,
            chunk_size: chunk_size3,
        },
    );

    // Simulate step-by-step processing
    for step in 0..total_chunks3 {
        for byte in 0..chunk_size3 {
            let current = step as u64 * chunk_size3 + byte;
            task3.update(
                current,
                &format!("pipeline [step {}/{}]", step + 1, total_chunks3),
            );
            std::thread::sleep(std::time::Duration::from_millis(15));
        }
    }

    task3.complete("pipeline");

    println!("\n{}", "=".repeat(70));
    println!("\n✅ DASHBOARD FEATURES:");
    println!("   • Three-row display: status, chunks, progress bar");
    println!("   • Chunk visualization: ■ complete, █ current (blinking), □ pending");
    println!("   • Live ETA and byte count tracking");
    println!("   • Color-coded states (running, complete, failed, cancelled)");
    println!("   • Perfect for batch processing and multi-step operations");
}
