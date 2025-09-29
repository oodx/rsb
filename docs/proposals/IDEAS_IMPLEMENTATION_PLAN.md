# RSB Enhancement Implementation Plan (v2)

## Overview
This document outlines the implementation plan for six enhancement proposals from IDEAS.txt, with detailed specifications, testing strategies, and implementation phases. Updated with community feedback for better integration and configurability.

---

## 1. CLEANUP_OPTIONS
**Auto-sort or remove flags from Args after processing**

### Problem Statement
Currently, flags remain in the Args array after being processed by `options!()`, even though they're already stored in the global context. This creates redundancy and complicates positional argument access. Users often incorrectly place flags before commands, which causes issues.

### Implementation Design

```rust
// src/cli/options.rs
pub enum OptionsStrategy {
    Default,  // Keep as-is (current behavior)
    Sort,     // Move flags to end
    Remove,   // Remove processed flags (BashFX style)
}

impl OptionsStrategy {
    pub fn from_env() -> Self {
        match get_var("RSB_OPTIONS_MODE").as_str() {
            "sort" => Self::Sort,
            "remove" => Self::Remove,
            _ => Self::Default,
        }
    }
}

// src/cli/args.rs
impl Args {
    /// Apply strategy after options processing
    pub fn apply_options_strategy(&mut self, strategy: OptionsStrategy, processed: &[String]) {
        // First validate we haven't consumed positional args as flag values
        if !self.validate_flag_boundaries() {
            warn!("Potential flag/value boundary issue detected");
        }

        match strategy {
            OptionsStrategy::Sort => self.sort_flags_last(),
            OptionsStrategy::Remove => self.remove_flags(processed),
            OptionsStrategy::Default => {},
        }
    }

    /// Check for problematic patterns like --flag value
    fn validate_flag_boundaries(&self) -> bool {
        for i in 0..self.args.len() - 1 {
            if self.args[i].starts_with("--") &&
               !self.args[i].contains('=') &&
               !self.args[i + 1].starts_with('-') {
                // Potential space-separated value
                return false;
            }
        }
        true
    }

    fn sort_flags_last(&mut self) {
        let (positionals, flags): (Vec<_>, Vec<_>) = self.args
            .drain(..)
            .partition(|arg| !arg.starts_with('-'));

        self.args = positionals;
        self.args.extend(flags);
    }

    fn remove_flags(&mut self, processed: &[String]) {
        self.args.retain(|arg| {
            !processed.iter().any(|flag| arg == flag || arg.starts_with(&format!("{}=", flag)))
        });
    }
}
```

### Macro Integration
```rust
// Enhanced options! macro with strategy parameter
#[macro_export]
macro_rules! options {
    ($args:expr) => {{
        // Strategy loaded from env/Cargo.toml during bootstrap
        let strategy = OptionsStrategy::from_config();
        let processed = cli::options($args);
        $args.apply_options_strategy(strategy, &processed);
    }};
    ($args:expr, strategy: $strat:literal) => {{
        // Explicit strategy overrides config
        let strategy = match $strat {
            "sort" => OptionsStrategy::Sort,
            "remove" => OptionsStrategy::Remove,
            _ => OptionsStrategy::Default,
        };
        let processed = cli::options($args);
        $args.apply_options_strategy(strategy, &processed);
    }};
}

// Strategy resolution hierarchy
impl OptionsStrategy {
    pub fn from_config() -> Self {
        // 1. Check explicit env var
        if has_var("RSB_OPTIONS_MODE") {
            return Self::from_str(&get_var("RSB_OPTIONS_MODE"));
        }
        // 2. Check Cargo.toml rsb section (loaded by bootstrap)
        if has_var("rsb_options_mode") {
            return Self::from_str(&get_var("rsb_options_mode"));
        }
        // 3. Default
        Self::Default
    }
}
```

### Configuration via Cargo.toml
```toml
[package.metadata.rsb]
options_mode = "remove"  # default, sort, or remove
# Other RSB-specific settings loaded during bootstrap
```

### Testing Strategy
```rust
// tests/features/cli/cleanup_options.rs
#[test]
fn test_flags_sorted_after_options() {
    let mut args = Args::new(&["prog", "file1", "--debug", "file2", "-q"]);
    options!(&args);

    assert_eq!(args.positionals(), &["file1", "file2"]);
    assert_eq!(args.get(1), Some("file1"));
    assert_eq!(args.get(2), Some("file2"));
}

#[test]
fn test_backward_compatibility_when_disabled() {
    set_var("RSB_CLEANUP_OPTIONS", "false");
    // Original behavior preserved
}
```

### UAT Tests
- Test with mixed positional and flag arguments
- Verify dispatch! still works correctly
- Test with complex flag values containing "="

---

## 2. CLI_ARGS_GLOBAL (cli_to_global)
**Add CLI arguments to global context as positionals**

### Implementation Design

```rust
// src/cli/bootstrap.rs
pub fn cli_bootstrap_enhanced(raw_args: &[String]) -> Args {
    let args = Args::new(raw_args);

    // Existing bootstrap
    hosts::bootstrap(&args);

    // NEW: Store positionals in global
    cli_to_global(&args);

    args
}

pub fn cli_to_global(args: &Args) {
    // Store count
    set_var("cli_argc", &args.len().to_string());

    // Store each positional (1-indexed to match bash)
    for i in 1..=args.len() {
        if let Some(arg) = args.get(i) {
            set_var(&format!("cli_arg_{}", i), arg);
        }
    }

    // Store all args joined with semicolon
    set_var("cli_args", &args.all().join(";"));
}
```

### Helper Functions
```rust
// src/global/cli_helpers.rs
/// Get CLI argument by position (1-indexed)
pub fn get_cli_arg(n: usize) -> String {
    get_var(&format!("cli_arg_{}", n))
}

/// Get CLI argument with default
pub fn get_cli_arg_or(n: usize, default: &str) -> String {
    let val = get_cli_arg(n);
    if val.is_empty() { default.to_string() } else { val }
}

/// Get total CLI argument count
pub fn get_cli_argc() -> usize {
    get_var("cli_argc").parse().unwrap_or(0)
}

/// Get all CLI args as vector
pub fn get_cli_argv() -> Vec<String> {
    let argc = get_cli_argc();
    (1..=argc).map(|i| get_cli_arg(i)).collect()
}
```

### Convenience Macros
```rust
// src/macros/cli.rs
#[macro_export]
macro_rules! cli_arg {
    ($n:expr) => { rsb::global::get_cli_arg($n) };
    ($n:expr, $default:expr) => { rsb::global::get_cli_arg_or($n, $default) };
}

#[macro_export]
macro_rules! cli_argc {
    () => { rsb::global::get_cli_argc() };
}

#[macro_export]
macro_rules! cli_args {
    () => { rsb::global::get_var("cli_args") };
}

#[macro_export]
macro_rules! cli_argv {
    () => { rsb::global::get_cli_argv() };
}
```

### Testing Strategy
```rust
// tests/features/cli/args_global.rs
#[test]
fn test_cli_args_stored_in_global() {
    let args = bootstrap!();

    assert_eq!(get_var("cli_arg_1"), "first");
    assert_eq!(get_var("cli_arg_2"), "second");
    assert_eq!(cli_arg!(1), "first");
    assert_eq!(cli_arg!(99, "default"), "default");
}
```

---

## 3. FLAG_COMMANDS
**Support commands via flags like --version/-v**

### Implementation Design

```rust
// src/cli/dispatch.rs
pub struct FlagCommand {
    long: &'static str,
    short: Option<char>,
    handler: CommandHandler,
    pre_dispatch: bool,  // Run before main dispatch
}

static FLAG_COMMANDS: Lazy<Vec<FlagCommand>> = Lazy::new(|| vec![
    FlagCommand {
        long: "--version",
        short: Some('v'),
        handler: show_version,
        pre_dispatch: true,
    },
    FlagCommand {
        long: "--help",
        short: Some('h'),
        handler: show_help,
        pre_dispatch: true,
    },
]);

pub fn register_flag_command(cmd: FlagCommand) {
    FLAG_COMMANDS.lock().push(cmd);
}

pub fn execute_flag_commands(args: &Args) -> Option<i32> {
    for cmd in FLAG_COMMANDS.iter() {
        if args.has(cmd.long) || cmd.short.map_or(false, |s| args.has(&format!("-{}", s))) {
            if cmd.pre_dispatch {
                return Some((cmd.handler)(args.clone()));
            }
        }
    }
    None
}
```

### Integration with Dispatch
```rust
// Modified dispatch! macro
macro_rules! dispatch {
    ($args:expr, { $($cmd:literal => $handler:expr),* }) => {{
        // Check flag commands first
        if let Some(exit_code) = execute_flag_commands($args) {
            std::process::exit(exit_code);
        }

        // Regular dispatch
        execute_dispatch($args, ...)
    }};
}
```

### Testing Strategy
```rust
#[test]
fn test_version_flag_command() {
    let args = Args::new(&["prog", "--version"]);
    let result = execute_flag_commands(&args);
    assert!(result.is_some());
}

#[test]
fn test_flag_command_priority() {
    // Flag commands execute before regular dispatch
    let args = Args::new(&["prog", "build", "--help"]);
    // Should show help, not run build
}
```

---

## 4. CLEAR_GLOBALS
**Clear global store with optional filters**

### Implementation Design

```rust
// src/global/store.rs
impl Global {
    /// Check if clearing is enabled
    fn is_reset_enabled() -> bool {
        get_var("RSB_GLOBAL_RESET") == "true" ||
        get_var("rsb_global_reset") == "true"
    }

    /// Get protected keys from configuration
    fn get_protected_keys() -> Vec<String> {
        // Load from rsb section in Cargo.toml (via bootstrap)
        if has_var("rsb_protected_keys") {
            get_var("rsb_protected_keys")
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        } else {
            // Minimal defaults if not configured
            vec!["HOME".into(), "PATH".into(), "USER".into()]
        }
    }

    /// Clear all variables except protected
    pub fn clear_all(&mut self) {
        if !Self::is_reset_enabled() {
            warn!("Global reset not enabled. Set RSB_GLOBAL_RESET=true");
            return;
        }

        let protected = Self::get_protected_keys();
        self.vars.retain(|k, _| protected.contains(k));
    }

    /// Clear variables matching prefix
    pub fn clear_prefix(&mut self, prefix: &str) {
        if !Self::is_reset_enabled() { return; }

        let protected = Self::get_protected_keys();
        self.vars.retain(|k, _| protected.contains(k) || !k.starts_with(prefix));
    }

    /// Clear variables matching suffix
    pub fn clear_suffix(&mut self, suffix: &str) {
        if !Self::is_reset_enabled() { return; }

        let protected = Self::get_protected_keys();
        self.vars.retain(|k, _| protected.contains(k) || !k.ends_with(suffix));
    }

    /// Clear variables matching pattern
    pub fn clear_pattern(&mut self, pattern: &str) {
        if !Self::is_reset_enabled() { return; }

        let protected = Self::get_protected_keys();
        if let Ok(re) = regex::Regex::new(pattern) {
            self.vars.retain(|k, _| protected.contains(k) || !re.is_match(k));
        }
    }
}

// Public API functions
pub fn clear_globals() {
    GLOBAL.lock().clear_all();
}

pub fn clear_globals_prefix(prefix: &str) {
    GLOBAL.lock().clear_prefix(prefix);
}

pub fn clear_globals_suffix(suffix: &str) {
    GLOBAL.lock().clear_suffix(suffix);
}

pub fn clear_globals_pattern(pattern: &str) {
    GLOBAL.lock().clear_pattern(pattern);
}
```

### Configuration via Cargo.toml
```toml
[package.metadata.rsb]
global_reset = true  # Enable clear_globals functionality
protected_keys = ["HOME", "PATH", "USER", "RSB_HOME", "CUSTOM_VAR"]
```

### Testing Strategy
```rust
#[test]
fn test_clear_prefix() {
    set_var("test_one", "1");
    set_var("test_two", "2");
    set_var("keep_this", "3");

    clear_globals_prefix("test_");

    assert!(!has_var("test_one"));
    assert!(!has_var("test_two"));
    assert!(has_var("keep_this"));
}

#[test]
fn test_protected_keys() {
    set_var("HOME", "/home/user");
    clear_globals();
    assert!(has_var("HOME")); // Protected
}
```

---

## 5. NEW_REPL_SUPPORT
**Command dispatcher for REPL mode**

### Implementation Design

```rust
// src/repl/mod.rs
pub struct Repl {
    prompt: String,
    history: Vec<String>,
    context: HashMap<String, String>,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            prompt: get_var_or("RSB_REPL_PROMPT", "rsb> "),
            history: Vec::new(),
            context: HashMap::new(),
        }
    }

    /// Process a REPL command line
    pub fn process_line(&mut self, line: &str) -> ReplResult {
        let cmd_args = Args::from_line(line);

        // Store in global as cmd_arg_*
        store_cmd_args_global(&cmd_args);

        // Execute via repl dispatch
        self.dispatch_command(cmd_args)
    }

    fn dispatch_command(&mut self, args: Args) -> ReplResult {
        // Check for built-in REPL commands
        match args.get(0).as_deref() {
            Some("exit") | Some("quit") => ReplResult::Exit,
            Some("clear") => {
                clear_globals_prefix("cmd_");
                ReplResult::Continue
            },
            Some("history") => {
                for (i, cmd) in self.history.iter().enumerate() {
                    println!("{}: {}", i + 1, cmd);
                }
                ReplResult::Continue
            },
            _ => {
                // Delegate to user-defined handlers
                ReplResult::Command(args)
            }
        }
    }
}

fn store_cmd_args_global(args: &Args) {
    set_var("cmd_argc", &args.len().to_string());

    for i in 0..args.len() {
        set_var(&format!("cmd_arg_{}", i), &args.all()[i]);
    }
}
```

### REPL Dispatch Macro
```rust
#[macro_export]
macro_rules! repl_dispatch {
    ($repl:expr, { $($cmd:literal => $handler:expr),* }) => {{
        let mut repl = $repl;
        loop {
            let line = read_line(&repl.prompt);
            match repl.process_line(&line) {
                ReplResult::Exit => break,
                ReplResult::Command(args) => {
                    let cmd = args.get(0).unwrap_or("");
                    match cmd {
                        $($cmd => $handler(args),)*
                        _ => println!("Unknown command: {}", cmd),
                    }
                },
                ReplResult::Continue => continue,
            }
        }
    }};
}
```

### Helper Functions
```rust
// src/global/repl_helpers.rs
pub fn get_cmd_arg(n: usize) -> String {
    get_var(&format!("cmd_arg_{}", n))
}

pub fn get_cmd_argc() -> usize {
    get_var("cmd_argc").parse().unwrap_or(0)
}

// Macros
#[macro_export]
macro_rules! cmd_arg {
    ($n:expr) => { rsb::global::get_cmd_arg($n) };
}
```

### Testing Strategy
```rust
#[test]
fn test_repl_command_processing() {
    let mut repl = Repl::new();
    repl.process_line("test arg1 arg2");

    assert_eq!(get_var("cmd_arg_0"), "test");
    assert_eq!(get_var("cmd_arg_1"), "arg1");
    assert_eq!(get_var("cmd_arg_2"), "arg2");
}

#[test]
fn test_repl_context_isolation() {
    // CLI args and REPL args are separate
    set_var("cli_arg_1", "cli");
    set_var("cmd_arg_1", "repl");

    assert_eq!(cli_arg!(1), "cli");
    assert_eq!(cmd_arg!(1), "repl");
}
```

---

## 5. Generic Object Type
**JavaScript-like Object with type parameter for shape faking**

### Implementation Design

```rust
// src/object/mod.rs
use std::collections::HashMap;
use std::marker::PhantomData;

/// Generic Object type that can fake any shape T
/// T is just for type hinting/documentation, internally all values are strings
#[derive(Debug, Clone, Default)]
pub struct Object<T = ()> {
    inner: HashMap<String, String>,
    namespace: String,
    _phantom: PhantomData<T>,
}

impl<T> Object<T> {
    pub fn new(namespace: &str) -> Self {
        Self {
            inner: HashMap::new(),
            namespace: namespace.to_string(),
            _phantom: PhantomData,
        }
    }

    /// Load from global variables with namespace prefix
    pub fn from_global(namespace: &str) -> Self {
        let mut obj = Self::new(namespace);
        let prefix = format!("{}_", namespace);

        for (key, value) in get_all_vars() {
            if key.starts_with(&prefix) {
                let short_key = key.strip_prefix(&prefix).unwrap();
                obj.inner.insert(short_key.to_string(), value);
            }
        }
        obj
    }

    /// Get value by key (always returns &str)
    pub fn get(&self, key: &str) -> &str {
        self.inner.get(key).map(|s| s.as_str()).unwrap_or("")
    }

    /// Get with default
    pub fn get_or(&self, key: &str, default: &str) -> &str {
        let val = self.get(key);
        if val.is_empty() { default } else { val }
    }

    /// Check if key exists
    pub fn has(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    /// Set a value (for building objects)
    pub fn set(&mut self, key: &str, value: &str) {
        self.inner.insert(key.to_string(), value.to_string());
    }

    /// Get all keys
    pub fn keys(&self) -> Vec<&String> {
        self.inner.keys().collect()
    }

    /// Get as HashMap for iteration
    pub fn as_map(&self) -> &HashMap<String, String> {
        &self.inner
    }

    /// JavaScript-like property access via Index
    /// Allows: obj["key"] or obj["nested.key"]
    pub fn index(&self, path: &str) -> &str {
        if path.contains('.') {
            // Support nested access like "api.url"
            let parts: Vec<&str> = path.split('.').collect();
            if parts.len() == 2 {
                let key = format!("{}_{}", parts[0], parts[1]);
                return self.get(&key);
            }
        }
        self.get(path)
    }

    /// Convert to a different phantom type (for type hints)
    pub fn as_type<U>(self) -> Object<U> {
        Object {
            inner: self.inner,
            namespace: self.namespace,
            _phantom: PhantomData,
        }
    }
}

// Implement Index trait for bracket notation
use std::ops::Index;

impl<T> Index<&str> for Object<T> {
    type Output = str;

    fn index(&self, key: &str) -> &Self::Output {
        self.index(key)
    }
}

// Type aliases for documentation/clarity
pub type AnyObject = Object<()>;
pub type HubConfig = Object<HubShape>;
pub type InfConfig = Object<InfShape>;
pub type RsbConfig = Object<RsbShape>;

// Phantom marker types (never instantiated, just for type hints)
pub struct HubShape;
pub struct InfShape;
pub struct RsbShape;
```

### Helper Functions and Macros
```rust
// src/object/helpers.rs

/// Get Object for a namespace
pub fn get_object<T>(namespace: &str) -> Object<T> {
    Object::from_global(namespace)
}

// Convenience functions for common namespaces with type hints
pub fn get_hub() -> Object<HubShape> {
    Object::from_global("hub")
}

pub fn get_inf() -> Object<InfShape> {
    Object::from_global("inf")
}

pub fn get_rsb() -> Object<RsbShape> {
    Object::from_global("rsb")
}

// src/macros/object.rs

/// Get value from hub namespace (string only)
#[macro_export]
macro_rules! hub_config {
    ($key:expr) => {
        rsb::global::get_var(&format!("hub_{}", $key))
    };
}

/// Get full hub Object
#[macro_export]
macro_rules! get_hub {
    () => {
        rsb::object::get_hub()
    };
}

/// Get value from inf namespace (string only)
#[macro_export]
macro_rules! inf_config {
    ($key:expr) => {
        rsb::global::get_var(&format!("inf_{}", $key))
    };
}

/// Get full inf Object
#[macro_export]
macro_rules! get_inf {
    () => {
        rsb::object::get_inf()
    };
}

/// Get value from rsb namespace (string only)
#[macro_export]
macro_rules! rsb_config {
    ($key:expr) => {
        rsb::global::get_var(&format!("rsb_{}", $key))
    };
}

/// Get full rsb Object
#[macro_export]
macro_rules! get_rsb {
    () => {
        rsb::object::get_rsb()
    };
}
```

### Usage Examples
```rust
// Define your own shape for type hints (optional)
struct MyAppConfig;

// Direct value access (always strings)
let api_url = hub_config!("api_url");  // Returns &str
let timeout = inf_config!("timeout");  // Returns &str

// Full Object access with type hints
let hub: Object<HubShape> = get_hub!();
println!("API: {}", hub["api_url"]);
println!("Timeout: {}", hub.get_or("timeout", "30"));

// Generic Object without type hints
let config: Object = Object::from_global("custom");

// Convert types for documentation
let typed_config: Object<MyAppConfig> = config.as_type();

// Build an Object manually
let mut obj = Object::<MyAppConfig>::new("app");
obj.set("version", "1.0.0");
obj.set("debug", "true");

// Iterate over all values
for (key, value) in hub.as_map() {
    println!("{}: {}", key, value);
}

// Check existence
if hub.has("feature_flags") {
    let flags = hub["feature_flags"];  // Always &str
}

// The type parameter is just for clarity/documentation
// All operations still return &str
fn process_hub(config: Object<HubShape>) {
    let url: &str = config.get("api_url");  // Type is always &str
    // But the Object<HubShape> type hints what shape we expect
}
```

---

## 6. NOSEY_TOML_SNOOPING
**Extract custom metadata from Cargo.toml including rsb section**

### Implementation Design

```rust
// src/toml/mod.rs
use toml::Value;

pub struct TomlSnooper {
    enabled: bool,
    namespaces: Vec<String>,
}

impl TomlSnooper {
    pub fn new() -> Self {
        Self {
            enabled: false,
            namespaces: vec!["hub".into(), "inf".into(), "rsb".into()],
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.snoop_cargo_toml();
    }

    fn snoop_cargo_toml(&self) {
        let cargo_path = find_cargo_toml();
        if let Ok(content) = std::fs::read_to_string(&cargo_path) {
            if let Ok(toml) = content.parse::<Value>() {
                self.extract_metadata(&toml);
            }
        }
    }

    fn extract_metadata(&self, toml: &Value) {
        for namespace in &self.namespaces {
            if let Some(metadata) = toml
                .get("package")
                .and_then(|p| p.get("metadata"))
                .and_then(|m| m.get(namespace))
            {
                self.store_namespace_values(namespace, metadata);
            }
        }
    }

    fn store_namespace_values(&self, namespace: &str, values: &Value) {
        if let Value::Table(table) = values {
            for (key, value) in table {
                // Convert key to snake_case
                let snake_key = rsb::string::to_snake_case(key);
                let global_key = format!("{}_{}", namespace, snake_key);

                // Store based on value type
                match value {
                    Value::String(s) => set_var(&global_key, s),
                    Value::Integer(i) => set_var(&global_key, &i.to_string()),
                    Value::Boolean(b) => set_var(&global_key, if *b { "true" } else { "false" }),
                    Value::Array(arr) => {
                        // Store as array using RSB convention
                        set_var(&format!("{}_LENGTH", global_key), &arr.len().to_string());
                        for (i, item) in arr.iter().enumerate() {
                            set_var(&format!("{}_{}", global_key, i), &item.to_string());
                        }
                    },
                    _ => {}, // Skip complex types
                }
            }
        }
    }
}

fn find_cargo_toml() -> Result<PathBuf, std::io::Error> {
    let mut path = std::env::current_dir()?;
    loop {
        let cargo_path = path.join("Cargo.toml");
        if cargo_path.exists() {
            return Ok(cargo_path);
        }
        if !path.pop() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Cargo.toml not found"
            ));
        }
    }
}

// Public API
static SNOOPER: Lazy<Mutex<TomlSnooper>> = Lazy::new(|| {
    Mutex::new(TomlSnooper::new())
});

pub fn enable_toml_snooping() {
    SNOOPER.lock().enable();
}

pub fn snoop_namespace(namespace: &str) {
    SNOOPER.lock().namespaces.push(namespace.to_string());
}
```

### Integration with Bootstrap
```rust
// Enhanced bootstrap to load RSB config first
macro_rules! bootstrap {
    () => {{
        let args = Args::from_env();

        // Load RSB section first for framework config
        load_rsb_config();

        // Standard bootstrap
        hosts::bootstrap(&args);
        cli_to_global(&args);

        args
    }};
    (toml) => {{
        let args = bootstrap!();
        enable_toml_snooping();
        args
    }};
    (toml: $($ns:literal),*) => {{
        let args = bootstrap!();
        $(snoop_namespace($ns);)*
        enable_toml_snooping();
        args
    }};
}

fn load_rsb_config() {
    // Load RSB section early for framework configuration
    if let Ok(cargo_toml) = load_cargo_toml() {
        if let Some(rsb) = cargo_toml.get("package.metadata.rsb") {
            // Store RSB configuration for framework use
            store_config_section("rsb", rsb);
        }
    }
}
```

### Example Cargo.toml
```toml
[package.metadata.rsb]
# Framework configuration
options_mode = "remove"  # default, sort, or remove
global_reset = true
protected_keys = ["HOME", "PATH", "USER", "CUSTOM_KEY"]
repl_prompt = "myapp> "

[package.metadata.hub]
api_url = "https://api.hub.example.com"
timeout = "30"  # Always stored as strings
features = ["auth", "cache", "metrics"]

[package.metadata.inf]
build_date = "2024-01-15"
team_name = "RSB Core"
support_email = "support@rsb.dev"  # snake_case conversion

### Testing Strategy
```rust
#[test]
fn test_toml_extraction() {
    // Create test Cargo.toml
    let toml_content = r#"
        [package.metadata.rsb]
        options_mode = "remove"
        protected_keys = ["HOME", "PATH"]

        [package.metadata.hub]
        api_url = "test.com"
        maxRetries = "3"
    "#;

    std::fs::write("test_Cargo.toml", toml_content).unwrap();

    enable_toml_snooping();

    // Test RSB config
    assert_eq!(get_var("rsb_options_mode"), "remove");
    assert_eq!(rsb_config!("options_mode"), "remove");

    // Test hub config
    assert_eq!(get_var("hub_api_url"), "test.com");
    assert_eq!(get_var("hub_max_retries"), "3"); // Converted to snake_case
    assert_eq!(hub_config!("max_retries"), "3");
}

#[test]
fn test_config_object() {
    let hub = get_hub!();
    assert_eq!(hub["api_url"], "test.com");
    assert_eq!(hub.get_or("missing", "default"), "default");

    // Test iteration
    for (key, value) in hub.as_map() {
        assert!(key.len() > 0);
        assert!(value.len() > 0);
    }
}

#[test]
fn test_array_extraction() {
    // Test array values are properly indexed
    assert_eq!(get_var("hub_features_LENGTH"), "3");
    assert_eq!(get_var("hub_features_0"), "auth");
    assert_eq!(get_var("hub_features_1"), "cache");
}

---

## Implementation Phases

### Phase 1: Foundation (Week 1-2)
1. **CLEAR_GLOBALS** - Simplest, no external dependencies
2. **CLI_ARGS_GLOBAL** - Builds on existing bootstrap

### Phase 2: Options Enhancement (Week 2-3)
3. **CLEANUP_OPTIONS** - Requires careful testing with existing code
4. **FLAG_COMMANDS** - Extends dispatch system

### Phase 3: Advanced Features (Week 3-4)
5. **NEW_REPL_SUPPORT** - New module, minimal impact on existing code
6. **NOSEY_TOML_SNOOPING** - Requires toml dependency

---

## Testing Plan

### Unit Tests
- Each feature gets dedicated test module
- Test both success and error paths
- Test edge cases (empty args, missing files, etc.)

### Integration Tests
- Test feature combinations (e.g., FLAG_COMMANDS with CLEANUP_OPTIONS)
- Test backward compatibility
- Test with real Cargo.toml files

### UAT Tests
- Create example applications using each feature
- Test in CI/CD pipeline
- Performance benchmarks for global operations

### Documentation Tests
- Update feature documentation
- Add examples to each module
- Update HOWTO.md with new patterns

---

## Risk Mitigation

### Backward Compatibility
- All features opt-in initially via feature flags or environment variables
- Deprecation warnings for breaking changes
- Migration guide for v2.0

### Performance
- Lazy initialization for TOML snooping
- Benchmark global store operations with many keys
- Consider using Arc<RwLock> if contention becomes issue

### Security
- Validate TOML paths to prevent directory traversal
- Protected keys in CLEAR_GLOBALS
- Sanitize REPL input to prevent injection

---

## Success Metrics

1. **Code Coverage**: >90% test coverage for new features
2. **Performance**: <1ms overhead for bootstrap with all features
3. **Adoption**: Features used in at least 3 RSB example projects
4. **Documentation**: All features documented with examples
5. **Compatibility**: Zero breaking changes in minor version

---

## Next Steps

1. Review and approve implementation plan
2. Create feature branches for each enhancement
3. Implement Phase 1 features
4. Gather feedback from early adopters
5. Iterate and proceed to Phase 2

---

## Appendix: Configuration Examples

### Full Feature Bootstrap
```rust
fn main() {
    // Enable all enhancements
    std::env::set_var("RSB_CLEANUP_OPTIONS", "true");

    let args = bootstrap!(toml: "hub", "inf", "app");
    options!(&args);

    // CLI args available globally
    println!("First arg: {}", cli_arg!(1, "none"));

    // TOML metadata available
    println!("API URL: {}", get_var("hub_api_url"));

    // Clean dispatch with flag commands
    dispatch!(&args, {
        "build" => build_cmd,
        "test" => test_cmd,
    });
}
```

### REPL Example
```rust
fn interactive_mode() {
    let repl = Repl::new();

    repl_dispatch!(repl, {
        "status" => |args| {
            println!("Status: {}", cmd_arg!(1, "OK"));
            0
        },
        "config" => |args| {
            let key = cmd_arg!(1, "");
            println!("{} = {}", key, get_var(&key));
            0
        },
    });
}
```