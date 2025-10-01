# RSB (Rebel String-Based) Rust

**RSB is an opinionated Rust framework and architecture for writing quick, robust, and portable UNIX-like tools.**

RSB is designed for the "too big for bash, too small for Rust" sweet spot — tools and scripts that need more structure and power than shell scripting can offer, but without the ceremonial complexity of mathmetically correct Rust applications. 


## Start Here

| Category | Description | Link |
|---|---|---|
| Docs Index | Entry point to all feature and reference docs | [docs/tech/INDEX.md](docs/tech/INDEX.md) |
| Dev Guide | How to update/refactor RSB safely | [HOWTO_UPDATE_RSB.md](docs/tech/development/HOWTO_UPDATE_RSB.md) |
| Dev Guide | Module specification and alignment requirements | [MODULE_SPEC.md](docs/tech/development/MODULE_SPEC.md) |
| Dev Guide | How to run tests and use the runner | [HOWTO_TEST.md](docs/tech/development/HOWTO_TEST.md) |
| Reference | Architecture overview and standards | [RSB_ARCH.md](docs/tech/reference/RSB_ARCH.md) |
| Reference | REBEL philosophy and principles | [REBEL.md](docs/tech/reference/REBEL.md) |

## The REBEL Philosophy

RSB is the official implementation of the **REBEL (Rust Equalized Beyond Esoteric Lingo)** philosophy, which prioritizes practitioner productivity over academic purity, through an ergonomic batteries-included BASH-like macro DSL and conventions. 

- **Accessibility Over Purity:** Familiar, bash-like patterns are chosen over complex Rust idioms to lower the cognitive load. If you know shell scripting, you should feel at home.
- **Pragmatic Safety:** Instead of relying solely on Rust's type system to prevent errors at compile-time, RSB provides a rich set of runtime validation functions and clear error messages.
- **Fail Fast, Fail Clear:** When something goes wrong, RSB exits with a helpful, colored message. No more wrestling with nested `Result` types for simple scripts.
- **String-First Design:** Strings are the universal interface of the shell. RSB embraces this, providing powerful tools to work with strings, rather than forcing you to convert everything into complex types.

RSB's focus on enabling productivity means developers can build meaningful tools faster without having to pay the heavy up-front cost of translating walls of alien hieroglyphics; instead, developers can defer the elaborate hula dances and slowly integrate oxidized patterns as they find their groove. The REBEL approach to Rust is to knock down the barries and lower the way-too-high gates for anyone dangerous enough to find their way into Rustland. Learning is a progressive cycle of experimenting, trying, failing and +1-ing your approach; beyond making tools RSB enables this learning evolution humans need to eventually understand the 5 lines of alien hieroglyphics that goes into defining mathmetically precise Rust functions.

RSB is for builders, for automators, for practitioners who need to "get stuff done."

## How RSB Differs from Standard Rust

| Standard Rust | RSB Approach |
|---------------|--------------|
| `Result<T, E>` error handling | `validate!()` macro with immediate exits |
| Complex type conversions | String-based operations with auto-expansion |
| Manual argument parsing | Built-in `Args` struct with bash-like access |
| Verbose file operations | `cat!()`, `write_file()` macros |
| Manual process management | `cmd!()`, `job!()` macros |
| Custom CLI frameworks | Built-in `dispatch!()` system |

## When to Use RSB

**Perfect For:**
- Build scripts and automation
- CLI tools and system utilities  
- Data processing pipelines
- Configuration management tools
- Deployment and installation scripts

**Not Ideal For:**
- Web servers or network services
- Game engines or real-time systems
- Libraries consumed by other Rust projects
- Performance-critical applications requiring zero-cost abstractions

## Core Architecture

RSB's design is heavily inspired by the mature **BashFX architecture**, incorporating battle-tested patterns for creating maintainable scripts:

- **XDG+ Compliance:** RSB tools are self-contained within the `~/.local` directory structure, keeping your home directory clean.
- **Function Ordinality:** A strict hierarchy for functions (High-Order, Mid-Level, Low-Level) ensures a clear separation of concerns and a predictable call stack.
- **Sentinel-Based Operations:** A system for making safe, reversible changes to files (e.g., for installers).
- **"Thisness" Pattern:** A context system that allows for the creation of generic, reusable library functions.

## Features

- **Bash-like Syntax:** Write scripts that feel like shell scripts, but with the power and safety of Rust.
- **Rich Macro DSL:** Powerful macros distributed across domain modules:
  - `bash` macros for control flow (`test!`, `case!`, `for_in!`, `curl!`, `get!`)
  - `fs` macros for filesystem operations (`file_in!`, `require_file!`, `require_dir!`, `chmod!`, `tar!`, `zip!`)
  - `global` macros for variable/config operations (`echo!`, `printf!`, `export!`, `load_config!`, `require_var!`)
  - `hosts` macros for host/command operations (`require_command!`, `mock_cmd!`, `hostname!`, `user!`)
  - `string` macros for string operations (case conversions, `str_in!`, `str_explode!`)
  - `com` macros for validation (`validate!`, `is_true!`, `is_false!`)
- **Fluent Stream Processing:** Chain commands together to process text and data, just like Unix pipes.
- **Integrated Argument Parsing:** A simple yet powerful argument parser built-in.
- **Config File Loading:** Easily load `.env` or `.conf` style configuration files.
- **Colorized Output (optional):** Feature-gated colors/backgrounds/glyphs for rich terminal output.
- **Job Control:** Run and manage background tasks with timeout support.
- **Event System:** A flexible `trap!` system for handling OS signals, script exit, and command errors.
- **Advanced String & Math Utilities**: Built-in random data generation and floating-point math.

## 📚 Feature Documentation

Index of Feature Modules

| Feature | Description | Documentation |
|---------|-------------|---------------|
| **Truth/Booleans** | Rust-native boolean semantics and exit codes | [`FEATURES_TRUTH.md`](docs/tech/features/FEATURES_TRUTH.md) |
| **Global** | Global store and configuration management | [`FEATURES_GLOBAL.md`](docs/tech/features/FEATURES_GLOBAL.md) |
| **Strings** | String helpers, macros, Unicode behavior | [`FEATURES_STRINGS.md`](docs/tech/features/FEATURES_STRINGS.md) |
| **Parameters** | Parameter expansion (bash-like `${var}` patterns) | [`FEATURES_PARAMS.md`](docs/tech/features/FEATURES_PARAMS.md) |
| **Host** | Environment, XDG/RSB paths, script contexts | [`FEATURES_HOST.md`](docs/tech/features/FEATURES_HOST.md) |
| **CLI** | Args parsing, bootstrap, command dispatch | [`FEATURES_CLI.md`](docs/tech/features/FEATURES_CLI.md) |
| **Colors** | Terminal colors, backgrounds, visual styling | [`FEATURES_COLORS.md`](docs/tech/features/FEATURES_COLORS.md) |
| **Prompts** | Interactive prompts with timeout support | [`FEATURES_PROMPTS.md`](docs/tech/features/FEATURES_PROMPTS.md) |
| **Math** | 🔥 Comprehensive mathematical framework | [`FEATURES_MATH.md`](docs/tech/features/FEATURES_MATH.md) |
| **Date** | Date/time operations and formatting | [`FEATURES_DATE.md`](docs/tech/features/FEATURES_DATE.md) |
| **Bash** | Shell command execution and job control | [`FEATURES_BASH.md`](docs/tech/features/FEATURES_BASH.md) |
| **Threads** | Threading utilities and background tasks | [`FEATURES_THREADS.md`](docs/tech/features/FEATURES_THREADS.md) |
| **Tokens** | Token generation and parsing utilities | [`FEATURES_TOKENS.md`](docs/tech/features/FEATURES_TOKENS.md) |
| **Object** | JavaScript-like generic object with global store integration | [`FEATURES_OBJECT.md`](docs/tech/features/FEATURES_OBJECT.md) |
| **TOML** | Cargo.toml metadata extraction and namespace management | [`FEATURES_TOML.md`](docs/tech/features/FEATURES_TOML.md) |
| **REPL** | Interactive command processing with repl_dispatch! | [`FEATURES_REPL.md`](docs/tech/features/FEATURES_REPL.md) |
| **Options** | Configuration options and feature flags | [`FEATURES_OPTIONS.md`](docs/tech/features/FEATURES_OPTIONS.md) |
| **Progress** | Modular progress indicators (spinner/bar/bytes) with color customization | [`FEATURES_PROGRESS.md`](docs/tech/features/FEATURES_PROGRESS.md) |
| **FS** | File system operations and helpers | [`FEATURES_FS.md`](docs/tech/features/FEATURES_FS.md) |

## Cargo Features

Feature flags are explicit and modular. Enable only what you need:

```
[features]
default = []

# Visual base + packages
visual = []
colors-simple = ["visual"]
colors-status = ["visual"]
colors-named  = ["visual", "colors-simple"]
colors-all    = ["visual", "colors-simple", "colors-named", "colors-status"]
colors        = ["visual", "colors-simple"]        # convenience baseline

# Other visual components
glyphs  = ["visual"]
prompts = ["visual", "colors-simple"]

# Umbrella for everything visual (colors + glyphs + prompts)
visuals = [
  "visual",
  "colors-simple", "colors-named", "colors-status",
  "glyphs", "prompts",
]

# Progress indicators (zero‑dep core)
progress = []

# Development/testing utilities (opt‑in)
dev-pty = [ ]  # PTY wrapper for interactive tests (rsb::dev::pty)

# Dependency re‑exports via rsb::deps (per‑dep opt‑ins + umbrella)
deps-base64 = []
deps-chrono = []
deps-glob = []
deps-lazy_static = []
deps-libc = []
deps-rand = []
deps-regex = []
deps-serde = []
deps-serde_json = []
deps-urlencoding = []
deps-uuid = []
deps-all = [
  "deps-base64","deps-chrono","deps-glob","deps-lazy_static","deps-libc",
  "deps-rand","deps-regex","deps-serde","deps-serde_json","deps-urlencoding","deps-uuid",
]
deps = ["deps-all"]      # convenience umbrella
```

Examples
- Visual demos: `cargo test --features visuals`
- Progress: `cargo test --features progress`
- PTY-backed tests: `cargo test --features dev-pty`
- Use a specific dep through RSB: `cargo test --features deps-chrono` then `use rsb::deps::chrono;`
- Enable all deps: `cargo test --features deps` then `use rsb::deps::*;`

## Testing

RSB uses a sophisticated BASHFX-aligned testing framework with module-based organization:

### Running Tests

```bash
# Run all tests
./bin/test.sh run all

# Run specific test category
./bin/test.sh run uat
./bin/test.sh run sanity

# NEW: Run module-specific tests
./bin/test.sh run uat math        # Run only math UAT tests
./bin/test.sh run sanity tokens   # Run only tokens sanity tests

# List available tests
./bin/test.sh list

# Check test organization compliance
./bin/test.sh lint
```

### Test Organization

Tests are organized by category and module with strict naming conventions:

- **UAT functions**: `uat_<module>_<description>()` (e.g., `uat_math_basic_demo`)
- **SANITY functions**: `sanity_<module>_<description>()` (e.g., `sanity_math_basic`)

This enables precise module filtering: `./bin/test.sh run uat math` runs all functions matching `uat_math_*`.

### Test Categories

- **sanity**: Core functionality validation (REQUIRED for all modules)
- **uat**: User Acceptance Tests with visual ceremony
- **unit**: Fast, isolated module tests
- **integration**: Cross-module interaction tests
- **smoke**: Minimal CI tests (<10s total)

See [`HOWTO_TEST.md`](docs/tech/development/HOWTO_TEST.md) for complete testing guide.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full guide.

- Follow the prelude policy: optional subsystems (visuals, logging, progress, etc.) must not be exported via the prelude.
- For new or changed behavior in a module, update or add a feature guide under `docs/tech/features/FEATURES_<NAME>.md`.
- Keep the index up to date: add your feature doc to `docs/tech/INDEX.md` (use relative links, table format).
- Align Cargo features in both `Cargo.toml` and the README “Cargo Features” table when adding/removing flags.
- For potentially user-visible behavior changes (e.g., formatting in progress output), open a brief RFC/discussion before changing defaults.

## Getting Started

### 1. Add RSB to your project

```toml
# Cargo.toml
[dependencies]
rsb = { path = "path/to/rsb/crate" } # Or from crates.io when published
```

### 2. Write your first script

Create a new Rust binary project and add the following to your `main.rs`:

```rust
// main.rs
use rsb::prelude::*;

fn main() {
    // The bootstrap! macro handles collecting args, loading the environment,
    // and setting up the context all in one go.
    let args = bootstrap!();

    // Dispatch commands to their handler functions.
    dispatch!(&args, {
        "hello" => say_hello,
        "process" => process_files
    });
}

// A simple command handler
fn say_hello(args: Args) -> i32 {
    let name = args.get_or(1, "World");
    info!("Preparing to greet...");
    echo!("Hello, {}!", name);
    okay!("Greeting successful.");
    0
}

// A more complex handler showcasing stream processing
fn process_files(_args: Args) -> i32 {
    write_file("data.txt", "line 1\nline 2\nERROR: bad line\nline 4");
    require_file!("data.txt");
    info!("Processing data.txt...");

    let error_count = cat!("data.txt")
        .grep("ERROR")
        .tee("errors.log")
        .count();

    if error_count > 0 {
        error!("Found {} errors. See errors.log for details.", error_count);
        return 1;
    }

    okay!("No errors found.");
    0
}
```

### 3. Run it!

```sh
$ cargo run -- hello RSB
# ℹ Preparing to greet...
# Hello, RSB!
# ✓ Greeting successful.
```

## API Reference

### Macro Organization

Macros are now organized into domain-specific modules:
- `use rsb::bash::macros::*` for bash-style control flow
- `use rsb::fs::macros::*` for filesystem operations
- `use rsb::global::macros::*` for global variable and config operations
- `use rsb::hosts::macros::*` for host and command utilities
- `use rsb::string::macros::*` for string manipulation
- `use rsb::com::macros::*` for validation macros

All macros are re-exported via `rsb::prelude::*` for convenience.

### Core & Bootstrap

- **`bootstrap!() -> Vec<String>`**: Initializes the RSB environment (loads env vars, sets up paths) and returns the command-line arguments. The one-stop-shop for starting your script.
- **`args!() -> Vec<String>`**: A standalone macro to just get the command-line arguments.
- **`dispatch!(&args, { ... })`**: The main command router. Takes the arguments and a block mapping command strings to handler functions.
- **`pre_dispatch!(&args, { ... })`**: A secondary dispatcher for "bootstrap" commands (like `install` or `init`) that should run before config files are loaded.

### Logging & Output

- **`info!(...)`**: For general informational messages.
- **`okay!(...)`**: For success messages.
- **`warn!(...)`**: For warnings.
- **`error!(...)`**: For non-fatal errors.
- **`fatal!(...)`**: For fatal errors (red 💀).
- **`debug!(...)`**: For debug messages (grey 🔍).
- **`trace!(...)`**: For trace-level messages (magenta 👁).
- **`echo!(...)`**: Prints to `stdout`. Use this for output that needs to be piped or captured.
- **`printf!(...)`**: Like `echo!` but without a trailing newline.
- **`line!('-', 20)`**: Creates a string by repeating a character.
- **`clear!()`**: Clears the terminal screen.

### Variable & Config Management

- **`set_var(key, value)` / `get_var(key)`**: Get or set variables in the global context.
- **`param!(...)`**: A powerful macro for bash-style parameter expansion (e.g., `param!("VAR", default: "val")`, `param!("VAR", suffix: ".txt")`).
- **`src!(path, ...)` / `load_config!(path, ...)`**: Loads variables from one or more configuration files.
- **`export!(path)`**: Saves all context variables to a file in `export` format.
- **`meta_key!(path, key)`**: Extracts a single metadata value from a file's header comments.
- **`meta_keys!(path, into: "META")`**: Parses `# key: value` comments from a file and loads them into an associative array named `META`.

### Array & Dictionary Utilities
- **`set_array(name, &["a", "b"])`**: Creates an array variable.
- **`get_array(name) -> Vec<String>`**: Retrieves an array.
- **`array_push(name, item)`**: Appends an item to an array.
- **`for_in!(item in "ARRAY_NAME" => { ... })`**: Iterates over an RSB array.
- **`dict!(path)`**: Reads a whitespace-delimited file into a `Vec<String>`.
- **`gen_dict!(type, n, into: array_name)`**: Generates an array of `n` random "words" of a given `type` (`alnum`, `hex`, etc.).
- **`rand_dict!(array_name)`**: Returns a single random word from an array.

### Stream Processing
- **Sources**: `cat!(path)`, `cmd!(command)`, `pipe!(string)`, `stream!(array: &vec)`.
- **Methods**: `.grep()`, `.sed()`, `.cut()`, `.sort()`, `.unique()`, `.tee(path)`, `.to_file(path)`, `.each(|line| ...)`
- **`sed_block!(start, end, sed_expr)`**: Applies a `sed`-style substitution to a block of text between two patterns.
- **`cap_stream!(stream)` / `subst!(stream)`**: Captures a stream's output to a temporary file and returns the path. Useful for commands that require file paths instead of stdin (e.g., `diff`).

### Conditional Logic
- **`validate!(condition, message)`**: Exits with an error if the condition is false.
- **`require_file!(path)`**: Exits if the file does not exist.
- **`test!(...)`**: A comprehensive macro for bash-style tests (e.g., `test!(-f "file")`, `test!(var -gt 10)`).
- **`case!(value, { ... })`**: A shell-style `case` statement with regex pattern matching.

### System, Time & Math
- **`sleep!(1)` / `sleep!(ms: 100)`**: Pauses execution.
- **`date!(iso)` / `date!(epoch)` / `date!("%Y-%m-%d")`**: Gets the current time in various formats.
- **`benchmark!({ ... })`**: Measures the execution time of a code block.
- **`math!("VAR = (A + B) * 2")`**: Evaluates a mathematical expression (with float support) and assigns the result to a variable.
- **`tmp!()` / `tmp!(pid)`**: Generates a temporary file path.

### Job Control
- **`job!(background: "...")`**: Runs a command in the background and returns a job ID.
- **`job!(wait: id)`**: Waits for a background job to complete.
- **`job!(timeout: 10, wait: id)`**: Waits for a background job with a timeout in seconds.
- **`job!(list)`**: Lists running background jobs.

### Event Handling
- **`trap!(|| ..., on: "SIGINT")`**: Traps OS signals.
- **`trap!(|| ..., on: "EXIT")`**: Executes a handler when the script exits.
- **`trap!(|data| ..., on: "COMMAND_ERROR")`**: Executes a handler when `cmd!` or `shell!` fails. The `data` argument contains `source`, `command`, `status`, and `stderr`.

### Random Data Generation
- **`rand_alnum!(n)`**: Generates `n` random alphanumeric characters.
- **`rand_alpha!(n)`**: Generates `n` random alphabetic characters.
- **`rand_hex!(n)`**: Generates `n` random hex characters.
- **`rand_string!(n)`**: Generates `n` random printable characters.
- **`rand_uuid!()`**: Generates a v4 UUID.

Welcome to a more rebellious, productive way of writing scripts in Rust.

## Toolchain

This repo includes a `rust-toolchain.toml` that pins the toolchain and common components:

```
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
```

Commands
- Format: `cargo fmt` (uses rustfmt component)
- Lint: `cargo clippy -- -D warnings` (treat clippy warnings as errors)

Notes
- rustup will automatically use the pinned toolchain/channel when you run Cargo.
- Having a consistent toolchain helps ensure reproducible builds locally and in CI.

## Running Tests Quickly

### Basic Testing Commands
- List suites: `./bin/test.sh list`
- Sanity: `./bin/test.sh run sanity`
- Host env UAT: `./bin/test.sh run host-env`
- Host paths UAT: `./bin/test.sh run host-paths`
- CLI macros E2E: `./bin/test.sh run cli`

### Module-Based Testing (New)
RSB now supports targeted testing by category and module for more efficient development:

```bash
# Run all math UAT tests
./bin/test.sh run uat math

# Run all tokens sanity tests
./bin/test.sh run sanity tokens

# Run all UAT tests across modules
./bin/test.sh run uat

# Run all sanity tests across modules
./bin/test.sh run sanity

# Legacy syntax still supported
./bin/test.sh run sanity
./bin/test.sh run uat-math
```

### Test Function Naming Standards
All tests must follow standardized naming patterns for module-based discovery:

- **UAT functions**: `uat_<module>_<description>()` (e.g., `uat_math_basic_demo`, `uat_tokens_validation_demo`)
- **SANITY functions**: `sanity_<module>_<description>()` (e.g., `sanity_math_basic`, `sanity_tokens_parsing`)

### Benefits of Module-Based Testing
- **Targeted Development**: Test only the module you're working on
- **Faster Feedback**: Skip unrelated test suites during development
- **Better Organization**: Clear separation between module testing categories
- **Easier Debugging**: Isolate issues to specific functional areas

## License

RSB Framework, Oxidex (ODX), and REBEL libraries, services, and software are offered under a **multi-license model**:

| License | Who it’s for | Obligations |
|---------|--------------|-------------|
| [AGPL-3.0](./LICENSE) | Open-source projects that agree to release their own source code under the AGPL. | Must comply with the AGPL for any distribution or network service. |
| [Community Edition License](./docs/LICENSE_COMMUNITY.txt) | Personal, educational, or academic use **only**. Not for companies, organizations, or anyone acting for the benefit of a business. | Must meet all CE eligibility requirements and follow its terms. |
| [Commercial License](./docs/LICENSE_COMMERCIAL.txt) | Companies, contractors, or anyone needing to embed the software in closed-source, SaaS, or other commercial products. | Requires a signed commercial agreement with Dr. Vegajunk Hackware. |

By **downloading, installing, linking to, or otherwise using RSB Framework, Oxidex, or REBEL**, you:

1. **Accept** the terms of one of the licenses above, **and**  
2. **Represent that you meet all eligibility requirements** for the license you have chosen.

> Questions about eligibility or commercial licensing: **licensing@vegajunk.com**
