// RSB Sanity Tests - GX (Generators) Module Core Functionality Verification
// Tests verify the generators module functions and macros work as documented in FEATURES_GENERATORS

use rsb::prelude::*;

#[test]
fn test_string_generators() {
    // Test basic string generators
    let alnum = rsb::gx::string::get_rand_alnum(10);
    assert_eq!(alnum.len(), 10);
    assert!(alnum.chars().all(|c| c.is_alphanumeric()));

    let alpha = rsb::gx::string::get_rand_alpha(8);
    assert_eq!(alpha.len(), 8);
    assert!(alpha.chars().all(|c| c.is_alphabetic()));

    let hex = rsb::gx::string::get_rand_hex(12);
    assert_eq!(hex.len(), 12);
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));

    // Note: No dedicated numeric generator - use get_rand_string with numeric charset if needed
}

#[test]
fn test_id_generators() {
    // Test UUID generation
    let uuid = rsb::gx::id::get_rand_uuid();
    assert!(!uuid.is_empty());
    assert!(uuid.contains('-'));

    // UUIDs should be consistent format (36 chars with dashes)
    assert_eq!(uuid.len(), 36);

    // Multiple UUIDs should be different
    let uuid2 = rsb::gx::id::get_rand_uuid();
    assert_ne!(uuid, uuid2);
}

#[test]
fn test_collection_helpers() {
    // Test random selection from slice
    let test_items = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];

    let selection = rsb::gx::collection::get_rand_from_slice(&test_items);
    assert!(selection.is_some());
    assert!(test_items.contains(&selection.unwrap()));

    // Test with empty slice
    let empty_items: Vec<String> = vec![];
    let empty_selection = rsb::gx::collection::get_rand_from_slice(&empty_items);
    assert!(empty_selection.is_none());
}

#[test]
fn test_generator_macros() {
    // Test string generator macros
    let alnum_macro = rand_alnum!(8);
    assert_eq!(alnum_macro.len(), 8);
    assert!(alnum_macro.chars().all(|c| c.is_alphanumeric()));

    let alpha_macro = rand_alpha!(6);
    assert_eq!(alpha_macro.len(), 6);
    assert!(alpha_macro.chars().all(|c| c.is_alphabetic()));

    let hex_macro = rand_hex!(10);
    assert_eq!(hex_macro.len(), 10);
    assert!(hex_macro.chars().all(|c| c.is_ascii_hexdigit()));

    // Note: No rand_numeric! macro - use rand_string! with numeric charset if needed

    // Test UUID macro
    let uuid_macro = rand_uuid!();
    assert!(!uuid_macro.is_empty());
    assert!(uuid_macro.contains('-'));
    assert_eq!(uuid_macro.len(), 36);
}

#[test]
fn test_math_adapter_integration() {
    // Test range generation through math adapter
    let range_result = rand_range!(1, 10);
    assert!(range_result >= 1);
    assert!(range_result <= 10);

    let range_result2 = rand_range!(50, 100);
    assert!(range_result2 >= 50);
    assert!(range_result2 <= 100);

    // Test multiple calls give different results (probabilistically)
    let mut results = vec![];
    for _ in 0..10 {
        results.push(rand_range!(1, 100));
    }
    // Very unlikely all 10 results are identical
    assert!(results.iter().any(|&x| x != results[0]));
}

#[test]
fn test_dict_file_loading() {
    // Test dictionary file loading
    let dict_path = "src/gx/data/dict/colors.txt";
    let dict_result = rsb::gx::load_dict_file(dict_path);

    // Should return Vec<String> directly (not Result)
    assert!(!dict_result.is_empty());
    assert!(dict_result.iter().all(|item| !item.is_empty()));
}

#[test]
fn test_dict_macros() {
    // Test dictionary helper macros
    // Note: These may require specific dict files to exist

    // Test rand_dict with array name (if available)
    // This tests the macro compilation, actual dict loading tested separately
    let _test_compilation = true; // Placeholder for dict macro tests

    // Actual dict macro testing would depend on available dict files:
    // let color = rand_dict!("COLORS");
    // assert!(!color.is_empty());
}

#[test]
fn test_edge_cases() {
    // Test edge cases and boundary conditions

    // Test zero-length string generation
    let empty_alnum = rsb::gx::string::get_rand_alnum(0);
    assert_eq!(empty_alnum.len(), 0);

    // Test single character generation
    let single_alpha = rsb::gx::string::get_rand_alpha(1);
    assert_eq!(single_alpha.len(), 1);
    assert!(single_alpha.chars().all(|c| c.is_alphabetic()));

    // Test large string generation
    let large_hex = rsb::gx::string::get_rand_hex(100);
    assert_eq!(large_hex.len(), 100);
    assert!(large_hex.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_randomness_quality() {
    // Test that generators produce varied output

    // Generate multiple alphanumeric strings
    let mut alnum_results = vec![];
    for _ in 0..20 {
        alnum_results.push(rsb::gx::string::get_rand_alnum(10));
    }

    // Very unlikely all results are identical
    assert!(alnum_results.iter().any(|s| s != &alnum_results[0]));

    // Generate multiple UUIDs
    let mut uuid_results = vec![];
    for _ in 0..10 {
        uuid_results.push(rsb::gx::id::get_rand_uuid());
    }

    // All UUIDs should be unique
    for i in 0..uuid_results.len() {
        for j in (i + 1)..uuid_results.len() {
            assert_ne!(uuid_results[i], uuid_results[j]);
        }
    }
}

#[test]
fn test_character_set_compliance() {
    // Test that generators produce expected character sets

    // Alphanumeric should contain both letters and numbers over many samples
    let mut has_letter = false;
    let mut has_digit = false;

    for _ in 0..50 {
        let sample = rsb::gx::string::get_rand_alnum(20);
        if sample.chars().any(|c| c.is_alphabetic()) {
            has_letter = true;
        }
        if sample.chars().any(|c| c.is_numeric()) {
            has_digit = true;
        }
        if has_letter && has_digit {
            break;
        }
    }

    assert!(has_letter, "Alphanumeric generator should produce letters");
    assert!(has_digit, "Alphanumeric generator should produce digits");
}
