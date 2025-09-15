//! GX adapter over fs (dictionary helpers)
//! Consumer-owned adapter functions for dictionary selection from files.

/// Load dictionary words from a file (whitespace-delimited).
pub fn load_dict(path: &str) -> Vec<String> {
    crate::fs::load_dict_from_file(path)
}

/// Pick a random word from a dictionary file.
pub fn rand_from_dict_file(path: &str) -> Option<String> {
    let words = load_dict(path);
    if words.is_empty() { return None; }
    crate::gx::collection::get_rand_from_slice(&words)
}

