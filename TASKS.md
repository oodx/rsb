# RSB v2.0 Implementation Tasks

## Task Tracking
- ⬜ Not Started
- 🔄 In Progress
- ✅ Completed
- 🚫 Blocked
- 📝 Documented

---

## Phase 1: Foundation (Weeks 1-2)

### 1. Generic Object Type [PRIORITY: HIGH]
**Owner**: TBD | **Status**: ⬜ Not Started

#### Implementation Tasks
- ⬜ Create `src/object/mod.rs` module structure
- ⬜ Implement `Object<T>` struct with PhantomData
- ⬜ Add Index trait for bracket notation
- ⬜ Create type aliases (AnyObject, HubConfig, etc.)
- ⬜ Implement helper functions:
  - ⬜ `new()`, `from_global()`
  - ⬜ `get()`, `get_or()`, `has()`
  - ⬜ `set()`, `keys()`, `as_map()`
  - ⬜ `as_type()` for phantom type conversion
  - ⬜ `sync_to_global()` method

#### Macro Tasks
- ⬜ Create `src/macros/object.rs`
- ⬜ Implement config access macros:
  - ⬜ `hub_config!`, `inf_config!`, `rsb_config!`
  - ⬜ `get_hub!`, `get_inf!`, `get_rsb!`
- ⬜ Add macro exports to prelude

#### Testing Tasks
- ⬜ Unit tests in `tests/object_core.rs`
- ⬜ Integration tests with global store
- ⬜ Property-based tests for Object operations
- ⬜ Benchmark Object performance

#### Documentation
- ✅ Create FEATURES_OBJECT.md
- ⬜ Add examples to `examples/object_usage.rs`
- ⬜ Update module documentation

---

### 2. Clear Globals [PRIORITY: HIGH]
**Owner**: TBD | **Status**: ⬜ Not Started

#### Implementation Tasks
- ⬜ Extend `src/global/store.rs` with clear methods:
  - ⬜ `clear_all()`
  - ⬜ `clear_prefix()`, `clear_suffix()`
  - ⬜ `clear_pattern()` with regex
  - ⬜ `clear_with()` custom filter
- ⬜ Add protected keys logic:
  - ⬜ `get_protected_keys()` from config
  - ⬜ `is_reset_enabled()` flag check
- ⬜ Export public API functions

#### Configuration Tasks
- ⬜ Add RSB_GLOBAL_RESET env var support
- ⬜ Load protected_keys from Cargo.toml rsb section
- ⬜ Default protected keys fallback

#### Testing Tasks
- ⬜ Test clear with various filters
- ⬜ Test protected keys preservation
- ⬜ Test configuration loading
- ⬜ Test flag requirements

#### Documentation
- ⬜ Update FEATURES_GLOBAL.md
- ⬜ Add clear_globals examples
- ⬜ Document protected keys configuration

---

### 3. CLI Args Global [PRIORITY: HIGH]
**Owner**: TBD | **Status**: ⬜ Not Started

#### Implementation Tasks
- ⬜ Create `cli_to_global()` function in `src/cli/bootstrap.rs`
- ⬜ Store arguments with proper indexing:
  - ⬜ `cli_arg_1`, `cli_arg_2`, etc. (1-indexed)
  - ⬜ `cli_argc` for count
  - ⬜ `cli_args` semicolon-joined
- ⬜ Add helper functions in `src/global/cli_helpers.rs`:
  - ⬜ `get_cli_arg()`, `get_cli_arg_or()`
  - ⬜ `get_cli_argc()`
  - ⬜ `get_cli_argv()` returning Vec<String>

#### Macro Tasks
- ⬜ Implement in `src/macros/cli.rs`:
  - ⬜ `cli_arg!` macro
  - ⬜ `cli_argc!` macro
  - ⬜ `cli_args!` macro
  - ⬜ `cli_argv!` macro

#### Testing Tasks
- ⬜ Test argument storage in global
- ⬜ Test 1-based indexing
- ⬜ Test semicolon joining
- ⬜ Test macro expansions

#### Documentation
- ⬜ Update FEATURES_CLI.md
- ⬜ Add usage examples
- ⬜ Document macro behavior

---

## Phase 2: Options Enhancement (Weeks 2-3)

### 4. Cleanup Options [PRIORITY: MEDIUM]
**Owner**: TBD | **Status**: ⬜ Not Started

#### Implementation Tasks
- ⬜ Create `OptionsStrategy` enum in `src/cli/options.rs`
- ⬜ Implement strategies:
  - ⬜ Default (no change)
  - ⬜ Sort (flags to end)
  - ⬜ Remove (delete processed flags)
- ⬜ Add `apply_options_strategy()` to Args
- ⬜ Implement `validate_flag_boundaries()`
- ⬜ Track processed flags in `options()`

#### Configuration Tasks
- ⬜ Load from RSB_OPTIONS_MODE env var
- ⬜ Load from rsb.options_mode in Cargo.toml
- ⬜ Strategy hierarchy: parameter → env → toml → default

#### Macro Enhancement
- ⬜ Update `options!` macro:
  - ⬜ Default behavior (load from config)
  - ⬜ `options!(&args, strategy: "remove")` override

#### Testing Tasks
- ⬜ Test each strategy behavior
- ⬜ Test flag boundary validation
- ⬜ Test configuration hierarchy
- ⬜ Test backward compatibility

#### Documentation
- ⬜ Update FEATURES_OPTIONS.md
- ⬜ Document strategies
- ⬜ Add migration guide

---

### 5. Flag Commands [PRIORITY: MEDIUM]
**Owner**: TBD | **Status**: ⬜ Not Started

#### Implementation Tasks
- ⬜ Create `FlagCommand` struct in `src/cli/dispatch.rs`
- ⬜ Implement flag command registry:
  - ⬜ Static registry with Lazy initialization
  - ⬜ `register_flag_command()` function
  - ⬜ `execute_flag_commands()` processor
- ⬜ Add default flag commands:
  - ⬜ --version/-v handler
  - ⬜ --help/-h handler
- ⬜ Integrate with dispatch! macro

#### Testing Tasks
- ⬜ Test flag command execution
- ⬜ Test priority over regular dispatch
- ⬜ Test custom flag commands
- ⬜ Test short and long forms

#### Documentation
- ⬜ Update dispatch documentation
- ⬜ Add flag command examples
- ⬜ Document pre_dispatch behavior

---

## Phase 3: Advanced Features (Weeks 3-4)

### 6. REPL Support [PRIORITY: LOW]
**Owner**: TBD | **Status**: ⬜ Not Started

#### Implementation Tasks
- ⬜ Create `src/repl/mod.rs` module
- ⬜ Implement `Repl` struct:
  - ⬜ Command processing
  - ⬜ History management
  - ⬜ Context storage
- ⬜ Add `cmd_to_global()` for REPL args
- ⬜ Built-in commands:
  - ⬜ exit/quit
  - ⬜ clear
  - ⬜ history
- ⬜ Create `ReplResult` enum

#### Macro Tasks
- ⬜ Implement `repl_dispatch!` macro
- ⬜ Add REPL-specific arg macros:
  - ⬜ `cmd_arg!`
  - ⬜ `cmd_argc!`
  - ⬜ `cmd_argv!`

#### Testing Tasks
- ⬜ Test command processing
- ⬜ Test context isolation (cli_arg vs cmd_arg)
- ⬜ Test built-in commands
- ⬜ Test macro dispatch

#### Documentation
- ⬜ Create FEATURES_REPL.md
- ⬜ Add REPL examples
- ⬜ Document command structure

---

### 7. TOML Snooping [PRIORITY: MEDIUM]
**Owner**: TBD | **Status**: ⬜ Not Started

#### Implementation Tasks
- ⬜ Create `src/toml/mod.rs` module
- ⬜ Implement `TomlSnooper`:
  - ⬜ Namespace registration
  - ⬜ Cargo.toml finding
  - ⬜ Metadata extraction
  - ⬜ Value storage in global
- ⬜ Handle value types:
  - ⬜ Strings, integers, booleans
  - ⬜ Arrays with RSB indexing convention
  - ⬜ Snake_case conversion
- ⬜ Early RSB config loading

#### Bootstrap Integration
- ⬜ Update bootstrap! macro:
  - ⬜ `bootstrap!(toml)` variant
  - ⬜ `bootstrap!(toml: "custom")` with namespaces
  - ⬜ Load RSB section first for framework config

#### Dependencies
- ⬜ Add `toml` crate to Cargo.toml
- ⬜ Configure feature flag if needed

#### Testing Tasks
- ⬜ Test TOML extraction
- ⬜ Test array expansion
- ⬜ Test case conversion
- ⬜ Test namespace handling
- ⬜ Mock Cargo.toml for tests

#### Documentation
- ⬜ Document in FEATURES_GLOBAL.md
- ⬜ Add Cargo.toml examples
- ⬜ Document namespace conventions

---

## Integration Tasks

### Cross-Feature Integration
- ⬜ Test Object with TOML data
- ⬜ Test clear_globals with Object
- ⬜ Test REPL with Objects
- ⬜ Test options strategies with cli_to_global
- ⬜ Test flag commands with new bootstrap

### Performance Validation
- ⬜ Benchmark bootstrap with all features
- ⬜ Profile Object access patterns
- ⬜ Measure global store overhead
- ⬜ Optimize hot paths

### Documentation Updates
- ⬜ Update main README.md
- ⬜ Create MIGRATION_v2.md
- ⬜ Update all examples
- ⬜ Add integration examples

---

## Release Preparation

### Pre-Release Checklist
- ⬜ All tests passing
- ⬜ Documentation complete
- ⬜ Examples working
- ⬜ Benchmarks acceptable
- ⬜ CHANGELOG.md updated
- ⬜ Version bumped

### Quality Assurance
- ⬜ Code review completed
- ⬜ Security audit passed
- ⬜ Performance validated
- ⬜ Breaking changes documented
- ⬜ Migration guide tested

---

## Task Assignment

### Suggested Ownership
- **Object System**: Core team lead
- **Global Enhancements**: Storage specialist
- **CLI Improvements**: CLI maintainer
- **REPL**: Interactive systems dev
- **TOML**: Configuration expert
- **Documentation**: Technical writer
- **Testing**: QA engineer

### Priority Order
1. Object type (foundation)
2. Clear globals + CLI args (immediate value)
3. Options cleanup (improves UX)
4. TOML snooping (configuration)
5. Flag commands (nice to have)
6. REPL support (future feature)

---

## Notes

### Dependencies
- Object type is prerequisite for several features
- Bootstrap changes affect multiple features
- Test in isolation first, then integration

### Risk Areas
- Performance of Object with many keys
- Backward compatibility of options changes
- TOML parsing overhead on bootstrap
- Protected keys configuration complexity

### Success Criteria
- All tests green
- No performance regression
- Documentation complete
- Examples functional
- Zero breaking changes

---

*Last Updated: 2024-12-28*
*Track progress in GitHub Issues/Projects*