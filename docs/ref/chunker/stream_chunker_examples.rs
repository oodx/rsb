use std::fs::File;
use std::io::Write;
use std::path::Path;

// Assuming the StreamChunker from the previous artifact is available
// use generic_stream_chunker::*;

/// Example 1: Basic file processing with resumability
fn example_basic_processing() -> std::io::Result<()> {
    println!("=== Example 1: Basic File Processing ===\n");
    
    let file_path = "large_data.json";
    let chunk_size = 100_000_000; // 100MB chunks
    
    let mut chunker = StreamChunker::new(file_path, chunk_size)?;
    
    println!("Processing file: {}", file_path);
    println!("File size: {} bytes", chunker.size());
    println!("Chunk size: {} bytes", chunk_size);
    println!("Total chunks: {}\n", chunker.get_chunks().len());
    
    // Process with automatic resumability
    let final_state = chunker.process_with(|data, chunk, state| {
        println!("Processing chunk {} ({} bytes at position {})", 
                 chunk.id, chunk.size, chunk.start);
        
        // Your processing logic here
        let mut new_state = state.clone();
        
        // Example: Count specific byte patterns
        let pattern_count = data.iter().filter(|&&b| b == b'{').count();
        new_state.custom_data.insert(
            format!("chunk_{}_patterns", chunk.id),
            pattern_count.to_string()
        );
        
        println!("  Found {} patterns in chunk {}", pattern_count, chunk.id);
        
        Ok(new_state)
    })?;
    
    println!("\nProcessing complete!");
    println!("Total bytes processed: {}", final_state.bytes_processed);
    println!("Chunks completed: {:?}\n", final_state.chunks_completed);
    
    Ok(())
}

/// Example 2: Processing with interruption and resume
fn example_resumable_processing() -> std::io::Result<()> {
    println!("=== Example 2: Resumable Processing ===\n");
    
    let file_path = "large_data.json";
    let chunk_size = 100_000_000;
    
    // First attempt - simulate interruption after 2 chunks
    println!("First processing attempt...");
    {
        let mut chunker = StreamChunker::new(file_path, chunk_size)?;
        
        let mut chunks_processed = 0;
        let result = chunker.process_streaming(|data, chunk, state| {
            println!("  Processing chunk {}", chunk.id);
            chunks_processed += 1;
            
            state.custom_data.insert(
                "last_chunk".to_string(),
                chunk.id.to_string()
            );
            
            // Simulate interruption after 2 chunks
            if chunks_processed >= 2 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Interrupted,
                    "Simulated interruption"
                ));
            }
            
            Ok(())
        });
        
        match result {
            Ok(_) => println!("Completed without interruption"),
            Err(e) => println!("Interrupted: {}", e),
        }
    }
    
    // Second attempt - resumes from checkpoint
    println!("\nResuming processing...");
    {
        let mut chunker = StreamChunker::new(file_path, chunk_size)?;
        
        let progress = chunker.progress()?;
        println!("Resuming from {:.1}% complete", progress * 100.0);
        
        let final_state = chunker.process_streaming(|data, chunk, state| {
            println!("  Processing chunk {}", chunk.id);
            
            state.custom_data.insert(
                "last_chunk".to_string(),
                chunk.id.to_string()
            );
            
            Ok(())
        })?;
        
        println!("\nProcessing complete!");
        println!("Final position: {} bytes", final_state.bytes_processed);
    }
    
    Ok(())
}

/// Example 3: Streaming large file with custom state accumulation
fn example_stateful_processing() -> std::io::Result<()> {
    println!("=== Example 3: Stateful Processing ===\n");
    
    let file_path = "large_data.json";
    let chunk_size = 100_000_000;
    
    let mut chunker = StreamChunker::new(file_path, chunk_size)?;
    
    // Track statistics across chunks
    #[derive(Default)]
    struct Stats {
        total_chars: u64,
        opening_brackets: u64,
        closing_brackets: u64,
        max_chunk_size: u64,
    }
    
    let mut stats = Stats::default();
    
    let final_state = chunker.process_streaming(|data, chunk, state| {
        // Accumulate statistics
        stats.total_chars += data.len() as u64;
        stats.opening_brackets += data.iter().filter(|&&b| b == b'{').count() as u64;
        stats.closing_brackets += data.iter().filter(|&&b| b == b'}').count() as u64;
        stats.max_chunk_size = stats.max_chunk_size.max(chunk.size);
        
        // Store chunk-specific data
        state.custom_data.insert(
            format!("chunk_{}_chars", chunk.id),
            data.len().to_string()
        );
        
        println!("Chunk {}: {} chars, {} brackets",
                 chunk.id, data.len(), 
                 stats.opening_brackets + stats.closing_brackets);
        
        Ok(())
    })?;
    
    println!("\n=== Final Statistics ===");
    println!("Total characters: {}", stats.total_chars);
    println!("Opening brackets: {}", stats.opening_brackets);
    println!("Closing brackets: {}", stats.closing_brackets);
    println!("Max chunk size: {} bytes", stats.max_chunk_size);
    println!("Bracket balance: {}", 
             stats.opening_brackets as i64 - stats.closing_brackets as i64);
    
    Ok(())
}

/// Example 4: Processing with byte range queries
fn example_random_access() -> std::io::Result<()> {
    println!("=== Example 4: Random Access Processing ===\n");
    
    let file_path = "large_data.json";
    let chunk_size = 100_000_000;
    
    let chunker = StreamChunker::new(file_path, chunk_size)?;
    
    // Read specific byte ranges without full processing
    println!("Reading specific byte ranges...\n");
    
    // Read first 1000 bytes
    let header = chunker.read_range(0, 999)?;
    println!("First 1000 bytes preview:");
    println!("{}\n", String::from_utf8_lossy(&header[..100.min(header.len())]));
    
    // Read middle section
    let file_size = chunker.size();
    let middle_start = file_size / 2;
    let middle_end = middle_start + 999;
    let middle = chunker.read_range(middle_start, middle_end)?;
    println!("Middle section (bytes {}-{}):", middle_start, middle_end);
    println!("{}\n", String::from_utf8_lossy(&middle[..100.min(middle.len())]));
    
    // Read last 1000 bytes
    let end_start = file_size.saturating_sub(1000);
    let end = chunker.read_range(end_start, file_size - 1)?;
    println!("Last 1000 bytes:");
    println!("{}", String::from_utf8_lossy(&end[..100.min(end.len())]));
    
    Ok(())
}

/// Example 5: Using builder pattern for configuration
fn example_builder_pattern() -> std::io::Result<()> {
    println!("=== Example 5: Builder Pattern Configuration ===\n");
    
    let file_path = "large_data.json";
    
    // Automatic chunk size selection
    let auto_chunker = StreamProcessorBuilder::new()
        .checkpoint_enabled(true)
        .build(file_path)?;
    
    println!("Auto-configured chunker:");
    println!("  Chunk size: {} bytes", auto_chunker.chunk_size);
    println!("  Total chunks: {}", auto_chunker.get_chunks().len());
    
    // Custom configuration
    let custom_chunker = StreamProcessorBuilder::new()
        .chunk_size(50_000_000) // 50MB chunks
        .checkpoint_enabled(false) // Disable checkpointing
        .build(file_path)?;
    
    println!("\nCustom-configured chunker:");
    println!("  Chunk size: {} bytes", custom_chunker.chunk_size);
    println!("  Total chunks: {}", custom_chunker.get_chunks().len());
    
    Ok(())
}

/// Example 6: Processing with progress tracking
fn example_progress_tracking() -> std::io::Result<()> {
    println!("=== Example 6: Progress Tracking ===\n");
    
    let file_path = "large_data.json";
    let chunk_size = 100_000_000;
    
    let mut chunker = StreamChunker::new(file_path, chunk_size)?;
    let total_chunks = chunker.get_chunks().len();
    
    chunker.process_streaming(|_data, chunk, _state| {
        let progress = ((chunk.id + 1) as f64 / total_chunks as f64) * 100.0;
        
        print!("\rProcessing: {:.1}% ({}/{} chunks)", 
               progress, chunk.id + 1, total_chunks);
        std::io::stdout().flush().unwrap();
        
        // Simulate processing time
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        Ok(())
    })?;
    
    println!("\n\nProcessing complete!");
    
    Ok(())
}

/// Example 7: Integrating with JSON processor
fn example_json_integration() -> std::io::Result<()> {
    println!("=== Example 7: JSON Processing Integration ===\n");
    
    let file_path = "large_data.json";
    let chunk_size = 100_000_000;
    
    let mut chunker = StreamChunker::new(file_path, chunk_size)?;
    
    // Track JSON-specific metrics
    let mut json_depth = 0;
    let mut max_depth = 0;
    let mut object_count = 0;
    let mut array_count = 0;
    
    chunker.process_streaming(|data, chunk, state| {
        println!("Processing chunk {} for JSON analysis", chunk.id);
        
        // Simple JSON structure analysis
        for &byte in data {
            match byte as char {
                '{' => {
                    json_depth += 1;
                    object_count += 1;
                    max_depth = max_depth.max(json_depth);
                }
                '}' => json_depth -= 1,
                '[' => {
                    json_depth += 1;
                    array_count += 1;
                    max_depth = max_depth.max(json_depth);
                }
                ']' => json_depth -= 1,
                _ => {}
            }
        }
        
        // Store per-chunk metrics
        state.custom_data.insert(
            format!("chunk_{}_objects", chunk.id),
            object_count.to_string()
        );
        
        Ok(())
    })?;
    
    println!("\n=== JSON Structure Analysis ===");
    println!("Objects found: {}", object_count);
    println!("Arrays found: {}", array_count);
    println!("Maximum nesting depth: {}", max_depth);
    println!("Final depth (should be 0): {}", json_depth);
    
    if json_depth != 0 {
        println!("WARNING: Unbalanced JSON structure detected!");
    }
    
    Ok(())
}

/// Main function demonstrating all examples
fn main() -> std::io::Result<()> {
    println!("Stream Chunker Usage Examples");
    println!("=============================\n");
    
    // Note: These examples assume a test file exists
    // In practice, you'd create test data or use actual files
    
    example_basic_processing()?;
    println!("\n---\n");
    
    example_resumable_processing()?;
    println!("\n---\n");
    
    example_stateful_processing()?;
    println!("\n---\n");
    
    example_random_access()?;
    println!("\n---\n");
    
    example_builder_pattern()?;
    println!("\n---\n");
    
    example_progress_tracking()?;
    println!("\n---\n");
    
    example_json_integration()?;
    
    Ok(())
}

// Helper function to create test data
#[allow(dead_code)]
fn create_test_file(path: &str, size_mb: usize) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    let chunk = vec![b'x'; 1024 * 1024]; // 1MB chunk
    
    for _ in 0..size_mb {
        file.write_all(&chunk)?;
    }
    
    Ok(())
}