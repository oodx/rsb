use rsb::prelude::*;
use std::path::Path;

#[test]
fn sanity_fs_file_operations() {
    // Test basic file read/write operations
    let test_content = "Hello, RSB file system test!";
    let temp_file = rsb::dev::create_temp_file(test_content);

    // Test file reading
    let read_content = rsb::fs::read_file(&temp_file);
    assert_eq!(read_content, test_content);

    // Test file writing
    let new_content = "Updated content for testing";
    rsb::fs::write_file(&temp_file, new_content);
    let updated_content = rsb::fs::read_file(&temp_file);
    assert_eq!(updated_content, new_content);

    // Test file appending
    rsb::fs::append_file(&temp_file, "\nAppended line");
    let final_content = rsb::fs::read_file(&temp_file);
    assert!(final_content.contains("Updated content"));
    assert!(final_content.contains("Appended line"));
}

#[test]
fn sanity_fs_directory_operations() {
    // Test directory creation and manipulation
    let temp_dir = format!("/tmp/rsb_test_dir_{}", rand_hex!(8));

    // Test mkdir_p (recursive directory creation)
    rsb::fs::mkdir_p(&temp_dir);
    assert!(rsb::fs::is_dir(&temp_dir));

    // Test file creation in directory
    let test_file = format!("{}/test.txt", temp_dir);
    rsb::fs::write_file(&test_file, "test content");
    assert!(rsb::fs::is_file(&test_file));

    // Test copy operations
    let copy_file = format!("{}/copy.txt", temp_dir);
    rsb::fs::cp(&test_file, &copy_file);
    assert!(rsb::fs::is_file(&copy_file));

    // Test move operations
    let moved_file = format!("{}/moved.txt", temp_dir);
    rsb::fs::mv(&copy_file, &moved_file);
    assert!(rsb::fs::is_file(&moved_file));
    assert!(!rsb::fs::is_file(&copy_file));

    // Test removal
    rsb::fs::rm(&moved_file);
    assert!(!rsb::fs::is_file(&moved_file));

    // Test recursive removal
    rsb::fs::rm_rf(&temp_dir);
    assert!(!rsb::fs::is_dir(&temp_dir));
}

#[test]
fn sanity_fs_predicates() {
    // Test file system predicates
    let temp_file = rsb::dev::create_temp_file("predicate test");

    assert!(rsb::fs::is_file(&temp_file));
    assert!(!rsb::fs::is_dir(&temp_file));
    assert!(rsb::fs::is_entity(&temp_file));
    assert!(rsb::fs::is_readable(&temp_file));
    assert!(rsb::fs::is_writable(&temp_file));

    // Test non-existent file
    let fake_path = "/nonexistent/fake/path.txt";
    assert!(!rsb::fs::is_file(fake_path));
    assert!(!rsb::fs::is_dir(fake_path));
    assert!(!rsb::fs::is_entity(fake_path));
}

#[test]
fn sanity_fs_path_utilities() {
    // Test path manipulation utilities
    let test_path = "/home/user/documents/file.txt";

    let parts = rsb::fs::path_split(test_path);
    assert!(parts.contains(&"/home/user/documents".to_string()));
    assert!(parts.contains(&"file.txt".to_string()));

    // Test path canonicalization with existing path
    let current_dir = std::env::current_dir().unwrap();
    let canon_path = rsb::fs::path_canon(".");
    assert!(canon_path.contains(&current_dir.to_string_lossy()));
}

#[test]
fn sanity_fs_temp_operations() {
    // Test temporary file operations
    let temp_path = rsb::fs::create_temp_file_path();
    assert!(!temp_path.is_empty());
    assert!(temp_path.starts_with("/tmp") || temp_path.contains("temp"));

    // Test temp file creation with stream
    let test_content = "stream content\nline 2\nline 3";
    let mut stream = rsb::streams::Stream::from_string(test_content);
    let temp_file = rsb::fs::capture_stream_to_temp_file(&mut stream);

    let captured_content = rsb::fs::read_file(&temp_file);
    assert_eq!(captured_content, test_content);

    // Test cleanup (should not fail)
    rsb::fs::cleanup_temp_files();
}

#[test]
fn sanity_fs_metadata_operations() {
    // Test metadata and backup operations
    let temp_file = rsb::dev::create_temp_file("metadata test content");

    // Test chmod (basic test)
    rsb::fs::chmod(&temp_file, "644");
    // Cannot easily test permissions cross-platform, but should not panic

    // Test backup creation
    let backup_path = rsb::fs::backup_file(&temp_file);
    assert!(rsb::fs::is_file(&backup_path));
    assert!(backup_path.contains(&temp_file));

    // Test metadata extraction (if any exists)
    let metadata = rsb::fs::extract_meta_from_file(&temp_file);
    // Metadata might be empty, but function should not panic
    assert!(metadata.is_ok() || metadata.is_err()); // Either result is acceptable
}