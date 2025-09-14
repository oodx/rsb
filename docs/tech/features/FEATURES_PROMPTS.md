# RSB Prompts (Interactive CLI Helpers)

Updated: 2025-09-14

## Purpose
- Provide simple, robust interactive prompts for CLI applications with RSB integration
- Support timeout-based fallbacks for automation and CI environments
- Integrate seamlessly with global context (`opt_yes`, `opt_quiet`, `opt_prompt_timeout`)
- Follow MODULE_SPEC thin macro pattern and feature flag architecture

## Feature Flags
- `prompts` - Base feature enabling interactive prompt functionality (requires `visual` and `colors-simple`)
- `visual` - Required base for all visual components
- `colors-simple` - Required for prompt styling and color rendering

## Imports
```rust
// Explicit imports - NOT included in prelude
use rsb::visual::prompts::*;  // Functions: confirm, ask, select
use rsb::{confirm, ask, select, prompt};  // Basic thin macros
use rsb::{confirm_timeout, ask_timeout, select_timeout, prompt_timeout};  // Timeout macros
```

## Core API

### Basic Functions
- `confirm(message: &str) -> bool` — Yes/no confirmation, returns `false` in quiet/non-TTY
- `confirm_default(message: &str, default: bool) -> bool` — Yes/no with explicit default
- `ask(message: &str, default: Option<&str>) -> String` — Text input with optional default
- `select(message: &str, options: &[&str], default_index: Option<usize>) -> String` — Option selection
- `default_from(key: &str, fallback: &str) -> String` — Helper to fetch defaults from context

### Thin Macros (Basic)
- `confirm!(message)` → `confirm(message)`
- `ask!(message)` → `ask(message, None)`
- `ask!(message, default)` → `ask(message, Some(default))`
- `select!(message, options)` → `select(message, options, None)`
- `select!(message, options, index)` → `select(message, options, Some(index))`

### General Prompt Macro
```rust
prompt!("confirm", "Continue?") -> bool
prompt!("ask", "Name?") -> String
prompt!("ask", "Name?", "default") -> String
prompt!("select", "Pick", &["a", "b"]) -> String
prompt!("select", "Pick", &["a", "b"], 1) -> String
```

### Timeout-Enhanced Functions
- `helpers::confirm_with_timeout(msg, timeout_override, default_value)` → bool
- `helpers::ask_with_timeout(msg, default, timeout_override)` → String
- `helpers::select_with_timeout(msg, options, default_idx, timeout_override)` → String

### Timeout-Enhanced Macros
```rust
// Uses context timeout or 30s default
confirm_timeout!("Continue?") -> bool
ask_timeout!("Name?") -> String
select_timeout!("Pick", &["a", "b"]) -> String

// Explicit timeout (seconds)
confirm_timeout!("Continue?", 10) -> bool
ask_timeout!("Name?", "default", 15) -> String
select_timeout!("Pick", &["a", "b"], 1, 20) -> String

// General timeout form
prompt_timeout!("confirm", "Continue?") -> bool
prompt_timeout!("ask", "Name?", "default", 10) -> String
prompt_timeout!("select", "Pick", &opts, 1, 15) -> String
```

## Global Context Integration

### Standard Option Flags
- `opt_yes` → All confirmations return `true` (non-interactive mode)
- `opt_quiet` → All prompts return defaults without interaction
- `opt_prompt_timeout` → Timeout in seconds for timeout-enhanced functions

### Environment Variables
- `PROMPT_TIMEOUT` → Default timeout in seconds (lower priority than `opt_prompt_timeout`)

### Context Resolution Priority
1. **Explicit timeout parameter** (highest priority)
2. **CLI flag**: `--prompt-timeout=N` → `opt_prompt_timeout`
3. **Environment variable**: `PROMPT_TIMEOUT`
4. **Default**: 30 seconds

### TTY Detection
- **TTY/Interactive**: Shows prompts and waits for user input
- **Non-TTY/CI**: Returns defaults immediately (no blocking)
- **Uses**: `libc::isatty(STDIN_FILENO)` for cross-platform detection

## Usage Examples

### Basic Interactive Prompts
```rust
use rsb::prelude::*;
use rsb::visual::prompts::*;

// Manual options processing
let args = bootstrap!();
options!(&args);

// Basic prompts (respect opt_yes, opt_quiet, TTY detection)
if confirm("Deploy to production?") {
    println!("Deploying...");
}

let name = ask("Enter project name", Some("my-app"));
let env = select("Choose environment",
                &["development", "staging", "production"],
                Some(0));
```

### Using Macros
```rust
use rsb::{confirm, ask, select, prompt};

// Thin macros
let proceed = confirm!("Continue with installation?");
let username = ask!("Username", "admin");
let color = select!("Theme", &["light", "dark"], 0);

// General prompt macro
let database = prompt!("ask", "Database URL", "sqlite://db.sqlite");
let confirmed = prompt!("confirm", "Delete all data?");
```

### Timeout-Enhanced Usage
```rust
use rsb::{confirm_timeout, ask_timeout, select_timeout};
use rsb::global::set_var;

// Set global timeout via context
set_var("opt_prompt_timeout", "15");  // 15 seconds

// Uses context timeout (15s)
let result = confirm_timeout!("Server ready, continue?");

// Explicit timeout override (5 seconds)
let quick_answer = ask_timeout!("Quick input", "skip", 5);

// Automation-friendly: times out to safe defaults
let choice = select_timeout!("Pick deployment",
                            &["blue", "green"],
                            0,  // defaults to "blue"
                            10);
```

### Command Line Integration
```bash
# Standard flags work automatically
my-tool --yes              # opt_yes=1, all confirms → true
my-tool --quiet            # opt_quiet=1, all prompts → defaults
my-tool --prompt-timeout=5 # 5 second timeout for enhanced functions

# Environment variable
PROMPT_TIMEOUT=30 my-tool  # 30 second default timeout
```

## Behavior Matrix

| Context | TTY | opt_yes | opt_quiet | Behavior |
|---------|-----|---------|-----------|----------|
| Interactive | ✓ | | | Shows prompt, waits for input |
| Interactive | ✓ | ✓ | | confirm→true, others→defaults |
| Interactive | ✓ | | ✓ | Returns defaults immediately |
| CI/Automation | ✗ | | | Returns defaults immediately |
| Timeout Enhanced | ✓ | | | Shows prompt, times out to defaults |

## Architecture Notes

### MODULE_SPEC Compliance
- **`mod.rs`**: Pure orchestration and curated public surface (no implementation code)
- **`interactive.rs`**: Core prompt function implementations (confirm, ask, select)
- **`utils.rs`**: Curated timeout functions (hoisted to `visual::utils` namespace)
- **Thin macros**: All macros delegate to `visual::utils` functions, no business logic
- **Feature gated**: Behind `prompts` feature flag

### Module Structure
```
src/visual/
├── mod.rs           # Orchestrator - includes visual::utils
├── utils.rs         # Curated functions → visual::utils::*
└── prompts/
    ├── mod.rs       # Orchestrator - re-exports, documentation
    ├── interactive.rs # Implementation - core TTY-aware functions
    └── utils.rs     # Implementation - timeout functions (→ visual::utils)
```

### Access Patterns
```rust
// Ergonomic macros (recommended)
use rsb::{confirm_timeout, ask_timeout};
let result = confirm_timeout!("Deploy?", 30);

// Explicit utils (advanced)
use rsb::visual::utils::*;
let result = confirm_with_timeout("Deploy?", Some(30), false);
```

### Error Handling
- **Timeout**: Returns safe defaults (false for confirm, empty/default for others)
- **IO errors**: Returns safe defaults, no panics
- **Invalid input**: Prompts again (basic) or returns defaults (timeout)

### Threading Model
- **Timeout implementation**: Thread-based with polling for cross-platform compatibility
- **Thread safety**: Uses Arc<Mutex<T>> for shared state
- **Resource cleanup**: Threads complete naturally, no forced termination

## Testing & UAT
- **Sanity tests**: `tests/prompts_sanity.rs` — 17 comprehensive tests
- **Features tests**: `tests/features_prompts.rs` — macro and function verification
- **UAT demo**: `tests/uat/prompts.rs` — visual demonstration of all functionality
- **Example**: `examples/prompts_demo.rs` — working demonstration with color support

## Integration Patterns

### CLI Applications
```rust
use rsb::prelude::*;
use rsb::{confirm, ask, select};

fn main() {
    let args = bootstrap!();
    options!(&args);  // Enables --yes, --quiet, --prompt-timeout

    if confirm!("Initialize new project?") {
        let name = ask!("Project name", "my-project");
        let template = select!("Template", &["basic", "web", "cli"], 0);
        create_project(&name, &template);
    }
}
```

### Automation Scripts
```rust
// Script sets appropriate flags for automation
set_var("opt_quiet", "1");  // Non-interactive
set_var("PROMPT_TIMEOUT", "5");  // Quick timeouts

// Prompts return defaults immediately
let result = confirm_timeout!("Deploy?");  // → false (safe default)
```

### Configuration Workflows
```rust
use rsb::global::{set_var, get_var};

// Interactive configuration
fn configure_app() {
    let db_url = ask!("Database URL", &get_var("DATABASE_URL"));
    let port = ask!("Port", "8080");

    set_var("DATABASE_URL", db_url);
    set_var("PORT", port);

    if confirm!("Save configuration?") {
        save_config();
    }
}
```

## Future Enhancements
- **Multi-select**: `select_multi!()` for checkbox-style selection
- **Password input**: Hidden input for sensitive data
- **Progress prompts**: Integration with progress indicators
- **Validation**: Built-in input validation patterns
- **Theming**: Extended color scheme support beyond simple colors

## Notes
- **Not in prelude**: All prompt functionality requires explicit import
- **CI-friendly**: Non-blocking behavior in automation environments
- **Cross-platform**: TTY detection and threading work on Unix-like systems
- **Conservative defaults**: Always choose safe defaults on timeout/error
- **Timeout precision**: 10ms polling interval, timeouts accurate to ~100ms

This implementation provides a robust, automation-friendly prompt system that integrates seamlessly with RSB's global context and architectural patterns.