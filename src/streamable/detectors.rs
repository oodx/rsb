// RSB Streamable Detectors - Pattern detection operations
// Generic text detection that returns boolean-like results

use super::traits::Streamable;

/// Detect if input is empty or whitespace only
pub struct DetectEmpty;
impl Streamable for DetectEmpty {
    type Args = ();
    
    fn stream_apply(stdin: &str, _args: Self::Args) -> String {
        if stdin.trim().is_empty() {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }
}

/// Detect if input contains a pattern
pub struct DetectPattern;
impl Streamable for DetectPattern {
    type Args = String;
    
    fn stream_apply(stdin: &str, pattern: Self::Args) -> String {
        if stdin.contains(&pattern) {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }
}

/// Detect if input contains binary (non-text) data
pub struct DetectBinary;
impl Streamable for DetectBinary {
    type Args = ();
    
    fn stream_apply(stdin: &str, _args: Self::Args) -> String {
        // Check for non-text bytes (excluding tab, newline, carriage return)
        let has_binary = stdin.bytes().any(|b| b < 0x20 && b != 0x09 && b != 0x0A && b != 0x0D);
        if has_binary {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }
}

/// Detect if input matches a regex pattern
pub struct DetectRegex;
impl Streamable for DetectRegex {
    type Args = String;
    
    fn stream_apply(stdin: &str, pattern: Self::Args) -> String {
        use regex::Regex;
        match Regex::new(&pattern) {
            Ok(re) => {
                if re.is_match(stdin) {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Err(_) => "error: invalid regex".to_string()
        }
    }
}

/// Detect if input has duplicate lines
pub struct DetectDuplicates;
impl Streamable for DetectDuplicates {
    type Args = ();
    
    fn stream_apply(stdin: &str, _args: Self::Args) -> String {
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        for line in stdin.lines() {
            if !seen.insert(line) {
                return "true".to_string();
            }
        }
        "false".to_string()
    }
}

/// Detect encoding (UTF-8, ASCII, or binary)
pub struct DetectEncoding;
impl Streamable for DetectEncoding {
    type Args = ();
    
    fn stream_apply(stdin: &str, _args: Self::Args) -> String {
        // Check if it's valid UTF-8
        if stdin.is_empty() {
            return "empty".to_string();
        }
        
        // Check if all bytes are ASCII
        if stdin.bytes().all(|b| b < 128) {
            "ascii".to_string()
        } else {
            // Since we have a &str, it's already valid UTF-8
            "utf8".to_string()
        }
    }
}

/// Count occurrences of a pattern
pub struct CountPattern;
impl Streamable for CountPattern {
    type Args = String;
    
    fn stream_apply(stdin: &str, pattern: Self::Args) -> String {
        stdin.matches(&pattern).count().to_string()
    }
}

/// Count lines in input
pub struct CountLines;
impl Streamable for CountLines {
    type Args = ();
    
    fn stream_apply(stdin: &str, _args: Self::Args) -> String {
        stdin.lines().count().to_string()
    }
}

/// Count words in input
pub struct CountWords;
impl Streamable for CountWords {
    type Args = ();
    
    fn stream_apply(stdin: &str, _args: Self::Args) -> String {
        stdin.split_whitespace().count().to_string()
    }
}

/// Detect if all lines match a pattern
pub struct DetectAllMatch;
impl Streamable for DetectAllMatch {
    type Args = String;
    
    fn stream_apply(stdin: &str, pattern: Self::Args) -> String {
        let all_match = stdin.lines().all(|line| line.contains(&pattern));
        if all_match {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }
}

/// Detect if any line matches a pattern
pub struct DetectAnyMatch;
impl Streamable for DetectAnyMatch {
    type Args = String;
    
    fn stream_apply(stdin: &str, pattern: Self::Args) -> String {
        let any_match = stdin.lines().any(|line| line.contains(&pattern));
        if any_match {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::streamable::traits::StreamApply;

    #[test]
    fn test_detect_empty() {
        assert_eq!("".stream_apply(DetectEmpty, ()), "true");
        assert_eq!("  \n  ".stream_apply(DetectEmpty, ()), "true");
        assert_eq!("content".stream_apply(DetectEmpty, ()), "false");
    }

    #[test]
    fn test_detect_pattern() {
        let input = "hello world";
        assert_eq!(input.stream_apply(DetectPattern, "world".to_string()), "true");
        assert_eq!(input.stream_apply(DetectPattern, "rust".to_string()), "false");
    }

    #[test]
    fn test_count_pattern() {
        let input = "foo bar foo baz foo";
        assert_eq!(input.stream_apply(CountPattern, "foo".to_string()), "3");
    }

    #[test]
    fn test_detect_duplicates() {
        let input_with_dup = "line1\nline2\nline1";
        let input_no_dup = "line1\nline2\nline3";
        assert_eq!(input_with_dup.stream_apply(DetectDuplicates, ()), "true");
        assert_eq!(input_no_dup.stream_apply(DetectDuplicates, ()), "false");
    }
}