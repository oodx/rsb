//! Sanity test wrapper for the colors module
#![cfg(feature = "colors-core")]

// Reuse the shared sanity implementations in tests/sanity/colors.rs
#[path = "sanity/colors.rs"]
mod colors;
