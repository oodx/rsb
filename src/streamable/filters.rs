// RSB Streamable Filters - Text filtering operations
// Generic text filters that don't require domain knowledge

use super::traits::Streamable;

/// Filter lines containing a pattern
pub struct FilterLines;
impl Streamable for FilterLines {
    type Args = String;
    
    fn stream_apply(stdin: &str, pattern: Self::Args) -> String {
        stdin.lines()
            .filter(|line| line.contains(&pattern))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Filter out empty lines
pub struct FilterEmpty;
impl Streamable for FilterEmpty {
    type Args = ();
    
    fn stream_apply(stdin: &str, _args: Self::Args) -> String {
        stdin.lines()
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Filter lines by length range
pub struct FilterByLength;
impl Streamable for FilterByLength {
    type Args = (usize, usize); // (min, max)
    
    fn stream_apply(stdin: &str, args: Self::Args) -> String {
        let (min, max) = args;
        stdin.lines()
            .filter(|line| {
                let len = line.len();
                len >= min && len <= max
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Filter lines matching regex pattern
pub struct FilterRegex;
impl Streamable for FilterRegex {
    type Args = String;
    
    fn stream_apply(stdin: &str, pattern: Self::Args) -> String {
        use regex::Regex;
        let re = Regex::new(&pattern).unwrap_or_else(|_| Regex::new(".*").unwrap());
        stdin.lines()
            .filter(|line| re.is_match(line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Filter lines NOT containing a pattern
pub struct FilterNotContains;
impl Streamable for FilterNotContains {
    type Args = String;
    
    fn stream_apply(stdin: &str, pattern: Self::Args) -> String {
        stdin.lines()
            .filter(|line| !line.contains(&pattern))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Filter duplicate lines (keep first occurrence)
pub struct FilterDuplicates;
impl Streamable for FilterDuplicates {
    type Args = ();
    
    fn stream_apply(stdin: &str, _args: Self::Args) -> String {
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        stdin.lines()
            .filter(|line| seen.insert(line.to_string()))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Filter lines that start with a pattern
pub struct FilterStartsWith;
impl Streamable for FilterStartsWith {
    type Args = String;
    
    fn stream_apply(stdin: &str, pattern: Self::Args) -> String {
        stdin.lines()
            .filter(|line| line.starts_with(&pattern))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Filter lines that end with a pattern
pub struct FilterEndsWith;
impl Streamable for FilterEndsWith {
    type Args = String;
    
    fn stream_apply(stdin: &str, pattern: Self::Args) -> String {
        stdin.lines()
            .filter(|line| line.ends_with(&pattern))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Take first N lines (like head)
pub struct TakeLines;
impl Streamable for TakeLines {
    type Args = usize;
    
    fn stream_apply(stdin: &str, n: Self::Args) -> String {
        stdin.lines()
            .take(n)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Skip first N lines
pub struct SkipLines;
impl Streamable for SkipLines {
    type Args = usize;
    
    fn stream_apply(stdin: &str, n: Self::Args) -> String {
        stdin.lines()
            .skip(n)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::streamable::traits::StreamApply;

    #[test]
    fn test_filter_lines() {
        let input = "hello world\nfoo bar\nhello rust";
        let result = input.stream_apply(FilterLines, "hello".to_string());
        assert_eq!(result, "hello world\nhello rust");
    }

    #[test]
    fn test_filter_empty() {
        let input = "line1\n\n  \nline2\n\nline3";
        let result = input.stream_apply(FilterEmpty, ());
        assert_eq!(result, "line1\nline2\nline3");
    }

    #[test]
    fn test_filter_by_length() {
        let input = "a\nshort\nmedium line\nvery long line here";
        let result = input.stream_apply(FilterByLength, (5, 15));
        assert_eq!(result, "short\nmedium line");
    }
}