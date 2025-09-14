// moved from tests/uat_colors.rs
use rsb::visual::colors::{color_enable_with, color_mode, color, colorize, colored, get_all_colors, bg, colorize_bg};

#[test]
fn uat_colors_demo() {
    color_mode("always");
    color_enable_with("simple,status,named,bg");
    println!("\n=== UAT: RSB Colors Demo ===\n");
    let basics = ["red","green","yellow","blue","magenta","cyan","white","black"];
    println!("-- Simple Colors --");
    for name in basics.iter() { println!("{:>10}: {}sample{}", name, color(name), color("reset")); }
    let status = ["magic","trace","note","silly","error","warning","success","info"];
    println!("\n-- Status Colors --");
    for name in status.iter() { println!("{:>10}: {}status{}", name, color(name), color("reset")); }
    let named = ["crimson","emerald","azure","amber","violet","aqua","slate","gold"];
    println!("\n-- Named Colors --");
    for name in named.iter() { println!("{:>10}: {}named{}", name, color(name), color("reset")); }
    let line = "Message: {info}info{reset} / {warning}warn{reset} / {error}error{reset}";
    println!("\n-- Inline Tags --\n{}", colored(line));
    println!("\n-- Helper --\n{}", colorize("Hello", "magic"));
    println!("\n-- Backgrounds (enabled) --");
    for name in basics.iter() { let b = bg(name); if !b.is_empty() { println!("{:>10}: {} bg {} ", name, b, color("reset")); } }
    println!("\n{}BG sample{} {}\n", bg("crimson"), colorize_bg("with bg", "amber"), color("reset"));
    let all = get_all_colors();
    println!("\nTotal registered colors: {}\n", all.len());
}

