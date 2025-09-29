//! Case conversion helpers and tokenization for string transformations.
//!
//! ASCII-SAFE: yes (normalizes to ASCII-friendly tokens and separators)
//! UNICODE-SAFE: input parsing uses Unicode scalars; output casing/joins target ASCII use cases

fn is_sep(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r' | '_' | '-' | '.' | '/' | '\\')
}

use crate::string::error::log_string_error;
use crate::string::guard::guard_size;

const CASE_MAX_LINE_BYTES: usize = 64 * 1024; // 64 KiB line-sized default

fn with_case_guard<F>(op: &str, s: &str, f: F) -> String
where
    F: FnOnce(&str) -> String,
{
    match guard_size(s, CASE_MAX_LINE_BYTES) {
        Ok(()) => f(s),
        Err(err) => log_string_error(op, &err),
    }
}

// Normalize input to ASCII-friendly separators: keep [A-Za-z0-9], turn everything else into a space.
fn ascii_normalize_to_separators(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
        } else {
            out.push(' ');
        }
    }
    out
}

/// Split string into word tokens using separators, case, and digit boundaries.
pub fn split_words(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 {
        return Vec::new();
    }

    let mut words: Vec<String> = Vec::new();
    let mut start = 0usize;
    let mut i = 0usize;

    // Helper to flush a token
    let mut flush = |from: usize, to: usize| {
        if to > from {
            let t: String = chars[from..to].iter().collect();
            if !t.is_empty() {
                words.push(t);
            }
        }
    };

    while i < n {
        let c = chars[i];
        let prev = if i > 0 { Some(chars[i - 1]) } else { None };
        let next = if i + 1 < n { Some(chars[i + 1]) } else { None };

        // Separator boundary
        if is_sep(c) {
            flush(start, i);
            start = i + 1;
            i += 1;
            continue;
        }

        // Digit/alpha boundaries
        if let Some(p) = prev {
            let p_is_alnum = p.is_alphanumeric();
            let c_is_alnum = c.is_alphanumeric();
            if p_is_alnum && c_is_alnum {
                // lower->Upper boundary: userName → user | Name
                if p.is_lowercase() && c.is_uppercase() {
                    flush(start, i);
                    start = i;
                } else if p.is_uppercase() && c.is_uppercase() {
                    // UPPER run followed by lower: HTTPServer → HTTP | Server (split before 'S')
                    if let Some(nc) = next {
                        if nc.is_lowercase() {
                            flush(start, i);
                            start = i;
                        }
                    }
                } else {
                    // letter<->digit boundaries
                    if (p.is_ascii_digit() && !c.is_ascii_digit())
                        || (!p.is_ascii_digit() && c.is_ascii_digit())
                    {
                        flush(start, i);
                        start = i;
                    }
                }
            } else if p_is_alnum && !c_is_alnum {
                // Already handled separators above; keep for completeness
            }
        }

        i += 1;
    }

    // Flush trailing
    flush(start, n);

    words
}

pub fn to_lower(s: &str) -> String {
    s.to_lowercase()
}
pub fn to_upper(s: &str) -> String {
    s.to_uppercase()
}

/// ASCII-SAFE: yes
pub fn to_snake_case(s: &str) -> String {
    with_case_guard("to_snake_case", s, |s| {
        let norm = ascii_normalize_to_separators(s);
        let words = split_words(&norm);
        words
            .into_iter()
            .map(|w| w.to_lowercase())
            .collect::<Vec<_>>()
            .join("_")
    })
}

/// ASCII-SAFE: yes
pub fn to_kebab_case(s: &str) -> String {
    with_case_guard("to_kebab_case", s, |s| {
        let norm = ascii_normalize_to_separators(s);
        let words = split_words(&norm);
        words
            .into_iter()
            .map(|w| w.to_lowercase())
            .collect::<Vec<_>>()
            .join("-")
    })
}

/// ASCII-SAFE: yes
pub fn to_dot_case(s: &str) -> String {
    with_case_guard("to_dot_case", s, |s| {
        let norm = ascii_normalize_to_separators(s);
        let words = split_words(&norm);
        words
            .into_iter()
            .map(|w| w.to_lowercase())
            .collect::<Vec<_>>()
            .join(".")
    })
}

/// ASCII-SAFE: yes
pub fn to_space_case(s: &str) -> String {
    with_case_guard("to_space_case", s, |s| {
        let norm = ascii_normalize_to_separators(s);
        let words = split_words(&norm);
        words
            .into_iter()
            .map(|w| w.to_lowercase())
            .collect::<Vec<_>>()
            .join(" ")
    })
}

/// ASCII-SAFE: yes
pub fn to_camel_case(s: &str) -> String {
    with_case_guard("to_camel_case", s, |s| {
        let norm = ascii_normalize_to_separators(s);
        let words = split_words(&norm);
        if words.is_empty() {
            return String::new();
        }
        let mut out = String::new();
        for (idx, w) in words.into_iter().enumerate() {
            if idx == 0 {
                out.push_str(&w.to_lowercase());
            } else {
                let mut c = w.chars();
                if let Some(f) = c.next() {
                    out.push_str(&f.to_uppercase().collect::<String>());
                    out.push_str(&c.as_str().to_lowercase());
                }
            }
        }
        out
    })
}

/// ASCII-SAFE: yes
/// Convert to PascalCase (UpperCamelCase) - first letter capitalized
pub fn to_pascal_case(s: &str) -> String {
    with_case_guard("to_pascal_case", s, |s| {
        let norm = ascii_normalize_to_separators(s);
        let words = split_words(&norm);
        if words.is_empty() {
            return String::new();
        }
        let mut out = String::new();
        for w in words.into_iter() {
            let mut c = w.chars();
            if let Some(f) = c.next() {
                out.push_str(&f.to_uppercase().collect::<String>());
                out.push_str(&c.as_str().to_lowercase());
            }
        }
        out
    })
}

/// ASCII-SAFE: yes
/// Convert to SCREAMING_SNAKE_CASE - all uppercase with underscores
pub fn to_screaming_snake_case(s: &str) -> String {
    with_case_guard("to_screaming_snake_case", s, |s| {
        to_snake_case(s).to_uppercase()
    })
}

// Try variants for size-guarded case conversions
pub fn try_to_snake_case(s: &str) -> Result<String, crate::string::error::StringError> {
    match guard_size(s, CASE_MAX_LINE_BYTES) {
        Ok(()) => Ok(to_snake_case(s)),
        Err(e) => Err(e),
    }
}

pub fn try_to_kebab_case(s: &str) -> Result<String, crate::string::error::StringError> {
    match guard_size(s, CASE_MAX_LINE_BYTES) {
        Ok(()) => Ok(to_kebab_case(s)),
        Err(e) => Err(e),
    }
}

pub fn try_to_dot_case(s: &str) -> Result<String, crate::string::error::StringError> {
    match guard_size(s, CASE_MAX_LINE_BYTES) {
        Ok(()) => Ok(to_dot_case(s)),
        Err(e) => Err(e),
    }
}

pub fn try_to_space_case(s: &str) -> Result<String, crate::string::error::StringError> {
    match guard_size(s, CASE_MAX_LINE_BYTES) {
        Ok(()) => Ok(to_space_case(s)),
        Err(e) => Err(e),
    }
}

pub fn try_to_camel_case(s: &str) -> Result<String, crate::string::error::StringError> {
    match guard_size(s, CASE_MAX_LINE_BYTES) {
        Ok(()) => Ok(to_camel_case(s)),
        Err(e) => Err(e),
    }
}

pub fn try_to_pascal_case(s: &str) -> Result<String, crate::string::error::StringError> {
    match guard_size(s, CASE_MAX_LINE_BYTES) {
        Ok(()) => Ok(to_pascal_case(s)),
        Err(e) => Err(e),
    }
}

pub fn try_to_screaming_snake_case(s: &str) -> Result<String, crate::string::error::StringError> {
    match guard_size(s, CASE_MAX_LINE_BYTES) {
        Ok(()) => Ok(to_screaming_snake_case(s)),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_words_boundaries() {
        assert_eq!(split_words("userName42"), vec!["user", "Name", "42"]);
        assert_eq!(split_words("HTTPServer"), vec!["HTTP", "Server"]);
        assert_eq!(split_words("HTTPSever"), vec!["HTTP", "Sever"]);
        assert_eq!(split_words("a-b_c.d/e"), vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn test_case_helpers() {
        assert_eq!(to_snake_case("HTTP server ID"), "http_server_id");
        assert_eq!(to_kebab_case("User Name 42"), "user-name-42");
        assert_eq!(to_dot_case("Log File Name"), "log.file.name");
        assert_eq!(to_space_case("My File NAME"), "my file name");
        assert_eq!(to_camel_case("HTTP server id"), "httpServerId");
        assert_eq!(to_pascal_case("user_name"), "UserName");
        assert_eq!(to_pascal_case("http_server"), "HttpServer");
        assert_eq!(to_screaming_snake_case("userName"), "USER_NAME");
        assert_eq!(to_screaming_snake_case("HttpServer"), "HTTP_SERVER");
    }
}
