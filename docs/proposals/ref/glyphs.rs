//! esc/glyphs_strum.rs
//! note enum != strum
//! A curated collection of Unicode (non-emoji) glyphs for use in terminal output.
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumIter};



/// A comprehensive, iterable enum of Unicode glyphs for UI design.
#[derive(Debug, Clone, Copy, EnumIter, Display, AsRefStr, PartialEq, Eq)]
pub enum Glyph {
    // --- General & Common ---
    #[strum(to_string = "\u{2756}")] Usage,
    #[strum(to_string = "\u{2318}")] Cmdr,
    #[strum(to_string = "\u{232C}")] Boto,
    #[strum(to_string = "\u{26ED}")] Gear,
    #[strum(to_string = "\u{25CE}")] Info, //no emojis  ◎ ⦿, U+29BF
    #[strum(to_string = "\u{2026}")] Ellipsis,

    // --- Status ---
    #[strum(to_string = "\u{2713}")] Pass,
    #[strum(to_string = "\u{2715}")] Fail,
    #[strum(to_string = "\u{292C}")] Mark,

    #[strum(to_string = "\u{2026}")] Dots,
    #[strum(to_string = "\u{2690}")] FlagOff,
    #[strum(to_string = "\u{2691}")] FlagOn,
    #[strum(to_string = "\u{21AF}")] Bolt,
    #[strum(to_string = "\u{2693}")] Anchor,
    #[strum(to_string = "\u{26BF}")] Unlock,


    // --- Bullets & Pointers ---
    #[strum(to_string = "\u{2022}")] Bullet,
    #[strum(to_string = "\u{2219}")] Dot,
    #[strum(to_string = "\u{25CE}")] Target,
    #[strum(to_string = "\u{25C9}")] RadioOn,
    #[strum(to_string = "\u{25CB}")] RadioOff,
    #[strum(to_string = "\u{25AB}")] SquareSmall,
    #[strum(to_string = "\u{25B6}")] Pointer,

    // --- Arrows ---
    #[strum(to_string = "\u{2191}")] Up,
    #[strum(to_string = "\u{2193}")] Down,
    #[strum(to_string = "\u{2192}")] Right,
    #[strum(to_string = "\u{2190}")] Left,
    #[strum(to_string = "\u{279C}")] HeavyArrowRight,
    #[strum(to_string = "\u{21B3}")] DownArr,
    #[strum(to_string = "\u{21B1}")] UpArr,
    #[strum(to_string = "\u{2B0E}")] ArrowSw,
    #[strum(to_string = "\u{2BA7}")] ArrowCurveSe,
    #[strum(to_string = "\u{21B6}")] CurveArrowLeft,
    #[strum(to_string = "\u{21B0}")] Uarr, // Upwards arrow with tip leftwards
    #[strum(to_string = "\u{21A9}")] ReturnSymbol,
    #[strum(to_string = "\u{21B2}")] NewlineSymbol,

    // --- Actions ---
    #[strum(to_string = "\u{238C}")] Undo,
    #[strum(to_string = "\u{27F2}")] Recover, //
    #[strum(to_string = "\u{27F3}")] RedoClosed,

    // --- Time & Date ---
    #[strum(to_string = "\u{23F1}")] Clock,
    #[strum(to_string = "\u{23F2}")] Timer,
    #[strum(to_string = "\u{29D6}")] Hourglass,
    #[strum(to_string = "\u{1F5D3}")] Calendar,

    // --- Fun & Miscellaneous ---
    #[strum(to_string = "\u{265C}")] Rook,
    #[strum(to_string = "\u{265F}")] Pawn,
    #[strum(to_string = "\u{265A}")] King,
    #[strum(to_string = "\u{265B}")] Queen,
    #[strum(to_string = "\u{25BD}")] TriDown,
    #[strum(to_string = "\u{25B3}")] Delta,
    #[strum(to_string = "\u{2605}")] Star,
    #[strum(to_string = "\u{264B}")] Snek,
    #[strum(to_string = "\u{16DC}")] Diamond,
    #[strum(to_string = "\u{27E1}")] USpark,
    #[strum(to_string = "\u{2694}")] Sword,
    #[strum(to_string = "\u{263E}")] Moon,
    #[strum(to_string = "\u{2600}")] Sun,
    #[strum(to_string = "\u{273B}")] Spark,
    #[strum(to_string = "\u{2237}")] Colon2,
    #[strum(to_string = "\u{2234}")] Therefore,
    #[strum(to_string = "\u{29BF}")] Bullseye,
    #[strum(to_string = "\u{00A7}")] Sect,
    #[strum(to_string = "\u{22C8}")] Bowtie,
    #[strum(to_string = "\u{2211}")] Sum,
    #[strum(to_string = "\u{220F}")] Prod,
    #[strum(to_string = "\u{2638}")] Dharma,
    #[strum(to_string = "\u{07F7}")] Scroll,
    #[strum(to_string = "\u{266A}")] Note,
    #[strum(to_string = "\u{27D0}")] Spindle,
    #[strum(to_string = "\u{260D}")] Anote,

    // --- Greek Letters ---
    #[strum(to_string = "\u{03B1}")] Alpha,
    #[strum(to_string = "\u{03B2}")] Beta,
    #[strum(to_string = "\u{03B3}")] Gamma,
    #[strum(to_string = "\u{03B4}")] DeltaSm,
    #[strum(to_string = "\u{03B5}")] Epsilon,
    #[strum(to_string = "\u{03B6}")] Zeta,
    #[strum(to_string = "\u{03B7}")] Eta,
    #[strum(to_string = "\u{03B8}")] Theta,
    #[strum(to_string = "\u{03B9}")] Iota,
    #[strum(to_string = "\u{03BA}")] Kappa,
    #[strum(to_string = "\u{03BB}")] Lambda,
    #[strum(to_string = "\u{03BC}")] Mu,
    #[strum(to_string = "\u{03BD}")] Nu,
    #[strum(to_string = "\u{03BE}")] Xi,
    #[strum(to_string = "\u{03BF}")] Omicron,
    #[strum(to_string = "\u{03C0}")] Pi,
    #[strum(to_string = "\u{03C1}")] Rho,
    #[strum(to_string = "\u{03C3}")] Sigma,
    #[strum(to_string = "\u{03C4}")] Tau,
    #[strum(to_string = "\u{03C5}")] Upsilon,
    #[strum(to_string = "\u{03C6}")] Phi,
    #[strum(to_string = "\u{03C7}")] Chi,
    #[strum(to_string = "\u{03C8}")] Psi,
    #[strum(to_string = "\u{03C9}")] Omega,

    // --- Box Drawing (a few key ones) ---
    #[strum(to_string = "\u{2500}")] HLine,
    #[strum(to_string = "\u{2502}")] VLine,
    #[strum(to_string = "\u{251C}")] TRight,
    #[strum(to_string = "\u{2514}")] CornerUr,
}


/// Generates a formatted string table of all available `Glyph` variants.
///
/// This function is intended for debugging or demonstration purposes. It
/// is fully automatic and will include any new glyphs added to the enum.
pub fn debug_glyphs_string() -> String {
    let all_glyphs: Vec<Glyph> = Glyph::iter().collect();
    let items_per_row = 5; // Adjust number of columns here

    // Estimate capacity to reduce reallocations
    let mut output = String::with_capacity(all_glyphs.len() * 25);

    // --- Build Banner ---
    // We'll create a simple banner since we don't have access to the Stderr methods
    let title = " Available Glyphs ";
    let width = 80;
    let padding = (width - title.len()) / 2;
    let bar = "=".repeat(padding);
    output.push_str(&format!("{}{}{}\n", bar, title, bar));

    // --- Build Table ---
    for row in all_glyphs.chunks(items_per_row) {
        for glyph in row {
            let name: &str = glyph.as_ref();
            let code = format!("{:X}", glyph.to_string().chars().next().unwrap() as u32);

            // Use `format_args!` and `push_str` for efficient string building
            output.push_str(&format!("{} {:<15} U+{: <8}", glyph, name, code));
        }
        output.push('\n');
    }

    output
}
