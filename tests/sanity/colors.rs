//! Sanity tests for the colors package
#![cfg(feature = "colors-core")]

use rsb::colors::{
    color, color_enable_with, color_mode, colorize, get_all_colors,
};

#[test]
fn sanity_colors_simple_palette_is_available() {
    color_mode("always");
    color_enable_with("simple");

    let red = color("red");
    assert!(!red.is_empty(), "expected basic palette color to be registered");

    let reset = color("reset");
    assert!(!reset.is_empty(), "expected reset color to be registered");

    let rendered = colorize("hello", "green");
    assert!(rendered.contains("hello"), "rendered text should include original content");
    assert_ne!(rendered, "hello", "rendered text should include color escape codes");
}

#[cfg(feature = "colors-status")]
#[test]
fn sanity_colors_status_palette_is_available() {
    color_mode("always");
    color_enable_with("status");

    let warning = color("warning");
    assert!(!warning.is_empty(), "expected status palette color to be registered");
}

#[cfg(feature = "colors-named")]
#[test]
fn sanity_colors_named_palette_is_available() {
    color_mode("always");
    color_enable_with("named");

    let crimson = color("crimson");
    assert!(!crimson.is_empty(), "expected named palette color to be registered");
}

#[test]
fn sanity_colors_registry_reports_entries() {
    color_mode("always");
    color_enable_with("simple,status,named");

    let all = get_all_colors();
    assert!(all.len() >= 8, "expected registry to contain at least the basic palette");
}
