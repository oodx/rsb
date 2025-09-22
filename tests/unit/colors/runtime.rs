#![cfg(all(
    feature = "colors-simple",
    feature = "colors-status",
    feature = "colors-named"
))]
// moved from tests/sanity-colors-runtime.rs
#[cfg(feature = "visual")]
use rsb::colored;
use rsb::colors::get_all_colors;
use rsb::colors::{bg, color, color_enable_with, color_mode};

#[test]
fn bg_disabled_by_default() {
    color_mode("always");
    color_enable_with("simple,status,named");
    assert_eq!(bg("red"), "");
}

#[test]
fn env_no_color_disables_output() {
    std::env::set_var("NO_COLOR", "1");
    assert_eq!(color("red"), "");
    std::env::remove_var("NO_COLOR");
}

#[test]
fn env_rsb_color_never_disables_output() {
    std::env::set_var("RSB_COLOR", "never");
    assert_eq!(color("green"), "");
    std::env::remove_var("RSB_COLOR");
}

#[test]
fn color_mode_never_disables_output() {
    color_mode("never");
    assert_eq!(color("blue"), "");
    color_mode("always");
}

#[cfg(feature = "visual")]
#[test]
fn glyph_tag_kept_when_not_enabled() {
    let s = colored!("pre {g:pass} post");
    assert!(s.contains("{g:pass}"));
}

#[test]
fn case_insensitive_lookup_direct() {
    color_mode("always");
    color_enable_with("simple");
    assert_eq!(color("RED"), color("red"));
}

#[test]
fn registry_count_when_enabled() {
    color_mode("always");
    color_enable_with("simple,status,named");
    let all = get_all_colors();
    assert!(all.len() >= 90);
}
