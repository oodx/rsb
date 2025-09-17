// RSB Streamable - Unix-style function pipelines for Rust
// (Based on working XStream implementation)

pub mod detectors;
pub mod filters;
pub mod functions;
pub mod traits;

// Re-export commonly used items
pub use traits::{StreamApply, Streamable};

// Re-export basic streamable structs
pub use functions::{
    base64_decode_fn,
    base64_encode_fn,
    lowercase_fn,
    // Function-style interfaces
    replace_fn,
    reverse_fn,
    trim_fn,
    uppercase_fn,
    url_decode_fn,
    url_encode_fn,
    Base64Decode,
    // Encoding transforms
    Base64Encode,
    Grep,
    // Unix-style streamables
    Head,
    Length,
    LowerCase,
    // Basic text transforms
    Replace,
    Reverse,
    // RSB integration streamables
    Sed,
    SedLines,
    Sort,
    Tail,
    Trim,
    Unique,
    UpperCase,
    UrlDecode,
    UrlEncode,
    WordCount,
};

// Re-export filters
pub use filters::{
    FilterByLength, FilterDuplicates, FilterEmpty, FilterEndsWith, FilterLines, FilterNotContains,
    FilterRegex, FilterStartsWith, SkipLines, TakeLines,
};

// Re-export detectors
pub use detectors::{
    CountLines, CountPattern, CountWords, DetectAllMatch, DetectAnyMatch, DetectBinary,
    DetectDuplicates, DetectEmpty, DetectEncoding, DetectPattern, DetectRegex,
};

// Re-export the streamable! macro
pub use crate::streamable;
