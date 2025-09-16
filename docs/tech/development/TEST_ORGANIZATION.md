# RSB Test Organization Standard (BASHFX Aligned)

**Version**: 1.0
**Updated**: 2025-09-16
**Status**: Implementation Ready

## Philosophy

This organization standard implements **BashFX v3 Visual Friendliness Principles** with **Ceremony with Automation** for Rust projects, ensuring predictable test structure that scales from simple utilities to complex systems.

**Core Tenets**:
- **Ceremony is Required**: Every test must provide clear visual progression and status
- **No Cargo Direct**: All testing flows through `test.sh` - no `cargo test` directly
- **Pattern Enforcement**: test.sh validates naming and structure compliance
- **Visual UAT**: User Acceptance Tests must demonstrate outputs with ceremony
- **Progressive Testing**: smoke → sanity → integration → e2e → chaos progression

## Directory Structure (ENFORCED)

```
tests/
├── unit/                    # Fast, isolated module tests (<1s each)
│   └── <module>/           # One folder per src module
│       └── *.rs            # Test files
│
├── sanity/                  # Core functionality validation (REQUIRED)
│   └── <module>.rs         # One file per module, comprehensive coverage
│
├── smoke/                   # Minimal CI tests (<10s total runtime)
│   └── core.rs             # Essential functionality only
│   └── <module>.rs         # Optional module smoke tests
│
├── integration/             # Cross-module interaction tests
│   └── <feature>.rs        # Tests by feature area (not module)
│
├── e2e/                     # End-to-end user workflow tests
│   └── <workflow>.rs       # Complete user scenarios
│   └── sh/                 # Shell-based e2e tests
│       └── *.sh
│
├── uat/                     # User Acceptance Tests (VISUAL CEREMONY)
│   └── <module>.rs         # Visual demonstrations per module
│
├── chaos/                   # Edge cases, stress tests, property tests
│   └── <module>/           # Module-specific chaos tests
│       └── *.rs
│
├── bench/                   # Performance benchmarks
│   └── <module>.rs         # Module benchmarks
│
├── _archive/                # Deprecated tests (prefixed with _)
│   └── *.rs
│
└── sh/                      # Shell scripts for complex workflows
    └── *.sh
```

## Naming Convention (STRICT ENFORCEMENT)

### Wrapper Files (tests/*.rs)
**REQUIRED PATTERN**: `<category>_<module>.rs` or `<category>.rs`

**Valid Examples**:
- `unit_com.rs` → includes `tests/unit/com/*.rs`
- `sanity_com.rs` → includes `tests/sanity/com.rs`
- `smoke.rs` → includes all `tests/smoke/*.rs`
- `uat_math.rs` → includes `tests/uat/math.rs`

**Invalid Examples** (test.sh will reject):
- `com_sanity.rs` (wrong order)
- `features_com.rs` (use `unit_com.rs`)
- `test_com.rs` (non-standard category)
- `random_name.rs` (no pattern match)

### Category Definitions

| Category | Purpose | Max Runtime | Requirements |
|----------|---------|-------------|--------------|
| **unit** | Fast, isolated tests per module | <1s each | One test per function/component |
| **sanity** | Core functionality validation | <30s total | **REQUIRED** for every module |
| **smoke** | Minimal CI tests | <10s total | Essential functionality only |
| **integration** | Cross-module interactions | <60s total | Feature-based organization |
| **e2e** | Complete user workflows | <300s total | Real-world scenarios |
| **uat** | Visual demonstrations | No limit | **MUST** include ceremony |
| **chaos** | Edge cases, stress tests | No limit | Property/fuzz testing |
| **bench** | Performance benchmarks | No limit | Baseline measurements |

## Ceremony Standards (BASHFX Compliance)

### Shell-Based Ceremony System

**RSB uses shell scripts for test ceremony** rather than inline Rust code. This approach:
- Leverages `boxy` for professional visual output
- Separates test logic from presentation ceremony
- Enables auto-detection and batching of tests
- Follows BASHFX Visual Friendliness Principles

### Ceremony Runner Usage

The primary ceremony interface is `tests/sh/ceremony.sh`:

```bash
# Run category ceremonies
./tests/sh/ceremony.sh sanity          # Core functionality validation
./tests/sh/ceremony.sh uat             # User acceptance tests with visuals
./tests/sh/ceremony.sh smoke           # Quick CI tests
./tests/sh/ceremony.sh all             # Complete test ceremony

# Utilities
./tests/sh/ceremony.sh --list          # List available ceremonies
./tests/sh/ceremony.sh --report        # Test organization report
```

### Visual Ceremony Features

The ceremony runner provides:

**Auto-Discovery**: Automatically detects test categories and files
```bash
📁 sanity (2 tests)
  • sanity
  • sanity_main

📁 uat (10 tests)
  • uat_bash
  • uat_date
  # ... etc
```

**Progressive Execution**: Tests run with visual feedback
```bash
🎭 RSB Test Ceremony - Core Functionality Validation
Running 2 test suites

🔄 [01/02] sanity - RUNNING
✅ [01/02] sanity (1s) - PASS
🔄 [02/02] sanity_main - RUNNING
✅ [02/02] sanity_main (1s) - PASS

📊 Test Results
Suite: sanity | Passed: 2 | Failed: 0 | Success Rate: 100%
```

**Boxy Integration**: Professional themed output using `boxy --theme` system
- `info` theme for headers and lists
- `success` theme for passed results
- `warning` theme for mixed results
- `error` theme for failures

### Test Requirements

**Rust tests remain minimal**: Focus on functionality, not ceremony
```rust
#[test]
fn test_boolean_conversion() {
    assert!(is_true!(true));
    assert!(is_false!(false));
    // No ceremony code needed - handled by shell wrapper
}
```

**UAT tests should be demonstrative**: Show actual outputs and behaviors
```rust
#[test]
fn demonstrate_color_usage() {
    println!("Color demonstration:");
    println!("Error: {}", red_text!("Something failed"));
    println!("Success: {}", green_text!("Operation completed"));
    // Ceremony wrapper will frame this appropriately
}
```

## Test.sh Pattern Enforcement

### Enforcement Modes

```bash
# Standard run (with warnings)
./bin/test.sh run sanity_com

# Strict enforcement (fails on violations)
./bin/test.sh run sanity_com --strict

# Skip enforcement (emergency bypass)
./bin/test.sh run sanity_com --skip-enforcement

# Lint mode (check compliance only)
./bin/test.sh lint
```

### Validation Rules

test.sh **MUST** validate:

1. **Naming Pattern**: All wrapper files follow `<category>_<module>.rs` pattern
2. **Category Validity**: Only approved categories (unit, sanity, smoke, integration, e2e, uat, chaos, bench)
3. **Required Tests**: Every module has sanity tests
4. **File Organization**: Tests in correct subdirectories
5. **Ceremony Compliance**: UAT tests include visual outputs
6. **Runtime Limits**: smoke tests complete within time limits

### Enforcement Implementation

```bash
# Pattern validation in test.sh
validate_test_structure() {
    local violations=()

    # Check wrapper naming
    for file in tests/*.rs; do
        basename="${file##*/}"
        basename="${basename%.rs}"

        if [[ ! "$basename" =~ ^(unit|sanity|smoke|integration|e2e|uat|chaos|bench)(_[a-z_]+)?$ ]]; then
            violations+=("Invalid wrapper name: $file")
        fi
    done

    # Check required sanity tests
    for module in $(find src -name "*.rs" -not -name "lib.rs" -not -name "main.rs"); do
        module_name=$(basename "$module" .rs)
        if [[ ! -f "tests/sanity_${module_name}.rs" && ! -f "tests/sanity/${module_name}.rs" ]]; then
            violations+=("Missing sanity tests for module: $module_name")
        fi
    done

    # Report violations
    if [[ ${#violations[@]} -gt 0 ]]; then
        if [[ "$STRICT_MODE" == "true" ]]; then
            printf "❌ Test structure violations:\n"
            printf " • %s\n" "${violations[@]}"
            exit 1
        else
            printf "⚠️  Test structure warnings:\n"
            printf " • %s\n" "${violations[@]}"
        fi
    fi
}
```

## Migration Strategy

### Phase 1: Structure Setup
1. Create category directories
2. Add TEST_ORGANIZATION.md documentation
3. Update test.sh with pattern detection and warnings

### Phase 2: Test Migration
4. Move existing tests to proper categories:
   - `com_sanity.rs` → `sanity_com.rs` (wrapper) + `sanity/com.rs` (tests)
   - `features_*.rs` → `unit_*.rs`
   - `uat_*.rs` → keep as `uat_*.rs`
   - Empty stubs → remove or implement

### Phase 3: Ceremony Implementation
5. Add ceremony functions to all test files
6. Enhance UAT tests with visual demonstrations
7. Implement test.sh enforcement modes

### Phase 4: Enforcement
8. Enable strict mode by default in CI
9. Add pre-commit hooks for test organization
10. Document ceremony patterns for new tests

## Usage Examples

### Running Tests with Ceremony (Recommended)

```bash
# Visual test ceremonies using boxy
./tests/sh/ceremony.sh sanity          # Sanity tests with ceremony
./tests/sh/ceremony.sh uat             # UAT with visual demonstrations
./tests/sh/ceremony.sh smoke           # Quick CI ceremony
./tests/sh/ceremony.sh all             # Complete ceremony suite

# List available ceremonies
./tests/sh/ceremony.sh --list

# Generate ceremony report
./tests/sh/ceremony.sh --report
```

### Direct Test Execution (Development)

```bash
# Standard test.sh runner (enforcement enabled by default)
./bin/test.sh run sanity               # Core functionality tests
./bin/test.sh run smoke                # Quick smoke tests
./bin/test.sh run sanity_com           # Specific module tests

# Skip enforcement (emergency bypass)
./bin/test.sh --skip-enforcement run sanity_com

# Organization management
./bin/test.sh lint                     # Check compliance
./bin/test.sh report                   # Generate report
./bin/test.sh --strict run sanity      # Fail on violations
```

### Adding New Tests

```bash
# 1. Create test file
tests/sanity/new_module.rs

# 2. Create wrapper
tests/sanity_new_module.rs

# 3. Verify compliance
./bin/test.sh lint

# 4. Run tests
./bin/test.sh sanity_new_module
```

## Ceremony Template Library

Standard ceremony functions available for import:

```rust
// Available in rsb::testing::ceremony
pub fn suite_header(name: &str);
pub fn test_step(step: usize, total: usize, name: &str);
pub fn test_result(name: &str, status: TestStatus);
pub fn suite_summary(results: &TestResults);
pub fn uat_demonstration(title: &str, demo_fn: fn());
```

## Compliance Checklist

- [ ] All wrapper files follow naming pattern
- [ ] Every module has sanity tests
- [ ] UAT tests include visual ceremony
- [ ] test.sh validates structure
- [ ] No direct `cargo test` usage
- [ ] Ceremony functions implemented
- [ ] Runtime limits respected
- [ ] Archive old/deprecated tests

## Benefits

### Predictability
- Know exactly where to find/add tests
- Consistent ceremony across all test suites
- Clear progression from quick to comprehensive testing

### Scalability
- Works for any Rust project size
- Enforced patterns prevent drift
- Visual standards aid maintenance

### CI/Development Workflow
- Fast smoke tests for rapid feedback
- Progressive testing levels
- Visual UAT for human verification
- Automated compliance checking

### BASHFX Alignment
- Implements Visual Friendliness Principle
- Ceremony with Automation patterns
- Proactive state communication
- Structured visual progression

This organization transforms testing from an ad-hoc collection of files into a structured, enforceable system that provides clear progression from basic validation to comprehensive verification while maintaining the visual communication standards essential to the BASHFX architecture.