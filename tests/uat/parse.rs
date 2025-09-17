use rsb::prelude::*;

#[test]
fn uat_parse_string_transforms_demo() {
    println!("\n=== UAT: Parse String Transforms Demo ===");

    // Test sed-like string transformation macros
    println!("Testing string transformation capabilities...");

    let sample_text = "Line 1: Hello\nLine 2: World\nLine 3: RSB\nLine 4: Parse\nLine 5: Demo";

    // Test line extraction
    let lines_2_to_4 = sed_lines!(sample_text, 2, 4);
    println!("✓ Lines 2-4 extracted:");
    println!("{}", lines_2_to_4);

    // Test pattern-based extraction with context
    let around_rsb = sed_around!(sample_text, "RSB", 1);
    println!("✓ Context around 'RSB' (±1 line):");
    println!("{}", around_rsb);

    // Test string replacement
    let replaced = sed_replace!(sample_text, "Hello", "Greetings");
    println!("✓ Replaced 'Hello' with 'Greetings':");
    println!("{}", replaced);

    // Test template insertion
    let template_content = "Line 1: Hello\n<!-- MARKER -->\nLine 5: Demo";
    let insertion = "Line 2: Inserted\nLine 3: Content";
    let result = sed_template!(template_content, "<!-- MARKER -->", insertion);
    println!("✓ Template insertion result:");
    println!("{}", result);

    println!("String transforms demo completed!");
}

#[test]
fn uat_parse_file_operations_demo() {
    println!("\n=== UAT: Parse File Operations Demo ===");

    // Test file-based sed operations
    println!("Testing file-based transformation capabilities...");

    // Create a temporary test file
    let test_content = "File Line 1\nFile Line 2\nFile Line 3\nTarget Line\nFile Line 5";

    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("rsb_uat_parse_test_{}.txt", std::process::id()));
    let temp_file_str = temp_file.to_string_lossy().to_string();
    rsb::fs::write_file(&temp_file_str, test_content);
    println!("✓ Test file created: {}", temp_file_str);

    // Test file line extraction
    let file_lines = sed_lines_file!(&temp_file_str, 2, 4);
    println!("✓ Lines 2-4 from file:");
    println!("{}", file_lines);

    // Test file pattern extraction
    let around_target = sed_around_file!(&temp_file_str, "Target", 1);
    println!("✓ Context around 'Target' from file:");
    println!("{}", around_target);

    // Test file insertion
    let insert_result = sed_insert_file!(&temp_file_str, "New inserted line", "Target Line");
    match insert_result {
        Ok(_) => println!("✓ Content inserted into file successfully"),
        Err(e) => println!("⚠ File insertion failed: {}", e),
    }

    // Test file template replacement
    let template_result = sed_template_file!(&temp_file_str, "Replacement content", "Target Line");
    match template_result {
        Ok(_) => println!("✓ Template replacement in file successful"),
        Err(e) => println!("⚠ File template replacement failed: {}", e),
    }

    println!("File operations demo completed!");
}

#[test]
fn uat_parse_content_analysis_demo() {
    println!("\n=== UAT: Parse Content Analysis Demo ===");

    // Test word count and analysis utilities
    println!("Testing content analysis capabilities...");

    let analysis_text = "The quick brown fox jumps over the lazy dog.\nThis is a second line.\nAnd a third line for analysis.";

    // Test line counting
    let line_count = analysis_text.lines().count();
    println!("✓ Line count: {}", line_count);

    // Test word counting
    let word_count = analysis_text.split_whitespace().count();
    println!("✓ Word count: {}", word_count);

    // Test character counting
    let char_count = analysis_text.len();
    println!("✓ Character count: {}", char_count);

    // Test pattern matching and extraction
    let words_with_o: Vec<&str> = analysis_text
        .split_whitespace()
        .filter(|word| word.contains('o'))
        .collect();
    println!("✓ Words containing 'o': {:?}", words_with_o);

    // Test multi-line pattern operations
    let lines_with_the: Vec<&str> = analysis_text
        .lines()
        .filter(|line| line.to_lowercase().contains("the"))
        .collect();
    println!("✓ Lines containing 'the': {} lines", lines_with_the.len());

    println!("Content analysis demo completed!");
}

#[test]
fn uat_parse_advanced_transforms_demo() {
    println!("\n=== UAT: Parse Advanced Transforms Demo ===");

    // Test advanced transformation scenarios
    println!("Testing advanced transformation scenarios...");

    let config_content = r#"
[section1]
key1=value1
key2=value2

[section2]
key3=value3
# This is a comment
key4=value4

[section3]
key5=value5
"#;

    // Test section extraction
    let section2 = sed_around!(config_content, "[section2]", 3);
    println!("✓ Section 2 extracted:");
    println!("{}", section2);

    // Test configuration modification
    let updated_config = sed_replace!(config_content, "value3", "updated_value3");
    println!("✓ Configuration value updated");

    // Test comment removal (simulation)
    let lines: Vec<&str> = config_content
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect();
    let no_comments = lines.join("\n");
    println!("✓ Comments filtered out ({} lines remaining)", lines.len());

    // Test multiple transformations
    let multi_transform = sed_replace!(
        &sed_replace!(config_content, "value1", "new_value1"),
        "value2",
        "new_value2"
    );
    println!("✓ Multiple transformations applied");

    println!("Advanced transforms demo completed!");
}