//! String generation implementation functions
//!
//! Core string generation functions migrated from src/random.rs

use rand::{Rng, distr::Alphanumeric};
use super::constants::*;

/// Generates a random alphanumeric string of a given length.
pub fn get_rand_alnum(n: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}

/// Generates a random alphabetic string of a given length.
pub fn get_rand_alpha(n: usize) -> String {
    let mut rng = rand::rng();
    (0..n)
        .map(|_| {
            ALPHA_CHARS.chars().nth(rng.random_range(0..ALPHA_CHARS.len())).unwrap()
        })
        .collect()
}

/// Generates a random hexadecimal string of a given length.
pub fn get_rand_hex(n: usize) -> String {
    let mut rng = rand::rng();
    (0..n)
        .map(|_| {
            HEX_CHARS.chars().nth(rng.random_range(0..HEX_CHARS.len())).unwrap()
        })
        .collect()
}

/// Generates a random string of printable, non-whitespace characters of a given length.
pub fn get_rand_string(n: usize) -> String {
    let mut rng = rand::rng();
    (0..n)
        .map(|_| {
            PRINTABLE_CHARS.chars().nth(rng.random_range(0..PRINTABLE_CHARS.len())).unwrap()
        })
        .collect()
}
