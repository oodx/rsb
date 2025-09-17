use rsb::prelude::*;

#[test]
fn sanity_gx_string_generators() {
    // Test string generation functions
    let alnum = rsb::gx::get_rand_alnum(10);
    assert_eq!(alnum.len(), 10);
    assert!(alnum.chars().all(|c| c.is_alphanumeric()));

    let alpha = rsb::gx::get_rand_alpha(8);
    assert_eq!(alpha.len(), 8);
    assert!(alpha.chars().all(|c| c.is_alphabetic()));

    let hex = rsb::gx::get_rand_hex(16);
    assert_eq!(hex.len(), 16);
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));

    let custom = rsb::gx::get_rand_string(12);
    assert_eq!(custom.len(), 12);
}

#[test]
fn sanity_gx_string_macros() {
    // Test string generation macros
    let macro_alnum = rand_alnum!(6);
    assert_eq!(macro_alnum.len(), 6);

    let macro_alpha = rand_alpha!(5);
    assert_eq!(macro_alpha.len(), 5);

    let macro_hex = rand_hex!(8);
    assert_eq!(macro_hex.len(), 8);
}

#[test]
fn sanity_gx_id_generators() {
    // Test UUID generation
    let uuid = rsb::gx::get_rand_uuid();
    assert!(!uuid.is_empty());
    assert!(uuid.contains('-')); // Basic UUID format

    let macro_uuid = rand_uuid!();
    assert!(!macro_uuid.is_empty());
}

#[test]
fn sanity_gx_collection_helpers() {
    // Test collection selection
    let items = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let selected = rsb::gx::get_rand_from_slice(&items);
    assert!(selected.is_some());
    assert!(items.contains(&selected.unwrap()));

    // Test empty collection
    let empty: Vec<String> = vec![];
    let empty_selection = rsb::gx::get_rand_from_slice(&empty);
    assert!(empty_selection.is_none());
}

#[test]
fn sanity_gx_math_adapter() {
    // Test math adapter for random ranges
    let rand_num = rsb::gx::rand_usize_inclusive(1, 10);
    assert!(rand_num >= 1 && rand_num <= 10);

    let macro_range = rand_range!(5, 15);
    assert!(macro_range >= 5 && macro_range <= 15);
}

#[test]
fn sanity_gx_fs_adapter() {
    // Test file system adapter basics
    // This tests the adapter interface without requiring actual dict files
    let nonexistent_path = "/nonexistent/path/file.txt";
    let result = rsb::gx::load_dict_file(nonexistent_path);
    assert!(result.is_err()); // Should fail gracefully for nonexistent files
}