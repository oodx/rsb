//! Random number generation sanity tests (gx::rand module)

#[cfg(test)]
mod tests {
    use rsb::gx::rand::*;  // Random number generation (moved from math)

    #[test]
    fn test_random_range() {
        let min = 1.0;
        let max = 10.0;

        // Generate multiple values to test range
        for _ in 0..100 {
            let value = random_range(min, max);
            assert!(value >= min);
            assert!(value <= max);
        }

        // Test with same min/max
        let same = random_range(5.0, 5.0);
        assert_eq!(same, 5.0);

        // Test with negative range
        let neg_value = random_range(-10.0, -1.0);
        assert!(neg_value >= -10.0);
        assert!(neg_value <= -1.0);
    }

    #[test]
    fn test_random_int_range() {
        let min = 1;
        let max = 100;

        // Generate multiple values to test range
        for _ in 0..100 {
            let value = random_int_range(min, max);
            assert!(value >= min);
            assert!(value <= max);
        }

        // Test with same min/max
        let same = random_int_range(42, 42);
        assert_eq!(same, 42);

        // Test with small range
        let small = random_int_range(1, 3);
        assert!(small >= 1 && small <= 3);
    }

    #[test]
    fn test_random_list_bool() {
        let bools = random_list_bool(5);
        assert_eq!(bools.len(), 5);

        // All values should be true or false
        for &value in &bools {
            assert!(value == true || value == false);
        }

        // Test empty list
        let empty = random_list_bool(0);
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn test_random_list_int() {
        let ints = random_list_int(5, 1, 100);
        assert_eq!(ints.len(), 5);

        // All values should be in range
        for &value in &ints {
            assert!(value >= 1 && value <= 100);
        }

        // Test single value range
        let singles = random_list_int(3, 42, 42);
        assert_eq!(singles.len(), 3);
        for &value in &singles {
            assert_eq!(value, 42);
        }
    }

    #[test]
    fn test_random_list_float() {
        let floats = random_list_float(5, 0.0, 1.0);
        assert_eq!(floats.len(), 5);

        // All values should be in range
        for &value in &floats {
            assert!(value >= 0.0 && value <= 1.0);
        }
    }

    #[test]
    fn test_random_list_string_bool() {
        let result = random_list_string("bool", 5, None).unwrap();
        let parts: Vec<&str> = result.split(',').collect();
        assert_eq!(parts.len(), 5);

        // All parts should be "0" or "1"
        for part in parts {
            assert!(part == "0" || part == "1");
        }

        // Test aliases
        assert!(random_list_string("boolean", 3, None).is_ok());
    }

    #[test]
    fn test_random_list_string_int() {
        let result = random_list_string("int", 3, Some("1:10")).unwrap();
        let parts: Vec<&str> = result.split(',').collect();
        assert_eq!(parts.len(), 3);

        // All parts should parse as integers in range
        for part in parts {
            let value: i64 = part.parse().unwrap();
            assert!(value >= 1 && value <= 10);
        }

        // Test default range
        let default_result = random_list_string("int", 3, None).unwrap();
        let default_parts: Vec<&str> = default_result.split(',').collect();
        assert_eq!(default_parts.len(), 3);

        // Test aliases
        assert!(random_list_string("integer", 2, Some("1:5")).is_ok());
    }

    #[test]
    fn test_random_list_string_float() {
        let result = random_list_string("float", 3, Some("0.0:1.0")).unwrap();
        let parts: Vec<&str> = result.split(',').collect();
        assert_eq!(parts.len(), 3);

        // All parts should parse as floats in range
        for part in parts {
            let value: f64 = part.parse().unwrap();
            assert!(value >= 0.0 && value <= 1.0);
        }

        // Test default range
        let default_result = random_list_string("float", 2, None).unwrap();
        let default_parts: Vec<&str> = default_result.split(',').collect();
        assert_eq!(default_parts.len(), 2);

        // Test aliases
        assert!(random_list_string("f64", 2, Some("1.0:5.0")).is_ok());
    }

    #[test]
    fn test_random_list_string_errors() {
        // Invalid type
        assert!(random_list_string("invalid_type", 5, None).is_err());

        // Invalid range format
        assert!(random_list_string("int", 3, Some("invalid")).is_err());
        assert!(random_list_string("int", 3, Some("10")).is_err()); // no colon

        // Invalid range values
        assert!(random_list_string("int", 3, Some("not_a_number:10")).is_err());
        assert!(random_list_string("int", 3, Some("10:not_a_number")).is_err());

        // Min greater than max
        assert!(random_list_string("int", 3, Some("10:5")).is_err());
        assert!(random_list_string("float", 3, Some("5.0:1.0")).is_err());
    }

    #[test]
    fn test_parse_range_functions() {
        // Test various range formats through the string interface
        let result1 = random_list_string("int", 2, Some("0:100")).unwrap();
        let parts1: Vec<&str> = result1.split(',').collect();
        assert_eq!(parts1.len(), 2);

        let result2 = random_list_string("float", 2, Some("-1.0:1.0")).unwrap();
        let parts2: Vec<&str> = result2.split(',').collect();
        assert_eq!(parts2.len(), 2);
    }

    #[test]
    fn test_edge_cases() {
        // Zero count
        let empty_result = random_list_string("int", 0, Some("1:10")).unwrap();
        assert_eq!(empty_result, "");

        // Single value
        let single_result = random_list_string("bool", 1, None).unwrap();
        assert!(single_result == "0" || single_result == "1");

        // Large count (performance test)
        let large_result = random_list_string("int", 1000, Some("1:2")).unwrap();
        let large_parts: Vec<&str> = large_result.split(',').collect();
        assert_eq!(large_parts.len(), 1000);
    }
}
