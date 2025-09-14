// moved from tests/uat_colors_macros.rs
use rsb::visual::colors::{color_enable_with, color_mode};
use rsb::colored;

#[test]
fn uat_colored_macro_inline_tags() {
    color_mode("always");
    color_enable_with("simple,status,named,bg");
    let s = colored!("{red}hello{reset}");
    assert!(s.contains("hello"));
    assert!(s.starts_with("\x1B["));
    assert!(s.ends_with("\x1B[0m"));
    let s2 = colored!("{bg:amber}{black}hi{reset}");
    assert!(s2.contains("hi"));
    let s3 = colored!("pre {unknown_tag} post");
    assert!(s3.contains("{unknown_tag}"));
    let up = colored!("{RED}X{reset}");
    let lo = colored!("{red}X{reset}");
    assert_eq!(up, lo);
}

#[test]
fn uat_colored_macro_with_glyphs_optional() {
    color_mode("always");
    color_enable_with("simple,status,bg,glyphs");
    let line = colored!("{green}{g:pass}{reset} PASS");
    assert!(line.contains("PASS"));
    assert!(line.contains("\x1B[0m"));
}

