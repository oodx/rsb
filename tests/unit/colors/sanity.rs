#![cfg(feature = "visual")]
// moved from tests/sanity-colors.rs
use rsb::prelude::*;
use rsb::visual::colors::{get_color, colorize, get_all_colors, color_enable_with, color_mode};

fn ensure_colors_enabled() {
    color_mode("always");
    color_enable_with("simple,status,named");
}

#[test]
fn test_basic_colors_exist() {
    ensure_colors_enabled();
    let basic_colors = ["red", "green", "blue", "yellow", "magenta", "cyan", "black", "white", "grey"];    
    for color in &basic_colors {
        let escape_code = get_color(color);
        assert!(!escape_code.is_empty());
        assert!(escape_code.starts_with('\x1B'));
    }
}

#[test]
fn test_status_colors_exist() {
    ensure_colors_enabled();
    let status_colors = ["magic", "trace", "note", "silly", "okay", "warn"];
    for color in &status_colors {
        let escape_code = get_color(color);
        assert!(!escape_code.is_empty());
        assert!(escape_code.starts_with('\x1B'));
    }
    let magic = get_color("magic");
    assert!(!magic.is_empty());
}

#[test]
fn test_extended_colors_exist() {
    ensure_colors_enabled();
    let extended_colors = ["crimson", "emerald", "azure", "purple", "amber", "lime", "orange", "coral", "bronze", "navy", "teal", "indigo"];    
    for color in &extended_colors {
        let escape_code = get_color(color);
        assert!(!escape_code.is_empty());
        assert!(escape_code.starts_with('\x1B'));
    }
}

#[test]
fn test_color_count() {
    ensure_colors_enabled();
    let all_colors = get_all_colors();
    assert!(all_colors.len() >= 90);
}

#[test]
fn test_colorize_function() {
    ensure_colors_enabled();
    let red_text = colorize("Hello", "red");
    assert!(red_text.contains("Hello"));
    assert!(red_text.starts_with('\x1B'));
    assert!(red_text.ends_with("\x1B[0m"));
    let magic_text = colorize("World", "magic");
    assert!(magic_text.contains("World"));
    assert!(magic_text != red_text);
}

#[test]
fn test_nonexistent_color() {
    ensure_colors_enabled();
    let fake_color = get_color("nonexistent_color_12345");
    assert!(fake_color.is_empty());
    let fake_colored = colorize("test", "nonexistent_color_12345");
    assert_eq!(fake_colored, "test");
}

#[test]
fn test_color_consistency() {
    ensure_colors_enabled();
    let red1 = get_color("red");
    let red2 = get_color("red");
    assert_eq!(red1, red2);
    let upper_red = get_color("RED");
    let upper_red2 = get_color("RED");
    assert_eq!(upper_red, upper_red2);
}

#[test]
fn test_global_context_integration() {
    ensure_colors_enabled();
    let red_escape = get_color("red");
    assert!(!red_escape.is_empty());
    let colors_to_test = ["red", "green", "blue", "magic", "crimson"];
    for color in &colors_to_test {
        let escape1 = get_color(color);
        let escape2 = get_color(color);
        assert_eq!(escape1, escape2);
    }
}

#[test]
fn test_specific_escape_codes() {
    ensure_colors_enabled();
    let red = get_color("red");
    assert!(red.contains("[31m") || red.contains("[38;5;") || red.contains("[38;2;"));
    let reset = "\x1B[0m";
    let colored = colorize("test", "red");
    assert!(colored.ends_with(reset));
}

#[test]
fn test_color_names_sampling() {
    ensure_colors_enabled();
    let sample_colors = ["red", "green", "blue", "crimson", "emerald", "azure", "magic", "trace", "okay", "lightred", "darkblue", "brightgreen"];    
    let mut found_colors = 0;
    for color in &sample_colors {
        let escape = get_color(color);
        if !escape.is_empty() { found_colors += 1; }
    }
    assert!(found_colors >= sample_colors.len() / 2);
}

