# REPL Support Strategy

## Overview
This document defines the implementation strategy for RSB REPL (Read-Eval-Print-Loop) support, providing interactive command processing with meteor token support, dynamic prompts, and seamless integration with the existing dispatch system.

**Epic**: Task 3.1: REPL Support [20 SP base + additional tickets identified below]

---

## Design Decisions

### 1. Input Handling
**Decision**: Use `std::io::stdin().read_line()` with no external dependencies.

**Rationale**:
- Aligns with RSB philosophy of minimal dependencies
- Sufficient for v1 functionality
- Pluggable architecture allows future readline integration

**Implementation**:
```rust
pub fn read_line(&self) -> Option<String> {
    print!("{}", self.prompt);
    io::stdout().flush().ok()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line).ok()?;
    Some(line.trim().to_string())
}
```

**Future Enhancement** (Backlog):
- Add optional rustyline/similar integration for arrow key history, tab completion
- Feature flag: `repl-readline` for enhanced input

---

### 2. Command Tokenization
**Decision**: Pluggable parser architecture with pattern-aware tokenization.

**Core Philosophy**:
- v1 provides pattern detection without semantic parsing
- Patterns are preserved as single arguments for downstream handlers
- Pluggable `ReplParser` trait allows future enhancements (meteor, flags)

**v1 Tokenization Rules**:
1. **Quoted strings**: `"my file.txt"` → single arg (preserve spaces)
2. **Token patterns**: Detect and preserve (`:` and `=` indicators)
   - Simple tokens: `key=value`
   - Prefixed tokens: `prefix:key=value`
   - Namespaced tokens: `a.b.c:key=value`
3. **Token streams**: Detect and preserve (`;` delimiter)
   - Single stream: `key1=val1;key2=val2`
   - Prefixed stream: `prefix:k1=v1;k2=v2`
4. **Comma lists**: Detect and preserve (`,` without spaces)
   - Lists: `item1,item2,item3`
5. **Everything else**: Split on whitespace

**Example Tokenization**:
```rust
// Input:
build --output=dist "my file" config:debug=true items=a,b,c theme=dark;timeout=30

// Tokenized:
[
  "build",
  "--output=dist",      // Flag (future: will be parsed)
  "my file",            // Quoted string
  "config:debug=true",  // Single token with namespace
  "items=a,b,c",        // Comma list
  "theme=dark;timeout=30" // Token stream
]
```

**Pattern Detection Rationale**:
These patterns are preserved now to enable **future flag parsing**:
- `--flag item,item2,item3` → flag + comma-list value
- `--flag token;token2` → flag + token stream value
- `--flag key=value` → flag + single token value

**Parser Architecture**:
```rust
// src/repl/parser.rs

/// Trait for REPL command line parsing strategies
pub trait ReplParser: Send + Sync {
    /// Parse a command line into arguments
    fn parse(&self, line: &str) -> Vec<String>;
}

/// Simple parser with quote, token, and list pattern support (v1)
pub struct SimpleParser;

impl ReplParser for SimpleParser {
    fn parse(&self, line: &str) -> Vec<String> {
        // 1. Scan for quoted strings (mark ranges)
        // 2. Detect comma patterns (`,` without spaces)
        // 3. Detect semicolon patterns (`;` for token streams)
        // 4. Split remaining on whitespace
        // Result: correctly grouped args ready for future flag parsing
    }
}

impl Args {
    /// Create Args from command line using parser
    pub fn from_line(line: &str) -> Self {
        SimpleParser.parse(line)
    }
}

impl Repl {
    /// Create REPL with custom parser
    pub fn with_parser(parser: Box<dyn ReplParser>) -> Self {
        // Allows future: MeteorParser, FlagParser, etc.
    }
}
```

**Future Parser Implementations** (Backlog):
- `MeteorParser`: Full meteor tokenstream folding with namespace directives
- `FlagParser`: RSB flag parsing with value association
- `CompoundParser`: Chain multiple parsers (quotes → flags → meteor → whitespace)

---

### 3. State Management
**Decision**: Use global store with `repl_arg_*` pattern (0-indexed), no context HashMap.

**Global Storage Pattern**:
```rust
// Stored in global after each command:
repl_arg_0 = "command"
repl_arg_1 = "first_arg"
repl_arg_2 = "second_arg"
repl_argc = "3"
repl_args = "command;first_arg;second_arg"  // Semicolon-separated
```

**Rationale**:
- Consistent with `cli_arg_*` pattern from CLI Args to Global feature
- No need for separate context HashMap - global store is sufficient
- Isolation from CLI args (`cli_arg_*` vs `repl_arg_*`)

**Implementation**:
```rust
fn store_repl_args_global(args: &Args) {
    set_var("repl_argc", &args.len().to_string());

    // 0-indexed storage (different from CLI's 1-indexed)
    for i in 0..args.len() {
        set_var(&format!("repl_arg_{}", i), &args.get(i + 1));
    }

    // Semicolon-separated for repl_args!() macro
    set_var("repl_args", &args.remaining().join(";"));
}
```

---

### 4. History Management
**Decision**: In-memory Vec only for v1. File persistence is backlog.

**v1 Implementation**:
```rust
pub struct Repl {
    prompt: String,
    history: Vec<String>,
}

impl Repl {
    fn show_history(&self) {
        for (i, cmd) in self.history.iter().enumerate() {
            println!("{}: {}", i + 1, cmd);
        }
    }
}
```

**Future Enhancement** (Backlog - REPL-02):
```toml
[package.metadata.rsb]
repl_history_file = true  # Enable history persistence
repl_history_size = 1000  # Max history entries

[package.metadata.inf]
project_name = "myapp"  # Used for ~/.myapp_history
```

**History File Pattern**:
- Location: `~/.{project_name}_history`
- Project name from `inf_project_name` (TOML snooping)
- Fallback to `rsb_repl_history_file` for custom path

---

### 5. Prompt Configuration
**Decision**: Dynamic prompts with `set_prompt()`, configurable initial prompt.

**Prompt Resolution Hierarchy**:
1. Runtime: `repl.set_prompt("custom> ")`
2. TOML: `rsb_repl_prompt` via `rsb_config!`
3. Environment: `RSB_REPL_PROMPT`
4. Default: `"repl> "`

**Implementation**:
```rust
impl Repl {
    pub fn new() -> Self {
        let prompt = if has_var("rsb_repl_prompt") {
            get_var("rsb_repl_prompt")
        } else if has_var("RSB_REPL_PROMPT") {
            get_var("RSB_REPL_PROMPT")
        } else {
            "repl> ".to_string()
        };

        Self {
            prompt,
            history: Vec::new(),
        }
    }

    /// Update prompt dynamically (for subcommand REPLs, context switching)
    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string();
    }
}
```

**Use Case - Subcommand REPL**:
```rust
let mut repl = Repl::new();
repl.set_prompt("myapp> ");

// User enters config subcommand:
repl.set_prompt("myapp:config> ");

// User exits subcommand:
repl.set_prompt("myapp> ");
```

---

### 6. Error Handling
**Decision**: Catch non-fatal errors and continue loop, let panics bubble up.

**Error Strategy**:
- **Non-fatal errors**: Print message, continue REPL loop
- **Fatal errors/panics**: Bubble up to original `dispatch!` caller
- **Handler results**: Support `Result<i32, String>` return types

**ReplResult Enum**:
```rust
pub enum ReplResult {
    Exit,              // Exit REPL (quit/exit commands)
    Continue,          // Built-in handled, continue loop
    Command(Args),     // User command to dispatch
    Error(String),     // Command error, print and continue
}
```

**Implementation**:
```rust
impl Repl {
    pub fn run(&mut self, handlers: &CommandHandlers) {
        loop {
            match self.read_line() {
                Some(line) => {
                    match self.process_line(&line) {
                        ReplResult::Exit => break,
                        ReplResult::Continue => continue,
                        ReplResult::Command(args) => {
                            // Dispatch to user handler
                            match self.dispatch_to_handler(&args, handlers) {
                                Ok(_) => {},
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                        ReplResult::Error(msg) => {
                            eprintln!("Error: {}", msg);
                        }
                    }
                }
                None => break,  // EOF or input error
            }
        }
    }
}
```

---

### 7. Integration with dispatch!
**Decision**: REPL entered from regular `dispatch!`, with auto-detection support.

**Integration Pattern**:

#### User-Defined REPL Handler:
```rust
fn main() {
    let args = bootstrap!();

    dispatch!(&args, {
        "build" => cmd_build,
        "test" => cmd_test,
        "repl" => cmd_repl,  // User's REPL implementation
    });
}

fn cmd_repl(args: Args) -> i32 {
    let repl = Repl::new();
    repl_dispatch!(repl, {
        "status" => repl_status,
        "config" => repl_config,
        "help" => repl_help,
    })
}
```

#### Auto-Detection with rsb_handle_repl:
```rust
// RSB provides built-in handler for REPL detection
pub fn rsb_handle_repl(args: &Args) -> Option<i32> {
    // Check if user has registered a REPL entry point
    if has_var("fn_repl_dispatch") || has_var("fn_cmd_repl") {
        // User has REPL implementation, call it
        // (requires function pointer storage - future enhancement)
        Some(0)
    } else {
        // No REPL implementation found
        eprintln!("Error: REPL implementation missing");
        eprintln!("Tip: Define a 'repl' command in your dispatch! macro");
        Some(1)
    }
}

// Integration in dispatch! macro (future):
dispatch!(&args, {
    "build" => cmd_build,
    // Auto-adds REPL detection if not defined
});
```

**Registration Pattern**:
```rust
// User registers REPL dispatcher
register_function("repl_dispatch", "Interactive REPL mode");

// Or registers custom REPL command
register_function("cmd_repl", "Enter REPL mode");
```

---

## Built-in Commands

### Standard REPL Commands:
```rust
impl Repl {
    fn dispatch_builtin(&self, args: &Args) -> ReplResult {
        match args.get(0).as_str() {
            "exit" | "quit" => ReplResult::Exit,

            "clear" => {
                clear_globals_prefix("repl_");
                println!("REPL context cleared");
                ReplResult::Continue
            },

            "history" => {
                self.show_history();
                ReplResult::Continue
            },

            "help" => {
                self.show_repl_help();
                ReplResult::Continue
            },

            _ => ReplResult::Command(args.clone()),
        }
    }

    fn show_repl_help(&self) {
        println!("\nREPL Built-in Commands:");
        println!("  exit, quit  - Exit REPL mode");
        println!("  clear       - Clear REPL context variables");
        println!("  history     - Show command history");
        println!("  help        - Show this help message");
        println!();
    }
}
```

---

## Macro Design

### repl_arg! Family
```rust
// Get REPL argument by position (0-indexed, different from CLI)
#[macro_export]
macro_rules! repl_arg {
    ($n:expr) => {{
        $crate::global::get_var(&format!("repl_arg_{}", $n))
    }};
}

// Get total count of REPL arguments
#[macro_export]
macro_rules! repl_argc {
    () => {{
        let argc_str = $crate::global::get_var("repl_argc");
        if argc_str.is_empty() {
            0
        } else {
            argc_str.parse::<usize>().unwrap_or(0)
        }
    }};
}

// Get all REPL arguments as semicolon-separated string
#[macro_export]
macro_rules! repl_args {
    () => {{
        $crate::global::get_var("repl_args")
    }};
}

// Get all REPL arguments as Vec<String>
#[macro_export]
macro_rules! repl_argv {
    () => {{
        let args_str = $crate::global::get_var("repl_args");
        if args_str.is_empty() {
            Vec::new()
        } else {
            args_str.split(';').map(String::from).collect::<Vec<String>>()
        }
    }};
}
```

### repl_dispatch! Macro
```rust
#[macro_export]
macro_rules! repl_dispatch {
    ($repl:expr, { $($cmd:literal => $handler:expr),* $(,)? }) => {{
        let mut repl = $repl;

        loop {
            match repl.read_line() {
                Some(line) => {
                    if line.is_empty() {
                        continue;
                    }

                    repl.history.push(line.clone());

                    let args = $crate::cli::Args::from_line(&line);
                    $crate::repl::store_repl_args_global(&args);

                    match repl.dispatch_builtin(&args) {
                        $crate::repl::ReplResult::Exit => break,
                        $crate::repl::ReplResult::Continue => continue,
                        $crate::repl::ReplResult::Command(cmd_args) => {
                            let cmd = cmd_args.get(0);
                            match cmd.as_str() {
                                $($cmd => {
                                    match $handler(cmd_args.clone()) {
                                        Ok(_) => {},
                                        Err(e) => eprintln!("Error: {}", e),
                                    }
                                },)*
                                "" => continue,
                                unknown => {
                                    eprintln!("Unknown command: {}", unknown);
                                    eprintln!("Type 'help' for available commands");
                                }
                            }
                        },
                        $crate::repl::ReplResult::Error(msg) => {
                            eprintln!("Error: {}", msg);
                        }
                    }
                }
                None => break,  // EOF
            }
        }

        0  // Exit code
    }};
}
```

---

## Implementation Tasks Breakdown

### Phase 1: Core Infrastructure [12 SP]

**REPL-01: Core Repl Struct and Read Loop** [4 SP]
- Create `src/repl/mod.rs` module
- Implement `Repl` struct with prompt, history
- Implement `read_line()` with basic stdin
- Implement `new()` and `with_prompt()` constructors
- Implement prompt configuration (TOML → env → default)
- Implement `set_prompt()` for dynamic updates

**REPL-02: Command Tokenization & Parser Trait** [4 SP]
- Create `src/repl/parser.rs` module
- Define `ReplParser` trait (Send + Sync)
- Implement `SimpleParser` with pattern detection:
  - Quote-aware tokenizer (preserve spaces in quotes)
  - Comma list detection (`,` without spaces)
  - Semicolon pattern detection (`;` for token streams)
  - Token pattern preservation (`:` and `=` indicators)
- Implement `Args::from_line()` using SimpleParser
- Implement `Repl::with_parser()` for pluggable parsers
- Handle edge cases (nested quotes, escaped chars)

**REPL-03: Global Storage Integration** [2 SP]
- Implement `store_repl_args_global()` function
- Store `repl_arg_0..N`, `repl_argc`, `repl_args`
- 0-indexed storage (different from CLI's 1-indexed)
- Integration with existing global store

**REPL-04: Built-in Commands** [2 SP]
- Implement `dispatch_builtin()` method
- exit/quit command handlers
- clear command (clear `repl_*` prefix)
- history command (show command history)
- help command (show built-in help)

---

### Phase 2: Macro System [4 SP]

**REPL-05: REPL Argument Macros** [2 SP]
- Create `src/repl/macros.rs`
- Implement `repl_arg!($n)` macro
- Implement `repl_argc!()` macro
- Implement `repl_args!()` macro
- Implement `repl_argv!()` macro
- Export in prelude

**REPL-06: repl_dispatch! Macro** [2 SP]
- Implement `repl_dispatch!` macro
- Command matching logic
- Error handling integration
- Built-in command detection
- Unknown command handling

---

### Phase 3: Integration & Error Handling [4 SP]

**REPL-07: ReplResult Enum & Error Handling** [2 SP]
- Define `ReplResult` enum (Exit, Continue, Command, Error)
- Implement error catching in dispatch loop
- Non-fatal error printing
- Panic propagation strategy

**REPL-08: dispatch! Integration** [2 SP]
- Document REPL entry pattern from `dispatch!`
- Implement `rsb_handle_repl()` auto-detection
- Registry check for `fn_repl_dispatch` / `fn_cmd_repl`
- Error messages for missing implementation
- Integration examples in documentation

---

### Phase 4: Testing [5 SP]

**REPL-09: Sanity Tests** [2 SP]
- Test `Args::from_line()` tokenization
- Test quote handling
- Test pattern preservation (tokens, comma lists, semicolon streams)
- Test global storage (`repl_arg_*`)
- Test built-in commands (exit, clear, history)
- Test ReplParser trait with SimpleParser

**REPL-10: UAT Tests** [3 SP]
- Test full REPL loop with demo commands
- Test dynamic prompt updates
- Test subcommand REPL scenario
- Test error handling (non-fatal, fatal)
- Test integration with dispatch!
- Test pattern preservation (tokens, streams, lists)

---

### Phase 5: Documentation [3 SP]

**REPL-11: Documentation** [3 SP]
- Create `docs/FEATURES_REPL.md`
- Update `docs/HOWTO.md` with REPL patterns
- Add examples to module docs
- Document macro usage
- Document integration patterns
- Add troubleshooting section

---

## Future Enhancements (Backlog)

### REPL-12: History Persistence [3 SP]
- File-based history (`~/.{project}_history`)
- Load history on startup
- Save history on exit
- Configurable history size
- TOML configuration via `rsb_repl_history_file`

### REPL-13: Enhanced Input [8 SP]
- Optional rustyline integration
- Arrow key history navigation
- Tab completion support
- Line editing (Ctrl+A, Ctrl+E, etc.)
- Feature flag: `repl-readline`

### REPL-14: Meteor Parser Integration [5 SP]
- Implement `MeteorParser` using abstracted meteor tokenstream parser
- Full token stream parsing (once meteor exports parser without engine)
- Namespace folding (`ns=app.config`)
- Context switching (`ctx=user`)
- Token stream expansion/compression
- Note: Deferred until meteor provides standalone parser

### REPL-17: Flag Parser Support [5 SP]
- Implement `FlagParser` for RSB flag patterns
- Flag + value association (`--flag value`)
- Flag + comma list (`--flag a,b,c`)
- Flag + token stream (`--flag key=val;key2=val2`)
- Integration with existing options! system

### REPL-15: Subcommand REPL Framework [8 SP]
- Nested REPL dispatcher
- Context stack management
- Automatic prompt updates
- Command routing hierarchy
- Exit back to parent REPL

### REPL-16: REPL Scripting [5 SP]
- Read commands from file
- Batch mode (non-interactive)
- Script execution: `repl < script.txt`
- Command chaining with `&&` and `;`

---

## Story Point Summary

### Base Implementation (v1): 28 SP
- Phase 1: Core Infrastructure [12 SP]
- Phase 2: Macro System [4 SP]
- Phase 3: Integration & Error Handling [4 SP]
- Phase 4: Testing [5 SP]
- Phase 5: Documentation [3 SP]

**Original Estimate**: 20 SP
**Revised Estimate**: 28 SP (+8 SP)

### Backlog Enhancements: 34 SP
- REPL-12: History Persistence [3 SP]
- REPL-13: Enhanced Input [8 SP]
- REPL-14: Meteor Parser Integration [5 SP] (deferred until meteor exports parser)
- REPL-15: Subcommand REPL Framework [8 SP]
- REPL-16: REPL Scripting [5 SP]
- REPL-17: Flag Parser Support [5 SP]

### Total Epic: 62 SP (28 SP v1 + 34 SP backlog)

---

## Risk Assessment

### Low Risk:
- Core REPL loop (stdin/stdout)
- Global storage integration
- Built-in commands

### Medium Risk:
- Quote-aware tokenizer (edge cases with nested/escaped quotes)
- Pattern detection (comma lists, semicolon streams - avoiding false positives)
- Error handling strategy (deciding what's fatal vs non-fatal)

### High Risk:
- Integration with dispatch! (requires careful design to avoid breaking existing code)
- Function pointer storage for auto-detection (may need enhanced registry)

---

## Success Criteria

1. **Functional**: REPL loop runs, accepts commands, exits gracefully
2. **Tokenization**: Correctly handles quotes and token/list patterns
3. **Integration**: Works seamlessly with existing `dispatch!` system
4. **Error Handling**: Non-fatal errors don't crash REPL
5. **Testing**: 90%+ test coverage for core functionality
6. **Documentation**: Complete FEATURES_REPL.md with examples
7. **Performance**: <1ms overhead per command iteration

---

## Open Questions

1. **Function Pointer Storage**: How to store function pointers in registry for auto-detection?
   - Current registry only stores strings (`fn_name` → "description")
   - Need: `fn_name` → actual function pointer
   - Solution: Enhanced registry or separate function map?

2. **Pattern False Positives**: How to avoid detecting non-token patterns?
   - Example: `http://example.com:8080` looks like a token but isn't
   - Solution: Require `=` in pattern (`:` alone isn't enough)
   - Example: URLs without `=` won't be detected as tokens ✓

3. **Nested REPL Handling**: How to manage REPL-within-REPL scenarios?
   - Stack-based context management?
   - Separate global prefixes (`repl1_arg_*`, `repl2_arg_*`)?

---

## Conclusion

This strategy provides a comprehensive roadmap for REPL support in RSB, with clear separation between v1 essentials (28 SP) and future enhancements (34 SP backlog). The design maintains RSB's philosophy of minimal dependencies while providing powerful interactive command processing with pluggable parser architecture and seamless dispatch integration.

**Key Design Principles**:
- Pluggable `ReplParser` trait for future extensibility
- Pattern detection without semantic parsing (defer to downstream handlers)
- Preserve patterns for future flag parsing support
- Meteor integration deferred until parser is abstracted from engine

**Recommended Action**: Proceed with Phase 1 implementation (REPL-01 through REPL-04: Core Infrastructure).