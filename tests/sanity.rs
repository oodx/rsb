// Category orchestrator: sanity tests
// This file includes all sanity test modules for category-level execution

#[path = "sanity/baseline.rs"]
mod baseline;

#[path = "sanity/bash.rs"]
mod bash;

#[path = "sanity/com.rs"]
mod com;

#[path = "sanity/core.rs"]
mod core;

#[path = "sanity/global_adapter.rs"]
mod global_adapter;

#[path = "sanity/global_clear.rs"]
mod global_clear;

#[path = "sanity/host_env.rs"]
mod host_env;

#[path = "sanity/host_paths.rs"]
mod host_paths;

#[path = "sanity/options_cleanup.rs"]
mod options_cleanup;

#[path = "sanity/threads.rs"]
mod threads;

// NEW: GX module subdirectory
#[path = "sanity/gx/string.rs"]
mod gx_string;

#[path = "sanity/gx/collection.rs"]
mod gx_collection;

#[path = "sanity/gx/id.rs"]
mod gx_id;

// NEW: Math module subdirectory
#[path = "sanity/math/basic.rs"]
mod math_basic;

#[path = "sanity/math/random.rs"]
mod math_random;

#[path = "sanity/math/integers.rs"]
mod math_integers;

#[path = "sanity/math/aggregators.rs"]
mod math_aggregators;

#[path = "sanity/math/percentage.rs"]
mod math_percentage;

#[path = "sanity/math/expressions.rs"]
mod math_expressions;

#[path = "sanity/math/base.rs"]
mod math_base;

#[path = "sanity/math/predicates.rs"]
mod math_predicates;

// NEW: Tokens module subdirectory
#[path = "sanity/tokens/basic.rs"]
mod tokens_basic;

// Date module sanity tests
#[path = "sanity/date.rs"]
mod date;

// Math module sanity tests
#[path = "sanity/math.rs"]
mod math;

// Global module sanity tests
#[path = "sanity/global.rs"]
mod global;

// Colors module sanity tests (visual primitives without ceremony)
#[cfg(feature = "colors-core")]
#[path = "sanity/colors.rs"]
mod colors;

// Progress module sanity tests
#[path = "sanity/progress.rs"]
mod progress;

// String module sanity tests (MODERN - freshly written)
#[path = "sanity/string.rs"]
mod string;

// GX comprehensive sanity test (MODERN - covers all generator functionality)
#[path = "sanity/gx.rs"]
mod gx;

// CLI module sanity tests (MODERN - comprehensive Args and bootstrap functionality)
#[path = "sanity/cli.rs"]
mod cli;

// CLI args to global tests (v0.7.0+)
#[path = "sanity/cli_args.rs"]
mod cli_args;

// Param module sanity tests (MODERN - parameter expansion and context operations)
#[path = "sanity/param.rs"]
mod param;

// Parse module sanity tests (MODERN - sed-like transformations and templates)
#[path = "sanity/parse.rs"]
mod parse;

// Dev module sanity tests (MODERN - PTY functionality with feature gating)
#[path = "sanity/dev.rs"]
mod dev;

// FS module sanity tests (MODERN - file system operations and utilities)
#[path = "sanity/fs.rs"]
mod fs;

// Visual module sanity tests (MODERN - colors, glyphs, prompts with feature gating)
#[path = "sanity/visual.rs"]
mod visual;

// Hosts module sanity tests (MODERN - environment, XDG/RSB paths, script context)
#[path = "sanity/hosts.rs"]
mod hosts;

// Token module sanity tests (MODERN - key=value parsing with namespace support)
#[path = "sanity/token.rs"]
mod token;

// TOML module sanity tests (v2.0+ - Cargo.toml metadata extraction)
#[path = "sanity/toml.rs"]
mod toml;

// Flags module sanity tests (v2.0+ - Flag commands --help, --version)
#[path = "sanity/flags.rs"]
mod flags;

// REPL module sanity tests (v2.0+ - Interactive command processing)
#[path = "sanity/repl.rs"]
mod repl;
