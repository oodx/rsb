// Sanity Baseline: user-visible demos with core functionality
use rsb::prelude::*;

#[test]
fn sanity_baseline_context_and_echo() {
    println!("\n=== SANITY: Context + Echo ===");
    set_var("PROJECT", "rsb");
    echo!("Hello $PROJECT");
    println!("PROJECT='{}'", get_var("PROJECT"));
}

#[test]
fn sanity_baseline_streams() {
    println!("\n=== SANITY: Streams (grep/sort/unique) ===");
    let out = pipe!("error\ninfo\nerror\nwarn")
        .grep("error")
        .sort()
        .unique()
        .to_string();
    println!("Filtered: {}", out.replace('\n', ","));
}

#[test]
fn sanity_baseline_param() {
    println!("\n=== SANITY: param! ===");
    set_var("FOO", "/path/to/file.txt");
    println!("FOO='{}'", param!("FOO"));
    println!("FOO no .txt => '{}'", param!("FOO", suffix: ".txt"));
}
