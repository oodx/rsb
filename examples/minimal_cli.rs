// Minimal, working CLI example using RSB prelude
use rsb::prelude::*;

fn main() {
    let args = bootstrap!();

    if pre_dispatch!(&args, {
        "init" => cmd_init,
        "check" => cmd_check
    }) {
        return;
    }

    dispatch!(&args, {
        "greet" => cmd_greet,
        "add" => cmd_add,
        "strings" => cmd_strings,
        "help" => cmd_help
    });
}

// Pre commands
fn cmd_init(_args: Args) -> i32 {
    mkdir_p(".minimal");
    echo!("initialized .minimal/");
    0
}

fn cmd_check(_args: Args) -> i32 {
    if !is_command("echo") {
        stderr!("echo not found in PATH");
        return 1;
    }
    echo!("system okay");
    0
}

// Main commands
fn cmd_help(_args: Args) -> i32 {
    echo!("Minimal CLI\n\nUsage:\n  cargo run --example minimal_cli -- <command> [args]\n\nCommands:\n  greet [name]     Greet a name (default: world)\n  add <a> <b>      Add two numbers\n  strings          Demo string helpers\n  init             Create .minimal/\n  check            Verify basic tooling");
    0
}

fn cmd_greet(args: Args) -> i32 {
    let name = args.get_or(1, "world");
    echo!("Hello, {}!", name);
    0
}

fn cmd_add(args: Args) -> i32 {
    let a = args.get_or(1, "0");
    let b = args.get_or(2, "0");
    if a.is_empty() || b.is_empty() {
        stderr!("Usage: add <a> <b>");
        return 1;
    }
    let sum = to_number!(&a) + to_number!(&b);
    echo!("{}", sum);
    0
}

fn cmd_strings(_args: Args) -> i32 {
    let path = "src/main.rs";
    echo!("prefix shortest: {}", str_prefix(path, "*/", false));
    echo!("prefix longest : {}", str_prefix(path, "*/", true));
    echo!("suffix .rs off : {}", str_suffix(path, ".rs", false));
    echo!("upper first    : {}", str_upper("hello", false));
    echo!("lower all      : {}", str_lower("WORLD", true));
    0
}
