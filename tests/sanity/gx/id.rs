//! GX ID Generation Sanity Tests
//!
//! Comprehensive tests for ID generation functions

#[cfg(test)]
mod tests {
    use rsb::gx::id::*;

    #[test]
    fn test_get_rand_uuid_basic() {
        let uuid = get_rand_uuid();

        // UUID v4 should be 36 characters (32 hex + 4 hyphens)
        assert_eq!(uuid.len(), 36);

        // Should contain hyphens in the right positions
        assert_eq!(uuid.chars().nth(8), Some('-'));
        assert_eq!(uuid.chars().nth(13), Some('-'));
        assert_eq!(uuid.chars().nth(18), Some('-'));
        assert_eq!(uuid.chars().nth(23), Some('-'));
    }

    #[test]
    fn test_get_rand_uuid_format() {
        let uuid = get_rand_uuid();

        // Split by hyphens and verify segment lengths
        let parts: Vec<&str> = uuid.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8); // First segment
        assert_eq!(parts[1].len(), 4); // Second segment
        assert_eq!(parts[2].len(), 4); // Third segment
        assert_eq!(parts[3].len(), 4); // Fourth segment
        assert_eq!(parts[4].len(), 12); // Fifth segment

        // Verify all parts are valid hex
        for part in parts {
            assert!(part.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    #[test]
    fn test_get_rand_uuid_uniqueness() {
        // Generate multiple UUIDs and verify they're unique
        let uuid1 = get_rand_uuid();
        let uuid2 = get_rand_uuid();
        let uuid3 = get_rand_uuid();

        assert_ne!(uuid1, uuid2);
        assert_ne!(uuid2, uuid3);
        assert_ne!(uuid1, uuid3);
    }

    #[test]
    fn test_get_rand_uuid_v4_compliance() {
        let uuid = get_rand_uuid();

        // V4 UUIDs should have version 4 in the correct position
        // The version is in the first character of the third segment
        let parts: Vec<&str> = uuid.split('-').collect();
        let version_char = parts[2].chars().nth(0).unwrap();
        assert_eq!(version_char, '4', "UUID should be version 4, got: {}", uuid);

        // The variant should be 8, 9, A, or B (first char of fourth segment)
        let variant_char = parts[3].chars().nth(0).unwrap();
        assert!(
            variant_char == '8'
                || variant_char == '9'
                || variant_char == 'a'
                || variant_char == 'b'
                || variant_char == 'A'
                || variant_char == 'B',
            "Invalid UUID v4 variant: {} in UUID: {}",
            variant_char,
            uuid
        );
    }
}
