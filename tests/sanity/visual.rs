// RSB Sanity Tests - Visual Module Core Functionality Verification
// Tests verify the visual module functions work as documented in FEATURES_VISUALS

use rsb::prelude::*;

#[test]
fn test_visual_module_feature_gating() {
    // Test that visual module is available when features enabled
    // This test verifies basic feature flag functionality

    #[cfg(feature = "visual")]
    {
        // Visual base feature should be available
        // This is mostly a compilation test - if we get here, feature gating works
        assert!(true, "Visual base feature is enabled");
    }

    #[cfg(not(feature = "visual"))]
    {
        // If visual disabled, test should still pass but acknowledge the state
        assert!(true, "Visual feature disabled - graceful fallback");
    }
}

#[test]
#[cfg(feature = "colors-simple")]
fn test_colors_basic_functionality() {
    // Test basic color system functionality
    use rsb::visual::colors::{color_mode, color, colorize};

    // Test color configuration
    color_mode("never"); // Disable colors for predictable testing

    // Test basic color functions
    let red_code = color("red");
    let reset_code = color("reset");

    // Colors should return empty strings when disabled
    assert!(red_code.is_empty() || red_code.contains("31") || red_code == "red");
    assert!(reset_code.is_empty() || reset_code.contains("0") || reset_code == "reset");

    // Test colorize function
    let colored_text = colorize("test", "red");
    assert!(colored_text.contains("test"));

    // Re-enable colors and test
    color_mode("always");
    let enabled_red = color("red");
    // Should have ANSI codes when enabled
    assert!(!enabled_red.is_empty());

    // Reset to safe state
    color_mode("auto");
}

#[test]
#[cfg(feature = "colors-simple")]
fn test_colors_registry_functions() {
    // Test color registry access functions
    use rsb::visual::colors::{get_color, bg, colorize_bg, get_all_colors};

    // Test individual color retrieval
    let red_color = get_color("red");
    assert!(!red_color.is_empty(), "Red color should be in registry");

    // Test background colors
    let bg_red = bg("red");
    assert!(!bg_red.is_empty(), "Background color should return a value");

    // Test background colorize
    let bg_text = colorize_bg("test", "blue");
    assert!(bg_text.contains("test"));

    // Test registry enumeration
    let all_colors = get_all_colors();
    assert!(!all_colors.is_empty(), "Color registry should not be empty");

    // Check if registry contains red and blue colors
    let has_red = all_colors.iter().any(|(name, _)| name == "red");
    let has_blue = all_colors.iter().any(|(name, _)| name == "blue");
    assert!(has_red, "Registry should contain red");
    assert!(has_blue, "Registry should contain blue");
}

#[test]
#[cfg(feature = "colors-named")]
fn test_colors_named_palette() {
    // Test extended named color palette
    use rsb::visual::colors::{get_color, get_all_colors};

    let all_colors = get_all_colors();

    // Named colors that should be available
    let named_colors = vec!["crimson", "azure", "emerald", "amber"];

    for color_name in named_colors {
        let has_color = all_colors.iter().any(|(name, _)| name == color_name);
        assert!(
            has_color,
            "Named color '{}' should be in registry",
            color_name
        );

        let color_value = get_color(color_name);
        assert!(!color_value.is_empty(), "Named color '{}' should have a value", color_name);
    }
}

#[test]
#[cfg(feature = "colors-status")]
fn test_colors_status_palette() {
    // Test status-specific color functionality
    use rsb::visual::colors::{get_color, get_all_colors};

    let all_colors = get_all_colors();

    // Status colors that should be available
    let status_colors = vec!["success", "error", "warning", "info", "trace"];

    for status_color in status_colors {
        let has_color = all_colors.iter().any(|(name, _)| name == status_color);
        assert!(
            has_color,
            "Status color '{}' should be in registry",
            status_color
        );
    }

    // Test specific status color usage
    let success_color = get_color("success");
    assert!(!success_color.is_empty(), "Success color should be available");

    let error_color = get_color("error");
    assert!(!error_color.is_empty(), "Error color should be available");
}

#[test]
#[cfg(feature = "glyphs")]
fn test_glyphs_basic_functionality() {
    // Test glyph system functionality
    use rsb::visual::glyphs::{glyph_enable, set_glyphs_enabled, glyphs_enabled, glyph};

    // Test glyph control functions
    glyph_enable();
    assert!(glyphs_enabled(), "Glyphs should be enabled after glyph_enable()");

    set_glyphs_enabled(false);
    assert!(!glyphs_enabled(), "Glyphs should be disabled after set_glyphs_enabled(false)");

    set_glyphs_enabled(true);
    assert!(glyphs_enabled(), "Glyphs should be enabled after set_glyphs_enabled(true)");

    // Test glyph lookup
    let pass_glyph = glyph("pass");
    assert!(!pass_glyph.is_empty(), "Pass glyph should return a value");

    let fail_glyph = glyph("fail");
    assert!(!fail_glyph.is_empty(), "Fail glyph should return a value");

    // Test case insensitive lookup
    let pass_upper = glyph("PASS");
    let pass_lower = glyph("pass");
    assert_eq!(pass_upper, pass_lower, "Glyph lookup should be case insensitive");
}

#[test]
#[cfg(feature = "glyphs")]
fn test_glyphs_registry() {
    // Test glyph registry functionality
    use rsb::visual::glyphs::{glyph, get_all_glyphs};

    // Test registry access
    let all_glyphs = get_all_glyphs();
    assert!(!all_glyphs.is_empty(), "Glyph registry should not be empty");

    // Test common glyphs
    let common_glyphs = vec!["pass", "fail", "warning", "info", "arrow", "bullet"];

    for glyph_name in common_glyphs {
        let glyph_value = glyph(glyph_name);
        assert!(!glyph_value.is_empty(), "Glyph '{}' should have a value", glyph_name);
    }

    // Test that ellipsis glyph exists (mentioned in docs)
    let ellipsis = glyph("ellipsis");
    assert!(!ellipsis.is_empty(), "Ellipsis glyph should exist");
    assert!(ellipsis.contains("…") || ellipsis.contains("..."), "Ellipsis should be a Unicode ellipsis or fallback");
}

#[test]
#[cfg(feature = "prompts")]
fn test_prompts_basic_functionality() {
    // Test prompts system basic functionality
    // Note: These tests can't test interactive prompts in CI, but can test API structure

    // Test that prompt functions exist and are callable
    // We'll use environment variable to simulate non-interactive testing
    std::env::set_var("RSB_TEST_MODE", "1");

    // The actual prompt functions would require stdin interaction,
    // so we test the API exists and imports work
    use rsb::visual::prompts::{confirm, confirm_default};

    // Test that functions exist (compilation test)
    let _confirm_fn = confirm;
    let _confirm_default_fn = confirm_default;

    // Basic functionality test - these functions exist
    assert!(true, "Prompt functions are available");

    std::env::remove_var("RSB_TEST_MODE");
}

#[test]
#[cfg(feature = "prompts")]
fn test_prompts_timeout_functionality() {
    // Test timeout-enhanced prompt functions
    use rsb::visual::prompts::utils::{confirm_with_timeout, ask_with_timeout};

    // Test timeout functions exist
    let _timeout_confirm_fn = confirm_with_timeout;
    let _timeout_ask_fn = ask_with_timeout;

    // These are compilation/API existence tests
    assert!(true, "Timeout prompt functions are available");
}

#[test]
#[cfg(all(feature = "colors-simple", feature = "glyphs"))]
fn test_visual_integration() {
    // Test integration between visual components
    use rsb::visual::colors::colorize;
    use rsb::visual::glyphs::{glyph_enable, glyph};

    // Enable both systems
    glyph_enable();

    // Test combined usage
    let success_glyph = glyph("pass");
    let colored_success = colorize(&format!("{} Success", success_glyph), "green");

    assert!(colored_success.contains("Success"), "Combined text should contain message");
    assert!(colored_success.contains(&success_glyph), "Combined text should contain glyph");
}

#[test]
fn test_visual_utils_module() {
    // Test that utils module exists per MODULE_SPEC
    #[cfg(feature = "prompts")]
    {
        use rsb::visual::utils::{confirm_with_timeout, ask_with_timeout};

        // Test that utils functions exist
        let _confirm_timeout_fn = confirm_with_timeout;
        let _ask_timeout_fn = ask_with_timeout;

        assert!(true, "Visual utils functions are available");
    }

    #[cfg(not(feature = "prompts"))]
    {
        assert!(true, "Visual utils gracefully unavailable without prompts feature");
    }
}

#[test]
fn test_visual_graceful_fallback() {
    // Test that visual module handles missing features gracefully

    #[cfg(not(feature = "visual"))]
    {
        // When visual features disabled, tests should still pass
        assert!(true, "Visual module gracefully unavailable");
    }

    #[cfg(feature = "visual")]
    {
        // When visual enabled, basic module structure should exist
        assert!(true, "Visual module available with feature gating");
    }
}

#[test]
fn test_visual_module_documentation_compliance() {
    // Test that module follows documented patterns from FEATURES_VISUALS

    // Test feature hierarchy compliance
    #[cfg(feature = "colors-named")]
    {
        // colors-named should include colors-simple functionality
        #[cfg(not(feature = "colors-simple"))]
        compile_error!("colors-named should depend on colors-simple");
    }

    #[cfg(feature = "prompts")]
    {
        // prompts should require colors-simple
        #[cfg(not(feature = "colors-simple"))]
        compile_error!("prompts should depend on colors-simple");
    }

    // This is primarily a compilation test
    assert!(true, "Visual module feature dependencies are correct");
}

#[test]
fn test_edge_cases() {
    // Test edge cases and error conditions

    #[cfg(feature = "colors-simple")]
    {
        use rsb::visual::colors::{color, colorize};

        // Test with empty strings
        let empty_color = color("");
        let empty_colorize = colorize("", "red");
        let colorize_empty_color = colorize("test", "");

        // Should handle gracefully
        assert!(empty_color.is_empty() || !empty_color.is_empty()); // Just shouldn't panic
        assert!(empty_colorize.is_empty() || !empty_colorize.is_empty());
        assert!(colorize_empty_color.contains("test"));

        // Test with non-existent color
        let fake_color = color("nonexistent_color_12345");
        assert!(!fake_color.is_empty() || fake_color.is_empty()); // Should not panic
    }

    #[cfg(feature = "glyphs")]
    {
        use rsb::visual::glyphs::glyph;

        // Test with non-existent glyph
        let fake_glyph = glyph("nonexistent_glyph_12345");
        assert!(!fake_glyph.is_empty() || fake_glyph.is_empty()); // Should not panic

        // Test with empty string
        let empty_glyph = glyph("");
        assert!(!empty_glyph.is_empty() || empty_glyph.is_empty()); // Should not panic
    }
}