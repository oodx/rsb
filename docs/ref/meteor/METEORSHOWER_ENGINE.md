# MeteorEngine: Stateful Data Manipulation Engine

## Overview

**MAJOR ARCHITECTURE CHANGE**: Meteor now provides a stateful data manipulation engine through the new **MeteorEngine** type, built in parallel to the existing MeteorShower. This enables both static data containers (MeteorShower) and dynamic stream processing (MeteorEngine).

While MeteorShower is not completely removed, it is largely deprecated in favor of MeteorEngine, but is still available for backward compatibility.

## Parallel Implementation Strategy

### **MeteorShower: Static Container (Preserved)**
```rust
// Original paradigm: Parse once, query static data (UNCHANGED)
let shower = MeteorShower::parse("app:ui:button=click :;: user:main:theme=dark")?;
let meteor = shower.find("app", "ui", "button");
// MeteorShower functionality completely preserved
```

### **MeteorEngine: Stateful Engine (NEW)**
```rust
// New paradigm: Persistent engine with streaming commands
let mut engine = MeteorEngine::new();
TokenStreamParser::process(&mut engine, "button=click;ns=ui;theme=dark")?;
engine.execute_control_command("delete", "app.ui.button")?;
TokenStreamParser::process(&mut engine, "ctx=user;profile=admin")?;
```

### **Architecture Benefits**
- ✅ **Backward Compatibility**: Existing MeteorShower code unchanged
- ✅ **New Capabilities**: MeteorEngine provides stateful processing
- ✅ **Clear Separation**: Static vs dynamic use cases served independently

## Architecture Components

### **1. MeteorEngine: Stateful Storage Engine**
```rust
pub struct MeteorEngine {
    /// Primary data storage: context → namespace → key → value
    storage: StorageData,

    /// Stream processing cursor state (persistent across calls)
    pub current_context: Context,      // Default: "app"
    pub current_namespace: Namespace,  // Default: "main"

    /// Command execution history (audit trail)
    command_history: Vec<ControlCommand>,
}

#[derive(Debug, Clone)]
pub struct ControlCommand {
    pub timestamp: u64,
    pub command_type: String,   // "delete", "reset"
    pub target: String,         // "app.ui.button", "cursor"
    pub success: bool,
    pub error_message: Option<String>,
}
```

### **2. Parser Module Delegation**

**Stream processing via dedicated parser modules:**

```rust
// TokenStreamParser - handles folding logic with cursor state
pub struct TokenStreamParser;

impl TokenStreamParser {
    /// Process token stream into MeteorEngine
    pub fn process(engine: &mut MeteorEngine, input: &str) -> Result<(), String> {
        // Parse input using current cursor context/namespace
        // Apply folding logic with ns=, ctx= control tokens
        // Execute ctl: commands via engine.execute_control_command()
        // Update cursor state for next stream
    }
}

// MeteorStreamParser - handles explicit addressing
pub struct MeteorStreamParser;

impl MeteorStreamParser {
    /// Process meteor stream into MeteorEngine
    pub fn process(engine: &mut MeteorEngine, input: &str) -> Result<(), String> {
        // Parse explicit meteors: app:ui:button=click :;: user:main:profile=admin
        // No cursor state changes
        // Still supports ctl: commands for data manipulation
    }
}
```

### **3. Control Token Command System**

**Control tokens manipulate stored data and cursor state:**

#### **Data Manipulation Commands:**
```rust
ctl:delete=app.ui.button        // Delete specific key
ctl:delete=app.ui              // Delete entire namespace
ctl:delete=app                 // Delete entire context
```

#### **Cursor State Commands:**
```rust
ctl:reset=cursor               // Reset cursor to defaults (app:main)
ctl:reset=storage              // Clear all stored data
ctl:reset=all                  // Reset cursor + clear storage
```

#### **Command Execution Flow:**
```rust
impl MeteorEngine {
    pub fn execute_control_command(&mut self, command: &str, target: &str) -> Result<(), String> {
        let mut cmd = ControlCommand::new(command, target);

        let result = match command {
            "delete" => self.delete(target).map(|_| ()),
            "reset" => {
                match target {
                    "cursor" => { self.reset_cursor(); Ok(()) }
                    "storage" => { self.clear_storage(); Ok(()) }
                    "all" => { self.reset_all(); Ok(()) }
                    _ => Err(format!("Unknown reset target: {}", target)),
                }
            }
            _ => Err(format!("Unknown control command: {}", command)),
        };

        // Record command execution in history
        cmd = if result.is_ok() {
            cmd.success()
        } else {
            cmd.failure(result.as_ref().unwrap_err())
        };
        self.command_history.push(cmd);

        result
    }
}
```

## Stream Processing Examples

### **Token Stream Processing (with folding):**
```rust
let mut engine = MeteorEngine::new();

// Stream 1: "button=click;ns=ui;theme=dark"
TokenStreamParser::process(&mut engine, "button=click;ns=ui;theme=dark")?;
// - Cursor starts: app:main
// - button=click → stored as app:main:button=click
// - ns=ui → cursor changes to app:ui
// - theme=dark → stored as app:ui:theme=dark
// - Cursor ends: app:ui

// Stream 2: "size=large;ctx=user;profile=admin"
TokenStreamParser::process(&mut engine, "size=large;ctx=user;profile=admin")?;
// - Cursor starts: app:ui (from previous stream!)
// - size=large → stored as app:ui:size=large
// - ctx=user → cursor changes to user:ui
// - profile=admin → stored as user:ui:profile=admin
// - Cursor ends: user:ui
```

### **Control Command Processing:**
```rust
// Stream 3: "ctl:delete=app.ui.theme;ctl:reset=cursor;name=John"
TokenStreamParser::process(&mut engine, "ctl:delete=app.ui.theme;ctl:reset=cursor;name=John")?;
// - ctl:delete=app.ui.theme → removes theme from app:ui namespace
// - ctl:reset=cursor → cursor resets to app:main
// - name=John → stored as app:main:name=John
// - Cursor ends: app:main

// Direct control command execution also available
engine.execute_control_command("delete", "app.ui.button")?;
engine.execute_control_command("reset", "cursor")?;
```

## Storage Manipulation API

### **Dot-Notation API:**
```rust
impl MeteorEngine {
    /// Set value at dot-notation path
    pub fn set(&mut self, path: &str, value: &str) -> Result<(), String> {
        let (context, namespace, key) = parse_dot_path(path)?;
        self.storage.set(&context, &namespace, &key, value);
        Ok(())
    }

    /// Get value at dot-notation path
    pub fn get(&self, path: &str) -> Option<&str> {
        let (context, namespace, key) = parse_dot_path(path).ok()?;
        self.storage.get(&context, &namespace, &key)
    }

    /// Delete item at dot-notation path
    /// Note: Command history managed by execute_control_command, not here
    pub fn delete(&mut self, path: &str) -> Result<bool, String> {
        match parse_dot_path(path) {
            Ok((context, namespace, key)) => {
                let result = if key.is_empty() {
                    if namespace.is_empty() {
                        // Delete entire context
                        self.delete_context(&context)
                    } else {
                        // Delete entire namespace
                        self.delete_namespace(&context, &namespace)
                    }
                } else {
                    // Delete specific key
                    self.delete_key(&context, &namespace, &key)
                };
                Ok(result)
            }
            Err(e) => Err(e)
        }
    }

    /// Find paths matching pattern (basic implementation)
    pub fn find(&self, pattern: &str) -> Vec<String> {
        // Pattern matching against stored paths
        // TODO: Implement wildcard matching
    }

    /// Check if path exists
    pub fn exists(&self, path: &str) -> bool {
        self.get(path).is_some()
    }
}
```

### **Command History Access:**
```rust
impl MeteorEngine {
    /// Get complete command history
    pub fn command_history(&self) -> &[ControlCommand] {
        &self.command_history
    }

    /// Get last executed command
    pub fn last_command(&self) -> Option<&ControlCommand> {
        self.command_history.last()
    }

    /// Get failed commands
    pub fn failed_commands(&self) -> Vec<&ControlCommand> {
        self.command_history.iter().filter(|cmd| !cmd.success).collect()
    }

    /// Clear command history
    pub fn clear_history(&mut self) {
        self.command_history.clear();
    }
}
```

## Parser Module Integration

### **Pure Validation + Delegation Pattern:**
```rust
// src/lib/parser/token_stream.rs
pub struct TokenStreamParser;

impl TokenStreamParser {
    /// Parse and process a token stream
    /// Validates tokens and delegates to MeteorEngine for state changes
    pub fn process(engine: &mut MeteorEngine, input: &str) -> Result<(), String> {
        // Split by semicolon (respecting quotes)
        let parts = Self::smart_split(input, ';');

        for part in parts {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Check if it's a control command
            if trimmed.starts_with("ctl:") {
                Self::process_control_command(engine, trimmed)?;
                continue;
            }

            // Validate token format
            if !is_valid_token_format(trimmed) {
                return Err(format!("Invalid token format: {}", trimmed));
            }

            // Parse the token
            let token = Token::from_str(trimmed)?;

            // Check for control tokens
            match token.key().transformed() {
                "ns" => {
                    // Namespace switch
                    engine.current_namespace = Namespace::from_string(token.value());
                }
                "ctx" => {
                    // Context switch
                    engine.current_context = Context::from_str(token.value())?;
                }
                _ => {
                    // Regular token - store using current cursor state
                    let path = format!("{}.{}.{}",
                        engine.current_context.to_string(),
                        engine.current_namespace.to_string(),
                        token.key().transformed()
                    );
                    engine.set(&path, token.value())?;
                }
            }
        }

        Ok(())
    }
}
```

### **MeteorShower Remains Static:**
```rust
// MeteorShower preserves original functionality (UNCHANGED)
impl FromStr for MeteorShower {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Original parsing logic preserved
        // Uses explicit meteor format only
    }
}

// MeteorEngine provides new capabilities
impl MeteorEngine {
    pub fn new() -> Self {
        // Creates new stateful engine
    }

    // Direct API - no delegation needed
    pub fn set(&mut self, path: &str, value: &str) -> Result<(), String> { ... }
    pub fn get(&self, path: &str) -> Option<&str> { ... }
    pub fn execute_control_command(&mut self, command: &str, target: &str) -> Result<(), String> { ... }
}
```

## Usage Patterns

### **Streaming Data Processor:**
```rust
let mut engine = MeteorEngine::new();

// Process configuration stream
TokenStreamParser::process(&mut engine, "host=localhost;port=8080;ns=db;user=admin;pass=secret")?;

// Process user data stream
TokenStreamParser::process(&mut engine, "ctx=user;name=John;email=john@example.com")?;

// Clean up sensitive data using control command
TokenStreamParser::process(&mut engine, "ctl:delete=app.db.pass")?;

// Query final state using dot-notation API
let host = engine.get("app.main.host");        // Some("localhost")
let user_name = engine.get("user.main.name");  // Some("John")
let password = engine.get("app.db.pass");      // None (deleted)
```

### **Audit Trail:**
```rust
// Check what commands were executed
for cmd in engine.command_history() {
    println!("Command: {} {} - {}", cmd.command_type, cmd.target,
             if cmd.success { "SUCCESS" } else { "FAILED" });
}

// Get only failed commands
let failed = engine.failed_commands();
println!("Failed commands: {}", failed.len());
```

## Key Architectural Principles

1. **Parallel Implementation**: MeteorEngine provides stateful capabilities alongside static MeteorShower
2. **Pure State Controller**: MeteorEngine controls state/data, parsers handle validation only
3. **Command Audit Trail**: All control commands logged with success/failure timestamps
4. **Parser Delegation**: Portable parsing logic in dedicated parser modules
5. **Dot-Notation API**: Uniform path-based addressing for data manipulation
6. **Stream Continuity**: Token streams build on previous cursor state across calls
7. **Backward Compatibility**: Existing MeteorShower code remains unchanged

## Architecture Benefits

### **No Breaking Changes**
- ✅ **MeteorShower preserved**: Original static functionality unchanged
- ✅ **Additive design**: MeteorEngine adds capabilities without disrupting existing code
- ✅ **Clear separation**: Static vs dynamic use cases served independently

### **New Capabilities**
- ✅ **Stateful stream processing**: Cursor state persists across operations
- ✅ **Control command system**: In-stream data manipulation with audit trail
- ✅ **Portable parsers**: Validation logic reusable across types
- ✅ **Dot-notation API**: Direct data access without parsing overhead

### **Implementation Strategy**
This represents a **parallel architecture expansion** rather than a breaking change - both static containers (MeteorShower) and dynamic engines (MeteorEngine) coexist to serve different use cases.
