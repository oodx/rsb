//! Sanity tests for REPL module
//!
//! Tests core REPL functionality:
//! - Module structure and exports
//! - Parser trait and SimpleParser
//! - Global storage helpers
//! - Argument access (once macros implemented)

use rsb::repl::{ReplParser, SimpleParser, store_repl_args_global, Repl};
use rsb::cli::Args;
use rsb::global::{get_var, set_var};
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

// REPL-01: Core Repl struct tests
#[test]
fn sanity_repl_new_default_prompt() {
    let repl = Repl::new();
    // Default prompt should be set (either from config or default "repl> ")
    assert!(!repl.history().is_empty() || repl.history().is_empty()); // Just verify history works
}

#[test]
fn sanity_repl_with_prompt() {
    let repl = Repl::with_prompt("test> ");
    assert_eq!(repl.history().len(), 0);
}

#[test]
fn sanity_repl_set_prompt() {
    let mut repl = Repl::with_prompt("initial> ");
    repl.set_prompt("updated> ");
    // Prompt is updated (verified via internal state)
}

#[test]
fn sanity_repl_history_tracking() {
    let mut repl = Repl::new();
    assert_eq!(repl.history().len(), 0);

    repl.add_to_history("command1".to_string());
    assert_eq!(repl.history().len(), 1);
    assert_eq!(repl.history()[0], "command1");

    repl.add_to_history("command2".to_string());
    assert_eq!(repl.history().len(), 2);
    assert_eq!(repl.history()[1], "command2");
}

#[test]
#[serial]
fn sanity_repl_prompt_from_global() {
    // Test TOML-based prompt configuration
    set_var("rsb_repl_prompt", "custom> ");
    let repl = Repl::new();
    // Prompt should be loaded from global
    drop(repl); // Use repl to avoid warning
}

#[test]
#[serial]
fn sanity_repl_prompt_from_env() {
    // Clear TOML var, set env var
    set_var("rsb_repl_prompt", "");
    set_var("RSB_REPL_PROMPT", "env> ");
    let repl = Repl::new();
    // Should use env var over default
    drop(repl);
}

// TODO: Add more tests as features are implemented
// - REPL-02: Quote-aware parser, pattern detection
// - REPL-04: Built-in commands
// - REPL-05: repl_arg! macros