// A minimal RSB-powered CLI demonstrating bootstrap, dispatch, and helpers
use rsb::prelude::*;

fn main() {
    let args = bootstrap!();

    // Pre-dispatch for setup-style commands
    if pre_dispatch!(&args, {
        "install" => do_install,
        "init" => do_init
    }) {
        return;
    }

    // Main dispatch
    dispatch!(&args, {
        "hello" => cmd_hello,
        "sum" => cmd_sum,
        "env" => cmd_env,
        "help" => cmd_help
    });
}

// --- Pre-dispatch commands ---
fn do_install(_args: Args) -> i32 {
    echo!("Installing dummy dependencies...");
    0
}

fn do_init(_args: Args) -> i32 {
    echo!("Initializing dummy project scaffold...");
    mkdir_p(".dummy");
    0
}

// --- Main commands ---
fn cmd_help(_args: Args) -> i32 {
    echo!("Dummy CLI\n\nUsage:\n  dummy-cli <command> [args]\n\nCommands:\n  hello [name]     Greet a name (default: world)\n  sum <a> <b>      Add two numbers\n  env              Show a couple of env/context values\n  install          Pre-dispatch: simulate install\n  init             Pre-dispatch: scaffold dummy files");
    0
}

fn cmd_hello(args: Args) -> i32 {
    let name = args.get_or(1, "world");
    echo!("Hello, {}!", name);
    0
}

fn cmd_sum(args: Args) -> i32 {
    let a = args.get_or(1, "0");
    let b = args.get_or(2, "0");
    if a.is_empty() || b.is_empty() {
        stderr!("Usage: sum <a> <b>");
        return 1;
    }
    let sum = to_number!(&a) + to_number!(&b);
    echo!("{}", sum);
    0
}

fn cmd_env(_args: Args) -> i32 {
    // Demonstrate a couple of helpers without relying on visual features
    echo!("HOME = {}", get_var("HOME"));
    echo!("PWD  = {}", current_dir!());
    0
}
