//! Runtime color registry and ergonomic API
//!
//! String-first, progressive enhancement interface:
//! - color_enable("simple,status,named")
//! - color("red") -> &str
//! - colorize("Hello", "red") -> String
//! - colored("Hello {red}world{reset}") -> String
//! - color_mode("auto|always|never")

use std::collections::HashMap;
use std::sync::RwLock;

use crate::global::get_var;

use super::simple::{SimpleColor, RESET};
#[cfg(feature = "colors-status")]
use super::status::StatusColor;
#[cfg(feature = "colors-named")]
use super::named;

use super::util::{colors_enabled, set_color_mode, set_backgrounds_enabled, backgrounds_enabled};
#[cfg(feature = "glyphs")]
use crate::visual::glyphs::{glyph, set_glyphs_enabled, glyphs_enabled};

use lazy_static::lazy_static;

lazy_static! {
    static ref REGISTRY: RwLock<HashMap<String, &'static str>> = RwLock::new(HashMap::new());
    static ref LOADED: RwLock<Loaded> = RwLock::new(Loaded::default());
}

#[derive(Clone, Copy, Default)]
struct Loaded {
    simple: bool,
    status: bool,
    named: bool,
}

fn ensure_initialized() {
    // If nothing is loaded yet, use env/context or fallback to "simple"
    if { let l = LOADED.read().unwrap(); !l.simple && !l.status && !l.named } {
        color_enable();
    }
}

fn merge_kv(pairs: &[(&'static str, &'static str)]) {
    let mut map = REGISTRY.write().unwrap();
    for (k, v) in pairs {
        map.insert((*k).to_string(), *v);
    }
}

fn load_simple() {
    let mut loaded = LOADED.write().unwrap();
    if loaded.simple { return; }
    let mut pairs: Vec<(&'static str, &'static str)> = Vec::new();
    for c in SimpleColor::all() { pairs.push((c.name(), c.code())); }
    merge_kv(&pairs);
    loaded.simple = true;
}

#[cfg(feature = "colors-status")]
fn load_status() {
    let mut loaded = LOADED.write().unwrap();
    if loaded.status { return; }
    let mut pairs: Vec<(&'static str, &'static str)> = Vec::new();
    for c in StatusColor::all() { pairs.push((c.name(), c.code())); }
    merge_kv(&pairs);
    loaded.status = true;
}

#[cfg(not(feature = "colors-status"))]
fn load_status() { /* not compiled in */ }

#[cfg(feature = "colors-named")]
fn load_named() {
    let mut loaded = LOADED.write().unwrap();
    if loaded.named { return; }
    // Build from known named list to avoid duplication
    let names: &[&str] = &[
        // Red spectrum
        "crimson","ruby","coral","salmon","rose","brick",
        // Orange spectrum
        "amber","tangerine","peach","rust","bronze","gold",
        // Yellow spectrum
        "lemon","mustard","sand","cream","khaki",
        // Green spectrum
        "lime","emerald","forest","mint","sage","jade","olive",
        // Blue spectrum
        "azure","navy","royal","ice","steel","teal","indigo",
        // Purple spectrum
        "violet","plum","lavender","orchid","mauve","amethyst",
        // Cyan spectrum
        "aqua","turquoise","sky","ocean",
        // Monochrome
        "charcoal","slate","silver","pearl","snow",
        // Legacy
        "red2","deep","deep_green","orange","green2","blue2","purple","purple2","white2","grey2","grey3",
        // Priority
        "critical","high","medium","low","trivial",
        // Bright variants
        "bright_red","bright_green","bright_yellow","bright_blue","bright_magenta","bright_cyan",
        // Dim variants
        "dim_red","dim_green","dim_yellow","dim_blue","dim_magenta","dim_cyan",
        // Pastel variants
        "pastel_red","pastel_green","pastel_yellow","pastel_blue","pastel_purple","pastel_orange",
    ];
    let mut pairs: Vec<(&'static str, &'static str)> = Vec::new();
    for &n in names {
        let code = named::get_named_color(n);
        if !code.is_empty() {
            pairs.push((n, code));
        }
    }
    merge_kv(&pairs);
    loaded.named = true;
}

#[cfg(not(feature = "colors-named"))]
fn load_named() { /* not compiled in */ }

fn parse_sets(spec: &str) -> (bool, bool, bool) {
    let mut s = false; // simple
    let mut n = false; // named
    let mut t = false; // status
    for part in spec.split(',').map(|p| p.trim().to_ascii_lowercase()) {
        match part.as_str() {
            "simple" => s = true,
            "named" => n = true,
            "status" => t = true,
            "all" => { s = true; n = true; t = true; set_backgrounds_enabled(true); #[cfg(feature = "glyphs")] set_glyphs_enabled(true); }
            // Backgrounds toggle keywords
            "bg" | "background" | "backgrounds" | "on" => set_backgrounds_enabled(true),
            // Glyph toggle keyword
            #[cfg(feature = "glyphs")] "glyphs" => set_glyphs_enabled(true),
            _ => {}
        }
    }
    (s, t, n)
}

fn color_enable_internal(spec: Option<&str>) {
    let wanted = spec
        .map(|s| s.to_string())
        .or_else(|| {
            let ctx = get_var("opt_colors");
            if !ctx.is_empty() { Some(ctx) } else { None }
        })
        .or_else(|| std::env::var("RSB_COLORS").ok())
        .unwrap_or_else(|| "simple".to_string());

    let (want_simple, want_status, want_named) = parse_sets(&wanted);
    if want_simple { load_simple(); }
    if want_status { load_status(); }
    if want_named { load_named(); }
}

/// Enable colors using environment/context or the default ("simple").
pub fn color_enable() { color_enable_internal(None); }

/// Enable colors by explicit spec (e.g., "simple,status", "named", "all").
pub fn color_enable_with(spec: &str) { color_enable_internal(Some(spec)); }

/// Set color mode: "auto" | "always" | "never".
pub fn color_mode(mode: &str) { set_color_mode(mode); }

/// Get a color code by name, or "" if not found/disabled.
pub fn color(name: &str) -> &'static str {
    ensure_initialized();
    if !colors_enabled() { return ""; }
    let n = name.to_ascii_lowercase();
    if let Some(v) = REGISTRY.read().unwrap().get(&n) { *v } else { "" }
}

/// Back-compat alias for color(name)
pub fn get_color(name: &str) -> &'static str { color(name) }

/// Colorize text with a named color; returns plain text if disabled/unknown.
pub fn colorize(text: &str, name: &str) -> String {
    let code = color(name);
    if code.is_empty() { text.to_string() } else { format!("{}{}{}", code, text, RESET) }
}

fn fg_to_bg(code: &str) -> Option<String> {
    if !code.starts_with("\x1B[") { return None; }
    // 256-color form: ESC[38;5;Nm -> ESC[48;5;Nm
    if code.contains("38;5;") {
        // replace the first occurrence only
        return Some(code.replacen("38;5;", "48;5;", 1));
    }
    // Simple 8/16 colors: 30-37 -> 40-47, 90-97 -> 100-107
    if code.len() >= 5 && code.ends_with('m') {
        // extract the number sequence inside ESC[ ... m
        let seq = &code[2..code.len()-1];
        // try parse as integer
        if let Ok(num) = seq.parse::<i32>() {
            if (30..=37).contains(&num) {
                return Some(format!("\x1B[{}m", num + 10));
            }
            if (90..=97).contains(&num) {
                return Some(format!("\x1B[{}m", num + 10));
            }
        }
    }
    None
}

/// Background code for a color name. Returns empty when disabled/unknown.
pub fn bg(name: &str) -> String {
    ensure_initialized();
    if !colors_enabled() || !backgrounds_enabled() { return String::new(); }
    let fg = color(name);
    if fg.is_empty() { String::new() } else { fg_to_bg(fg).unwrap_or_default() }
}

/// Colorize with background color by name.
pub fn colorize_bg(text: &str, name: &str) -> String {
    let code = bg(name);
    if code.is_empty() { text.to_string() } else { format!("{}{}{}", code, text, RESET) }
}

/// Replace {color} tokens inline. Unknown tags are kept verbatim.
pub fn colored(s: &str) -> String {
    ensure_initialized();
    let mut out = String::with_capacity(s.len() + 8);
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '{' {
            // capture until next '}'
            let mut name = String::new();
            while let Some(&c) = chars.peek() {
                chars.next();
                if c == '}' { break; }
                name.push(c);
            }
            if name == "reset" {
                out.push_str(RESET);
            } else if name.starts_with("bg:") && backgrounds_enabled() {
                let key = &name[ name.find(':').unwrap() + 1 .. ];
                let code = bg(key);
                if code.is_empty() { out.push('{'); out.push_str(&name); out.push('}'); }
                else { out.push_str(&code); }
            } else if name.starts_with("g:") {
                #[cfg(feature = "glyphs")]
                {
                    let key = &name[ name.find(':').unwrap() + 1 .. ];
                    let g = if glyphs_enabled() { glyph(key) } else { "" };
                    if g.is_empty() { out.push('{'); out.push_str(&name); out.push('}'); } else { out.push_str(g); }
                }
                #[cfg(not(feature = "glyphs"))]
                {
                    out.push('{'); out.push_str(&name); out.push('}');
                }
            } else {
                let code = color(&name);
                if code.is_empty() { out.push('{'); out.push_str(&name); out.push('}'); }
                else { out.push_str(code); }
            }
        } else { out.push(ch); }
    }
    out
}

/// Return a vector of all registered colors (name, code).
pub fn get_all_colors() -> Vec<(String, &'static str)> {
    ensure_initialized();
    let mut v: Vec<(String, &'static str)> = REGISTRY
        .read().unwrap()
        .iter()
        .map(|(k, &v)| (k.clone(), v))
        .collect();
    v.sort_by(|a, b| a.0.cmp(&b.0));
    v
}
