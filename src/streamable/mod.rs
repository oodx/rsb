// RSB Streamable - Unix-style function pipelines for Rust
// (Based on working XStream implementation)

pub mod traits;
pub mod functions;
pub mod filters;
pub mod detectors;

// Re-export commonly used items
pub use traits::{Streamable, StreamApply};

// Re-export basic streamable structs
pub use functions::{
    // Basic text transforms
    Replace, UpperCase, LowerCase, Trim, Reverse, Length,
    // Encoding transforms
    Base64Encode, Base64Decode, UrlEncode, UrlDecode,
    // Unix-style streamables
    Head, Tail, Grep, Sort, Unique, WordCount,
    // RSB integration streamables
    Sed, SedLines,
    // Function-style interfaces
    replace_fn, uppercase_fn, lowercase_fn, reverse_fn, trim_fn,
    base64_encode_fn, base64_decode_fn, url_encode_fn, url_decode_fn,
};

// Re-export filters
pub use filters::{
    FilterLines, FilterEmpty, FilterByLength, FilterRegex, FilterNotContains,
    FilterDuplicates, FilterStartsWith, FilterEndsWith, TakeLines, SkipLines,
};

// Re-export detectors
pub use detectors::{
    DetectEmpty, DetectPattern, DetectBinary, DetectRegex, DetectDuplicates,
    DetectEncoding, CountPattern, CountLines, CountWords, DetectAllMatch, DetectAnyMatch,
};

// Re-export the streamable! macro
pub use crate::streamable;
