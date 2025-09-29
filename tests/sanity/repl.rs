//! Sanity tests for REPL module
//!
//! Tests core REPL functionality:
//! - Module structure and exports
//! - Parser trait and SimpleParser
//! - Global storage helpers
//! - Argument access (once macros implemented)

use rsb::repl::{ReplParser, SimpleParser, store_repl_args_global};
use rsb::cli::Args;
use rsb::global::get_var;
use serial_test::serial;

#[test]
fn sanity_repl_module_exports() {
    // Verify core types are exported
    let _parser: SimpleParser = SimpleParser;
}

#[test]
#[serial]
fn sanity_simple_parser_basic() {
    let parser = SimpleParser;
    let tokens = parser.parse("build --output=dist test");

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], "build");
    assert_eq!(tokens[1], "--output=dist");
    assert_eq!(tokens[2], "test");
}

#[test]
#[serial]
fn sanity_store_repl_args_global_basic() {
    let args = Args::from_strs(&["status", "--verbose"]);
    store_repl_args_global(&args);

    assert_eq!(get_var("repl_argc"), "2");
    assert_eq!(get_var("repl_arg_0"), "status");
    assert_eq!(get_var("repl_arg_1"), "--verbose");
}

#[test]
#[serial]
fn sanity_store_repl_args_semicolon_joined() {
    let args = Args::from_strs(&["cmd", "arg1", "arg2"]);
    store_repl_args_global(&args);

    assert_eq!(get_var("repl_args"), "cmd;arg1;arg2");
}

// TODO: Add more tests as features are implemented
// - REPL-01: Repl struct, read_line, prompts
// - REPL-02: Quote-aware parser, pattern detection
// - REPL-04: Built-in commands
// - REPL-05: repl_arg! macros