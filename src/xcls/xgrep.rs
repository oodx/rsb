// xgrep - Enhanced grep with closure support
// Like grep but with transformation capabilities

/// XGrep - grep with closure transformations
pub struct XGrep {
    content: String,
}

impl XGrep {
    /// Create new XGrep from content
    pub fn new(content: impl Into<String>) -> Self {
        XGrep {
            content: content.into(),
        }
    }
    
    /// Filter lines with a closure predicate
    pub fn filter_lines<F>(self, predicate: F) -> Self 
    where
        F: Fn(&str) -> bool
    {
        let filtered = self.content
            .lines()
            .filter(|line| predicate(line))
            .collect::<Vec<_>>()
            .join("\n");
        XGrep::new(filtered)
    }
    
    /// Map lines with a transformation closure
    pub fn map_lines<F>(self, transform: F) -> Self
    where
        F: Fn(&str) -> String
    {
        let mapped = self.content
            .lines()
            .map(|line| transform(line))
            .collect::<Vec<_>>()
            .join("\n");
        XGrep::new(mapped)
    }
    
    /// Filter and map in one operation
    pub fn filter_map<F>(self, func: F) -> Self
    where
        F: Fn(&str) -> Option<String>
    {
        let result = self.content
            .lines()
            .filter_map(|line| func(line))
            .collect::<Vec<_>>()
            .join("\n");
        XGrep::new(result)
    }
    
    /// Find lines matching pattern and transform them
    pub fn grep_transform<F>(self, pattern: &str, transform: F) -> Self
    where
        F: Fn(&str) -> String
    {
        let result = self.content
            .lines()
            .map(|line| {
                if line.contains(pattern) {
                    transform(line)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        XGrep::new(result)
    }
    
    /// Extract matches with a closure
    pub fn extract_matches<F>(self, pattern: &str, extractor: F) -> Self
    where
        F: Fn(&str) -> String
    {
        let result = self.content
            .lines()
            .filter(|line| line.contains(pattern))
            .map(|line| extractor(line))
            .collect::<Vec<_>>()
            .join("\n");
        XGrep::new(result)
    }
    
    /// Count lines matching a closure predicate
    pub fn count_matching<F>(self, predicate: F) -> usize
    where
        F: Fn(&str) -> bool
    {
        self.content
            .lines()
            .filter(|line| predicate(line))
            .count()
    }
    
    /// Get the resulting string
    pub fn to_string(self) -> String {
        self.content
    }
}

/// Convenience function for creating XGrep
pub fn xgrep(content: impl Into<String>) -> XGrep {
    XGrep::new(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_lines() {
        let input = "hello world\nfoo bar\nhello rust";
        let result = xgrep(input)
            .filter_lines(|line| line.contains("hello"))
            .to_string();
        assert_eq!(result, "hello world\nhello rust");
    }
    
    #[test]
    fn test_map_lines() {
        let input = "line1\nline2";
        let result = xgrep(input)
            .map_lines(|line| format!("[{}]", line))
            .to_string();
        assert_eq!(result, "[line1]\n[line2]");
    }
    
    #[test]
    fn test_grep_transform() {
        let input = "foo: 123\nbar: 456\nfoo: 789";
        let result = xgrep(input)
            .grep_transform("foo", |line| line.to_uppercase())
            .to_string();
        assert_eq!(result, "FOO: 123\nbar: 456\nFOO: 789");
    }
    
    #[test]
    fn test_extract_matches() {
        let input = "name: alice\nage: 30\nname: bob";
        let result = xgrep(input)
            .extract_matches("name", |line| {
                line.split(": ").nth(1).unwrap_or("").to_string()
            })
            .to_string();
        assert_eq!(result, "alice\nbob");
    }
}