# RSB CLI (Args, Bootstrap, Dispatch Surfaces)

Updated: 2025-09-12

Purpose
- Provide ergonomic, bash‑like CLI utilities for RSB binaries.
- Wrap command‑line arguments (`Args`) with helpers similar to shell usage.
- Offer a simple CLI bootstrap that builds on the Host bootstrap.
- Coordinate with macro surfaces (`bootstrap!`, `options!`, `dispatch!`).

Imports
```rust
use rsb::prelude::*;    // Includes Args (re-export of cli::Args) and macros
use rsb::cli;           // CLI module (bootstrap helpers, optional)
use rsb::cli::Args;     // Explicit struct import when avoiding the prelude
```

Core API
- Args (bash‑style)
  - `Args::new(&[String])` — construct wrapper from raw args.
  - `Args::from_strs(&[&str])` — convenience method for creating `Args` from string slices, particularly useful in tests.
  - `get(n)`, `get_or(n, default)` — positional access (1‑indexed, skips `argv[0]`).
  - `has(flag)`, `has_pop(flag)` — presence and consume.
  - `has_val(flag)` — supports `--flag=value` and `--flag value`.
  - `get_kv(key)` / `get_array(key)` — parse `key=value` or `key:a,b,c`.
  - `remaining()`, `all()`, `join(sep)`, `len()`.
  - `expand(template)` — expand `$1..$N`, `$@`, `$#`, then Global `$VARS`.
- Bootstrap
  - `cli::cli_bootstrap(args)` — runs `hosts::bootstrap(args)` then applies CLI extensions (hook point).
    - NEW in v0.7.0+: Automatically calls `cli_to_global()` to store args in global
  - `cli::cli_bootstrap_from_env()` — convenience wrapper using `std::env::args()`.
  - `cli::cli_to_global(args)` — stores CLI args in global with bash-style 1-based indexing (v0.7.0+)
- Options helper
  - `cli::options(&Args)` — entry point used by `options!` to hydrate stdopts/global flags.
  - `cli::has_option(&Args, opt)` / `cli::get_option_value(&Args, opt)` — helpers surfaced for custom option walkers.
- Dispatch/Options/Help
  - Enhanced dispatch system with smart error handling and command suggestions
  - Built-in commands: `help`, `inspect`, `stack` for introspection
  - Unknown command errors provide intelligent suggestions based on edit distance
  - See `FEATURES_OPTIONS.md` for options parsing features and macros.
  - Core macros `bootstrap!`, `dispatch!`, etc., live under `src/macros/` and integrate with these utilities.
- Dispatch internals (for advanced integration/testing)
  - `CommandHandler = fn(Args) -> i32` — canonical handler signature.
  - `cli::execute_dispatch(&Args, lookup)` — macro-backed dispatcher that handles built-ins and exits with handler codes.
  - `cli::execute_pre_dispatch(&Args, lookup) -> bool` — test-friendly dispatcher (no process exit when `CARGO_TEST` is set).
  - `cli::register_handlers(&[(&str, CommandHandler)])` — populates the Global registry used by `inspect` and docs.

CLI Arguments in Global Store (v0.7.0+)
- Bootstrap automatically stores CLI arguments in the global store for bash-like access
- Arguments follow bash convention: 1-based indexing, $0 is program name
- Stored variables:
  - `CLI_0` — program name (argv[0])
  - `CLI_1`, `CLI_2`, ... — positional arguments
  - `CLI_COUNT` — count of arguments (excluding program name)
  - `CLI_ALL` — all arguments as semicolon-separated string
- Access via macros (see below) or directly via `get_var("CLI_1")` etc.
- Enables bash-style scripting patterns in Rust code
- Automatically handled by `bootstrap!()` macro - no manual setup needed

Examples
```rust
use rsb::prelude::*;

fn main() {
    // Easiest: use the macro → returns Args
    let args = bootstrap!();

    if args.has("--verbose") { rsb::global::set_var("VERBOSE_MODE", "1"); }
    let cfg = args.has_val("--config").unwrap_or_else(|| "$XDG_ETC_HOME/app.conf".into());
    println!("Using config: {}", rsb::global::expand_vars(&cfg));
}

// Convenience method for tests or simple cases
fn test_example() {
    // Creating Args directly from string slices
    let test_args = Args::from_strs(&["--verbose", "build", "debug"]);
    assert!(test_args.has("--verbose"));
    assert_eq!(test_args.get(1), "build");
}
```

Enhanced Dispatch System
- Intelligent error handling with command suggestions using edit distance algorithm
- Built-in commands automatically available: `help`, `inspect`, `stack`
- Vanity descriptions support via `desc: "..."` syntax in dispatch macros
- Command registry integration for documentation and introspection
- Error messages provide "Did you mean?" suggestions for typos
- Colorized output when visual features are enabled (auto-strips tags in plain mode)
- Test-friendly `pre_dispatch!` variant that returns bool instead of exiting

Enhanced Dispatch Examples
```rust
use rsb::prelude::*;

fn main() {
    let args = bootstrap!();

    // Dispatch with automatic error handling
    // You can attach vanity descriptions for `inspect` via `desc:`
    dispatch!(&args, {
        "build" => build_command, desc: "Build a target (default: debug)",
        "test"  => test_command,  desc: "Run the test suite",
        "help"  => help_command
    });
}

// Built-in commands available:
// - "help": Shows help information
// - "inspect": Lists registered command handlers (shows descriptions when provided)
// - "stack": Shows call stack for debugging

// Unknown command example:
// $ myapp buld
// Error: Unknown command 'buld'
//
// Did you mean one of these?
//   build
//
// Use 'help' to see all available commands.
// Use 'inspect' to see registered command handlers.

fn build_command(args: Args) -> i32 {
    let target = args.get_or(1, "debug");
    println!("Building target: {}", target);
    0  // Success exit code
}
```

Vanity Descriptions
- `dispatch!` and `pre_dispatch!` support optional `desc: "..."` after each handler mapping. Descriptions are registered via `global::register_function(name, desc)` and shown by the `inspect` built-in.
- Both macros also auto‑register the handler names to power built‑ins like `inspect` without requiring separate calls.
- You may also register functions manually anywhere prior to dispatch:
  ```rust
  rsb::global::register_function("demo", "Runs the uat demo");
  ```

Macros (front doors)
- `bootstrap!()` — bootstrap host + CLI, returning `Args`.
  - v0.7.0+: Now automatically stores args in global via `cli_to_global()`
- `args!()` / `appref!()` — raw `std::env::args()` helpers used in minimal binaries.
- `options!(&args)` — invoke stdopts parser (`cli::options`).
- `dispatch!(&args, { ... })` — register handlers + execute dispatcher with vanity descriptions.
- `pre_dispatch!` — same mapping syntax, but returns `bool` for tests or staged execution.
- CLI Args Access Macros (v0.7.0+):
  - `cli_arg!(n)` — get CLI argument by position (1-based, bash convention)
  - `cli_argc!()` — get count of args (excluding program name)
  - `cli_args!()` — get all args as semicolon-separated string
  - `cli_argv!()` — get all args as Vec<String>
  - `cli_prog!()` — get program name (argv[0])
  - `cli_has_arg!(n)` — check if argument exists at position n
  - These macros provide bash-like $1, $2, $#, $@ access patterns
  - Work seamlessly with args stored in global by `cli_to_global()`
  - Enable writing script-like logic in Rust with familiar conventions

Help/Inspect Output
- Built-in `help`, `inspect`, and `stack` employ inline color tags for readability. When compiled with `--features visuals`, colors and styles render. Without visuals, tags are stripped so output remains clean in plain mode.

Testing & UAT
- Args behavior is exercised broadly via sanity and options tests (`tests/sanity.rs`, `tests/options.rs`, stdopts feature tests).
- CLI bootstrap hooks into Host bootstrap; see `FEATURES_HOST.md` and host UATs.

Notes
- Use `Args` from the prelude (re-export of `rsb::cli::Args`).
- CLI bootstrap intentionally delegates environment discovery and script context to the Host layer.
- Dispatch and richer helpers live under `src/cli/` and integrate with macros; expand incrementally as needed.
- When writing tests or embedding CLI surfaces in other tooling, prefer `pre_dispatch!`/`execute_pre_dispatch` so you can assert on the outcome without triggering `process::exit`.

<!-- feat:cli -->

_Generated by bin/feat.py --update-doc._

* `src/cli/args.rs`
  - struct Args (line 6)
  - fn new (line 12)
  - fn from_strs (line 20) — convenience method for creating Args from &[&str]
  - fn get (line 40)
  - fn get_or (line 50)
  - fn has (line 59)
  - fn has_pop (line 66)
  - fn has_val (line 83)
  - fn get_kv (line 109)
  - fn get_array (line 126)
  - fn remaining (line 131)
  - fn all (line 134)
  - fn join (line 137)
  - fn len (line 140)
  - fn expand (line 145)

* `src/cli/bootstrap.rs`
  - fn cli_to_global (line 11)
  - fn cli_bootstrap (line 54)
  - fn cli_bootstrap_from_env (line 65)

* `src/cli/dispatch.rs`
  - type CommandHandler (line 10)
  - fn execute_dispatch (line 20)
  - fn execute_pre_dispatch (line 60)
  - fn register_handlers (line 91)

* `src/cli/macros.rs`
  - macro bootstrap! (line 5)
  - macro args! (line 14)
  - macro appref! (line 21)
  - macro options! (line 29)
  - macro dispatch! (line 37)
  - macro pre_dispatch! (line 62)
  - macro cli_arg! (line 104)
  - macro cli_argc! (line 113)
  - macro cli_args! (line 126)
  - macro cli_argv! (line 134)
  - macro cli_prog! (line 147)
  - macro cli_has_arg! (line 155)

* `src/cli/mod.rs`
  - pub use utils::* (line 7)
  - pub use dispatch::* (line 10)
  - pub use args::* (line 13)
  - pub use help::* (line 16)
  - pub use bootstrap::* (line 19)
  - pub use options::* (line 22)

* `src/cli/options.rs`
  - fn options (line 32)
  - fn has_option (line 123)
  - fn get_option_value (line 138)

* `src/cli/utils.rs`
  - pub use super::helpers::* (line 7)

<!-- /feat:cli -->



