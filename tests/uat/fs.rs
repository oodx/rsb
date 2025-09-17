use rsb::prelude::*;
use std::fs;

#[test]
fn uat_fs_file_operations_demo() {
    println!("\n=== UAT: File System Operations Demo ===");

    // Create a temporary test file
    let test_file = "/tmp/rsb_fs_test.txt";
    let test_content = "Hello, RSB filesystem operations!\nLine 2\nLine 3";

    println!("Creating test file: {}", test_file);
    rsb::fs::write_file(test_file, test_content);

    println!("Reading file back:");
    let content = rsb::fs::read_file(test_file);
    println!("{}", content);

    println!(
        "\nFile exists check: {}",
        std::path::Path::new(test_file).exists()
    );

    // Get file info
    if let Ok(metadata) = fs::metadata(test_file) {
        println!("File size: {} bytes", metadata.len());
        println!("Is file: {}", metadata.is_file());
    }

    // Append to file
    println!("\nAppending to file...");
    rsb::fs::append_file(test_file, "Appended line");

    let updated_content = rsb::fs::read_file(test_file);
    println!("Updated content:\n{}", updated_content);

    // Clean up
    println!("\nCleaning up test file...");
    fs::remove_file(test_file).ok();
    println!(
        "File deleted: {}",
        !std::path::Path::new(test_file).exists()
    );
}

#[test]
fn uat_fs_directory_operations_demo() {
    println!("\n=== UAT: Directory Operations Demo ===");

    let test_dir = "/tmp/rsb_test_dir";
    let nested_dir = "/tmp/rsb_test_dir/nested";

    println!("Creating test directory: {}", test_dir);
    rsb::fs::mkdir_p(test_dir);
    println!(
        "Directory exists: {}",
        std::path::Path::new(test_dir).exists()
    );

    println!("Creating nested directory: {}", nested_dir);
    rsb::fs::mkdir_p(nested_dir);
    println!(
        "Nested directory exists: {}",
        std::path::Path::new(nested_dir).exists()
    );

    // Create some test files
    let file1 = format!("{}/file1.txt", test_dir);
    let file2 = format!("{}/file2.log", test_dir);
    let file3 = format!("{}/data.json", nested_dir);

    rsb::fs::write_file(&file1, "File 1 content");
    rsb::fs::write_file(&file2, "Log entry 1\nLog entry 2");
    rsb::fs::write_file(&file3, r#"{"name": "test", "value": 42}"#);

    println!("\nListing directory contents:");
    if let Ok(entries) = fs::read_dir(test_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("  - {}", entry.file_name().to_string_lossy());
            }
        }
    }

    // Test copy operations
    let copy_file = format!("{}/copy_of_file1.txt", test_dir);
    println!("\nCopying file...");
    if rsb::fs::cp(&file1, &copy_file) {
        println!("File copied successfully");
    }

    // Clean up
    println!("\nCleaning up test directories...");
    fs::remove_dir_all(test_dir).ok();
    println!(
        "Directory removed: {}",
        !std::path::Path::new(test_dir).exists()
    );
}

#[test]
fn uat_fs_path_operations_demo() {
    println!("\n=== UAT: Path Operations Demo ===");

    let file_path = "/home/user/documents/project/readme.txt";
    let path = std::path::Path::new(file_path);

    println!("Path: {}", file_path);
    println!(
        "Basename: {}",
        path.file_name().unwrap_or_default().to_string_lossy()
    );
    println!(
        "Dirname: {}",
        path.parent()
            .unwrap_or_else(|| std::path::Path::new("/"))
            .display()
    );
    println!(
        "Extension: {}",
        path.extension().unwrap_or_default().to_string_lossy()
    );

    // Path manipulation using std::path
    let without_ext = path.with_extension("");
    println!("Without extension: {}", without_ext.display());

    let with_new_ext = path.with_extension("md");
    println!("With .md extension: {}", with_new_ext.display());

    // Path joining
    let joined_path = std::path::Path::new("/usr/local").join("bin/rsb");
    println!("Joined path: {}", joined_path.display());

    // Absolute vs relative
    println!("Is absolute: {}", path.is_absolute());
    let rel_path = std::path::Path::new("./local/file.txt");
    println!("Is relative: {}", rel_path.is_relative());
}

#[test]
fn uat_fs_rsb_utilities_demo() {
    println!("\n=== UAT: RSB File System Utilities Demo ===");

    let test_file = "/tmp/rsb_util_test.txt";
    let test_dir = "/tmp/rsb_util_dir";

    // Test RSB utilities
    println!("Creating directory with mkdir_p: {}", test_dir);
    let success = rsb::fs::mkdir_p(test_dir);
    println!("mkdir_p success: {}", success);

    println!("Creating test file...");
    rsb::fs::write_file(test_file, "test content");

    println!("Copying file...");
    let copy_dest = "/tmp/rsb_util_copy.txt";
    let cp_success = rsb::fs::cp(test_file, copy_dest);
    println!("Copy success: {}", cp_success);

    if cp_success {
        println!("Copy content: {}", rsb::fs::read_file(copy_dest));
    }

    // Clean up using RSB utilities
    println!("\nCleaning up with rm...");
    let rm1 = rsb::fs::rm(test_file);
    let rm2 = rsb::fs::rm(copy_dest);
    let rm3 = rsb::fs::rm_rf(test_dir);
    println!("Remove results: file={}, copy={}, dir={}", rm1, rm2, rm3);
}
