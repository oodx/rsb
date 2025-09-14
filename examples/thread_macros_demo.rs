// Thread Macros Demo - benchmark! and sleep! now in threads module
use rsb::prelude::*;
use rsb::{benchmark, sleep};

fn main() {
    println!("=== Thread Macros Demo ===\n");

    println!("1. Testing sleep! macro:");
    println!("   Sleeping for 1 second...");
    sleep!(1);
    println!("   ✅ sleep!(1) completed");

    println!("   Sleeping for 500ms...");
    sleep!(ms: 500);
    println!("   ✅ sleep!(ms: 500) completed");

    println!("\n2. Testing benchmark! macro:");
    let duration = benchmark! {{
        // Simulate some work
        let mut sum = 0u64;
        for i in 0..100000 {
            sum += i;
        }
        println!("   Computed sum: {}", sum);
    }};
    println!("   ✅ benchmark! returned duration: {:?}", duration);

    println!("\n=== Module Reorganization Complete ===");
    println!("✅ benchmark! moved from time_math.rs → threads/macros.rs");
    println!("✅ sleep! moved from time_math.rs → threads/macros.rs");
    println!("✅ math! remains in macros/math.rs (renamed from time_math.rs)");
    println!("✅ Thread-related functionality properly grouped in threads module");
}