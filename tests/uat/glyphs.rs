// moved from tests/uat_glyphs.rs
use rsb::visual::colors::{color, color_enable_with, color_mode, colored};
#[cfg(feature = "glyphs")]
use rsb::visual::glyphs::{get_all_glyphs, glyph, glyph_enable};

#[test]
fn uat_visual_glyphs_demo() {
    color_mode("always");
    color_enable_with("simple,status");
    #[cfg(feature = "glyphs")]
    {
        glyph_enable();
        println!("\n=== UAT: RSB Glyphs Demo ===\n");
        let all = get_all_glyphs();
        for (name, ch) in all.iter().take(16) {
            println!(
                "{:>10}: {} (U+{:X})",
                name,
                ch,
                ch.chars().next().unwrap() as u32
            );
        }
        println!("\n-- Inline --");
        println!("{}", colored("{green}{g:pass}{reset} Success  {yellow}{g:flag_on}{reset} Flag  {red}{g:fail}{reset} Fail"));
        println!("{}", colored("{cyan}{g:info}{reset} Info {magenta}{g:bolt}{reset} Bolt {blue}{g:return}{reset} Return"));
        println!(
            "\nDirect: pass='{}' info='{}'",
            glyph("pass"),
            glyph("info")
        );
        println!(
            "Colors with glyph: {}{}{}",
            color("green"),
            glyph("pass"),
            color("reset")
        );
    }
}
