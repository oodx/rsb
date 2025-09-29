# RSB v2.0 Roadmap

## Overview
This roadmap outlines the implementation plan for RSB v2.0, introducing six major enhancements designed to improve CLI ergonomics, configuration management, and developer experience.

## Timeline
**Target Release: Q1 2025**

### Phase 1: Foundation (Weeks 1-2)
**Goal: Core infrastructure and simple features**

#### 1.1 Generic Object Type [NEW]
- **Priority**: HIGH (prerequisite for other features)
- **Dependencies**: None
- **Features**:
  - Generic `Object<T>` with phantom types
  - JavaScript-like bracket notation
  - Integration with global store
  - Helper macros (`get_hub!`, `hub_config!`, etc.)
- **Docs**: FEATURES_OBJECT.md

#### 1.2 Clear Globals
- **Priority**: HIGH
- **Dependencies**: Object type
- **Features**:
  - Selective clearing with prefix/suffix/pattern
  - Protected keys via Cargo.toml configuration
  - RSB_GLOBAL_RESET flag requirement
- **Config**: `rsb.protected_keys` in Cargo.toml

#### 1.3 CLI Args Global (cli_to_global)
- **Priority**: HIGH
- **Dependencies**: None
- **Features**:
  - Store CLI args in global as `cli_arg_1`, `cli_arg_2`, etc.
  - Add `cli_argc`, `cli_args` (semicolon-joined)
  - Macros: `cli_arg!`, `cli_argc!`, `cli_args!`, `cli_argv!`

### Phase 2: Options Enhancement (Weeks 2-3)
**Goal: Improve CLI options handling and dispatch**

#### 2.1 Cleanup Options
- **Priority**: MEDIUM
- **Dependencies**: Bootstrap enhancements
- **Features**:
  - Three strategies: default, sort, remove
  - Configurable via `options!(&args, strategy: "remove")`
  - Environment variable: `RSB_OPTIONS_MODE`
  - Cargo.toml: `rsb.options_mode`
  - Flag boundary validation

#### 2.2 Flag Commands
- **Priority**: MEDIUM
- **Dependencies**: Dispatch system
- **Features**:
  - Pre-dispatch commands via flags (`--version`, `--help`)
  - Register custom flag commands
  - Priority execution before main dispatch

### Phase 3: Advanced Features (Weeks 3-4)
**Goal: REPL support and configuration loading**

#### 3.1 REPL Support
- **Priority**: LOW
- **Dependencies**: Object type, cli_to_global pattern
- **Features**:
  - REPL struct with command processing
  - Separate context: `cmd_arg_*` variables
  - `repl_dispatch!` macro
  - Built-in commands: exit, clear, history

#### 3.2 TOML Snooping
- **Priority**: MEDIUM
- **Dependencies**: Object type, bootstrap
- **Features**:
  - Load `[package.metadata.rsb]` for framework config
  - Load `[package.metadata.hub]` and `[package.metadata.inf]`
  - Support custom namespaces
  - Automatic snake_case conversion
  - Array expansion to indexed variables

## Feature Dependencies Graph
```
Object<T>
    ├── Clear Globals (uses Object for config)
    ├── TOML Snooping (creates Objects)
    └── REPL Support (uses Object for state)

Bootstrap Enhancements
    ├── cli_to_global
    ├── Cleanup Options
    └── TOML Snooping (early RSB config)

Dispatch System
    └── Flag Commands
```

## Configuration Schema

### Cargo.toml
```toml
[package.metadata.rsb]
# Framework configuration
options_mode = "remove"        # default|sort|remove
global_reset = true            # Enable clear_globals
protected_keys = ["HOME", "PATH", "USER"]
repl_prompt = "myapp> "

[package.metadata.hub]
# Application hub configuration
api_url = "https://api.example.com"
timeout = "30"

[package.metadata.inf]
# Information/metadata
team = "RSB Core"
version = "2.0.0"
```

### Environment Variables
- `RSB_OPTIONS_MODE` - Override options strategy
- `RSB_GLOBAL_RESET` - Enable global clearing
- `RSB_REPL_PROMPT` - Custom REPL prompt

## Breaking Changes
None in v2.0 - all features are opt-in:
- Options cleanup requires explicit configuration
- Clear globals requires flag to be set
- TOML snooping requires `bootstrap!(toml)`
- REPL is a new module (no conflicts)

## Migration Path

### v1.x to v2.0
1. No immediate changes required
2. Gradually adopt new features:
   ```rust
   // Old (still works)
   let args = bootstrap!();
   options!(&args);

   // New (opt-in features)
   let args = bootstrap!(toml);  // Enable TOML
   options!(&args, strategy: "remove");  // Cleanup
   ```

### v2.0 to v3.0
- Consider making `remove` the default options strategy
- Enable TOML snooping by default
- Deprecate direct Args manipulation in favor of global access

## Testing Strategy

### Unit Tests
- Each feature has dedicated test module
- Mock Cargo.toml for TOML tests
- Test all option strategies
- Verify protected keys work

### Integration Tests
- Test feature combinations
- Verify bootstrap order
- Test macro expansions
- Cross-feature data flow

### Performance Benchmarks
- Object creation/access overhead
- Global store with many keys
- TOML parsing impact on bootstrap
- Options strategy performance

## Documentation Updates

### New Documentation
- [x] FEATURES_OBJECT.md
- [ ] FEATURES_REPL.md
- [ ] MIGRATION_v2.md

### Updates Required
- [ ] FEATURES_CLI.md (flag commands, cli_to_global)
- [ ] FEATURES_OPTIONS.md (strategies)
- [ ] FEATURES_GLOBAL.md (clear functions)
- [ ] HOWTO.md (new patterns)

## Success Metrics

### Adoption
- [ ] All example projects updated
- [ ] 3+ community projects using v2.0
- [ ] Positive feedback on ergonomics

### Quality
- [ ] >90% test coverage on new code
- [ ] Zero breaking changes
- [ ] <1ms bootstrap overhead
- [ ] All features documented

### Performance
- [ ] Object access <100ns
- [ ] Global clear <1μs for 1000 keys
- [ ] TOML parsing <10ms

## Risk Mitigation

### Technical Risks
- **Phantom types confusion**: Extensive documentation and examples
- **Performance regression**: Benchmark in CI
- **Breaking changes**: Feature flags for all new behavior

### Adoption Risks
- **Complexity**: Keep simple things simple
- **Documentation**: Examples for every feature
- **Migration**: Clear upgrade path

## Future Considerations (v3.0+)

### Potential Enhancements
1. **Object serialization** - JSON/YAML support
2. **Nested Objects** - True hierarchical structures
3. **Async REPL** - Non-blocking command processing
4. **Config validation** - Schema enforcement
5. **Hot reload** - Watch Cargo.toml changes

### Deferred Features
- Generic value types (breaks string-first philosophy)
- Complex REPL modes (keep it simple)
- Automatic type conversion (explicit is better)

## Release Checklist

### Pre-Release
- [ ] All tests passing
- [ ] Documentation complete
- [ ] Examples updated
- [ ] CHANGELOG.md updated
- [ ] Performance benchmarks run

### Release
- [ ] Version bump in Cargo.toml
- [ ] Tag release in git
- [ ] Publish to crates.io
- [ ] Announce on forums/social

### Post-Release
- [ ] Monitor issue tracker
- [ ] Gather feedback
- [ ] Plan v2.1 patch if needed

---

*Last Updated: 2024-12-28*
*Target Release: Q1 2025*