# RSB Quick Reference

*Practical patterns and code examples for immediate productivity*

## Essential Setup

```rust
use rsb::prelude::*;

fn main() {
    let args = bootstrap!();
    dispatch!(&args, {
        "build" => build_command,
        "test" => test_command
    });
}

fn build_command(args: Args) -> i32 {
    // Your command logic here
    0  // Return exit code
}

// Built-ins available via dispatch:
// - "help"    → usage + registered commands
// - "inspect" → list registered handlers with descriptions
// - "stack"   → show call stack (most recent first)

// Visuals (colors/glyphs/prompts) are optional:
// Enable with: `--features visuals` and env `RSB_COLOR=always RSB_COLORS=simple,status,named`
// For quick validation: `./bin/test.sh run uat-visual`

// Features at a glance:
// - visual (base), colors-simple, colors-named, colors-status
// - glyphs, prompts (opt-in)
// - visuals (umbrella: visual + colors + glyphs + prompts)
// - stdopts (short-flag expansion in options!)
```

## Core Patterns

### Argument Handling
```rust
fn handler(mut args: Args) -> i32 {
    // Positional arguments (1-indexed like bash)
    let target = args.get_or(1, "debug");           // $1 with default
    let project = args.get(2);                      // $2 or empty string
    
    // Flags
    let verbose = args.has_pop("--verbose");        // Check and remove flag
    let force = args.has("--force");                // Check without removing
    
    // Flag values: --output=dir or --output dir
    if let Some(output_dir) = args.has_val("--output") {
        set_var("OUTPUT_DIR", &output_dir);
    }
    
    // Key-value pairs: key=value or key:value  
    if let Some(env) = args.get_kv("env") {
        echo!("Environment: {}", env);
    }
    
    // Array values: features=feat1,feat2,feat3
    if let Some(features) = args.get_array("features") {
        echo!("Features: {}", features.join(", "));
    }
    
    // Remaining unprocessed args
    let files = args.remaining();
    
    0
}
```

### Variable Management
```rust
// Set and get variables
set_var("PROJECT_NAME", "my-app");
let name = get_var("PROJECT_NAME");

// Check if variable exists
if has_var("DEBUG_MODE") {
    #[cfg(feature = "visual")]
    debug!("Debug mode enabled");
}

// Parameter expansion with defaults
let build_dir = param!("BUILD_DIR", default: "/tmp/build");
let log_file = param!("PROJECT_NAME", suffix: ".log");
let config_path = param!("HOME", prefix: ".", suffix: "/.myapp.conf");

// Variable expansion in strings
echo!("Building $PROJECT_NAME in $BUILD_DIR");
let path = rsb::global::expand_vars("$HOME/.local/bin/$PROJECT_NAME");
```

### Stream Processing
```rust
// File operations
cat!("data.txt")
    .grep("error")
    .sed("ERROR", "⚠️")
    .tee("errors.log")
    .each(|line| warn!("{}", line));

// Command output
cmd!("ls -la /tmp")
    .grep("\\.log$")
    .cut(9, " ")
    .sort()
    .unique()
    .to_file("log_files.txt");

// String processing
pipe!("apple,banana,cherry")
    .sed(",", "\n")
    .grep("a")
    .head(2)
    .each(|line| echo!("Fruit: {}", line));

// Multi-file concatenation
cat!("file1.txt", "file2.txt", "file3.txt")
    .sort()
    .unique()
    .to_file("combined.txt");
```

### Validation and Testing
```rust
// File and directory checks
require_file!("config.yaml");
require_dir!("./src");
require_command!("git");
require_var!("HOME");

// Custom validation
validate!(args.len() > 0, "No arguments provided");
validate!(test!("42", -gt, "0"), "Value must be positive");

// Test conditions (bash-style)
if test!(-f "Cargo.toml") {
    #[cfg(feature = "visual")]
    info!("Found Rust project");
}

let count = "12";
if test!(count, -gt, "10") {
    #[cfg(feature = "visual")]
    warn!("Count is too high: {}", count);
}

// Case matching with regex
case!(get_var("ENVIRONMENT"), {
    "dev|development" => info!("Development mode"),
    "prod|production" => warn!("Production mode"),
    _ => error!("Unknown environment")
});
```

### Configuration Management
```rust
// Load configuration files
src!("~/.myapp.conf", "./myapp.conf");
load_config!("/etc/myapp/config");

// Array manipulation
set_array("SERVERS", &["web1", "web2", "db1"]);
let servers = get_array("SERVERS");
array_push("SERVERS", "cache1");

// Save configuration
save_config_file("./myapp.conf", &["SERVER_HOST", "SERVER_PORT", "SERVERS"]);
```

### Job Control and Events
```rust
// Background jobs
let job_id = job!(background: "long-running-command");
echo!("Started job: {}", job_id);

// Wait with timeout
if job!(timeout: 30, wait: job_id) {
    okay!("Job completed successfully");
} else {
    warn!("Job timed out");
}

// Event handling
trap!(|data: &EventData| {
    let cmd = data.data.get("command").unwrap();
    error!("Command failed: {}", cmd);
}, on: "COMMAND_ERROR");

trap!(|| {
    info!("Cleaning up temporary files...");
    rm_rf("$TEMP_DIR");
}, on: "EXIT");
```

### File System Operations
```rust
// Basic file operations
write_file("output.txt", "Hello, RSB!");
let content = read_file("input.txt");

// Directory operations
mkdir_p("path/to/deep/directory");
rm_rf("temporary-directory");

// File permissions
chmod!("executable", "+x");
chmod!("config.txt", "600");

// Path operations
let canon_path = path_canon!("../some/path");
path_split!("/usr/local/bin/tool", into: "TOOL_PATH");
echo!("Directory: {}", get_var("TOOL_PATH_parent"));
echo!("Filename: {}", get_var("TOOL_PATH_file_name"));
```

### Logging and Output
```rust
// Structured logging (colors/glyphs with visuals feature)
#[cfg(feature = "visual")]
{
    info!("Application starting...");           // ℹ blue
    okay!("Operation completed successfully");  // ✓ green  
    warn!("This might be a problem");          // ⚠ yellow
    error!("Something went wrong");            // ✗ red
    debug!("Detailed debug information");      // 🔍 grey
    trace!("Very detailed trace info");        // 👁 magenta
    // fatal! exits the process
}

// Plain output (for piping)
echo!("Hello, World!");                   // stdout
printf!("No newline");                    // stdout, no \n

// Colored manual output (visuals)
#[cfg(feature = "visual")]
{
    echo!("{green}Success!{reset}");
    echo!("{red}Error: {bold}critical failure{reset}");
}
```

### Math and Random Operations
```rust
// Mathematical expressions
set_var("A", "10");
set_var("B", "3.5");
math!("RESULT = (A + 5) * B / 2");  // 26.25
math!("RESULT += 10");              // 36.25

// Random data generation  
let uuid = rand_uuid!();
let token = rand_alnum!(32);
let hex_id = rand_hex!(16);
let password = rand_string!(12);

// Random number in range
let dice = rand_range!(1, 6);
```

### Archive Operations
```rust
// Auto-detect format based on extension
pack!("backup.tar.gz", "src/", "docs/", "README.md");
unpack!("backup.tar.gz", to: "restore/");

// Explicit format operations
tar!(create: "archive.tar", "file1", "file2", "directory/");
let contents = tar!(list: "archive.tar");

zip!(create: "package.zip", "build/", "assets/");
zip!(extract: "package.zip", to: "extracted/");
```

### System Information
```rust
// System queries
echo!("Hostname: {}", hostname!());
echo!("Current user: {}", user!());
echo!("Home directory: {}", home_dir!());
echo!("Working directory: {}", current_dir!());

// Process management
let pid = pid_of!("nginx");
if process_exists!("mysql") {
    #[cfg(feature = "visual")]
    info!("MySQL is running");
}

// Date and time
echo!("Current time: {}", date!());
echo!("Epoch time: {}", date!(epoch));
echo!("Custom format: {}", date!("%Y-%m-%d %H:%M:%S"));

// Benchmark operations
let duration = benchmark!({
    // Some operation to measure
    sleep!(ms: 100);
});
#[cfg(feature = "visual")]
info!("Operation took: {:?}", duration);
```

### Network Operations
```rust
// HTTP requests (requires curl)
let response = get!("https://api.example.com/data");
let html = get!("https://example.com", options: "-H 'User-Agent: MyApp/1.0'");

// JSON processing (requires jq)  
let json_data = r#"{"users": [{"name": "Alice"}, {"name": "Bob"}]}"#;
let first_user = json_get!(&json_data, ".users[0].name");
echo!("First user: {}", first_user);

// Save JSON field to file
let results = json_get_file!("response.json", ".data.results");
write_file("results.txt", &results);
```

### Advanced Text Processing
```rust
// Advanced sed operations
let result = sed_lines!("long text", 5, 10);          // Extract lines 5-10
let context = sed_around!("log text", "ERROR", 3);    // 3 lines around "ERROR"
let templated = sed_template!("Alice", "{{NAME}}", "Hello {{NAME}}!");

// File-based sed operations
sed_insert_file!("template.html", "new content", "<!-- INSERT HERE -->");
// Replace occurrences in file via Stream helpers
let cfg = read_file("config.txt");
let replaced = rsb::streams::Stream::from_string(&cfg).sed("old_value", "new_value").to_string();
write_file("config.txt", &replaced);
```

### Utilities and Helpers
```rust
// String utilities (var-based macros)
set_var("NAME", "  spaced  ");
let length = str_len!("NAME");
let trimmed = str_trim!("NAME");
str_explode!("comma,separated,values", on: ",", into: "WORDS");
let words = rsb::utils::get_array("WORDS");

// Temporary files
let temp_file = tmp!();          // Random temp file
let temp_with_pid = tmp!(pid);   // Include PID in name

// Locking mechanisms
with_lock!("/tmp/myapp.lock" => {
    #[cfg(feature = "visual")]
    info!("Critical section - only one instance running");
    // Your critical code here
});

// Path utilities
if test!(-e "./config") {
    #[cfg(feature = "visual")]
    info!("Config entity found");
}
```

## Common Patterns

### CLI Tool Structure
```rust
use rsb::prelude::*;

fn main() {
    let args = bootstrap!();
    
    // Handle pre-context commands
    if pre_dispatch!(&args, {
        "install" => install,
        "uninstall" => uninstall,
        "version" => show_version
    }) {
        return;
    }
    
    // Load configuration
    src!("~/.config/mytool.conf", "./mytool.conf");
    
    // Main command dispatch
    dispatch!(&args, {
        "build" => build,
        "deploy" => deploy,
        "status" => status,
        "logs" => logs
    });
}
```

### Error Handling Pattern
```rust
fn robust_operation(args: Args) -> i32 {
    // Validate preconditions
    require_file!("input.txt");
    require_command!("git");
    validate!(args.len() >= 1, "Usage: command <target>");
    
    // Set up error handling
    trap!(|data: &EventData| {
        let cmd = data.data.get("command").unwrap();
        error!("Command '{}' failed", cmd);
        set_var("HAS_ERRORS", "true");
    }, on: "COMMAND_ERROR");
    
    // Perform operations
    run!("git status");
    run!("make build");
    
    // Check for errors
    if has_var("HAS_ERRORS") {
        fatal!("Build failed due to errors");
        return 1;
    }
    
    okay!("Operation completed successfully");
    0
}
```

### Configuration-Driven Behavior
```rust
fn configurable_deploy(args: Args) -> i32 {
    // Load configuration with fallbacks
    src!("/etc/deploy.conf", "~/.deploy.conf", "./deploy.conf");
    
    // Command-line overrides config
    let environment = args.get_or(1, &get_var("DEFAULT_ENV"));
    let timeout = args.has_val("--timeout")
        .unwrap_or_else(|| param!("DEPLOY_TIMEOUT", default: "300"));
    
    // Use configuration
    let servers = get_array("SERVERS");
    for server in servers {
        info!("Deploying to {}", server);
        run!("rsync -av ./build/ {}:/var/www/", server);
    }
    
    0
}
```

## RSB Architecture Patterns

### Foundational Principles

#### The REBEL Philosophy
**"Rust Bends to Ease Life" - Prioritizing developer productivity over academic purity**

- **Developer Experience First**: If it feels familiar to shell scripters, it's preferred over "pure" Rust patterns
- **String-Biased Design**: Strings are the universal interface - embrace text processing over complex type systems
- **Pragmatic Safety**: Runtime validation with clear messages beats compile-time complexity for scripts
- **Immediate Clarity**: Fail fast with colored, descriptive error messages

#### Function Ordinality Hierarchy
**Strict three-tier function organization inspired by BashFX architecture:**

**High-Order Functions**
- **Purpose**: User-facing command handlers, main business logic
- **Signature**: `fn handler_name(args: Args) -> i32`  
- **Responsibility**: Orchestrate workflows, call mid-level functions

**Mid-Level Functions**  
- **Purpose**: Feature-specific operations, complex workflows
- **Responsibility**: Coordinate multiple low-level operations

**Low-Level Functions**
- **Purpose**: Atomic operations, primitive utilities
- **Responsibility**: Single-purpose operations

#### Context-Centric Design
**Global context replaces complex state management:**

```rust
// Global context automatically available
set_var("PROJECT_NAME", "my-app");
echo!("Building $PROJECT_NAME..."); // Auto-expands variables

// Context flows through the application
let expanded_path = rsb::global::expand_vars("$HOME/.local/bin/$PROJECT_NAME");
```

### Core Architecture Patterns

#### 1. Dual-Dispatch Pattern
**Separate pre-context and post-context command routing:**

```rust
fn main() {
    let args = bootstrap!();
    
    // Pre-dispatch: commands that run before config loading
    if pre_dispatch!(&args, {
        "install" => install_deps,
        "init" => init_project  
    }) {
        return;
    }
    
    // Load configuration after pre-commands
    src!("./app.conf");
    
    // Main dispatch: context-aware commands
    dispatch!(&args, {
        "build" => build_project,
        "deploy" => deploy_app
    });
}
```

#### 2. Stream-First Processing
**Unix pipeline philosophy with method chaining:**

```rust
// Traditional Unix pipeline: cat file | grep pattern | cut -f1 | sort | uniq
cat!("data.csv")
    .grep("active") 
    .cut(1, ",")
    .sort()
    .unique()
    .tee("processed.txt")
    .each(|line| echo!("User: {}", line));
```

#### 3. Sentinel-Based Operations  
**Safe, reversible file modifications using sentinels:**

```rust
// Insert content at a unique sentinel line in a file
sed_insert_file!("~/.bashrc", "export PATH=$PATH:~/.local/bin", "# RSB_PATH_ADDITION");

// Replace a sentinel marker with content (template mode)
sed_template_file("~/.bashrc", "export RUSTUP_HOME=$HOME/.rustup", "# RUSTUP_HOME_SENTINEL");
```

#### 4. Event-Driven Error Handling
**Trap system for centralized error management:**

```rust
// Set up global error handler
trap!(|data: &EventData| {
    let cmd = data.data.get("command").unwrap();
    let status = data.data.get("status").unwrap(); 
    error!("Command '{}' failed with status {}", cmd, status);
    math!("ERROR_COUNT += 1");
}, on: "COMMAND_ERROR");

// All cmd!() and run!() failures trigger the trap
run!("some-failing-command"); // Automatically handled
```

## RSB Compliance & Migration Guide

### Common RSB Anti-Patterns to Avoid

#### ❌ Complex Type Systems
```rust
// DON'T: Complex generics and type parameters
pub fn process<T: Clone + Send + Sync>(data: T) -> Result<ProcessedData<T>, ProcessingError>

// DO: String-based with runtime validation
pub fn process(data: &str) -> String {
    validate!(!data.is_empty(), "Input data cannot be empty");
    // ... processing
}
```

#### ❌ Nested Result Handling  
```rust
// DON'T: Complex error propagation
match read_file("config") {
    Ok(content) => match parse_config(content) {
        Ok(config) => process_config(config),
        Err(e) => return Err(e)
    },
    Err(e) => return Err(e)
}

// DO: Immediate validation
let content = read_file("config");
validate!(!content.is_empty(), "Config file is empty");
parse_config_content(&content);
```

#### ❌ Manual Argument Parsing
```rust  
// DON'T: Manual argv handling
let mut args = env::args().skip(1);
let command = args.next().unwrap_or("help".to_string());

// DO: RSB Args struct
let args = bootstrap!();
let command = args.get_or(1, "help");
```

### Migration from Non-RSB Patterns

#### From Clap to RSB Args
```rust
// OLD: Clap argument parser
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("MyApp")
        .subcommand(SubCommand::with_name("build")
            .arg(Arg::with_name("target")
                .short("t")
                .takes_value(true)))
        .get_matches();
        
    if let Some(build_matches) = matches.subcommand_matches("build") {
        let target = build_matches.value_of("target").unwrap_or("debug");
        // ...
    }
}

// ✅ FIXED: RSB Args pattern
use rsb::prelude::*;

fn main() {
    let args = bootstrap!();
    dispatch!(&args, {
        "build" => build_command
    });
}

fn build_command(mut args: Args) -> i32 {
    let target = args.has_val("--target").unwrap_or_else(|| "debug".to_string());
    // OR: let target = args.get_or(1, "debug");  // positional
    0
}
```

#### From Standard File I/O to RSB Streams
```rust
// OLD: Verbose file handling
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read");
    
    let processed = content.lines()
        .filter(|line| line.contains("error"))
        .collect::<Vec<_>>()
        .join("\n");
    
    let mut output = File::create("output.txt").expect("Failed to create");
    output.write_all(processed.as_bytes()).expect("Failed to write");
}

// ✅ FIXED: RSB stream processing
use rsb::prelude::*;

fn main() {
    let args = bootstrap!();
    dispatch!(&args, {
        "process" => process_command
    });
}

fn process_command(_args: Args) -> i32 {
    cat!("input.txt")
        .grep("error")
        .to_file("output.txt");
    
    okay!("Processing complete");
    0
}
```

### RSB Compliance Checklist

When building RSB-compliant applications:

1. **Start with `use rsb::prelude::*`** - Get the full RSB experience
2. **Use `bootstrap!()` for initialization** - Standard environment setup
3. **Structure commands with `dispatch!()`** - Consistent command routing  
4. **Prefer macros over functions** - `echo!()` not `println!()`, `cat!()` not `File::open()`
5. **Handle errors with `validate!()`** - Immediate assertions over Result chains
6. **Use context variables** - `set_var()`, `get_var()`, automatic `$VAR` expansion  
7. **Follow function ordinality** - High/Mid/Low level separation
8. **Return exit codes** - Functions return `i32`, main uses `std::process::exit()`

## Design Constraints

### 1. String-First Types
- **Primary Data Type**: `String` for all external interfaces
- **Conversion Philosophy**: Convert at boundaries, not internally
- **Text Processing**: Built-in regex, splitting, joining utilities
- **Avoid**: Complex type hierarchies, excessive generics

### 2. Immediate Error Handling
- **No Result Chains**: Use `validate!()` for immediate assertion-based exits
- **Clear Messages**: Always include context in error messages
- **Exit Codes**: Functions return `i32` exit codes, not `Result`
- **Colored Output**: Available when the `visuals` feature is enabled

### 3. Global State Management
- **Context-Centric**: Global `CTX` for all application state
- **Variable Expansion**: Automatic `$VAR` expansion in all string contexts
- **No Complex State**: Avoid complex state machines or ownership patterns
- **Simplicity**: If bash can do it simply, RSB should too

### 4. XDG+ Compliance
- **Path Standards**: All tools use `~/.local/*` directory structure
- **Self-Contained**: No global system modification without explicit user consent
- **Clean Installation**: Everything under user control, easy to remove

---

*The RSB architecture enables rapid development of reliable CLI tools while maintaining the safety and performance benefits of Rust.*

This reference covers the most common RSB patterns. For complete API documentation, see the README.md file.
