// Category orchestrator: UAT tests
// This file includes all UAT test modules for category-level execution

#[path = "uat/bash.rs"]
mod bash;

#[path = "uat/cli.rs"]
mod cli;

#[path = "uat/cli_args.rs"]
mod cli_args;

#[path = "uat/fs.rs"]
mod fs;

#[path = "uat/colors.rs"]
mod colors;

#[path = "uat/colors_macros.rs"]
mod colors_macros;

#[path = "uat/com.rs"]
mod com;

#[path = "uat/global.rs"]
mod global;

#[path = "uat/global_clear.rs"]
mod global_clear;

#[path = "uat/glyphs.rs"]
mod glyphs;

#[path = "uat/hosts.rs"]
mod hosts;

#[path = "uat/param.rs"]
mod param;

#[path = "uat/prompts.rs"]
mod prompts;

#[path = "uat/string.rs"]
mod string;

#[path = "uat/threads.rs"]
mod threads;

#[path = "uat/token.rs"]
mod token;

#[path = "uat/visual.rs"]
mod visual;

// Date module UAT tests
#[path = "uat/date.rs"]
mod date;

// Math module UAT tests
#[path = "uat/math.rs"]
mod math;

// Progress module UAT tests
#[path = "uat/progress.rs"]
mod progress;

// Dev module UAT tests
#[path = "uat/dev.rs"]
mod dev;

// GX (Generators) module UAT tests
#[path = "uat/gx.rs"]
mod gx;

// Parse module UAT tests
#[path = "uat/parse.rs"]
mod parse;

// TOML module UAT tests (v2.0+ - Cargo.toml metadata extraction demos)
#[path = "uat/toml.rs"]
mod toml;

// Flags module UAT tests (v2.0+ - Flag commands --help, --version demos)
#[path = "uat/flags.rs"]
mod flags;
