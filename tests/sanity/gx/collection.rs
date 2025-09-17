//! GX Collection Generation Sanity Tests
//!
//! Comprehensive tests for collection utility functions

#[cfg(test)]
mod tests {
    use rsb::gx::collection::*;

    #[test]
    fn test_get_rand_from_slice_basic() {
        let words = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "date".to_string(),
        ];

        let result = get_rand_from_slice(&words);
        assert!(result.is_some());

        let selected = result.unwrap();
        assert!(words.contains(&selected));
    }

    #[test]
    fn test_get_rand_from_slice_empty() {
        let empty_words: Vec<String> = vec![];
        let result = get_rand_from_slice(&empty_words);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_rand_from_slice_single() {
        let single_word = vec!["onlyone".to_string()];
        let result = get_rand_from_slice(&single_word);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "onlyone");
    }

    #[test]
    fn test_get_rand_from_slice_distribution() {
        let words = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];

        // Run many selections and verify we get different results
        // (basic randomness check)
        let mut results = std::collections::HashSet::new();
        for _ in 0..50 {
            if let Some(result) = get_rand_from_slice(&words) {
                results.insert(result);
            }
        }

        // Should see multiple different results over 50 tries
        // (would be extremely unlikely to only get one value)
        assert!(
            results.len() > 1,
            "Expected variety in random selection, got: {:?}",
            results
        );

        // All results should be valid words from our list
        for result in &results {
            assert!(words.contains(result));
        }
    }

    #[test]
    fn test_get_rand_from_slice_large_collection() {
        let large_words: Vec<String> = (0..1000).map(|i| format!("word_{}", i)).collect();

        for _ in 0..20 {
            let result = get_rand_from_slice(&large_words);
            assert!(result.is_some());

            let selected = result.unwrap();
            assert!(large_words.contains(&selected));
            assert!(selected.starts_with("word_"));
        }
    }

    #[test]
    fn test_get_rand_from_slice_clone_behavior() {
        let words = vec!["original".to_string()];

        let result = get_rand_from_slice(&words);
        assert!(result.is_some());

        let selected = result.unwrap();

        // Verify we got a clone, not a reference
        assert_eq!(selected, "original");

        // Original vector should still exist and be unchanged
        assert_eq!(words.len(), 1);
        assert_eq!(words[0], "original");
    }
}
