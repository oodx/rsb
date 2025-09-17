//! Named Color Package - Extended boxy color palette (includes simple colors)  
//!
//! This package provides the comprehensive 90+ color palette from boxy,
//! including all simple colors plus extended named colors organized by spectrum.
//! Automatically includes the simple color package to avoid duplication.

pub use super::simple::*;
use std::collections::HashMap; // Include all simple colors

/// Get the complete named color palette (simple + extended named colors)
pub fn get_named_colors() -> HashMap<String, String> {
    let mut colors = get_simple_colors(); // Start with simple colors

    // Add all boxy extended colors
    let extended = get_boxy_extended_colors();
    colors.extend(extended);

    colors
}

/// Get only the extended named colors from boxy (not including simple colors)
pub fn get_boxy_extended_colors() -> HashMap<String, String> {
    let mut colors = HashMap::new();

    // === EXTENDED RED SPECTRUM ===
    colors.insert("crimson".to_string(), "\x1B[38;5;196m".to_string()); // Pure red - critical alerts
    colors.insert("ruby".to_string(), "\x1B[38;5;160m".to_string()); // Dark red - errors
    colors.insert("coral".to_string(), "\x1B[38;5;203m".to_string()); // Red-orange - warnings
    colors.insert("salmon".to_string(), "\x1B[38;5;209m".to_string()); // Light red-orange - notices
    colors.insert("rose".to_string(), "\x1B[38;5;217m".to_string()); // Pink-red - highlights
    colors.insert("brick".to_string(), "\x1B[38;5;124m".to_string()); // Dark brick red - severe

    // === EXTENDED ORANGE SPECTRUM ===
    colors.insert("amber".to_string(), "\x1B[38;5;220m".to_string()); // Golden orange - attention
    colors.insert("tangerine".to_string(), "\x1B[38;5;208m".to_string()); // Bright orange - active
    colors.insert("peach".to_string(), "\x1B[38;5;216m".to_string()); // Light orange - soft alerts
    colors.insert("rust".to_string(), "\x1B[38;5;166m".to_string()); // Dark orange - deprecation
    colors.insert("bronze".to_string(), "\x1B[38;5;130m".to_string()); // Brown-orange - legacy
    colors.insert("gold".to_string(), "\x1B[38;5;178m".to_string()); // Golden - achievements

    // === EXTENDED YELLOW SPECTRUM ===
    colors.insert("lemon".to_string(), "\x1B[38;5;226m".to_string()); // Bright yellow - warnings
    colors.insert("mustard".to_string(), "\x1B[38;5;184m".to_string()); // Muted yellow - caution
    colors.insert("sand".to_string(), "\x1B[38;5;223m".to_string()); // Beige-yellow - neutral
    colors.insert("cream".to_string(), "\x1B[38;5;230m".to_string()); // Light yellow - info
    colors.insert("khaki".to_string(), "\x1B[38;5;143m".to_string()); // Olive-yellow - pending

    // === EXTENDED GREEN SPECTRUM ===
    colors.insert("lime".to_string(), "\x1B[38;5;46m".to_string()); // Bright green - success
    colors.insert("emerald".to_string(), "\x1B[38;5;34m".to_string()); // Pure green - completed
    colors.insert("forest".to_string(), "\x1B[38;5;22m".to_string()); // Dark green - stable
    colors.insert("mint".to_string(), "\x1B[38;5;121m".to_string()); // Light green - fresh
    colors.insert("sage".to_string(), "\x1B[38;5;108m".to_string()); // Muted green - accepted
    colors.insert("jade".to_string(), "\x1B[38;5;35m".to_string()); // Blue-green - verified
    colors.insert("olive".to_string(), "\x1B[38;5;58m".to_string()); // Brown-green - archived

    // === EXTENDED BLUE SPECTRUM ===
    colors.insert("azure".to_string(), "\x1B[38;5;33m".to_string()); // Sky blue - information
    colors.insert("navy".to_string(), "\x1B[38;5;17m".to_string()); // Dark blue - system
    colors.insert("royal".to_string(), "\x1B[38;5;21m".to_string()); // Royal blue - primary
    colors.insert("ice".to_string(), "\x1B[38;5;159m".to_string()); // Light blue - secondary
    colors.insert("steel".to_string(), "\x1B[38;5;67m".to_string()); // Grey-blue - infrastructure
    colors.insert("teal".to_string(), "\x1B[38;5;30m".to_string()); // Blue-green - data
    colors.insert("indigo".to_string(), "\x1B[38;5;54m".to_string()); // Deep blue - configuration

    // === EXTENDED PURPLE SPECTRUM ===
    colors.insert("violet".to_string(), "\x1B[38;5;129m".to_string()); // Blue-purple - special
    colors.insert("plum".to_string(), "\x1B[38;5;96m".to_string()); // Dark purple - reserved
    colors.insert("lavender".to_string(), "\x1B[38;5;183m".to_string()); // Light purple - optional
    colors.insert("orchid".to_string(), "\x1B[38;5;170m".to_string()); // Pink-purple - enhanced
    colors.insert("mauve".to_string(), "\x1B[38;5;139m".to_string()); // Muted purple - metadata
    colors.insert("amethyst".to_string(), "\x1B[38;5;98m".to_string()); // Deep purple - advanced

    // === EXTENDED CYAN SPECTRUM ===
    colors.insert("aqua".to_string(), "\x1B[38;5;51m".to_string()); // Bright cyan - active data
    colors.insert("turquoise".to_string(), "\x1B[38;5;45m".to_string()); // Blue-cyan - processing
    colors.insert("sky".to_string(), "\x1B[38;5;117m".to_string()); // Light cyan - status
    colors.insert("ocean".to_string(), "\x1B[38;5;31m".to_string()); // Deep cyan - persistence

    // === MONOCHROME SPECTRUM ===
    colors.insert("charcoal".to_string(), "\x1B[38;5;235m".to_string()); // Dark grey - inactive
    colors.insert("slate".to_string(), "\x1B[38;5;244m".to_string()); // Medium grey - secondary
    colors.insert("silver".to_string(), "\x1B[38;5;250m".to_string()); // Light grey - tertiary
    colors.insert("pearl".to_string(), "\x1B[38;5;253m".to_string()); // Very light grey - background
    colors.insert("snow".to_string(), "\x1B[38;5;255m".to_string()); // Pure white - emphasis

    // === LEGACY COMPATIBILITY (from boxy v0.5.0) ===
    colors.insert("red2".to_string(), "\x1B[38;5;197m".to_string());
    colors.insert("deep".to_string(), "\x1B[38;5;61m".to_string());
    colors.insert("deep_green".to_string(), "\x1B[38;5;60m".to_string());
    colors.insert("orange".to_string(), "\x1B[38;5;214m".to_string());
    colors.insert("green2".to_string(), "\x1B[32m".to_string());
    colors.insert("blue2".to_string(), "\x1B[38;5;39m".to_string());
    colors.insert("purple".to_string(), "\x1B[38;5;213m".to_string());
    colors.insert("purple2".to_string(), "\x1B[38;5;141m".to_string());
    colors.insert("white2".to_string(), "\x1B[38;5;15m".to_string());
    colors.insert("grey2".to_string(), "\x1B[38;5;240m".to_string());
    colors.insert("grey3".to_string(), "\x1B[38;5;237m".to_string());

    // === PRIORITY LEVELS ===
    colors.insert("critical".to_string(), "\x1B[38;5;196m".to_string()); // Critical priority
    colors.insert("high".to_string(), "\x1B[38;5;208m".to_string()); // High priority
    colors.insert("medium".to_string(), "\x1B[38;5;220m".to_string()); // Medium priority
    colors.insert("low".to_string(), "\x1B[38;5;250m".to_string()); // Low priority
    colors.insert("trivial".to_string(), "\x1B[38;5;237m".to_string()); // Trivial priority

    colors
}

/// Get a named color code by name (includes simple + extended colors)
pub fn get_named_color(name: &str) -> &'static str {
    // Check simple colors first (most common)
    let simple = get_simple_color(name);
    if !simple.is_empty() {
        return simple;
    }

    // Check extended named colors
    match name {
        // Red spectrum
        "crimson" => "\x1B[38;5;196m",
        "ruby" => "\x1B[38;5;160m",
        "coral" => "\x1B[38;5;203m",
        "salmon" => "\x1B[38;5;209m",
        "rose" => "\x1B[38;5;217m",
        "brick" => "\x1B[38;5;124m",

        // Orange spectrum
        "amber" => "\x1B[38;5;220m",
        "tangerine" => "\x1B[38;5;208m",
        "peach" => "\x1B[38;5;216m",
        "rust" => "\x1B[38;5;166m",
        "bronze" => "\x1B[38;5;130m",
        "gold" => "\x1B[38;5;178m",

        // Yellow spectrum
        "lemon" => "\x1B[38;5;226m",
        "mustard" => "\x1B[38;5;184m",
        "sand" => "\x1B[38;5;223m",
        "cream" => "\x1B[38;5;230m",
        "khaki" => "\x1B[38;5;143m",

        // Green spectrum
        "lime" => "\x1B[38;5;46m",
        "emerald" => "\x1B[38;5;34m",
        "forest" => "\x1B[38;5;22m",
        "mint" => "\x1B[38;5;121m",
        "sage" => "\x1B[38;5;108m",
        "jade" => "\x1B[38;5;35m",
        "olive" => "\x1B[38;5;58m",

        // Blue spectrum
        "azure" => "\x1B[38;5;33m",
        "navy" => "\x1B[38;5;17m",
        "royal" => "\x1B[38;5;21m",
        "ice" => "\x1B[38;5;159m",
        "steel" => "\x1B[38;5;67m",
        "teal" => "\x1B[38;5;30m",
        "indigo" => "\x1B[38;5;54m",

        // Purple spectrum
        "violet" => "\x1B[38;5;129m",
        "plum" => "\x1B[38;5;96m",
        "lavender" => "\x1B[38;5;183m",
        "orchid" => "\x1B[38;5;170m",
        "mauve" => "\x1B[38;5;139m",
        "amethyst" => "\x1B[38;5;98m",

        // Cyan spectrum
        "aqua" => "\x1B[38;5;51m",
        "turquoise" => "\x1B[38;5;45m",
        "sky" => "\x1B[38;5;117m",
        "ocean" => "\x1B[38;5;31m",

        // Monochrome
        "charcoal" => "\x1B[38;5;235m",
        "slate" => "\x1B[38;5;244m",
        "silver" => "\x1B[38;5;250m",
        "pearl" => "\x1B[38;5;253m",
        "snow" => "\x1B[38;5;255m",

        // Legacy compatibility
        "red2" => "\x1B[38;5;197m",
        "deep" => "\x1B[38;5;61m",
        "deep_green" => "\x1B[38;5;60m",
        "orange" => "\x1B[38;5;214m",
        "green2" => "\x1B[32m",
        "blue2" => "\x1B[38;5;39m",
        "purple" => "\x1B[38;5;213m",
        "purple2" => "\x1B[38;5;141m",
        "white2" => "\x1B[38;5;15m",
        "grey2" => "\x1B[38;5;240m",
        "grey3" => "\x1B[38;5;237m",

        // Priority levels
        "critical" => "\x1B[38;5;196m",
        "high" => "\x1B[38;5;208m",
        "medium" => "\x1B[38;5;220m",
        "low" => "\x1B[38;5;250m",
        "trivial" => "\x1B[38;5;237m",

        // Unknown
        _ => "",
    }
}

/// Colorize text with a named color (includes simple + extended colors)
pub fn colorize_named(text: &str, color: &str) -> String {
    let color_code = get_named_color(color);
    if color_code.is_empty() {
        text.to_string()
    } else {
        format!("{}{}{}", color_code, text, RESET)
    }
}

/// Check if a color is available in the named palette
pub fn is_named_color(color: &str) -> bool {
    !get_named_color(color).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_includes_simple_colors() {
        // Should include all simple colors
        assert_eq!(get_named_color("red"), "\x1B[31m");
        assert_eq!(get_named_color("success"), "\x1B[92m");
    }

    #[test]
    fn test_extended_named_colors() {
        assert_eq!(get_named_color("crimson"), "\x1B[38;5;196m");
        assert_eq!(get_named_color("emerald"), "\x1B[38;5;34m");
        assert_eq!(get_named_color("azure"), "\x1B[38;5;33m");
        assert_eq!(get_named_color("amber"), "\x1B[38;5;220m");
    }

    #[test]
    fn test_legacy_colors_preserved() {
        assert_eq!(get_named_color("red2"), "\x1B[38;5;197m");
        assert_eq!(get_named_color("deep_green"), "\x1B[38;5;60m");
        assert_eq!(get_named_color("orange"), "\x1B[38;5;214m");
    }

    #[test]
    fn test_colorize_named() {
        let result = colorize_named("Hello", "crimson");
        assert_eq!(result, "\x1B[38;5;196mHello\x1B[0m");

        let simple = colorize_named("Hello", "red"); // Should work with simple colors too
        assert_eq!(simple, "\x1B[31mHello\x1B[0m");
    }
}
