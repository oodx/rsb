// RSB Streamable - Unix pipe patterns for Rust functions
// (Based on working XStream implementation)

/// Streamable trait - functions that take stdin + args and produce stdout
/// The unix pipe pattern generalized for any function
pub trait Streamable {
    type Args;
    
    /// Apply this streamable function to stdin with given args
    fn stream_apply(stdin: &str, args: Self::Args) -> String;
}

/// StreamApply trait - adds .stream_apply() method to any type
pub trait StreamApply {
    fn stream_apply<S: Streamable>(self, streamable: S, args: S::Args) -> String;
}
// We'll implement this for specific types that need it!

// Implementation for String - enables direct string pipeline usage
impl StreamApply for String {
    fn stream_apply<S: Streamable>(self, _streamable: S, args: S::Args) -> String {
        S::stream_apply(&self, args)
    }
}

// Implementation for &str convenience
impl StreamApply for &str {
    fn stream_apply<S: Streamable>(self, _streamable: S, args: S::Args) -> String {
        S::stream_apply(self, args)
    }
}
