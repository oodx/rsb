# RSB Testing: HOWTO Guide

**Updated**: 2025-09-16
**Status**: Current Implementation

## Quick Start

```bash
# View test organization requirements
./bin/test.sh docs

# Check test organization compliance
./bin/test.sh lint

# Run tests with status overview (default when no command given)
./bin/test.sh

# Run specific test categories
./bin/test.sh run sanity
./bin/test.sh run smoke
./bin/test.sh run uat

# NEW: Module-based testing (v2.0)
./bin/test.sh run uat math        # Run all math UAT tests
./bin/test.sh run sanity tokens   # Run all tokens sanity tests
./bin/test.sh run uat            # Run all UAT tests across modules
./bin/test.sh run sanity         # Run all sanity tests across modules

# List all available tests
./bin/test.sh list

# Work with experimental tests
./bin/test.sh adhoc
./bin/test.sh adhoc my_experiment
```

## Module-Based Testing System (New in v2.0)

RSB now supports **module filtering subcommands** that allow targeted testing by category and module for more efficient development workflows.

### New Subcommand Syntax

```bash
# General syntax
./bin/test.sh run <category> <module>

# Examples
./bin/test.sh run uat math        # Run all math UAT tests
./bin/test.sh run sanity tokens   # Run all tokens sanity tests
./bin/test.sh run uat            # Run all UAT tests across modules
./bin/test.sh run sanity         # Run all sanity tests across modules

# Legacy syntax still supported
./bin/test.sh run sanity         # Traditional command
./bin/test.sh run uat-math       # Legacy module syntax
```

### Function Naming Requirements

**CRITICAL**: All test functions must follow standardized naming patterns for module-based discovery:

#### UAT Functions
```rust
// Pattern: uat_<module>_<description>()
fn uat_math_basic_demo() { /* Visual demonstration of math basics */ }
fn uat_math_floating_point_demo() { /* Float operations demo */ }
fn uat_tokens_validation_demo() { /* Token validation showcase */ }
fn uat_tokens_parsing_edge_cases() { /* Edge case demonstrations */ }
```

#### SANITY Functions
```rust
// Pattern: sanity_<module>_<description>()
fn sanity_math_basic() { /* Core math functionality tests */ }
fn sanity_math_operations() { /* Mathematical operation tests */ }
fn sanity_tokens_parsing() { /* Token parsing validation */ }
fn sanity_tokens_generation() { /* Token generation tests */ }
```

#### Other Categories
```rust
// UNIT tests
fn unit_<module>_<description>() { /* Fast isolated tests */ }

// SMOKE tests
fn smoke_<module>_<description>() { /* Essential functionality */ }

// INTEGRATION tests (cross-module)
fn integration_<feature>_<description>() { /* Feature-based testing */ }
```

### Benefits of Module-Based Testing

- **Targeted Development**: Test only the module you're working on
- **Faster Feedback**: Skip unrelated test suites during development
- **Better Organization**: Clear separation between module testing categories
- **Easier Debugging**: Isolate issues to specific functional areas
- **Parallel Development**: Teams can work on different modules independently

## Test Organization System

RSB uses a **strict, enforced test organization system** following BASHFX Visual Friendliness Principles. All tests must follow the prescribed structure or they will be blocked from running.

### Directory Structure (ENFORCED)

```
tests/
├── unit/                    # Fast, isolated module tests (<1s each)
│   └── strings/            # Example: folder per src module
│       └── *.rs            # Test files
│
├── sanity/                  # Core functionality validation (REQUIRED)
│   └── strings.rs          # One file per module, comprehensive coverage
│
├── smoke/                   # Minimal CI tests (<10s total runtime)
│   └── core.rs             # Essential functionality only
│   └── strings.rs          # Optional module smoke tests
│
├── integration/             # Cross-module interaction tests
│   └── params_strings.rs   # Tests by feature area (cross-module)
│
├── e2e/                     # End-to-end user workflow tests
│   └── cli_workflow.rs     # Complete user scenarios
│   └── sh/                 # Shell-based e2e tests
│       └── *.sh
│
├── uat/                     # User Acceptance Tests (VISUAL CEREMONY)
│   └── strings.rs          # Visual demonstrations per module
│
├── chaos/                   # Edge cases, stress tests, property tests
│   └── strings/            # Module-specific chaos tests
│       └── *.rs
│
├── bench/                   # Performance benchmarks
│   └── strings.rs          # Module benchmarks
│
├── _adhoc/                  # Experimental tests (outside enforcement)
│   └── *.rs, *.sh          # Temporary/experimental test files
│
├── _archive/                # Deprecated tests (prefixed with _)
│   └── *.rs
│
└── sh/                      # Shell scripts for complex workflows
    └── *.sh
```

### Wrapper Files (tests/*.rs) - REQUIRED PATTERN

All test wrapper files in `tests/` root must follow the strict naming pattern:

**Valid Examples:**
- `sanity.rs` → includes all `tests/sanity/*.rs`
- `sanity_strings.rs` → includes `tests/sanity/strings.rs`
- `unit_strings.rs` → includes `tests/unit/strings/*.rs`
- `uat_strings.rs` → includes `tests/uat/strings.rs`
- `integration_params_strings.rs` → includes `tests/integration/params_strings.rs`

**Invalid Examples (BLOCKED):**
- `strings_sanity.rs` (wrong order - category must come first)
- `test_strings.rs` (non-standard category)
- `random_name.rs` (no pattern match)

## Test Runner Commands

### Core Commands

```bash
# Show help and current test status (default)
./bin/test.sh

# Run specific tests
./bin/test.sh run <test_name>

# NEW: Module-based testing
./bin/test.sh run <category> <module>    # Run module tests in category
./bin/test.sh run uat math              # Run all math UAT tests
./bin/test.sh run sanity tokens         # Run all tokens sanity tests

# List available tests
./bin/test.sh list

# Check test organization compliance
./bin/test.sh lint

# Show detailed violation report
./bin/test.sh --violations

# Generate test organization report
./bin/test.sh report

# Display test organization requirements
./bin/test.sh docs
```

### Enforcement Modes

```bash
# Strict mode (DEFAULT) - blocks tests if violations exist
./bin/test.sh run sanity

# Override mode - run despite violations with warnings
./bin/test.sh --override run sanity

# Skip enforcement entirely (emergency bypass)
./bin/test.sh --skip-enforcement run sanity
```

### Adhoc Tests (Experimental)

```bash
# List experimental tests
./bin/test.sh adhoc

# Run specific adhoc test
./bin/test.sh adhoc my_experiment

# Only list adhoc tests
./bin/test.sh list-adhoc
```

## Test Categories

| Category | Purpose | Max Runtime | Requirements |
|----------|---------|-------------|--------------|
| **sanity** | Core functionality validation | <30s total | **REQUIRED** for every module |
| **smoke** | Minimal CI tests | <10s total | Essential functionality only |
| **unit** | Fast, isolated tests per module | <1s each | One test per function/component |
| **integration** | Cross-module interactions | <60s total | Feature-based organization |
| **e2e** | Complete user workflows | <300s total | Real-world scenarios |
| **uat** | Visual demonstrations | No limit | **REQUIRED** for every module |
| **chaos** | Edge cases, stress tests | No limit | Property/fuzz testing |
| **bench** | Performance benchmarks | No limit | Baseline measurements |

### Required Tests Per Module

**Every module MUST have:**
1. **Sanity tests** - `tests/sanity/strings.rs` (example for strings module)
2. **UAT tests** - `tests/uat/strings.rs` (example for strings module)

Missing either will block all tests until created.

## Visual Ceremony System

RSB uses **shell-based ceremony** (not inline Rust code) following BASHFX principles with the boxy orchestrator for consistent visual presentation.

### Ceremony Runner

```bash
# Run visual test ceremonies
./tests/sh/ceremony.sh sanity          # Core functionality validation
./tests/sh/ceremony.sh uat             # UAT with visual demonstrations
./tests/sh/ceremony.sh smoke           # Quick CI tests
./tests/sh/ceremony.sh all             # Complete test ceremony

# List available ceremonies
./tests/sh/ceremony.sh --list

# Generate ceremony report
./tests/sh/ceremony.sh --report
```

### Boxy Orchestrator Usage

Use the shell ceremony system for boxy formatting - keep Rust tests simple:

```bash
# In your shell ceremony scripts, use the boxy orchestrator from test.sh
source ./bin/test.sh

# Available themes: info, success, warning, error
boxy_display "Content text" "info" "Title"
boxy_display "Success message" "success" "Results"
boxy_display "Warning text" "warning" "Notice"
boxy_display "Error details" "error" "Failure"

# Nested boxy pattern for complex ceremonies
echo "Inner content" | boxy --theme info --style ascii --width 80 | boxy --style rounded --width max
```

### UAT Test Pattern (REQUIRED FORMAT)

UAT tests should be **simple Rust tests** that output clean text - let the ceremony system handle formatting:

```rust
#[test]
fn uat_color_demonstrations() {
    println!("Colors Module UAT Ceremony");
    println!("==========================");

    // UAT 1: Basic Color Output
    println!("\nUAT 1: Basic Color Output");
    println!("Command: red_text!(\"Error message\")");
    println!("Expected: Red colored text output");
    println!("Running...");
    let result = red_text!("Error message");
    println!("Output: {}", result);
    println!("Status: PASS");

    // UAT 2: Multiple Color Combinations
    println!("\nUAT 2: Multiple Color Combinations");
    println!("Command: green_text!(\"Success\") + yellow_text!(\"Warning\")");
    println!("Expected: Green text followed by yellow text");
    println!("Running...");
    println!("Output: {} {}", green_text!("Success"), yellow_text!("Warning"));
    println!("Status: PASS");

    // UAT 3: Color with Background
    println!("\nUAT 3: Color with Background");
    println!("Command: bg_red!(\"Alert message\")");
    println!("Expected: Text with red background");
    println!("Running...");
    println!("Output: {}", bg_red!("Alert message"));
    println!("Status: PASS");

    println!("\nUAT Ceremony Complete");
}
```

### UAT Multiple Variations Pattern

For testing multiple command variations - simple and clean:

```rust
#[test]
fn uat_param_expansion_variations() {
    println!("Parameter Expansion UAT Ceremony");
    println!("=================================");

    let test_cases = vec![
        ("param!(\"HOME\")", "Environment variable expansion"),
        ("param!(\"USER.name\")", "Nested property access"),
        ("param!(\"app.version\", \"1.0.0\")", "Default value fallback"),
    ];

    for (i, (command, description)) in test_cases.iter().enumerate() {
        println!("\nUAT {}: {}", i + 1, description);
        println!("Command: {}", command);
        println!("Expected: {}", description);
        println!("Running...");

        // Execute the actual command
        let result = match i {
            0 => param!("HOME"),
            1 => param!("USER.name"),
            2 => param!("app.version", "1.0.0"),
            _ => unreachable!(),
        };

        println!("Output: {}", result);
        println!("Status: PASS");
    }

    println!("\nUAT Ceremony Complete");
}
```

### UAT Shell Script Pattern

For shell-based UAT tests using nested boxy structure:

```bash
#!/usr/bin/env bash
# tests/_adhoc/uat_example.sh

# Build UAT sections with nested boxy
build_uat_section() {
    local content="$1"
    local theme="$2"
    echo "$content" | boxy --theme "$theme" --style ascii --width 80 | boxy --style rounded --width max
}

# Main ceremony container
ceremony_content=""

# UAT 1: Basic functionality
uat1_content="
UAT 1: Basic Command Execution
Command: rsb param HOME
Expected: User's home directory path
Running...
$(result=$(rsb param HOME); echo "Output: $result")
Status: PASS"

ceremony_content+=$(build_uat_section "$uat1_content" "info")

# UAT 2: Error handling
uat2_content="
UAT 2: Error Handling
Command: rsb param NONEXISTENT
Expected: Error message with fallback
Running...
$(result=$(rsb param NONEXISTENT 2>&1 || echo "Error handled gracefully"); echo "Output: $result")
Status: PASS"

ceremony_content+=$(build_uat_section "$uat2_content" "warning")

# UAT 3: Complex scenario
uat3_content="
UAT 3: Complex Parameter Chain
Command: rsb param 'user.config.theme' 'dark'
Expected: Theme setting with default fallback
Running...
$(result=$(rsb param 'user.config.theme' 'dark'); echo "Output: $result")
Status: PASS"

ceremony_content+=$(build_uat_section "$uat3_content" "success")

# Output complete ceremony
final_ceremony="RSB UAT: Example Module Demonstration

$ceremony_content

UAT Summary: All demonstrations completed successfully"

echo "$final_ceremony" | boxy --style thick --width max --title "Example Module UAT Ceremony"
```

### Ceremony Integration

The ceremony runner captures Rust test output and applies boxy formatting:

```rust
// tests/uat/colors.rs - Simple Rust test
use rsb::prelude::*;

#[test]
fn colors_uat_ceremony() {
    println!("Colors Module UAT Ceremony");
    println!("==========================");

    // UAT 1-3: Basic color functions
    uat_basic_colors();

    // UAT 4: Color combinations
    uat_color_combinations();

    // UAT 5: Background colors
    uat_background_colors();

    println!("\nUAT Ceremony Complete");
}

fn uat_basic_colors() {
    let commands = vec![
        ("red_text!(\"Error\")", red_text!("Error")),
        ("green_text!(\"Success\")", green_text!("Success")),
        ("blue_text!(\"Info\")", blue_text!("Info")),
    ];

    for (i, (cmd, output)) in commands.iter().enumerate() {
        println!("\nUAT {}: Basic Color Function", i + 1);
        println!("Command: {}", cmd);
        println!("Expected: Colored text output");
        println!("Running...");
        println!("Output: {}", output);
        println!("Status: PASS");
    }
}

fn uat_color_combinations() {
    println!("\nUAT 4: Color Combinations");
    println!("Command: green_text!(\"Success\") + yellow_text!(\"Warning\")");
    println!("Expected: Multiple colored outputs");
    println!("Running...");
    println!("Output: {} {}", green_text!("Success"), yellow_text!("Warning"));
    println!("Status: PASS");
}

fn uat_background_colors() {
    println!("\nUAT 5: Background Colors");
    println!("Command: bg_red!(\"Alert\")");
    println!("Expected: Text with colored background");
    println!("Running...");
    println!("Output: {}", bg_red!("Alert"));
    println!("Status: PASS");
}
```

Then the ceremony runner handles the visual formatting:

```bash
# The ceremony runner pipes Rust output through boxy
cargo test --test uat_colors -- --nocapture | boxy --theme info --style rounded --width max
```

## Cross-Module Integration Testing

### Progressive Enhancement Pattern

RSB uses a **progressive enhancement** approach for cross-module testing that isolates pure module functionality from cross-module dependencies:

1. **Pure Module Tests** (sanity): Test modules in isolation without dependencies
2. **Cross-Module Adapters** (UAT): Test integration between modules with feature gating
3. **Dependency Chain Validation**: Cross-module tests depend on stable core modules

### Cross-Module Adapter Pattern

Create adapter tests for cross-module functionality using feature gating:

```rust
// tests/uat/string_gx_adapter.rs - String + Generators integration
use rsb::prelude::*;

#[test]
fn uat_string_gx_random_case_transforms() {
    // Generate random strings and test case transformations
    let random_alnum = rsb::gx::string::get_rand_alnum(10);

    // Test case transformations on generated content
    let snake = rsb::string::to_snake_case(&random_alnum);
    let kebab = rsb::string::to_kebab_case(&random_alnum);

    assert!(!snake.is_empty());
    assert!(!kebab.is_empty());
}

// tests/uat/string_dev_adapter.rs - String + Dev integration
#[cfg(feature = "dev-pty")]
use rsb::dev::{PtyOptions, spawn_pty};

#[test]
#[cfg(feature = "dev-pty")]
fn uat_string_dev_pty_output_processing() {
    // Test string processing of PTY command output
    let mut sess = spawn_pty("echo 'Hello World'", &PtyOptions::default())
        .expect("PTY spawn should work");

    let output = sess.read_for(std::time::Duration::from_millis(500))
        .expect("PTY read should work");

    // Process PTY output with string utilities
    let snake = rsb::string::to_snake_case(&output.trim());
    assert_eq!(snake, "hello_world");
}

#[test]
#[cfg(not(feature = "dev-pty"))]
fn uat_string_dev_fallback_test() {
    // Graceful fallback when feature not available
    assert_eq!(rsb::string::to_snake_case("Hello World"), "hello_world");
}
```

### Benefits of Progressive Enhancement

- **Dependency Isolation**: Pure tests work regardless of cross-module issues
- **API Drift Detection**: Adapter tests reveal when modules evolve incompatibly
- **Feature Gating**: Cross-module functionality only tested when features available
- **Development Order**: Fix core modules first, then cross-module integrations

### Cross-Module Testing Strategy

1. **Write pure sanity tests first** for each module
2. **Establish stable core APIs** before cross-module work
3. **Create adapter tests** for cross-module functionality
4. **Use feature gates** to handle optional dependencies
5. **Provide fallbacks** when features unavailable

## Adding New Tests

### 1. Create Test Files

```bash
# Create the actual test (example: strings module)
touch tests/sanity/strings.rs

# Create the wrapper (REQUIRED)
touch tests/sanity_strings.rs
```

### 2. Wrapper Content

```rust
// tests/sanity_strings.rs
#[path = "sanity/strings.rs"]
mod sanity_strings;
```

### 3. Verify Compliance

```bash
# Check compliance
./bin/test.sh lint

# Run your new test
./bin/test.sh run sanity_strings
```

## Feature Flags and Visual Tests

```bash
# Visual tests with features
export RSB_COLORS="simple,status,named"
export RSB_COLOR="always"
./bin/test.sh run uat-colors

# Direct cargo equivalents (NOT RECOMMENDED - use test.sh)
cargo test --features visuals --test uat_main -- --nocapture
```

## Module-Based Development Workflow

### Typical Development Cycle

When working on a specific module, use the new module-based commands for efficient testing:

```bash
# 1. Work on a module (e.g., math)
vim src/math.rs

# 2. Test only that module quickly
./bin/test.sh run sanity math    # Core functionality
./bin/test.sh run uat math       # Visual demonstrations

# 3. Test related modules if needed
./bin/test.sh run sanity tokens  # If math affects tokens
./bin/test.sh run sanity strings # If math affects strings

# 4. Full category testing before commit
./bin/test.sh run sanity        # All sanity tests
./bin/test.sh run uat          # All UAT tests
```

### Module Development Best Practices

1. **Start with Sanity**: Write sanity tests first for core functionality
2. **Add UAT Demonstrations**: Create visual tests showing module capabilities
3. **Follow Naming**: Use `<category>_<module>_<description>()` pattern
4. **Test Incrementally**: Use module filtering during development
5. **Validate Organization**: Run `./bin/test.sh lint` regularly

### Creating Tests for New Modules

```bash
# 1. Create test files for new module 'example'
touch tests/sanity/example.rs
touch tests/uat/example.rs

# 2. Create wrappers (REQUIRED)
touch tests/sanity_example.rs
touch tests/uat_example.rs

# 3. Implement functions with proper naming
# In tests/sanity/example.rs:
#[test]
fn sanity_example_basic() {
    // Core functionality tests
}

# In tests/uat/example.rs:
#[test]
fn uat_example_demo() {
    // Visual demonstrations
}

# 4. Verify compliance and run
./bin/test.sh lint
./bin/test.sh run sanity example
./bin/test.sh run uat example
```

## Working with Adhoc Tests

For experimental or temporary tests that don't fit the organization:

```bash
# Create experimental test
echo '#!/bin/bash\necho "Experimental test"' > tests/_adhoc/experiment.sh

# Run it
./bin/test.sh adhoc experiment

# Clean up when done
rm tests/_adhoc/experiment.sh
```

## Troubleshooting

### Test Organization Violations

```bash
# See all violations with fix instructions
./bin/test.sh --violations

# Emergency bypass (shows warnings)
./bin/test.sh --override run sanity

# Skip enforcement entirely
./bin/test.sh --skip-enforcement run sanity
```

### Common Issues

1. **Missing sanity/UAT tests** - Every module needs both
2. **Wrong wrapper naming** - Must follow `<category>_<module>.rs` pattern
3. **Unauthorized root files** - Only approved patterns allowed in `tests/`
4. **Missing category entries** - Need `sanity.rs`, `smoke.rs`, etc.

## Key Principles

1. **No Direct Cargo** - All testing flows through `test.sh`
2. **Pattern Enforcement** - Strict naming and structure compliance
3. **Visual Ceremony** - Shell-based visual presentation using boxy
4. **Progressive Testing** - smoke → sanity → integration → e2e progression
5. **Required Coverage** - Every module needs sanity AND UAT tests

## Documentation

- **Complete Requirements**: `./bin/test.sh docs`
- **Organization Standard**: `docs/tech/development/TEST_ORGANIZATION.md`
- **Module Specification**: `docs/tech/development/MODULE_SPEC.md`

---

**Remember**: The test organization system is **strictly enforced**. Tests will be blocked if the organization doesn't comply. Use `./bin/test.sh docs` to understand the complete requirements.


## NEW


# How To Update RSB With fx-testsh Changes

**Status**: Draft
**Audience**: Maintainers syncing `fx-testsh` back into `rsb/bin/test.sh`

## Checklist

1. **Refresh Build Artifacts**
   - Run `./bin/build.sh build` to regenerate `test.sh` from `parts/`.
   - Include `parts/` (and `bin/build.sh`) in the sync if the RSB repo will adopt the BashFX build pattern.

2. **Review Profiles**
   - Ensure the `rsb` profile in `test.sh` exports the full lane map.
   - Confirm doc overrides (‘org’, ‘modules’, ‘prelude’, `runner`, `howto`) resolve inside the RSB tree.

3. **Sync Script**
   - Copy the regenerated `test.sh` to `rsb/bin/test.sh`.
   - Verify shebang and execution bit (`chmod +x`).
   - Update version meta via `semv bump` to match release intent.

4. **Docs**
   - Mirror `docs/reference/RSB_TEST_RUNNER.md` into RSB’s docs tree.
   - Update `docs/rebel/` artefacts if new sections were referenced.
   - Keep `docs/development/HOWTO_UPDATE_RSB.md` in sync so profile overrides stay aligned.

5. **Tests**
   - `test.sh lint` (strict) – should show the “perfect compliance” banner.
   - `test.sh --rsb run sanity` / `run uat` / `run smoke` – confirm run headers + summaries render via Boxy.
   - `test.sh --rsb docs runner` / `docs org` – verify doc overlays resolve in the RSB tree.
   - Kick an adhoc lane if editing experimental suites.

6. **Commit**
   - Use semantic commit messages (`fix:` for patches, `feat:` for surfaced lanes).
   - Reference the sync in changelog or release notes.

7. **Follow-up**
   - Open an RSB PR with summary + testing results.
   - Tag maintainers if ceremony or enforcement rules changed.

## Notes

- Keep BashFX naming conventions intact when editing helpers—downstream automation expects the tiers.
- The runner now ships as a `parts/` assembly; treat direct edits to `test.sh` as generated output.
- Run/status commands emit Boxy headers and summaries—ensure RSB’s CI environments have `boxy` (or expect the stderr fallback).
- If new shared libraries are introduced (e.g., Boxy helpers), place them under `rsb/lib/` and source from the script.
- Treat Tina integration as a follow-up task; the profile hook is ready but unset.
