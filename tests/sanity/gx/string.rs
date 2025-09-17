//! GX String Generation Sanity Tests
//!
//! Comprehensive tests for string generation functions migrated from random.rs

#[cfg(test)]
mod tests {
    use rsb::gx::string::*;

    #[test]
    fn test_get_rand_alnum_basic() {
        // Test basic functionality
        let result = get_rand_alnum(10);
        assert_eq!(result.len(), 10);

        // Verify all characters are alphanumeric
        assert!(result.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_get_rand_alnum_edge_cases() {
        // Test edge cases
        let empty = get_rand_alnum(0);
        assert_eq!(empty.len(), 0);

        let single = get_rand_alnum(1);
        assert_eq!(single.len(), 1);
        assert!(single.chars().next().unwrap().is_ascii_alphanumeric());

        // Test larger size
        let large = get_rand_alnum(1000);
        assert_eq!(large.len(), 1000);
        assert!(large.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_get_rand_alpha_basic() {
        let result = get_rand_alpha(15);
        assert_eq!(result.len(), 15);

        // Verify all characters are alphabetic
        assert!(result.chars().all(|c| c.is_ascii_alphabetic()));
    }

    #[test]
    fn test_get_rand_alpha_character_set() {
        let result = get_rand_alpha(100);

        // Verify only expected characters appear
        assert!(result
            .chars()
            .all(|c| { (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') }));
    }

    #[test]
    fn test_get_rand_hex_basic() {
        let result = get_rand_hex(8);
        assert_eq!(result.len(), 8);

        // Verify all characters are valid hex
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_get_rand_hex_character_set() {
        let result = get_rand_hex(100);

        // Verify only lowercase hex characters
        assert!(result
            .chars()
            .all(|c| { (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') }));
    }

    #[test]
    fn test_get_rand_string_basic() {
        let result = get_rand_string(20);
        assert_eq!(result.len(), 20);

        // Verify all characters are printable and non-whitespace
        assert!(result
            .chars()
            .all(|c| { c.is_ascii_graphic() && !c.is_whitespace() }));
    }

    #[test]
    fn test_get_rand_string_character_set() {
        let result = get_rand_string(200);

        // Verify characters come from expected printable set
        for c in result.chars() {
            assert!(PRINTABLE_CHARS.contains(c), "Unexpected character: {}", c);
        }
    }

    #[test]
    fn test_randomness_basic() {
        // Generate multiple strings and verify they're different
        // (extremely unlikely to be identical for reasonable lengths)
        let s1 = get_rand_alnum(20);
        let s2 = get_rand_alnum(20);
        let s3 = get_rand_alnum(20);

        // Basic randomness check - shouldn't all be identical
        assert!(
            !(s1 == s2 && s2 == s3),
            "Generated strings are identical: {}, {}, {}",
            s1,
            s2,
            s3
        );
    }

    #[test]
    fn test_constants_availability() {
        // Verify constants are accessible and non-empty
        assert!(!PRINTABLE_CHARS.is_empty());
        assert!(!HEX_CHARS.is_empty());
        assert!(!ALPHA_CHARS.is_empty());

        // Verify expected content in constants
        assert!(HEX_CHARS.contains('0'));
        assert!(HEX_CHARS.contains('9'));
        assert!(HEX_CHARS.contains('a'));
        assert!(HEX_CHARS.contains('f'));

        assert!(ALPHA_CHARS.contains('A'));
        assert!(ALPHA_CHARS.contains('Z'));
        assert!(ALPHA_CHARS.contains('a'));
        assert!(ALPHA_CHARS.contains('z'));
    }
}
