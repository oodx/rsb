// RSB Sanity Tests - FS Module Core Functionality Verification
// Tests verify the fs module functions work as documented in FEATURES_FS

use rsb::prelude::*;

#[test]
fn test_basic_file_operations() {
    // Test basic file read/write operations
    let test_content = "Hello, world!\nThis is a test file.";
    let test_path = "/tmp/rsb_test_file.txt";

    // Write file
    rsb::fs::write_file(test_path, test_content);

    // Read file back
    let read_content = rsb::fs::read_file(test_path);
    assert_eq!(read_content, test_content);

    // Clean up
    let _ = std::fs::remove_file(test_path);
}

#[test]
fn test_file_append_operations() {
    // Test file append functionality
    let initial_content = "First line\n";
    let append_content = "Second line\n";
    let test_path = "/tmp/rsb_test_append.txt";

    // Write initial content
    rsb::fs::write_file(test_path, initial_content);

    // Append content
    rsb::fs::append_file(test_path, append_content);

    // Verify both contents are present
    let final_content = rsb::fs::read_file(test_path);
    assert!(final_content.contains("First line"));
    assert!(final_content.contains("Second line"));

    // Clean up
    let _ = std::fs::remove_file(test_path);
}

#[test]
fn test_directory_operations() {
    // Test directory creation and manipulation
    let test_dir = "/tmp/rsb_test_dir";
    let nested_dir = "/tmp/rsb_test_dir/nested/deep";

    // Create directories (including parents)
    rsb::fs::mkdir_p(nested_dir);

    // Verify directories exist
    assert!(rsb::fs::is_dir(test_dir));
    assert!(rsb::fs::is_dir(nested_dir));

    // Create a file in the directory
    let test_file = "/tmp/rsb_test_dir/nested/test.txt";
    rsb::fs::write_file(test_file, "test content");
    assert!(rsb::fs::is_file(test_file));

    // Clean up
    rsb::fs::rm_rf(test_dir);
    assert!(!rsb::fs::is_dir(test_dir));
}

#[test]
fn test_file_predicates() {
    // Test file system predicates
    let test_file = "/tmp/rsb_predicate_test.txt";
    let test_dir = "/tmp/rsb_predicate_dir";

    // Create test file and directory
    rsb::fs::write_file(test_file, "test");
    rsb::fs::mkdir_p(test_dir);

    // Test file predicates
    assert!(rsb::fs::is_file(test_file));
    assert!(!rsb::fs::is_dir(test_file));
    assert!(rsb::fs::is_entity(test_file));
    assert!(rsb::fs::is_readable(test_file));

    // Test directory predicates
    assert!(rsb::fs::is_dir(test_dir));
    assert!(!rsb::fs::is_file(test_dir));
    assert!(rsb::fs::is_entity(test_dir));

    // Test non-existent path
    assert!(!rsb::fs::is_entity("/tmp/nonexistent_file"));
    assert!(!rsb::fs::is_file("/tmp/nonexistent_file"));
    assert!(!rsb::fs::is_dir("/tmp/nonexistent_file"));

    // Clean up
    let _ = std::fs::remove_file(test_file);
    let _ = std::fs::remove_dir(test_dir);
}

#[test]
fn test_path_utilities() {
    // Test path manipulation utilities
    let test_path = "/home/user/documents/file.txt";

    // Test path canonicalization (basic test)
    let canon_result = rsb::fs::path_canon(test_path);
    // Should return some form of path (might not exist)
    assert!(!canon_result.is_empty());

    // Test path splitting
    let parts = rsb::fs::path_split(test_path);
    assert!(!parts.is_empty());
    // Should contain path components
}

#[test]
fn test_temp_file_operations() {
    // Test temporary file creation and management
    let temp_path = rsb::fs::create_temp_file_path("test");

    // Temp path should be created
    assert!(!temp_path.is_empty());
    assert!(temp_path.starts_with("/tmp") || temp_path.contains("temp"));

    // Write to temp file
    rsb::fs::write_file(&temp_path, "temporary content");
    assert!(rsb::fs::is_file(&temp_path));

    // Cleanup temp files
    rsb::fs::cleanup_temp_files();
}

#[test]
fn test_file_copy_move_operations() {
    // Test file copy and move operations
    let source_file = "/tmp/rsb_source.txt";
    let copy_file = "/tmp/rsb_copy.txt";
    let move_file = "/tmp/rsb_moved.txt";

    // Create source file
    rsb::fs::write_file(source_file, "source content");

    // Test copy
    rsb::fs::cp(source_file, copy_file);
    assert!(rsb::fs::is_file(copy_file));
    assert_eq!(rsb::fs::read_file(copy_file), "source content");

    // Test move
    rsb::fs::mv(copy_file, move_file);
    assert!(!rsb::fs::is_file(copy_file)); // Should be moved
    assert!(rsb::fs::is_file(move_file));

    // Clean up
    let _ = std::fs::remove_file(source_file);
    let _ = std::fs::remove_file(move_file);
}

#[test]
fn test_file_backup_operations() {
    // Test file backup functionality
    let test_file = "/tmp/rsb_backup_test.txt";
    let content = "content to backup";

    // Create file
    rsb::fs::write_file(test_file, content);

    // Create backup
    let backup_path = rsb::fs::backup_file(test_file);

    // Backup should exist and have same content
    assert!(rsb::fs::is_file(&backup_path));
    assert_eq!(rsb::fs::read_file(&backup_path), content);

    // Clean up
    let _ = std::fs::remove_file(test_file);
    let _ = std::fs::remove_file(&backup_path);
}

#[test]
fn test_touch_operations() {
    // Test touch (create empty file) functionality
    let touch_file = "/tmp/rsb_touch_test.txt";

    // Touch should create the file
    rsb::fs::touch(touch_file);
    assert!(rsb::fs::is_file(touch_file));

    // File should be empty or very small
    let content = rsb::fs::read_file(touch_file);
    assert!(content.is_empty() || content.trim().is_empty());

    // Clean up
    let _ = std::fs::remove_file(touch_file);
}

#[test]
fn test_file_metadata_operations() {
    // Test file metadata extraction
    let test_file = "/tmp/rsb_meta_test.txt";
    let content = "# TITLE: Test File\n# AUTHOR: RSB\nContent here";

    // Create file with metadata
    rsb::fs::write_file(test_file, content);

    // Extract metadata
    let meta = rsb::fs::extract_meta_from_file(test_file);

    // Should contain some metadata information
    assert!(!meta.is_empty());

    // Parse meta keys
    let keys = rsb::fs::parse_meta_keys(content);
    assert!(!keys.is_empty());

    // Clean up
    let _ = std::fs::remove_file(test_file);
}

#[test]
fn test_directory_copy_operations() {
    // Test recursive directory operations
    let source_dir = "/tmp/rsb_source_dir";
    let target_dir = "/tmp/rsb_target_dir";

    // Create source directory with content
    rsb::fs::mkdir_p(source_dir);
    rsb::fs::write_file(&format!("{}/file1.txt", source_dir), "content1");
    rsb::fs::mkdir_p(&format!("{}/subdir", source_dir));
    rsb::fs::write_file(&format!("{}/subdir/file2.txt", source_dir), "content2");

    // Test recursive copy
    rsb::fs::cp_r(source_dir, target_dir);

    // Verify copy worked
    assert!(rsb::fs::is_dir(target_dir));
    assert!(rsb::fs::is_file(&format!("{}/file1.txt", target_dir)));
    assert!(rsb::fs::is_file(&format!("{}/subdir/file2.txt", target_dir)));

    // Clean up
    rsb::fs::rm_rf(source_dir);
    rsb::fs::rm_rf(target_dir);
}

#[test]
fn test_fs_macros() {
    // Test FS module macros
    let test_file = "/tmp/rsb_macro_test.txt";

    // Create file using write_file function
    rsb::fs::write_file(test_file, "macro test content");

    // Test backup macro
    let backup_path = backup!(test_file);
    assert!(rsb::fs::is_file(&backup_path));

    // Test touch macro
    let touch_file = "/tmp/rsb_touch_macro.txt";
    touch!(touch_file);
    assert!(rsb::fs::is_file(touch_file));

    // Clean up
    let _ = std::fs::remove_file(test_file);
    let _ = std::fs::remove_file(&backup_path);
    let _ = std::fs::remove_file(touch_file);
}

#[test]
fn test_edge_cases() {
    // Test edge cases and error conditions

    // Test reading non-existent file (should handle gracefully)
    let nonexistent_content = rsb::fs::read_file("/tmp/nonexistent_file_12345.txt");
    // Should return empty string or handle gracefully

    // Test creating file in non-existent directory
    let nested_file = "/tmp/rsb_deep/nested/path/file.txt";
    // This might fail or create directories automatically depending on implementation
    // The test should not panic
    rsb::fs::write_file(nested_file, "content");
    // Try to clean up if it was created
    let _ = std::fs::remove_file(nested_file);
    let _ = std::fs::remove_dir_all("/tmp/rsb_deep");

    // Test with empty content
    let empty_file = "/tmp/rsb_empty_test.txt";
    rsb::fs::write_file(empty_file, "");
    let empty_content = rsb::fs::read_file(empty_file);
    assert_eq!(empty_content, "");

    let _ = std::fs::remove_file(empty_file);
}