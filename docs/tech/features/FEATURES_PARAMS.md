# RSB Param Expansion — Progressive Pattern (Stub)

## Purpose
- Capture the organization and evolution path for the `param!` macro and helpers.
- Provide a stable surface for consumers while allowing internal refactors.

## RSB Global Context Foundation

**Critical**: RSB provides a **global Context** that acts as a bash-like variable hub for the entire execution environment.

### Context Architecture
- **Global Store**: Thread-safe `lazy_static` Context (`CTX`) holds all variables in a `HashMap<String, String>`
- **Bash-Like Semantics**: Mimics shell environment variables with `set_var()`, `get_var()`, `has_var()`, `unset_var()`
- **Variable Expansion**: Built-in support for `$VAR` and `${VAR}` expansion in strings via `expand_vars()`
- **Persistence**: Variables persist across function calls, providing true global state

### Context API (from `context.rs`)
```rust
// Global context operations
set_var("FOO", "/home/user/file.txt");     // Set variable
let value = get_var("FOO");                // Get variable (empty string if not found)
let exists = has_var("FOO");               // Check existence
unset_var("FOO");                          // Remove variable

// Variable expansion in strings
let expanded = expand_vars("Path is $FOO/subdir"); // -> "Path is /home/user/file.txt/subdir"
```

### Bootstrap System
RSB automatically populates the global context during `rsb_bootstrap()`:
- **Environment Variables**: All `env::vars()` imported into context
- **XDG Paths**: Standard XDG directories (XDG_CONFIG_HOME, XDG_DATA_HOME, etc.)
- **Script Awareness**: SCRIPT_NAME, SCRIPT_PATH, SCRIPT_DIR from args
- **Mode Flags**: DEBUG_MODE, DEV_MODE, QUIET_MODE, TRACE_MODE from environment

## Param Expansion Design
- **Macro**: `param!` remains exported at the crate root for ergonomic use in apps.
- **Helpers**: non-macro implementation is structured under `rsb::param` with progressive layers:
  - `param::basic` — core operations used by `param!` (get, len, sub, prefix, suffix, replace, upper/lower)
  - `param::advanced` — reserved for richer behaviors (tracing, pattern-aware logic) without breaking callers

## Imports
```rust
use rsb::prelude::*;          // for param!, set_var, get_var, expand_vars
use rsb::param::basic as p;   // optional: direct helper use
```

## Example Usage

### Basic Context Operations
```rust
// Set variables in global context
set_var("USER_HOME", "/home/user");
set_var("FILENAME", "document.txt.bak");

// Use param! macro with context variables
println!("{}", param!("USER_HOME"));                    // -> "/home/user"
println!("{}", param!("FILENAME", suffix: ".bak"));     // -> "document.txt"
println!("{}", param!("FILENAME", prefix: "document")); // -> ".txt.bak"

// Variable expansion in strings
let path = expand_vars("$USER_HOME/docs/$FILENAME");
println!("{}", path); // -> "/home/user/docs/document.txt.bak"

// Wildcard-aware prefix/suffix (bash-like #/##/%/%%)
set_var("ARCHIVE", "logs/2025/app.error.log");
println!("{}", param!("ARCHIVE", prefix: "*/"));          // -> "2025/app.error.log"
println!("{}", param!("ARCHIVE", suffix: "*.log"));       // -> "logs/2025/app.error"
println!("{}", param!("ARCHIVE", suffix: "*.error.log")); // -> "logs/2025/app"

// Patterned first-match case transforms (${VAR^pat}/${VAR,pat})
set_var("NAME", "foo_bar_baz");
println!("{}", param!("NAME", upper: "ba*"));   // -> "foo_Bar_baz"
set_var("PHRASE", "alpha BETA gamma");
println!("{}", param!("PHRASE", lower: "BE*")); // -> "alpha bETA gamma"

// Required variable (${VAR:?msg}) prints to stderr and exits with code 1 when missing
set_var("REQ", "value");
println!("{}", param!("REQ", require: "must be set")); // -> "value"; missing case hard-exits
```

### Advanced Context Features
```rust
// Bootstrap loads environment automatically
rsb_bootstrap(&args); // Imports all env vars, sets up XDG paths, script awareness

// Access bootstrap-loaded variables
println!("Script: {}", get_var("SCRIPT_NAME"));
println!("Config: {}", get_var("XDG_CONFIG_HOME"));
println!("Debug: {}", has_var("DEBUG_MODE"));

// Configuration file parsing
load_config_file("$XDG_CONFIG_HOME/myapp.conf");
// Now config file variables are available via get_var()
```

### Using Helpers Directly (Optional)
```rust
// Using helpers bypasses context - direct string operations
let s = "/home/user/file.txt";
println!("{}", p::replace(s, "/", "_", true)); // -> "_home_user_file.txt"
println!("{}", p::sub_rel(s, -7, Some(3)));        // -> "fil"
println!("{}", p::lower_pat_first("FooBar", "Fo*")); // -> "fooBar"
println!("{}", p::upper_pat_first("fooBar", "ba*")); // -> "fooBar" -> "fooBar" with pattern case tweak
```

## Implementation Notes
- **Context Integration**: The `param!` macro uses `get_var()` to pull from global context, then delegates to `param::basic` for transformations.
- **Thread Safety**: Global context is thread-safe via `Arc<Mutex<Context>>` wrapping.
- **Memory Model**: All variables stored as `String` values, bash-style string-first approach.
- **Progressive Enhancement**: `param::advanced` will host optional tracing/preview and extended patterns; opt-in via API (no feature gates yet).
- **Trace Introspection**: `param::advanced::TraceStep` captures each transformation when tracing is enabled, giving future tooling step-by-step visibility.
 - **Module Spec Alignment**: See `docs/tech/development/MODULE_SPEC.md` for the helper/macro/prelude exposure pattern. `param!` acts as a DSL and delegates to `rsb::string` (ASCII-SAFE/UNICODE-SAFE helpers noted in FEATURES_STRINGS.md).

## Policy: expand_vars vs param!

- `expand_vars("...$VAR...")` performs simple variable substitution only. It does not evaluate bash-like defaults, alts, or transforms.
- Use `param!` for bash-style parameter expansion and transforms:
  - Defaults/alt/require, substring/length, replace, prefix/suffix (glob), case (snake/kebab/dot/space/camel/upper/lower), patterned case transforms.
- Guidance: Prefer `param!` anywhere behavior could be ambiguous under `expand_vars`.

### Audit (call sites to review)
- Search for usages of `expand_vars` in contexts that expect default/alt/prefix/suffix semantics.
- Replace with `param!` pipelines or restructure to use helpers where needed.
- Track findings under RSB-019.
## Additional Context Features

### Function Registry & Call Stack (Introspection)
```rust
// Register functions for help system
register_function("my_command", "Does something useful");

// Automatic call stack tracking
push_call("my_command", &["arg1", "arg2"]);
// ... function execution ...
pop_call(); // Returns CallFrame with timestamp and context snapshot

// Built-in help system
show_help();      // Shows registered functions
show_call_stack(); // Shows execution trace
```

### Configuration Management
```rust
// Parse configuration content
parse_config_content(r#"
    DEBUG=true
    PATHS=(path1 path2 "path with spaces")
    USER_NAME="John Doe"
"#);

// Save current context to file
save_config_file("config.conf", &["DEBUG", "USER_NAME"]);

// Export as shell variables
export_vars("env.sh"); // Creates: export DEBUG='true'
```

### Color & Glyph Registry (Visual System Integration)
The global context also manages:
- **Color Registry**: Dynamic color definitions for visual output
- **Glyph Registry**: Unicode symbols for status indicators (✓, ✗, ⚠, ℹ)
- **RSB_COLORS Environment**: Parsing of custom color/glyph definitions

## Testing
- `tests/param_test.rs` — comprehensive param! behaviors
- `tests/param_helpers.rs` — helper layer sanity  
- `tests/param_test_uat.rs` — visual/printed demo of typical usage

## Future Development (TBD)
- **Tracing API**: Capture step-by-step parameter transformations
- **Error Conventions**: Standardized error reporting for invalid parameter specs
- **Context Scoping**: Potential support for nested/scoped contexts





  - Defaults/alt/require
      - ${VAR:-default} → param!("VAR", default: "default")
      - ${VAR:+alt} → param!("VAR", alt: "alt") (returns alt only when set and non-empty)
      - ${VAR:?msg} → param!("VAR", require: "msg") (prints to stderr and returns empty)
  - Substring/length
      - ${VAR:off[:len]} → param!("VAR", sub: off[, len]) with negative indices supported
      - ${#VAR} → param!("VAR", len)
  - Replace
      - ${VAR/pat/repl} → param!("VAR", replace: "pat" => "repl")
      - ${VAR//pat/repl} → param!("VAR", replace: "pat" => "repl", all)
  - Case transforms
      - ${VAR^^} → param!("VAR", upper)
      - ${VAR,,} → param!("VAR", lower)
      - First-char variants → param!("VAR", upper: first), param!("VAR", lower: first)
  - Prefix/suffix removal (glob-aware, shortest/longest)
      - ${VAR#pat}/${VAR##pat} → param!("VAR", prefix: "pat"[ ,longest])
      - ${VAR%pat}/${VAR%%pat} → param!("VAR", suffix: "pat"[ ,longest])
      - Wildcards supported via * and ? (implemented in string::helpers, used by param)

<!-- feat:params -->

_Generated by bin/feat.py --update-doc._

* `src/param/advanced.rs`
  - struct TraceStep (line 13)

* `src/param/basic.rs`
  - fn get (line 10)
  - fn sub (line 14)
  - fn sub_rel (line 19)
  - fn prefix (line 59)
  - fn suffix (line 63)
  - fn replace (line 67)
  - fn upper (line 71)
  - fn lower (line 75)
  - fn len (line 79)
  - fn upper_pat_first (line 84)
  - fn lower_pat_first (line 88)

* `src/param/macros.rs`
  - macro param! (line 5)

<!-- /feat:params -->
