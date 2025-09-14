// Wrapper: UAT tests (visual demos + param uat)
#![cfg(feature = "visual")]

#[path = "uat/colors.rs"]
mod uat_colors;
#[path = "uat/colors_macros.rs"]
mod uat_colors_macros;
#[path = "uat/glyphs.rs"]
mod uat_glyphs;
#[path = "uat/prompts.rs"]
mod uat_prompts;
#[path = "uat/visual.rs"]
mod uat_visual;
#[path = "uat/param_uat.rs"]
mod uat_param;

