# REPL (Read-Eval-Print-Loop) Support

Interactive command processing with quote-aware parsing, built-in commands, and seamless dispatch integration.

## Overview

RSB REPL provides a flexible, modular REPL system with:
- Quote-aware command line parsing
- Pluggable parser architecture
- Built-in commands (exit, quit, clear, history, help)
- Dynamic prompt configuration
- Global argument storage
- Integration with dispatch! macro

## Basic Usage

### Simple REPL

```rust
use rsb::prelude::*;

fn main() {
    let repl = Repl::new();
    repl_dispatch!(repl, {
        "status" => cmd_status,
        "config" => cmd_config,
    })
}

fn cmd_status(args: Args) -> Result<i32, String> {
    println!("Status: OK");
    Ok(0)
}

fn cmd_config(args: Args) -> Result<i32, String> {
    let key = args.get(1);
    println!("Config key: {}", key);
    Ok(0)
}
```

### Integration with dispatch!

```rust
use rsb::prelude::*;

fn main() {
    let args = bootstrap!();

    dispatch!(&args, {
        "build" => cmd_build,
        "repl" => cmd_repl,
    });
}

fn cmd_build(args: Args) -> i32 {
    println!("Building...");
    0
}

fn cmd_repl(_args: Args) -> i32 {
    println!("Entering REPL mode\n");

    let repl = Repl::new();
    repl_dispatch!(repl, {
        "build" => repl_build,
        "test" => repl_test,
    })
}

fn repl_build(args: Args) -> Result<i32, String> {
    println!("REPL: Building...");
    Ok(0)
}

fn repl_test(args: Args) -> Result<i32, String> {
    println!("REPL: Running tests...");
    Ok(0)
}
```

## Core Components

### Repl Struct

The main REPL structure with dynamic configuration:

```rust
let repl = Repl::new();                    // Default configuration
let repl = Repl::with_prompt("myapp> ");   // Custom prompt
```

**Prompt Configuration Hierarchy:**
1. TOML: `rsb_repl_prompt` (via rsb_config!)
2. Environment: `RSB_REPL_PROMPT`
3. Default: `"repl> "`

**Dynamic Prompts:**
```rust
let mut repl = Repl::new();
repl.set_prompt("myapp:config> ");  // Update prompt dynamically
```

### Command Line Parsing

Quote-aware tokenization with pattern preservation:

```rust
// Input: build --output=dist "my file" items=a,b,c theme=dark;timeout=30
// Tokens: ["build", "--output=dist", "my file", "items=a,b,c", "theme=dark;timeout=30"]

let args = Args::from_line("build \"my file\" --verbose");
```

**Pattern Detection:**
- Quoted strings: `"my file.txt"` → preserves spaces
- Token patterns: `key=value`, `prefix:key=value`
- Comma lists: `items=a,b,c`
- Semicolon streams: `theme=dark;timeout=30`
- Flags: `--output=dist`, `--verbose`

### Built-in Commands

Available in all REPLs:

- `exit`, `quit` - Exit REPL mode
- `clear` - Clear REPL context variables (repl_*)
- `history` - Show command history
- `help` - Show built-in help message

### REPL Macros

Access command arguments from global storage:

```rust
// After command: "build target --verbose"

let cmd = repl_arg!(0);      // "build"
let arg1 = repl_arg!(1);     // "target"
let arg2 = repl_arg!(2);     // "--verbose"

let count = repl_argc!();    // 3

let args_str = repl_args!(); // "build;target;--verbose"

let argv = repl_argv!();     // vec!["build", "target", "--verbose"]
```

**Note:** REPL arguments are 0-indexed (unlike CLI args which are 1-indexed to skip program name)

## Advanced Features

### Custom Parsers

Implement pluggable parsing strategies:

```rust
use rsb::repl::{ReplParser, Repl};

struct MyCustomParser;

impl ReplParser for MyCustomParser {
    fn parse(&self, line: &str) -> Vec<String> {
        // Custom tokenization logic
        line.split('|').map(|s| s.trim().to_string()).collect()
    }
}

let parser = Box::new(MyCustomParser);
let repl = Repl::with_parser(parser);
```

### Error Handling

Handler functions return `Result<i32, String>`:

```rust
fn cmd_process(args: Args) -> Result<i32, String> {
    let file = args.get(1);
    if file.is_empty() {
        return Err("File argument required".to_string());
    }

    // Process file...
    Ok(0)
}
```

Errors are automatically printed and the REPL continues:

```
repl> process
Error: File argument required
repl>
```

### Command History

History is tracked automatically:

```rust
let mut repl = Repl::new();
repl.add_to_history("build".to_string());

// Users can type "history" command to see all commands
```

### Subcommand REPLs

Create nested REPL contexts:

```rust
fn cmd_config(args: Args) -> Result<i32, String> {
    let mut repl = Repl::with_prompt("myapp:config> ");

    repl_dispatch!(repl, {
        "set" => config_set,
        "get" => config_get,
        "back" => |_| Ok(0),  // Return to main REPL
    })
}
```

## Pattern Examples

### Quote Handling

```rust
// Input
"cmd \"my file.txt\" test"

// Tokens
["cmd", "my file.txt", "test"]
```

### Complex Patterns

```rust
// Input
"build --output=dist \"my file\" config:debug=true items=a,b,c theme=dark;timeout=30"

// Tokens
[
  "build",
  "--output=dist",
  "my file",
  "config:debug=true",
  "items=a,b,c",
  "theme=dark;timeout=30"
]
```

## Configuration

### Via TOML

```toml
[package.metadata.rsb]
repl_prompt = "myapp> "
```

### Via Environment

```bash
export RSB_REPL_PROMPT="dev> "
```

### Programmatic

```rust
let repl = Repl::with_prompt("custom> ");
```

## Integration Patterns

### CLI + REPL Hybrid

```rust
fn main() {
    let args = bootstrap!();

    if args.remaining().is_empty() {
        // No args? Enter REPL
        return cmd_repl(args);
    }

    dispatch!(&args, {
        "build" => cmd_build,
        "repl" => cmd_repl,
    });
}
```

### Context-Aware Prompts

```rust
fn main() {
    let mut repl = Repl::new();
    let env = get_var("APP_ENV");

    if env == "dev" {
        repl.set_prompt("dev> ");
    } else if env == "prod" {
        repl.set_prompt("prod> ");
    }

    repl_dispatch!(repl, { /* commands */ })
}
```

## Testing

### Test Command Parsing

```rust
#[test]
fn test_repl_parsing() {
    let args = Args::from_line("build \"my file\" --verbose");

    assert_eq!(args.all()[0], "build");
    assert_eq!(args.get(1), "my file");
    assert_eq!(args.get(2), "--verbose");
}
```

### Test REPL Macros

```rust
#[test]
#[serial]
fn test_repl_macros() {
    let args = Args::from_line("cmd arg1 arg2");
    store_repl_args_global(&args);

    assert_eq!(repl_arg!(0), "cmd");
    assert_eq!(repl_argc!(), 3);
    assert_eq!(repl_argv!(), vec!["cmd", "arg1", "arg2"]);
}
```

## Known Limitations (v1)

- No escaped quote support (e.g., `\"` inside quotes)
- No mixed quote types (single/double)
- No readline features (arrow keys, tab completion)
- History is in-memory only (not persisted)

See `docs/proposals/REPL_STRATEGY.md` for roadmap of future enhancements.

## See Also

- `examples/repl_demo.rs` - Complete integration example
- `docs/proposals/REPL_STRATEGY.md` - Design document
- `src/repl/` - Implementation details