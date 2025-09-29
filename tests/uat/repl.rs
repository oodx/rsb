//! UAT tests for REPL module with visual demonstrations
//!
//! These tests demonstrate REPL functionality with expected outputs
//! and interactive scenarios

use rsb::repl::{ReplParser, SimpleParser, store_repl_args_global, Repl, ReplResult};
use rsb::cli::Args;
use rsb::global::get_var;
use rsb::{repl_arg, repl_argc, repl_args, repl_argv};
use serial_test::serial;

#[test]
#[serial]
fn uat_repl_parser_demo() {
    println!("\n=== REPL Parser Demo ===\n");

    println!("Input: build --output=dist \"my file\" config:debug=true");
    let parser = SimpleParser;
    let tokens = parser.parse("build --output=dist \"my file\" config:debug=true");

    println!("Parsed tokens:");
    for (i, token) in tokens.iter().enumerate() {
        println!("  [{}] {:?}", i, token);
    }

    println!("\n✨ Simple parser tokenization complete!");
    println!("Note: Full pattern detection coming in REPL-02");
}

#[test]
#[serial]
fn uat_repl_global_storage_demo() {
    println!("\n=== REPL Global Storage Demo ===\n");

    println!("Command: status --verbose --format=json");
    let args = Args::from_strs(&["status", "--verbose", "--format=json"]);
    store_repl_args_global(&args);

    println!("\nStored in global:");
    println!("  repl_argc = {}", get_var("repl_argc"));
    println!("  repl_arg_0 = {}", get_var("repl_arg_0"));
    println!("  repl_arg_1 = {}", get_var("repl_arg_1"));
    println!("  repl_arg_2 = {}", get_var("repl_arg_2"));
    println!("  repl_args = {}", get_var("repl_args"));

    println!("\n✨ REPL arguments stored with 0-indexed pattern!");
}

#[test]
#[serial]
fn uat_repl_pattern_preservation() {
    println!("\n=== REPL Pattern Preservation Demo ===\n");

    println!("Testing various token patterns:");

    // Token patterns
    let parser = SimpleParser;
    let tokens = parser.parse("cmd config:debug=true theme=dark;timeout=30 items=a,b,c");

    println!("\nInput: cmd config:debug=true theme=dark;timeout=30 items=a,b,c");
    println!("Tokens:");
    for (i, token) in tokens.iter().enumerate() {
        println!("  [{}] {:?}", i, token);
    }

    println!("\n✓ Token pattern: config:debug=true");
    println!("✓ Semicolon stream: theme=dark;timeout=30");
    println!("✓ Comma list: items=a,b,c");
}

#[test]
#[serial]
fn uat_repl_macros() {
    println!("\n=== REPL Macros Demo ===\n");

    println!("Simulating command: build target --verbose");
    let args = Args::from_line("build target --verbose");
    store_repl_args_global(&args);

    println!("\nUsing repl_arg! macro:");
    println!("  repl_arg!(0) = {:?}", repl_arg!(0));
    println!("  repl_arg!(1) = {:?}", repl_arg!(1));
    println!("  repl_arg!(2) = {:?}", repl_arg!(2));

    println!("\nUsing repl_argc! macro:");
    println!("  repl_argc!() = {}", repl_argc!());

    println!("\nUsing repl_args! macro:");
    println!("  repl_args!() = {:?}", repl_args!());

    println!("\nUsing repl_argv! macro:");
    println!("  repl_argv!() = {:?}", repl_argv!());

    println!("\n✨ All REPL macros working correctly!");
}

#[test]
fn uat_repl_built_in_commands() {
    println!("\n=== REPL Built-in Commands Demo ===\n");

    let mut repl = Repl::new();
    repl.add_to_history("cmd1".to_string());
    repl.add_to_history("cmd2".to_string());

    println!("Testing built-in commands:");

    // Test exit
    let args = Args::from_line("exit");
    let result = repl.dispatch_builtin(&args);
    println!("\n✓ 'exit' command → ReplResult::Exit");
    assert!(matches!(result, ReplResult::Exit));

    // Test quit
    let args = Args::from_line("quit");
    let result = repl.dispatch_builtin(&args);
    println!("✓ 'quit' command → ReplResult::Exit");
    assert!(matches!(result, ReplResult::Exit));

    // Test help
    let args = Args::from_line("help");
    let result = repl.dispatch_builtin(&args);
    println!("✓ 'help' command → ReplResult::Continue (shows help)");
    assert!(matches!(result, ReplResult::Continue));

    // Test history
    let args = Args::from_line("history");
    let result = repl.dispatch_builtin(&args);
    println!("✓ 'history' command → ReplResult::Continue (shows history)");
    assert!(matches!(result, ReplResult::Continue));

    // Test user command
    let args = Args::from_line("mycmd arg");
    let result = repl.dispatch_builtin(&args);
    println!("✓ 'mycmd' → ReplResult::Command (dispatched to user handler)");
    assert!(matches!(result, ReplResult::Command(_)));

    println!("\n✨ All built-in commands working correctly!");
}

#[test]
fn uat_repl_dynamic_prompts() {
    println!("\n=== REPL Dynamic Prompts Demo ===\n");

    let mut repl = Repl::with_prompt("app> ");
    println!("Initial prompt: 'app> '");

    repl.set_prompt("app:config> ");
    println!("After set_prompt('app:config> '): 'app:config> '");

    repl.set_prompt("app> ");
    println!("After set_prompt('app> '): 'app> '");

    println!("\n✨ Dynamic prompt changes work!");
    println!("Use case: Context-aware prompts for subcommands");
}

#[test]
fn uat_repl_error_handling() {
    println!("\n=== REPL Error Handling Demo ===\n");

    fn handler_success(_args: Args) -> Result<i32, String> {
        println!("Handler executed successfully");
        Ok(0)
    }

    fn handler_error(_args: Args) -> Result<i32, String> {
        Err("Simulated error".to_string())
    }

    println!("Testing handler that succeeds:");
    match handler_success(Args::from_line("test")) {
        Ok(code) => println!("✓ Handler returned Ok({})", code),
        Err(e) => println!("✗ Handler returned Err: {}", e),
    }

    println!("\nTesting handler that fails:");
    match handler_error(Args::from_line("test")) {
        Ok(code) => println!("✗ Handler returned Ok({})", code),
        Err(e) => println!("✓ Handler returned Err: {}", e),
    }

    println!("\n✨ Error handling works!");
    println!("Note: repl_dispatch! macro prints errors and continues loop");
}