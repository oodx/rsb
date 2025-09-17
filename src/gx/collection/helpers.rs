//! Collection generation implementation functions
//!
//! Core collection utility functions migrated from src/random.rs

use rand::prelude::IndexedRandom;

/// Selects a random word from a slice of strings.
pub fn get_rand_from_slice(words: &[String]) -> Option<String> {
    let mut rng = rand::rng();
    words.choose(&mut rng).cloned()
}
