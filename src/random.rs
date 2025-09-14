use rand::{prelude::IndexedRandom, Rng, distr::Alphanumeric};

const PRINTABLE_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

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
            let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
            chars.chars().nth(rng.random_range(0..chars.len())).unwrap()
        })
        .collect()
}

/// Generates a random hexadecimal string of a given length.
pub fn get_rand_hex(n: usize) -> String {
    let mut rng = rand::rng();
    (0..n)
        .map(|_| {
            let chars = "0123456789abcdef";
            chars.chars().nth(rng.random_range(0..chars.len())).unwrap()
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

/// Generates a new v4 UUID.
pub fn get_rand_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Selects a random word from a slice of strings.
pub fn get_rand_from_slice(words: &[String]) -> Option<String> {
    let mut rng = rand::rng();
    words.choose(&mut rng).cloned()
}

/// Returns a random integer in [min, max] inclusive as usize.
pub fn rand_range_usize(min: usize, max: usize) -> usize {
    let mut rng = rand::rng();
    if min >= max {
        return min;
    }
    rng.random_range(min..=max)
}


// TODO: support jynx/boxy stderr colors
