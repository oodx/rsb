use rsb::cli::Args;
use rsb::prelude::*;

#[test]
fn uat_cli_args_demo() {
    println!("\n=== UAT: CLI Args Processing Demo ===");

    // Simulate command line arguments
    let args_vec = vec![
        "program".to_string(),
        "command".to_string(),
        "--verbose".to_string(),
        "--config=test.conf".to_string(),
        "-d".to_string(),
        "input.txt".to_string(),
        "output.txt".to_string(),
    ];

    let mut args = Args::new(&args_vec);

    println!("Program: {}", args_vec[0]);
    println!("Command: {}", args.get(1));
    println!("Args count: {}", args_vec.len());

    println!("\nFlag checks:");
    println!("  Has --verbose: {}", args.has("--verbose"));
    println!("  Has --quiet: {}", args.has("--quiet"));
    println!("  Has -d: {}", args.has("-d"));

    println!("\nOption values:");
    let config_val = args.has_val("--config").unwrap_or("none".to_string());
    println!("  --config value: {}", config_val);

    println!("\nPositional access:");
    println!("  args[0]: {}", args.get_or(0, "missing"));
    println!("  args[1]: {}", args.get_or(1, "missing"));
    println!("  args[2]: {}", args.get_or(2, "missing"));
}

#[test]
fn uat_cli_options_demo() {
    println!("\n=== UAT: CLI Options Processing Demo ===");

    let args_vec = vec![
        "rsb-tool".to_string(),
        "build".to_string(),
        "--target=release".to_string(),
        "--features=visual,progress".to_string(),
        "--jobs=4".to_string(),
        "-v".to_string(),
        "--".to_string(),
        "extra".to_string(),
        "args".to_string(),
    ];

    let args = Args::new(&args_vec);

    // Process into options
    rsb::cli::options(&args);

    println!("Options processed into global variables:");
    println!("  opt_target: {}", rsb::global::get_var("opt_target"));
    println!("  opt_features: {}", rsb::global::get_var("opt_features"));
    println!("  opt_jobs: {}", rsb::global::get_var("opt_jobs"));
    println!("  opt_v: {}", rsb::global::get_var("opt_v"));

    println!("\nRemaining args: {:?}", args.remaining());
}

#[test]
fn uat_cli_dispatch_demo() {
    println!("\n=== UAT: CLI Dispatch Demo ===");

    // Simulate command dispatch
    fn cmd_hello(_args: &Args) -> i32 {
        println!("  → Executing hello command");
        0
    }

    fn cmd_build(_args: &Args) -> i32 {
        println!("  → Executing build command");
        0
    }

    let test_cases = vec![
        vec!["prog", "hello"],
        vec!["prog", "build", "--release"],
        vec!["prog", "unknown"],
    ];

    for case in test_cases {
        let args_vec: Vec<String> = case.iter().map(|s| s.to_string()).collect();
        let args = Args::new(&args_vec);

        println!("\nDispatching: {:?}", case);
        let command = args.get(1);
        let result = match command.as_str() {
            "hello" => cmd_hello(&args),
            "build" => cmd_build(&args),
            _ => {
                println!("  → Unknown command: {}", command);
                1
            }
        };
        println!("  Exit code: {}", result);
    }
}
