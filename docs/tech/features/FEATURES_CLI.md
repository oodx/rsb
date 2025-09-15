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
```

Core API
- Args (bash‑style)
  - `Args::new(&[String])` — construct wrapper from raw args.
  - `get(n)`, `get_or(n, default)` — positional access (1‑indexed, skips `argv[0]`).
  - `has(flag)`, `has_pop(flag)` — presence and consume.
  - `has_val(flag)` — supports `--flag=value` and `--flag value`.
  - `get_kv(key)` / `get_array(key)` — parse `key=value` or `key:a,b,c`.
  - `remaining()`, `all()`, `join(sep)`, `len()`.
  - `expand(template)` — expand `$1..$N`, `$@`, `$#`, then Global `$VARS`.
- Bootstrap
  - `cli::cli_bootstrap(args)` — runs `hosts::bootstrap(args)` then applies CLI extensions (hook point).
  - `cli::cli_bootstrap_from_env()` — convenience wrapper using `std::env::args()`.
- Dispatch/Options/Help
  - Enhanced dispatch system with smart error handling and command suggestions
  - Built-in commands: `help`, `inspect`, `stack` for introspection
  - Unknown command errors provide intelligent suggestions based on edit distance
  - See `FEATURES_OPTIONS.md` for options parsing features and macros.
  - Core macros `bootstrap!`, `dispatch!`, etc., live under `src/macros/` and integrate with these utilities.

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
```

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

Help/Inspect Output
- Built-in `help`, `inspect`, and `stack` employ inline color tags for readability. When compiled with `--features visuals`, colors and styles render. Without visuals, tags are stripped so output remains clean in plain mode.

Testing & UAT
- Args behavior is exercised broadly via sanity and options tests (`tests/sanity.rs`, `tests/options.rs`, stdopts feature tests).
- CLI bootstrap hooks into Host bootstrap; see `FEATURES_HOST.md` and host UATs.

Notes
- Use `Args` from the prelude (re-export of `rsb::cli::Args`).
- CLI bootstrap intentionally delegates environment discovery and script context to the Host layer.
- Dispatch and richer helpers live under `src/cli/` and integrate with macros; expand incrementally as needed.
