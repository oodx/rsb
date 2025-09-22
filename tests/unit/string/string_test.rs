use rsb::prelude::*;

#[test]
fn test_str_sub_basic_and_unicode() {
    // ASCII
    let s = "abcdefgh";
    assert_eq!(str_sub(s, 0, Some(3)), "abc");
    assert_eq!(str_sub(s, 3, Some(2)), "de");
    assert_eq!(str_sub(s, 5, None), "fgh");

    // Unicode (codepoint-safe)
    let u = "😀áß"; // 3 Unicode scalars
    assert_eq!(str_sub(u, 0, Some(1)), "😀");
    assert_eq!(str_sub(u, 1, Some(1)), "á");
    assert_eq!(str_sub(u, 2, Some(1)), "ß");
}

#[test]
fn test_str_prefix_literal_and_wildcards() {
    // Literal
    assert_eq!(str_prefix("/home/user", "/home", false), "/user");
    assert_eq!(
        str_prefix("/path/to/file.txt", "/path/to", false),
        "/file.txt"
    );
    assert_eq!(str_prefix("/home/user", "/none", false), "/home/user");

    // Wildcards (shortest vs longest)
    let p = "src/main.rs";
    assert_eq!(str_prefix(p, "*/", false), "main.rs");
    // Pattern that could match more than one level (shortest vs longest)
    assert_eq!(str_prefix("a/b/c.txt", "*/", false), "b/c.txt");
    assert_eq!(str_prefix("a/b/c.txt", "*/", true), "c.txt");
}

#[test]
fn test_str_suffix_literal_and_wildcards() {
    // Literal
    assert_eq!(str_suffix("document.txt", ".txt", false), "document");
    assert_eq!(str_suffix("backup.tar.gz", ".gz", false), "backup.tar");
    assert_eq!(str_suffix("file.txt", ".pdf", false), "file.txt");

    // Wildcards
    assert_eq!(
        str_suffix("document.backup.txt", "*.txt", false),
        "document.backup"
    );
    assert_eq!(
        str_suffix("/path/to/file.backup.txt", "*.backup.txt", false),
        "/path/to/file"
    );
    // Longest match removes the longest suffix matching the pattern
    assert_eq!(str_suffix("a.b.c.txt", "*.txt", true), "");
}

#[test]
fn test_str_replace_first_and_all() {
    assert_eq!(str_replace("a/b/c", "/", "_", false), "a_b/c");
    assert_eq!(str_replace("a/b/c", "/", "_", true), "a_b_c");
}

#[test]
fn test_str_upper_lower_first_and_all() {
    assert_eq!(str_upper("hello", false), "Hello");
    assert_eq!(str_upper("hello", true), "HELLO");
    assert_eq!(str_lower("WORLD", false), "wORLD");
    assert_eq!(str_lower("WORLD", true), "world");
}
