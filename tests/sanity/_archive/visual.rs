use rsb::prelude::*;

#[test]
fn sanity_visual_color_basic() {
    // Test basic color functionality
    let red_text = rsb::visual::colorize("Hello", "red");
    assert!(!red_text.is_empty());
    assert!(red_text.contains("Hello"));

    let blue_text = rsb::visual::colorize("World", "blue");
    assert!(!blue_text.is_empty());
    assert!(blue_text.contains("World"));
}

#[test]
fn sanity_visual_background_colors() {
    // Test background color functionality
    let bg_red = rsb::visual::background("Text", "red");
    assert!(!bg_red.is_empty());
    assert!(bg_red.contains("Text"));

    let bg_blue = rsb::visual::background("Content", "blue");
    assert!(!bg_blue.is_empty());
    assert!(bg_blue.contains("Content"));
}

#[test]
fn sanity_visual_glyphs() {
    // Test glyph functionality
    let check_glyph = rsb::visual::glyph("check");
    assert!(!check_glyph.is_empty());

    let error_glyph = rsb::visual::glyph("error");
    assert!(!error_glyph.is_empty());

    let warning_glyph = rsb::visual::glyph("warning");
    assert!(!warning_glyph.is_empty());
}

#[test]
fn sanity_visual_style_combinations() {
    // Test combining multiple visual styles
    let styled = rsb::visual::style_text("Important", &["bold", "red"]);
    assert!(!styled.is_empty());
    assert!(styled.contains("Important"));

    let complex_style = rsb::visual::style_text("Notice", &["underline", "yellow", "bold"]);
    assert!(!complex_style.is_empty());
    assert!(complex_style.contains("Notice"));
}

#[test]
fn sanity_visual_reset() {
    // Test color/style reset functionality
    let colored = rsb::visual::colorize("Colored", "green");
    let with_reset = format!("{}{}", colored, rsb::visual::reset());

    assert!(with_reset.contains("Colored"));
    assert!(!with_reset.is_empty());
}

#[test]
fn sanity_visual_progress_indicators() {
    // Test progress indicator visuals
    let spinner = rsb::visual::spinner_frame(0);
    assert!(!spinner.is_empty());

    let different_spinner = rsb::visual::spinner_frame(1);
    assert!(!different_spinner.is_empty());

    // Test progress bar visual
    let progress_bar = rsb::visual::progress_bar(50, 100);
    assert!(!progress_bar.is_empty());
    assert!(progress_bar.contains("50") || progress_bar.contains("█") || progress_bar.contains("▓"));
}

#[test]
fn sanity_visual_status_indicators() {
    // Test status indicator visuals
    let success = rsb::visual::status("success");
    assert!(!success.is_empty());

    let failure = rsb::visual::status("failure");
    assert!(!failure.is_empty());

    let pending = rsb::visual::status("pending");
    assert!(!pending.is_empty());

    let warning = rsb::visual::status("warning");
    assert!(!warning.is_empty());
}

#[test]
fn sanity_visual_borders_frames() {
    // Test border and frame functionality
    let bordered = rsb::visual::border("Content", "single");
    assert!(!bordered.is_empty());
    assert!(bordered.contains("Content"));

    let framed = rsb::visual::frame("Title", "Framed content");
    assert!(!framed.is_empty());
    assert!(framed.contains("Title"));
    assert!(framed.contains("Framed content"));
}

#[test]
fn sanity_visual_disable_colors() {
    // Test color disabling functionality
    rsb::visual::disable_colors();

    let should_be_plain = rsb::visual::colorize("Plain", "red");
    assert_eq!(should_be_plain, "Plain"); // Should be unmodified when colors disabled

    // Re-enable for other tests
    rsb::visual::enable_colors();

    let should_be_colored = rsb::visual::colorize("Colored", "red");
    assert!(should_be_colored.contains("Colored"));
}