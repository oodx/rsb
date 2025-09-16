// Category orchestrator: UAT tests
// This file includes all UAT test modules for category-level execution

#[path = "uat/bash.rs"]
mod bash;

#[path = "uat/colors.rs"]
mod colors;

#[path = "uat/colors_macros.rs"]
mod colors_macros;

#[path = "uat/com.rs"]
mod com;

#[path = "uat/global.rs"]
mod global;

#[path = "uat/glyphs.rs"]
mod glyphs;

#[path = "uat/host_env.rs"]
mod host_env;

#[path = "uat/host_paths.rs"]
mod host_paths;

#[path = "uat/param_uat.rs"]
mod param_uat;

#[path = "uat/prompts.rs"]
mod prompts;

#[path = "uat/string.rs"]
mod string;

#[path = "uat/threads.rs"]
mod threads;

#[path = "uat/tokens.rs"]
mod tokens;

#[path = "uat/visual.rs"]
mod visual;