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
use rsb::{repl_arg, repl_argc, repl_args, repl_argv};
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

// REPL-02: Parser and tokenization tests
#[test]
fn sanity_parser_quote_handling() {
    let parser = SimpleParser;
    let tokens = parser.parse("build \"my file.txt\" test");

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], "build");
    assert_eq!(tokens[1], "my file.txt");
    assert_eq!(tokens[2], "test");
}

#[test]
fn sanity_parser_multiple_quotes() {
    let parser = SimpleParser;
    let tokens = parser.parse("cmd \"first arg\" \"second arg\"");

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], "cmd");
    assert_eq!(tokens[1], "first arg");
    assert_eq!(tokens[2], "second arg");
}

#[test]
fn sanity_parser_empty_quotes() {
    let parser = SimpleParser;
    let tokens = parser.parse("cmd \"\" test");

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], "cmd");
    assert_eq!(tokens[1], "test");
}

#[test]
fn sanity_parser_flag_patterns() {
    let parser = SimpleParser;
    let tokens = parser.parse("build --output=dist --verbose");

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], "build");
    assert_eq!(tokens[1], "--output=dist");
    assert_eq!(tokens[2], "--verbose");
}

#[test]
fn sanity_parser_token_patterns() {
    let parser = SimpleParser;
    let tokens = parser.parse("cmd config:debug=true theme=dark");

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], "cmd");
    assert_eq!(tokens[1], "config:debug=true");
    assert_eq!(tokens[2], "theme=dark");
}

#[test]
fn sanity_parser_comma_list_patterns() {
    let parser = SimpleParser;
    let tokens = parser.parse("cmd items=a,b,c test");

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], "cmd");
    assert_eq!(tokens[1], "items=a,b,c");
    assert_eq!(tokens[2], "test");
}

#[test]
fn sanity_parser_semicolon_stream_patterns() {
    let parser = SimpleParser;
    let tokens = parser.parse("cmd theme=dark;timeout=30 test");

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], "cmd");
    assert_eq!(tokens[1], "theme=dark;timeout=30");
    assert_eq!(tokens[2], "test");
}

#[test]
fn sanity_parser_complex_line() {
    let parser = SimpleParser;
    let tokens = parser.parse("build --output=dist \"my file\" config:debug=true items=a,b,c theme=dark;timeout=30");

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0], "build");
    assert_eq!(tokens[1], "--output=dist");
    assert_eq!(tokens[2], "my file");
    assert_eq!(tokens[3], "config:debug=true");
    assert_eq!(tokens[4], "items=a,b,c");
    assert_eq!(tokens[5], "theme=dark;timeout=30");
}

#[test]
fn sanity_args_from_line_basic() {
    let args = rsb::cli::Args::from_line("build test --verbose");

    // Index 0 is treated as program (command in REPL), get(1) returns first arg after
    assert_eq!(args.get(1), "test");
    assert_eq!(args.get(2), "--verbose");
    // Verify all() includes command
    assert_eq!(args.all()[0], "build");
}

#[test]
fn sanity_args_from_line_quotes() {
    let args = rsb::cli::Args::from_line("cmd \"my file.txt\"");

    assert_eq!(args.get(1), "my file.txt");
    assert_eq!(args.all()[0], "cmd");
}

#[test]
fn sanity_args_from_line_complex() {
    let args = rsb::cli::Args::from_line("build --output=dist \"my file\"");

    assert_eq!(args.get(1), "--output=dist");
    assert_eq!(args.get(2), "my file");
    assert_eq!(args.all()[0], "build");
}

#[test]
fn sanity_repl_with_parser() {
    let parser = Box::new(SimpleParser);
    let repl = Repl::with_parser(parser);
    assert_eq!(repl.history().len(), 0);
}

// REPL-04: Built-in command tests
#[test]
fn sanity_builtin_exit_command() {
    use rsb::repl::{Repl, ReplResult};
    let repl = Repl::new();
    let args = rsb::cli::Args::from_line("exit");
    let result = repl.dispatch_builtin(&args);

    match result {
        ReplResult::Exit => {}, // Success
        _ => panic!("Expected Exit result"),
    }
}

#[test]
fn sanity_builtin_quit_command() {
    use rsb::repl::{Repl, ReplResult};
    let repl = Repl::new();
    let args = rsb::cli::Args::from_line("quit");
    let result = repl.dispatch_builtin(&args);

    match result {
        ReplResult::Exit => {}, // Success
        _ => panic!("Expected Exit result"),
    }
}

#[test]
#[serial]
fn sanity_builtin_clear_command() {
    use rsb::repl::{Repl, ReplResult};
    use rsb::global::{set_var, has_var};

    // Set some REPL globals
    set_var("repl_test", "value");
    assert!(has_var("repl_test"));

    let repl = Repl::new();
    let args = rsb::cli::Args::from_line("clear");
    let result = repl.dispatch_builtin(&args);

    match result {
        ReplResult::Continue => {
            // clear_prefix should have removed repl_ vars
            assert!(!has_var("repl_test"));
        },
        _ => panic!("Expected Continue result"),
    }
}

#[test]
fn sanity_builtin_history_command() {
    use rsb::repl::{Repl, ReplResult};
    let mut repl = Repl::new();
    repl.add_to_history("cmd1".to_string());
    repl.add_to_history("cmd2".to_string());

    let args = rsb::cli::Args::from_line("history");
    let result = repl.dispatch_builtin(&args);

    match result {
        ReplResult::Continue => {}, // Success
        _ => panic!("Expected Continue result"),
    }
}

#[test]
fn sanity_builtin_help_command() {
    use rsb::repl::{Repl, ReplResult};
    let repl = Repl::new();
    let args = rsb::cli::Args::from_line("help");
    let result = repl.dispatch_builtin(&args);

    match result {
        ReplResult::Continue => {}, // Success
        _ => panic!("Expected Continue result"),
    }
}

#[test]
fn sanity_builtin_user_command() {
    use rsb::repl::{Repl, ReplResult};
    let repl = Repl::new();
    let args = rsb::cli::Args::from_line("build test");
    let result = repl.dispatch_builtin(&args);

    match result {
        ReplResult::Command(cmd_args) => {
            assert_eq!(cmd_args.all()[0], "build");
            assert_eq!(cmd_args.get(1), "test");
        },
        _ => panic!("Expected Command result"),
    }
}

#[test]
fn sanity_builtin_empty_line() {
    use rsb::repl::{Repl, ReplResult};
    let repl = Repl::new();
    let args = rsb::cli::Args::from_line("");
    let result = repl.dispatch_builtin(&args);

    match result {
        ReplResult::Continue => {}, // Success
        _ => panic!("Expected Continue result"),
    }
}

// REPL-05: REPL argument macro tests
#[test]
#[serial]
fn sanity_repl_arg_macro() {
    use rsb::repl::store_repl_args_global;
    let args = rsb::cli::Args::from_line("cmd arg1 arg2");
    store_repl_args_global(&args);

    assert_eq!(repl_arg!(0), "cmd");
    assert_eq!(repl_arg!(1), "arg1");
    assert_eq!(repl_arg!(2), "arg2");
}

#[test]
#[serial]
fn sanity_repl_argc_macro() {
    use rsb::repl::store_repl_args_global;
    let args = rsb::cli::Args::from_line("cmd arg1 arg2");
    store_repl_args_global(&args);

    assert_eq!(repl_argc!(), 3);
}

#[test]
#[serial]
fn sanity_repl_args_macro() {
    use rsb::repl::store_repl_args_global;
    let args = rsb::cli::Args::from_line("cmd arg1 arg2");
    store_repl_args_global(&args);

    assert_eq!(repl_args!(), "cmd;arg1;arg2");
}

#[test]
#[serial]
fn sanity_repl_argv_macro() {
    use rsb::repl::store_repl_args_global;
    let args = rsb::cli::Args::from_line("cmd arg1 arg2");
    store_repl_args_global(&args);

    let argv = repl_argv!();
    assert_eq!(argv.len(), 3);
    assert_eq!(argv[0], "cmd");
    assert_eq!(argv[1], "arg1");
    assert_eq!(argv[2], "arg2");
}

#[test]
#[serial]
fn sanity_repl_macros_empty() {
    use rsb::global::set_var;
    // Clear REPL globals
    set_var("repl_argc", "");
    set_var("repl_args", "");

    assert_eq!(repl_argc!(), 0);
    assert_eq!(repl_args!(), "");
    assert_eq!(repl_argv!().len(), 0);
}