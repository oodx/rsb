//! RSB Glyphs Package - Optional unicode glyph lookups
//!
//! String-first, case-insensitive glyphs with a tiny footprint.
//! Enable at runtime via `glyph_enable()` or include "glyphs" in visuals features.

use std::sync::atomic::{AtomicBool, Ordering};

static GLYPHS_ENABLED: AtomicBool = AtomicBool::new(false);

// Full set ported from src/ref/glyphs.rs (case-insensitive names lowered)
const GLYPHS: &[(&str, &str)] = &[
    // General & Common
    ("usage", "\u{2756}"),
    ("cmdr", "\u{2318}"),
    ("boto", "\u{232C}"),
    ("gear", "\u{26ED}"),
    ("info", "\u{25CE}"),
    ("ellipsis", "\u{2026}"),

    // Status
    ("pass", "\u{2713}"),
    ("fail", "\u{2715}"),
    ("mark", "\u{292C}"),
    ("dots", "\u{2026}"),
    ("flag_off", "\u{2690}"),
    ("flag_on", "\u{2691}"),
    ("bolt", "\u{21AF}"),
    ("anchor", "\u{2693}"),
    ("unlock", "\u{26BF}"),

    // Bullets & Pointers
    ("bullet", "\u{2022}"),
    ("dot", "\u{2219}"),
    ("target", "\u{25CE}"),
    ("radio_on", "\u{25C9}"),
    ("radio_off", "\u{25CB}"),
    ("square_small", "\u{25AB}"),
    ("pointer", "\u{25B6}"),

    // Arrows
    ("up", "\u{2191}"),
    ("down", "\u{2193}"),
    ("right", "\u{2192}"),
    ("left", "\u{2190}"),
    ("heavy_arrow_right", "\u{279C}"),
    ("down_arr", "\u{21B3}"),
    ("up_arr", "\u{21B1}"),
    ("arrow_sw", "\u{2B0E}"),
    ("arrow_curve_se", "\u{2BA7}"),
    ("curve_arrow_left", "\u{21B6}"),
    ("uarr", "\u{21B0}"),
    ("return", "\u{21A9}"),
    ("newline", "\u{21B2}"),

    // Actions
    ("undo", "\u{238C}"),
    ("recover", "\u{27F2}"),
    ("redo_closed", "\u{27F3}"),

    // Time & Date
    ("clock", "\u{23F1}"),
    ("timer", "\u{23F2}"),
    ("hourglass", "\u{29D6}"),
    ("calendar", "\u{1F5D3}"),

    // Fun & Misc
    ("rook", "\u{265C}"),
    ("pawn", "\u{265F}"),
    ("king", "\u{265A}"),
    ("queen", "\u{265B}"),
    ("tri_down", "\u{25BD}"),
    ("delta", "\u{25B3}"),
    ("star", "\u{2605}"),
    ("snek", "\u{264B}"),
    ("diamond", "\u{16DC}"),
    ("u_spark", "\u{27E1}"),
    ("sword", "\u{2694}"),
    ("moon", "\u{263E}"),
    ("sun", "\u{2600}"),
    ("spark", "\u{273B}"),
    ("colon2", "\u{2237}"),
    ("therefore", "\u{2234}"),
    ("bullseye", "\u{29BF}"),
    ("sect", "\u{00A7}"),
    ("bowtie", "\u{22C8}"),
    ("sum", "\u{2211}"),
    ("prod", "\u{220F}"),
    ("dharma", "\u{2638}"),
    ("scroll", "\u{07F7}"),
    ("note", "\u{266A}"),
    ("spindle", "\u{27D0}"),
    ("anote", "\u{260D}"),

    // Greek Letters
    ("alpha", "\u{03B1}"),("beta", "\u{03B2}"),("gamma", "\u{03B3}"),("delta_sm", "\u{03B4}"),
    ("epsilon", "\u{03B5}"),("zeta", "\u{03B6}"),("eta", "\u{03B7}"),("theta", "\u{03B8}"),
    ("iota", "\u{03B9}"),("kappa", "\u{03BA}"),("lambda", "\u{03BB}"),("mu", "\u{03BC}"),
    ("nu", "\u{03BD}"),("xi", "\u{03BE}"),("omicron", "\u{03BF}"),("pi", "\u{03C0}"),
    ("rho", "\u{03C1}"),("sigma", "\u{03C3}"),("tau", "\u{03C4}"),("upsilon", "\u{03C5}"),
    ("phi", "\u{03C6}"),("chi", "\u{03C7}"),("psi", "\u{03C8}"),("omega", "\u{03C9}"),

    // Box Drawing
    ("hline", "\u{2500}"),
    ("vline", "\u{2502}"),
    ("t_right", "\u{251C}"),
    ("corner_ur", "\u{2514}"),

    // Extra: solid box requested
    ("box", "\u{25A0}"),
];

pub fn glyph_enable() { GLYPHS_ENABLED.store(true, Ordering::SeqCst); }
pub fn set_glyphs_enabled(enable: bool) { GLYPHS_ENABLED.store(enable, Ordering::SeqCst); }
pub fn glyphs_enabled() -> bool { GLYPHS_ENABLED.load(Ordering::SeqCst) }

/// Lookup a glyph by name (case-insensitive). Returns empty string if disabled/unknown.
pub fn glyph(name: &str) -> &'static str {
    if !glyphs_enabled() { return ""; }
    let key = name.to_ascii_lowercase();
    for (n, v) in GLYPHS.iter() {
        if *n == key { return v; }
    }
    ""
}

pub fn get_all_glyphs() -> Vec<(String, &'static str)> {
    GLYPHS.iter().map(|(k, v)| (k.to_string(), *v)).collect()
}
