//! String helper functions used across the crate.

/// Substring by character indices with optional length.
/// On invalid offset, logs and returns input unchanged (string-first ergonomics).
pub fn str_sub(var: &str, offset: usize, length: Option<usize>) -> String {
    match try_str_sub_abs(var, offset, length) {
        Ok(s) => s,
        Err(err) => crate::string::error::log_string_error("sub", &err),
    }
}

/// Try substring by absolute index and optional length.
pub fn try_str_sub_abs(
    var: &str,
    offset: usize,
    length: Option<usize>,
) -> Result<String, StringError> {
    let chars: Vec<char> = var.chars().collect();
    let n = chars.len();
    if offset > n {
        return Err(StringError::IndexOutOfBounds {
            index: offset as isize,
            len: n,
        });
    }
    let take_len = length.unwrap_or(n - offset);
    let end = offset.saturating_add(take_len).min(n);
    Ok(chars[offset..end].iter().collect())
}

/// Try substring with relative indexing (negative indices count from end).
pub fn try_str_sub_rel(var: &str, start: isize, len: Option<isize>) -> Result<String, StringError> {
    let chars: Vec<char> = var.chars().collect();
    let n = chars.len() as isize;
    let i = crate::string::guard::guard_index(chars.len(), start)? as isize;
    let j = match len {
        None => n,
        Some(l) if l >= 0 => (i + l).min(n),
        Some(l) => (n + l).max(i),
    };
    if j < i || i < 0 || j > n {
        return Err(StringError::IndexOutOfBounds {
            index: start,
            len: chars.len(),
        });
    }
    Ok(chars[i as usize..j as usize].iter().collect())
}

use crate::string::error::{log_string_error, StringError};

// --- Try variants (Result-returning) ---

/// Try to remove a prefix by literal or wildcard pattern.
pub fn try_str_prefix(var: &str, pattern: &str, longest: bool) -> Result<String, StringError> {
    if pattern.contains('*') || pattern.contains('?') {
        let regex_pattern = pattern
            .replace('.', r"\.")
            .replace('*', ".*")
            .replace('?', ".");
        let re = regex::Regex::new(&format!("^{}$", regex_pattern)).map_err(|_| {
            StringError::RegexCompile {
                pattern: pattern.to_string(),
            }
        })?;

        let mut best_match_len = 0usize;
        let mut found = false;
        for (i, _) in var.char_indices().skip(1) {
            let prefix = &var[..i];
            if re.is_match(prefix) {
                if !found || (longest && i > best_match_len) || (!longest && i < best_match_len) {
                    best_match_len = i;
                    found = true;
                    if !longest {
                        break;
                    }
                }
            }
        }
        if found {
            Ok(var[best_match_len..].to_string())
        } else {
            Ok(var.to_string())
        }
    } else if var.starts_with(pattern) {
        Ok(var.strip_prefix(pattern).unwrap_or(var).to_string())
    } else {
        Ok(var.to_string())
    }
}

/// Try to remove a suffix by literal or wildcard pattern.
pub fn try_str_suffix(var: &str, pattern: &str, longest: bool) -> Result<String, StringError> {
    if pattern.contains('*') || pattern.contains('?') {
        let regex_pattern = pattern
            .replace('.', r"\.")
            .replace('*', ".*")
            .replace('?', ".");
        let re = regex::Regex::new(&format!("^{}$", regex_pattern)).map_err(|_| {
            StringError::RegexCompile {
                pattern: pattern.to_string(),
            }
        })?;

        let mut best_start = var.len();
        let mut found = false;
        // Build a list of char boundary indices including 0 and len
        let mut indices: Vec<usize> = var.char_indices().map(|(i, _)| i).collect();
        indices.push(var.len());
        for &i in indices.iter().rev() {
            let suffix = &var[i..];
            if re.is_match(suffix) {
                if !found || (longest && i < best_start) || (!longest && i > best_start) {
                    best_start = i;
                    found = true;
                    if !longest {
                        break;
                    }
                }
            }
        }
        if found {
            Ok(var[..best_start].to_string())
        } else {
            Ok(var.to_string())
        }
    } else if var.ends_with(pattern) {
        Ok(var.strip_suffix(pattern).unwrap_or(var).to_string())
    } else {
        Ok(var.to_string())
    }
}

/// Try to uppercase/lowercase the first character of the first substring matching a glob pattern.
pub fn try_str_case_first_match(
    var: &str,
    pattern: &str,
    to_upper: bool,
) -> Result<String, StringError> {
    let mut regex_pattern = regex::escape(pattern);
    regex_pattern = regex_pattern.replace(r"\*", ".*").replace(r"\?", ".");
    let anchored = format!("^{}", regex_pattern);
    let re = regex::Regex::new(&anchored).map_err(|_| StringError::RegexCompile {
        pattern: pattern.to_string(),
    })?;

    for (start, _) in std::iter::once((0usize, '\0'))
        .chain(var.char_indices())
        .take_while(|(i, _)| *i <= var.len())
    {
        let suffix = &var[start..];
        if let Some(m) = re.find(suffix) {
            if m.start() == 0 {
                let prefix = &var[..start];
                let mut chars = suffix.chars();
                if let Some(first) = chars.next() {
                    let transformed = if to_upper {
                        first.to_uppercase().collect::<String>()
                    } else {
                        first.to_lowercase().collect::<String>()
                    };
                    let rest = &suffix[first.len_utf8()..];
                    return Ok(format!("{}{}{}", prefix, transformed, rest));
                }
            }
        }
    }
    Ok(var.to_string())
}

/// Remove a prefix by literal or wildcard pattern. If `longest` is true,
/// prefers the longest matching prefix; otherwise the shortest.
pub fn str_prefix(var: &str, pattern: &str, longest: bool) -> String {
    match try_str_prefix(var, pattern, longest) {
        Ok(s) => s,
        Err(err) => log_string_error("prefix", &err),
    }
}

/// Remove a suffix by literal or wildcard pattern. If `longest` is true,
/// prefers the longest matching suffix; otherwise the shortest.
pub fn str_suffix(var: &str, pattern: &str, longest: bool) -> String {
    match try_str_suffix(var, pattern, longest) {
        Ok(s) => s,
        Err(err) => log_string_error("suffix", &err),
    }
}

/// Replace first occurrence or all occurrences of a pattern.
pub fn str_replace(var: &str, pattern: &str, replacement: &str, all: bool) -> String {
    if all {
        var.replace(pattern, replacement)
    } else {
        var.replacen(pattern, replacement, 1)
    }
}

/// Uppercase the first character or the whole string depending on `all`.
pub fn str_upper(var: &str, all: bool) -> String {
    if all {
        var.to_uppercase()
    } else {
        let mut c = var.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

/// Lowercase the first character or the whole string depending on `all`.
pub fn str_lower(var: &str, all: bool) -> String {
    if all {
        var.to_lowercase()
    } else {
        let mut c = var.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
        }
    }
}

/// Uppercase/lowercase the first character of the first substring matching a glob pattern.
/// Pattern supports '*' and '?' wildcards. If no match is found, returns the original string.
pub fn str_case_first_match(var: &str, pattern: &str, to_upper: bool) -> String {
    // Convert simple glob to regex and anchor to start (we will slide the start index)
    // Escape regex meta, then restore '*' and '?' wildcard semantics
    match try_str_case_first_match(var, pattern, to_upper) {
        Ok(s) => s,
        Err(err) => log_string_error("case_first_match", &err),
    }
}

// --- Name and Comparison Helpers ---
pub fn is_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

pub fn str_equals(a: &str, b: &str) -> bool {
    a == b
}

pub fn str_matches(text: &str, pattern: &str) -> bool {
    match regex::Regex::new(pattern) {
        Ok(re) => re.is_match(text),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_str_replace() {
        assert_eq!(
            str_replace("hello world", "world", "rust", false),
            "hello rust"
        );
        assert_eq!(
            str_replace("hello world world", "world", "rust", false),
            "hello rust world"
        );
        assert_eq!(
            str_replace("hello world world", "world", "rust", true),
            "hello rust rust"
        );
    }

    #[test]
    fn test_is_name_and_matches() {
        assert!(is_name("valid-name"));
        assert!(is_name("valid_name_123"));
        assert!(!is_name("invalid name"));
        assert!(!is_name("invalid!@#"));

        assert!(str_matches("hello123", r"^hello\d+$"));
        assert!(!str_matches("hello", r"^hello\d+$"));
        assert!(str_equals("a", "a"));
        assert!(!str_equals("a", "b"));
    }
}
