//! REPL Integration Demo
//!
//! Demonstrates how to integrate RSB REPL with the dispatch! macro

use rsb::prelude::*;

fn main() {
    let args = bootstrap!();

    dispatch!(&args, {
        "build" => cmd_build,
        "test" => cmd_test,
        "repl" => cmd_repl,
    });
}

fn cmd_build(args: Args) -> i32 {
    println!("Building project...");
    let target = args.get(1);
    if !target.is_empty() {
        println!("Target: {}", target);
    }
    0
}

fn cmd_test(args: Args) -> i32 {
    println!("Running tests...");
    0
}

fn cmd_repl(_args: Args) -> i32 {
    println!("Entering REPL mode. Type 'help' for commands, 'exit' to quit.\n");

    let repl = Repl::new();
    repl_dispatch!(repl, {
        "build" => repl_build,
        "test" => repl_test,
        "status" => repl_status,
    })
}

// REPL command handlers
fn repl_build(args: Args) -> Result<i32, String> {
    println!("REPL: Building...");

    // Access args via Args methods
    let target = args.get(1);
    if !target.is_empty() {
        println!("  Target: {}", target);
    }

    // Or use REPL macros
    println!("  Command: {}", repl_arg!(0));
    println!("  Arg count: {}", repl_argc!());

    Ok(0)
}

fn repl_test(_args: Args) -> Result<i32, String> {
    println!("REPL: Running tests...");

    // Demonstrate all REPL args
    let argv = repl_argv!();
    if argv.len() > 1 {
        println!("  Args: {:?}", &argv[1..]);
    }

    Ok(0)
}

fn repl_status(_args: Args) -> Result<i32, String> {
    println!("Status: OK");
    println!("  History count: Available via REPL internal state");

    Ok(0)
}