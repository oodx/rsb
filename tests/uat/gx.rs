use rsb::prelude::*;

#[test]
fn uat_gx_string_generators_demo() {
    println!("\n=== UAT: GX String Generators Demo ===");

    // Test string generation functions
    println!("Testing string generation capabilities...");

    // Test alphanumeric string generation
    let alnum_str = rsb::gx::get_rand_alnum(10);
    println!("✓ Random alphanumeric (10 chars): {}", alnum_str);
    assert_eq!(alnum_str.len(), 10);

    // Test alphabetic string generation
    let alpha_str = rsb::gx::get_rand_alpha(8);
    println!("✓ Random alphabetic (8 chars): {}", alpha_str);
    assert_eq!(alpha_str.len(), 8);

    // Test hexadecimal string generation
    let hex_str = rsb::gx::get_rand_hex(16);
    println!("✓ Random hexadecimal (16 chars): {}", hex_str);
    assert_eq!(hex_str.len(), 16);

    // Test custom string generation
    let custom_str = rsb::gx::get_rand_string(12);
    println!("✓ Random custom string (12 chars): {}", custom_str);
    assert_eq!(custom_str.len(), 12);

    // Test string generation macros
    let macro_alnum = rand_alnum!(6);
    println!("✓ Macro alphanumeric (6 chars): {}", macro_alnum);

    let macro_alpha = rand_alpha!(5);
    println!("✓ Macro alphabetic (5 chars): {}", macro_alpha);

    let macro_hex = rand_hex!(8);
    println!("✓ Macro hexadecimal (8 chars): {}", macro_hex);

    println!("String generators demo completed!");
}

#[test]
fn uat_gx_id_generators_demo() {
    println!("\n=== UAT: GX ID Generators Demo ===");

    // Test UUID generation
    println!("Testing ID generation capabilities...");

    let uuid = rsb::gx::get_rand_uuid();
    println!("✓ Random UUID: {}", uuid);
    assert!(!uuid.is_empty());
    assert!(uuid.contains('-')); // Basic UUID format check

    // Test UUID macro
    let macro_uuid = rand_uuid!();
    println!("✓ Macro UUID: {}", macro_uuid);
    assert!(!macro_uuid.is_empty());

    println!("ID generators demo completed!");
}

#[test]
fn uat_gx_collection_helpers_demo() {
    println!("\n=== UAT: GX Collection Helpers Demo ===");

    // Test collection selection utilities
    println!("Testing collection helper capabilities...");

    let test_items = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
    ];

    // Test random selection from slice
    let selected = rsb::gx::get_rand_from_slice(&test_items);
    match selected {
        Some(item) => {
            println!("✓ Random selection from collection: {}", item);
            assert!(test_items.contains(&item));
        }
        None => {
            println!("⚠ No item selected (empty collection)");
        }
    }

    // Test with empty collection
    let empty_vec: Vec<String> = vec![];
    let empty_selection = rsb::gx::get_rand_from_slice(&empty_vec);
    assert!(empty_selection.is_none());
    println!("✓ Empty collection handled correctly");

    println!("Collection helpers demo completed!");
}

#[test]
fn uat_gx_adapters_and_ranges_demo() {
    println!("\n=== UAT: GX Adapters and Ranges Demo ===");

    // Test cross-module adapter functionality
    println!("Testing adapter and range capabilities...");

    // Test math adapter for random ranges
    let rand_num = rsb::gx::rand_usize_inclusive(1, 100);
    println!("✓ Random number (1-100): {}", rand_num);
    assert!(rand_num >= 1 && rand_num <= 100);

    // Test range macro
    let macro_range = rand_range!(10, 50);
    println!("✓ Macro range (10-50): {}", macro_range);
    assert!(macro_range >= 10 && macro_range <= 50);

    // Test file system adapter for dictionary loading
    let dict_path = "src/gx/data/dict/colors.txt";
    if std::path::Path::new(dict_path).exists() {
        match rsb::gx::load_dict_file(dict_path) {
            Ok(dict) => {
                println!("✓ Dictionary loaded: {} entries", dict.len());
                if !dict.is_empty() {
                    // Test random selection from loaded dictionary
                    if let Some(random_item) = rsb::gx::rand_from_dict_file(dict_path) {
                        println!("✓ Random from dict file: {}", random_item);
                    }
                }
            }
            Err(e) => {
                println!("⚠ Dictionary load failed: {}", e);
            }
        }
    } else {
        println!("ℹ Sample dictionary file not found (optional)");
    }

    println!("Adapters and ranges demo completed!");
}
