// moved from tests/uat_visual.rs
use rsb::prelude::*;
use rsb::visual::colors::{color_enable_with, color_mode, color, bg, colored, colorize, colorize_bg};
#[cfg(feature = "glyphs")]
use rsb::visual::glyphs::glyph_enable;

#[test]
fn uat_visual_combo_demo() {
    color_mode("always");
    color_enable_with("simple,status,named,bg,glyphs");
    #[cfg(feature = "glyphs")]
    glyph_enable();
    println!("\n=== UAT: Visual Combo (BG + Color + Glyphs) ===\n");
    let samples = vec![
        "{bg:crimson}{white}{g:fail} ERROR{reset}",
        "{bg:amber}{black}{g:delta} WARN{reset}",
        "{bg:emerald}{black}{g:pass} OKAY{reset}",
        "{bg:azure}{black}{g:info} INFO{reset}",
        "{bg:steel}{white}{g:bolt} DEBUG{reset}",
        "{bg:slate}{white}{g:dots} TRACE{reset}",
        "{bg:gold}{black}{g:box} BOX{reset}",
    ];
    for s in samples { println!("{}", colored(s)); }
    let line = format!("{}{} {}{}", bg("amber"), color("black"), "HELPER-BG", color("reset") );
    println!("{}", line);
    println!("{}{}{}", color("red"), "FG-ONLY", color("reset"));
    println!("{}", colorize_bg(&colorize("GLYPH", "white"), "violet"));
}

