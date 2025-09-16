# RSB Testing Framework

**Status**: Current Implementation
**Updated**: 2025-09-16
**Version**: 2.0 (Module-Based Testing)

## Overview

RSB implements a comprehensive testing framework with module-based organization, ceremony-driven presentation, and enforced naming standards. The system supports both legacy single-test commands and new module filtering capabilities for efficient development workflows.

## Module-Based Testing System

### Core Concept

The testing system now supports **module filtering subcommands** that allow targeted testing by category and module:

```bash
# New Module-Based Syntax
./bin/test.sh run <category> <module>

# Examples
./bin/test.sh run uat math        # All math UAT tests
./bin/test.sh run sanity tokens   # All tokens sanity tests
./bin/test.sh run uat            # All UAT tests across modules
./bin/test.sh run sanity         # All sanity tests across modules
```

### Benefits

- **Targeted Development**: Test only the module you're working on
- **Faster Feedback**: Skip unrelated test suites during development
- **Better Organization**: Clear separation between module testing categories
- **Easier Debugging**: Isolate issues to specific functional areas
- **Parallel Development**: Teams can work on different modules independently

## Command Reference

### Module-Based Commands

| Command | Description | Example |
|---------|-------------|---------|
| `run <category>` | Run all tests in category | `./bin/test.sh run uat` |
| `run <category> <module>` | Run module tests in category | `./bin/test.sh run uat math` |
| `run sanity <module>` | Run sanity tests for module | `./bin/test.sh run sanity tokens` |
| `run uat <module>` | Run UAT tests for module | `./bin/test.sh run uat date` |

### Legacy Commands (Still Supported)

| Command | Description | Example |
|---------|-------------|---------|
| `run <test>` | Run specific test | `./bin/test.sh run sanity` |
| `run uat-<module>` | Module UAT (legacy) | `./bin/test.sh run uat-math` |
| `run <module>-sanity` | Module sanity (legacy) | `./bin/test.sh run math-sanity` |

### Management Commands

| Command | Description | Purpose |
|---------|-------------|---------|
| `list` | List all available tests | Test discovery |
| `lint` | Check test organization compliance | Structure validation |
| `report` | Generate test organization report | Project health |
| `adhoc [test]` | Run/list experimental tests | Development |
| `--violations` | Show detailed violation report | Debugging |

## Function Naming Requirements

### Standardized Patterns

All test functions must follow these naming patterns for module-based discovery:

#### UAT Functions
```rust
// Pattern: uat_<module>_<description>()
fn uat_math_basic_demo() { /* ... */ }
fn uat_math_floating_point_demo() { /* ... */ }
fn uat_tokens_validation_demo() { /* ... */ }
fn uat_tokens_parsing_edge_cases() { /* ... */ }
```

#### SANITY Functions
```rust
// Pattern: sanity_<module>_<description>()
fn sanity_math_basic() { /* ... */ }
fn sanity_math_operations() { /* ... */ }
fn sanity_tokens_parsing() { /* ... */ }
fn sanity_tokens_generation() { /* ... */ }
```

### Module Discovery

The test runner discovers modules by scanning function names:

1. **Function Pattern Matching**: Identifies `uat_<module>_*` and `sanity_<module>_*` functions
2. **Module Extraction**: Extracts module name from function prefix
3. **Category Filtering**: Groups functions by category (uat, sanity, etc.)
4. **Targeted Execution**: Runs only functions matching the specified module

## Test Categories

### Category Definitions

| Category | Purpose | Max Runtime | Module Support |
|----------|---------|-------------|----------------|
| **sanity** | Core functionality validation | <30s total | ✅ Full |
| **uat** | Visual demonstrations | No limit | ✅ Full |
| **unit** | Fast, isolated tests | <1s each | ✅ Full |
| **smoke** | Minimal CI tests | <10s total | ✅ Full |
| **integration** | Cross-module interactions | <60s total | ⚠️ Limited |
| **e2e** | Complete user workflows | <300s total | ❌ Not applicable |
| **chaos** | Edge cases, stress tests | No limit | ⚠️ Limited |
| **bench** | Performance benchmarks | No limit | ✅ Full |

**Legend:**
- ✅ **Full**: Complete module filtering support
- ⚠️ **Limited**: Partial module filtering (cross-module nature)
- ❌ **Not applicable**: Category doesn't use module organization

### Category Usage Guidelines

#### SANITY Tests
- **Purpose**: Validate core module functionality
- **Requirements**: Every module MUST have sanity tests
- **Naming**: `sanity_<module>_<description>()`
- **Runtime**: Keep total execution under 30 seconds
- **Example**:
```bash
./bin/test.sh run sanity math    # Test math module core functions
./bin/test.sh run sanity         # Test all module sanity suites
```

#### UAT Tests
- **Purpose**: Visual demonstrations and user acceptance
- **Requirements**: Every module MUST have UAT tests
- **Naming**: `uat_<module>_<description>()`
- **Ceremony**: Must include visual output and demonstrations
- **Example**:
```bash
./bin/test.sh run uat tokens     # Demonstrate tokens module features
./bin/test.sh run uat           # All module demonstrations
```

#### UNIT Tests
- **Purpose**: Fast, isolated component testing
- **Organization**: One test per function/component
- **Naming**: `unit_<module>_<description>()`
- **Runtime**: Each test should complete in <1 second
- **Example**:
```bash
./bin/test.sh run unit strings   # Test strings module units
```

## Implementation Details

### Test Discovery Process

1. **Pattern Scanning**: The test runner scans test functions for naming patterns
2. **Module Extraction**: Extracts module names from function prefixes
3. **Category Mapping**: Groups functions by test category
4. **Filter Application**: Applies user-specified category/module filters
5. **Execution**: Runs matching test functions with ceremony

### Function Discovery Algorithm

```bash
# Pseudo-algorithm for module filtering
discover_module_tests() {
    local category="$1"
    local module="$2"

    # Scan for functions matching pattern
    local pattern="${category}_${module}_*"

    # Find matching test functions
    local functions=$(grep -o "${pattern}[a-zA-Z0-9_]*" test_files)

    # Execute with test runner
    for func in $functions; do
        run_test_function "$func"
    done
}
```

### Integration with Test Organization

The module-based system integrates with RSB's test organization requirements:

1. **Wrapper Files**: `tests/<category>_<module>.rs` format maintained
2. **Function Naming**: Enforced through validation in `test.sh`
3. **Directory Structure**: Compatible with existing category directories
4. **Ceremony System**: Works with shell-based visual presentation

## Developer Workflow

### Typical Development Cycle

1. **Develop Module Feature**:
   ```bash
   # Work on math module
   vim src/math.rs
   ```

2. **Test Module Quickly**:
   ```bash
   # Test only math module
   ./bin/test.sh run sanity math
   ./bin/test.sh run uat math
   ```

3. **Expand Testing**:
   ```bash
   # Test related modules
   ./bin/test.sh run sanity strings
   ./bin/test.sh run uat strings
   ```

4. **Full Category Testing**:
   ```bash
   # Test all sanity before commit
   ./bin/test.sh run sanity
   ```

### Module Development Best Practices

1. **Start with Sanity**: Write sanity tests first for core functionality
2. **Add UAT Demonstrations**: Create visual tests showing module capabilities
3. **Follow Naming**: Use `<category>_<module>_<description>()` pattern
4. **Test Incrementally**: Use module filtering during development
5. **Validate Organization**: Run `./bin/test.sh lint` regularly

## Advanced Features

### Experimental Tests

The system supports experimental tests for development:

```bash
# List experimental tests
./bin/test.sh adhoc

# Run specific experimental test
./bin/test.sh adhoc my_experiment

# Create experimental test
echo '#!/bin/bash\necho "Testing new feature"' > tests/_adhoc/experiment.sh
```

### Test Organization Enforcement

The framework enforces test organization through validation:

```bash
# Check compliance
./bin/test.sh lint

# Detailed violation report
./bin/test.sh --violations

# Emergency bypass (with warnings)
./bin/test.sh --override run sanity

# Skip enforcement entirely
./bin/test.sh --skip-enforcement run sanity
```

### Visual Ceremony Integration

Module-based tests integrate with RSB's visual ceremony system:

- **Boxy Formatting**: Professional themed output using `boxy --theme`
- **Progressive Execution**: Visual feedback during test runs
- **Auto-Discovery**: Automatic detection of test categories and modules
- **Status Reporting**: Clear success/failure indication

## Migration Guide

### From Legacy to Module-Based

1. **Update Function Names**:
   ```rust
   // Old naming
   fn test_math_basic() { /* ... */ }

   // New naming
   fn sanity_math_basic() { /* ... */ }
   fn uat_math_basic_demo() { /* ... */ }
   ```

2. **Use New Commands**:
   ```bash
   # Old command
   ./bin/test.sh run math-sanity

   # New command
   ./bin/test.sh run sanity math
   ```

3. **Maintain Compatibility**:
   - Legacy commands still work
   - Gradual migration supported
   - No breaking changes to existing tests

### Adding New Modules

1. **Create Test Files**:
   ```bash
   # Create module tests
   touch tests/sanity/new_module.rs
   touch tests/uat/new_module.rs

   # Create wrappers
   touch tests/sanity_new_module.rs
   touch tests/uat_new_module.rs
   ```

2. **Implement Functions**:
   ```rust
   // tests/sanity/new_module.rs
   #[test]
   fn sanity_new_module_basic() {
       // Core functionality tests
   }

   // tests/uat/new_module.rs
   #[test]
   fn uat_new_module_demo() {
       // Visual demonstrations
   }
   ```

3. **Verify Organization**:
   ```bash
   ./bin/test.sh lint
   ./bin/test.sh run sanity new_module
   ```

## Configuration

### Environment Variables

| Variable | Purpose | Default | Example |
|----------|---------|---------|---------|
| `RSB_TEST_TIMEOUT` | Test timeout in seconds | 600 | `export RSB_TEST_TIMEOUT=300` |
| `RSB_COLORS` | Color feature flags | - | `export RSB_COLORS="simple,status,named"` |
| `RSB_COLOR` | Color output mode | auto | `export RSB_COLOR="always"` |

### Feature Flags

Module-based testing works with all RSB feature flags:

```bash
# Visual tests with features
export RSB_COLORS="simple,status,named"
export RSB_COLOR="always"
./bin/test.sh run uat colors

# Performance tests
./bin/test.sh run bench math
```

## Troubleshooting

### Common Issues

1. **Function Not Found**:
   ```
   Error: No tests found for module 'xyz'
   ```
   **Solution**: Check function naming follows `<category>_<module>_*` pattern

2. **Organization Violations**:
   ```
   Error: Test structure violations detected
   ```
   **Solution**: Run `./bin/test.sh --violations` for detailed report

3. **Module Discovery Fails**:
   ```
   Error: Module 'abc' not recognized
   ```
   **Solution**: Ensure module has proper test files and functions

### Debug Commands

```bash
# Check test organization
./bin/test.sh lint

# See detailed violations
./bin/test.sh --violations

# List all available tests
./bin/test.sh list

# Generate organization report
./bin/test.sh report
```

## Future Enhancements

### Planned Features

1. **Parallel Module Testing**: Run multiple modules concurrently
2. **Module Dependencies**: Define test execution order based on module deps
3. **Interactive Module Selection**: TUI for selecting modules to test
4. **Module Coverage Reports**: Per-module test coverage analysis
5. **Auto-Module Detection**: Automatic discovery of new modules

### Integration Roadmap

1. **CI Integration**: Module-based CI pipeline optimization
2. **IDE Support**: Editor plugins for module-based test running
3. **Documentation Generation**: Auto-generated test documentation per module
4. **Performance Tracking**: Module-level performance regression detection

---

The module-based testing system represents a significant evolution in RSB's testing capabilities, providing developers with the tools needed for efficient, targeted testing while maintaining the framework's commitment to visual ceremony and organizational excellence.