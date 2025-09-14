// RSB Streamable Functions - Basic text processing functions
// (Based on working XStream implementation)

use super::traits::Streamable;

/// Macro to create streamable functions easily
#[macro_export]
macro_rules! streamable {
    // Simple case: fn_name(stdin, arg1, arg2) => { body }
    ($fn_name:ident($stdin:ident, $($arg:ident: $arg_type:ty),*) => $body:block) => {
        pub struct $fn_name;
        
        impl Streamable for $fn_name {
            type Args = ($($arg_type,)*);
            
            fn stream_apply(stdin: &str, args: Self::Args) -> String {
                let $stdin = stdin;
                #[allow(unused_variables)]
                let ($($arg,)*) = args;
                $body
            }
        }
    };
}

// === BASIC TEXT OPERATIONS ===

// Text transforms using the streamable! macro
streamable!(Replace(stdin, find: String, replace: String) => {
    stdin.replace(&find, &replace)
});

streamable!(UpperCase(stdin,) => {
    stdin.to_uppercase()
});

streamable!(LowerCase(stdin,) => {
    stdin.to_lowercase()
});

streamable!(Reverse(stdin,) => {
    stdin.chars().rev().collect::<String>()
});

streamable!(Trim(stdin,) => {
    stdin.trim().to_string()
});


streamable!(Length(stdin,) => {
    stdin.len().to_string()
});


// === ENCODING FUNCTIONS ===

streamable!(Base64Encode(stdin,) => {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.encode(stdin.as_bytes())
});

streamable!(Base64Decode(stdin,) => {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD
        .decode(stdin)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .unwrap_or_else(|| stdin.to_string())
});

streamable!(UrlEncode(stdin,) => {
    urlencoding::encode(stdin).to_string()
});

streamable!(UrlDecode(stdin,) => {
    urlencoding::decode(stdin)
        .map(|s| s.to_string())
        .unwrap_or_else(|_| stdin.to_string())
});

// === CONVENIENCE FUNCTIONS ===

/// Function-style interface for replace (alternative to struct)
pub fn replace_fn(input: &str, args: (String, String)) -> String {
    Replace::stream_apply(input, args)
}

/// Function-style interface for uppercase
pub fn uppercase_fn(input: &str, _args: ()) -> String {
    UpperCase::stream_apply(input, ())
}

/// Function-style interface for lowercase
pub fn lowercase_fn(input: &str, _args: ()) -> String {
    LowerCase::stream_apply(input, ())
}

/// Function-style interface for reverse
pub fn reverse_fn(input: &str, _args: ()) -> String {
    Reverse::stream_apply(input, ())
}

/// Function-style interface for trim
pub fn trim_fn(input: &str, _args: ()) -> String {
    Trim::stream_apply(input, ())
}

/// Function-style interface for base64 encode
pub fn base64_encode_fn(input: &str, _args: ()) -> String {
    Base64Encode::stream_apply(input, ())
}

/// Function-style interface for base64 decode
pub fn base64_decode_fn(input: &str, _args: ()) -> String {
    Base64Decode::stream_apply(input, ())
}

/// Function-style interface for URL encode
pub fn url_encode_fn(input: &str, _args: ()) -> String {
    UrlEncode::stream_apply(input, ())
}

/// Function-style interface for URL decode
pub fn url_decode_fn(input: &str, _args: ()) -> String {
    UrlDecode::stream_apply(input, ())
}

// === UNIX-STYLE STREAMABLES ===




streamable!(Head(stdin, n: usize) => {
    stdin.lines().take(n).collect::<Vec<_>>().join("\n")
});

streamable!(Tail(stdin, n: usize) => {
    let lines: Vec<&str> = stdin.lines().collect();
    lines.iter().skip(lines.len().saturating_sub(n)).cloned().collect::<Vec<_>>().join("\n")
});

streamable!(Grep(stdin, pattern: String) => {
    stdin.lines()
        .filter(|line| line.contains(&pattern))
        .collect::<Vec<_>>()
        .join("\n")
});

streamable!(Sort(stdin,) => {
    let mut lines: Vec<&str> = stdin.lines().collect();
    lines.sort();
    lines.join("\n")
});

streamable!(Unique(stdin,) => {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    stdin.lines()
        .filter(|line| seen.insert(*line))
        .collect::<Vec<_>>()
        .join("\n")
});

streamable!(WordCount(stdin,) => {
    let lines = stdin.lines().count();
    let words = stdin.split_whitespace().count(); 
    let chars = stdin.chars().count();
    format!("{} {} {}", lines, words, chars)
});

// Note: Token-specific streamables moved to xstream/types/streamable
// RSB focuses on general text processing, xstream handles token semantics

// === RSB INTEGRATION STREAMABLES ===

streamable!(Sed(stdin, pattern: String, replacement: String) => {
    use crate::streams::Stream;
    Stream::from_string(stdin)
        .sed(&pattern, &replacement)
        .to_string()
});

streamable!(SedLines(stdin, start: usize, end: usize) => {
    use crate::streams::Stream;
    Stream::from_string(stdin)
        .sed_lines(start, end)
        .to_string()
});




// Advanced streamables
streamable!(Pipeline(stdin, commands: Vec<String>) => {
    let mut result = stdin.to_string();
    for cmd in commands {
        // This could parse and execute simple commands
        // For now, just return the result
        result = format!("# Executed: {}\n{}", cmd, result);
    }
    result
});




#[cfg(test)]
mod tests {
    use super::*;
    use crate::streamable::traits::StreamApply;

    #[test]
    fn test_basic_functions() {
        let result = "hello world"
            .stream_apply(Replace, ("world".to_string(), "rust".to_string()))
            .stream_apply(UpperCase, ());
        assert_eq!(result, "HELLO RUST");
    }

    #[test]
    fn test_function_style() {
        let result = replace_fn("hello world", ("world".to_string(), "rust".to_string()));
        assert_eq!(result, "hello rust");
        
        let result = uppercase_fn(&result, ());
        assert_eq!(result, "HELLO RUST");
    }
}
