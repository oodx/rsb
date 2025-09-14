// RSB XSed - Enhanced sed operations with closure support
// This extends RSB's basic sed with closure compatibility

use crate::streams::Stream;

/// XSed - Enhanced sed operations
pub struct XSed {
    content: String,
}

impl XSed {
    pub fn new(content: impl Into<String>) -> Self {
        XSed {
            content: content.into(),
        }
    }
    
    /// Basic string replacement (like RSB sed)
    pub fn replace(self, from: &str, to: &str) -> Self {
        XSed {
            content: self.content.replace(from, to),
        }
    }
    
    /// Replace with a closure (the magic!)
    pub fn replace_with<F>(self, pattern: &str, f: F) -> Self 
    where
        F: Fn(&str) -> String
    {
        // For now, do simple string matching and replacement
        // Later we could add regex support
        let result = self.content;
        
        // Find all occurrences and replace with closure result
        let mut last_end = 0;
        let mut new_content = String::new();
        
        while let Some(start) = result[last_end..].find(pattern) {
            let actual_start = last_end + start;
            let actual_end = actual_start + pattern.len();
            
            // Add everything before the match
            new_content.push_str(&result[last_end..actual_start]);
            
            // Apply the closure to the matched text
            let replacement = f(&result[actual_start..actual_end]);
            new_content.push_str(&replacement);
            
            last_end = actual_end;
        }
        
        // Add the rest
        new_content.push_str(&result[last_end..]);
        
        XSed {
            content: new_content,
        }
    }
    
    /// Replace with regex support (future enhancement)
    pub fn replace_regex(self, pattern: &str, replacement: &str) -> Self {
        // For now, fallback to simple replace
        // Later: use regex crate
        self.replace(pattern, replacement)
    }
    
    /// Replace with regex + closure (ultimate power!)
    pub fn replace_regex_with<F>(self, _pattern: &str, f: F) -> Self 
    where
        F: Fn(&str) -> String
    {
        // Placeholder for regex + closure combo
        // For now, just apply closure to whole content
        XSed {
            content: f(&self.content),
        }
    }
    
    /// Transform token values specifically
    pub fn transform_values<F>(self, f: F) -> Self
    where
        F: Fn(&str) -> String
    {
        let tokens: Vec<&str> = self.content.split(';').collect();
        let transformed: Vec<String> = tokens
            .iter()
            .map(|token| {
                if let Some(eq_pos) = token.find('=') {
                    let (key, value) = token.split_at(eq_pos + 1);
                    let clean_value = value.trim_matches('"').trim_matches('\'');
                    format!("{}\"{}\"", key, f(clean_value))
                } else {
                    token.to_string()
                }
            })
            .collect();
        
        XSed {
            content: transformed.join(";"),
        }
    }
    
    /// Transform token keys
    pub fn transform_keys<F>(self, f: F) -> Self
    where
        F: Fn(&str) -> String
    {
        let tokens: Vec<&str> = self.content.split(';').collect();
        let transformed: Vec<String> = tokens
            .iter()
            .map(|token| {
                if let Some(eq_pos) = token.find('=') {
                    let (key_part, value) = token.split_at(eq_pos);
                    let transformed_key = if key_part.contains(':') {
                        // Handle namespace:key format
                        let parts: Vec<&str> = key_part.split(':').collect();
                        if parts.len() == 2 {
                            format!("{}:{}", parts[0], f(parts[1]))
                        } else {
                            f(key_part)
                        }
                    } else {
                        f(key_part.trim())
                    };
                    format!("{}{}", transformed_key, value)
                } else {
                    token.to_string()
                }
            })
            .collect();
            
        XSed {
            content: transformed.join(";"),
        }
    }
    
    /// Chain with RSB stream operations
    pub fn rsb_stream<F>(self, f: F) -> Self
    where
        F: FnOnce(Stream) -> Stream
    {
        let stream = Stream::from_string(&self.content);
        let result = f(stream).to_string();
        XSed {
            content: result,
        }
    }
    
    /// Convert back to string
    pub fn to_string(self) -> String {
        self.content
    }
    
}

/// Convenience function for creating XSed
pub fn xsed(content: impl Into<String>) -> XSed {
    XSed::new(content)
}

/// Trait to add xsed to any string-like type
pub trait ToXSed {
    fn xsed(self) -> XSed;
}

impl ToXSed for String {
    fn xsed(self) -> XSed {
        XSed::new(self)
    }
}

impl ToXSed for &str {
    fn xsed(self) -> XSed {
        XSed::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_replace() {
        let result = xsed("hello world")
            .replace("world", "rust")
            .to_string();
        assert_eq!(result, "hello rust");
    }
    
    #[test]
    fn test_replace_with_closure() {
        let result = xsed("hello world")
            .replace_with("world", |matched| matched.to_uppercase())
            .to_string();
        assert_eq!(result, "hello WORLD");
    }
    
    #[test]
    fn test_transform_values() {
        let result = xsed("key=\"hello\"; user=\"world\"")
            .transform_values(|v| v.to_uppercase())
            .to_string();
        // Preserve original spacing between tokens
        assert_eq!(result, "key=\"HELLO\"; user=\"WORLD\"");
    }
}
