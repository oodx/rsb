#!/bin/bash
# RSB Test Entry Point  
# Unified interface for running all RSB tests

set -e

# Configuration
ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
TEST_DIR="$ROOT_DIR/tests"

# Try to find boxy for pretty output (optional)
BOXY=""
if command -v boxy >/dev/null 2>&1; then
    BOXY="boxy"
elif [[ -f "./target/release/boxy" ]]; then
    BOXY="./target/release/boxy"
elif [[ -f "../boxy/target/release/boxy" ]]; then
    BOXY="../boxy/target/release/boxy"
fi

# Optional timeout wrapper for cargo (prevents hangs on TTY‑waiting tests)
TIMEOUT_BIN=""
if command -v timeout >/dev/null 2>&1; then
    TIMEOUT_BIN="timeout"
elif command -v gtimeout >/dev/null 2>&1; then
    TIMEOUT_BIN="gtimeout"
fi

# Helper: run cargo with optional timeout
# Usage: ctest <args...> → runs `cargo <args...>` with timeout when available
ctest() {
    if [[ -n "$TIMEOUT_BIN" ]]; then
        # Default to 10 minutes if not provided; override via RSB_TEST_TIMEOUT (in seconds)
        local secs="${RSB_TEST_TIMEOUT:-600}"
        "$TIMEOUT_BIN" "${secs}s" cargo "$@"
    else
        cargo "$@"
    fi
}

# Boxy Orchestrator - Centralized boxy call handler
# Usage: boxy_display <content> <theme> <title> [width]
boxy_display() {
    local content="$1"
    local theme="$2"
    local title="$3"
    local width="${4:-max}"

    if command -v boxy &> /dev/null; then
        if [[ -n "$title" ]]; then
            echo "$content" | boxy --theme "$theme" --title "$title" --width "$width"
        else
            echo "$content" | boxy --theme "$theme" --width "$width"
        fi
    else
        # Fallback ASCII presentation
        local border_symbol
        case "$theme" in
            error) border_symbol="❌" ;;
            warning) border_symbol="⚠️" ;;
            success) border_symbol="✅" ;;
            info) border_symbol="ℹ️" ;;
            *) border_symbol="•" ;;
        esac

        echo "$border_symbol ═══════════════════════════════════════════════════════════════════════════════"
        [[ -n "$title" ]] && echo "   $title"
        echo "$content" | sed 's/^/   /'
        echo "   ═══════════════════════════════════════════════════════════════════════════════"
    fi
    echo
}

# Show override warning with boxy
show_override_warning() {
    local warning_text="⚠️  OVERRIDE MODE ACTIVE ⚠️

Tests are being run despite organization violations.
This should only be used for emergency situations.

Recommended actions:
• Fix test naming patterns: <category>_<module>.rs
• Create missing sanity tests
• Move tests to proper directories
• Run './bin/test.sh lint' to see violations

Use './bin/test.sh run <test>' for standard enforcement."

    boxy_display "$warning_text" "warning" "⚠️  Test Organization Override"
}


# Test Organization Enforcement (BASHFX Aligned)
validate_test_structure() {
    # Categorized violation arrays
    local naming_violations=()
    local missing_sanity_violations=()
    local missing_uat_violations=()
    local directory_violations=()
    local missing_category_entry_violations=()
    local unauthorized_root_violations=()

    # Valid categories for test organization
    local valid_categories="unit|sanity|smoke|integration|e2e|uat|chaos|bench"
    local required_category_entries=(sanity smoke unit integration e2e uat chaos bench)

    if [[ "$SKIP_ENFORCEMENT" == "true" ]]; then
        return 0
    fi

    # Show override warning if using --override mode
    if [[ "$OVERRIDE_MODE" == "true" ]]; then
        show_override_warning
    fi

    echo "🔍 Validating test structure..."

    # Check wrapper naming patterns
    for file in tests/*.rs; do
        [[ ! -f "$file" ]] && continue

        basename="${file##*/}"
        basename="${basename%.rs}"

        # Skip archive files
        [[ "$basename" =~ ^_ ]] && continue

        # Check naming pattern
        if [[ ! "$basename" =~ ^($valid_categories)(_[a-z_]+)?$ ]]; then
            naming_violations+=("$file")
        fi
    done

    # Check for required sanity tests - look for modules in src/ directory
    # Pattern 1: src/module.rs files (direct module files)
    for module_file in src/*.rs; do
        [[ ! -f "$module_file" ]] && continue
        module_name=$(basename "$module_file" .rs)

        # Skip lib.rs and main.rs
        [[ "$module_name" == "lib" || "$module_name" == "main" ]] && continue

        # Check for sanity test existence
        if [[ ! -f "tests/sanity_${module_name}.rs" && ! -f "tests/sanity/${module_name}.rs" ]]; then
            missing_sanity_violations+=("$module_name")
        fi
    done

    # Pattern 2: src/module/mod.rs files (directory modules)
    for module_dir in src/*/; do
        [[ ! -d "$module_dir" ]] && continue
        [[ ! -f "${module_dir}mod.rs" ]] && continue

        module_name=$(basename "$module_dir")

        # Check for sanity test existence
        if [[ ! -f "tests/sanity_${module_name}.rs" && ! -f "tests/sanity/${module_name}.rs" ]]; then
            missing_sanity_violations+=("$module_name")
        fi

        # Check for UAT test existence (BOTH sanity AND uat required)
        if [[ ! -f "tests/uat_${module_name}.rs" && ! -f "tests/uat/${module_name}.rs" ]]; then
            missing_uat_violations+=("$module_name")
        fi
    done

    # Check for required category entry files
    for category in "${required_category_entries[@]}"; do
        if [[ ! -f "tests/${category}.rs" ]]; then
            missing_category_entry_violations+=("$category")
        fi
    done

    # Check for unauthorized files in tests/ root
    for file in tests/*.rs; do
        [[ ! -f "$file" ]] && continue

        basename="${file##*/}"
        basename="${basename%.rs}"

        # Skip archive files
        [[ "$basename" =~ ^_ ]] && continue

        # Check if it's a valid category entry file OR valid module-specific file
        local is_valid=false

        # Check if it's a category entry file
        for category in "${required_category_entries[@]}"; do
            if [[ "$basename" == "$category" ]]; then
                is_valid=true
                break
            fi
        done

        # Check if it's a valid module-specific file
        if [[ ! $is_valid == true ]] && [[ "$basename" =~ ^($valid_categories)_[a-z_]+$ ]]; then
            is_valid=true
        fi

        # If neither, it's unauthorized
        if [[ ! $is_valid == true ]]; then
            unauthorized_root_violations+=("$file")
        fi
    done

    # Check for orphaned test directories
    for test_dir in tests/*/; do
        [[ ! -d "$test_dir" ]] && continue

        dir_name=$(basename "$test_dir")

        # Skip valid directories and archive
        if [[ ! "$dir_name" =~ ^($valid_categories|sh|old|_archive)$ ]]; then
            directory_violations+=("$test_dir")
        fi
    done

    # Calculate total violations
    local total_violations=$((${#naming_violations[@]} + ${#missing_sanity_violations[@]} + ${#missing_uat_violations[@]} + ${#directory_violations[@]} + ${#missing_category_entry_violations[@]} + ${#unauthorized_root_violations[@]}))

    # Report violations
    if [[ $total_violations -gt 0 ]]; then

        # If --violations flag is used, show organized report and exit
        if [[ "$VIOLATIONS_MODE" == "true" ]]; then
            echo "📋 Test Organization Violations Report ($total_violations total)"
            echo "================================================================"
            echo

            # Naming violations section
            if [[ ${#naming_violations[@]} -gt 0 ]]; then
                echo "🏷️  NAMING VIOLATIONS (${#naming_violations[@]} files)"
                echo "────────────────────────────────────────────────────────"
                echo "Issue: Test wrapper files don't follow naming pattern"
                echo "Required: <category>_<module>.rs (e.g., sanity_com.rs, uat_math.rs)"
                echo "Valid categories: unit, sanity, smoke, integration, e2e, uat, chaos, bench"
                echo
                for i in "${!naming_violations[@]}"; do
                    printf "%3d. %s\n" $((i + 1)) "${naming_violations[i]}"
                done
                echo
                echo "Fix: Rename files to match pattern (e.g., com_sanity.rs → sanity_com.rs)"
                echo
            fi

            # Missing sanity tests section
            if [[ ${#missing_sanity_violations[@]} -gt 0 ]]; then
                echo "🚨 MISSING SANITY TESTS (${#missing_sanity_violations[@]} modules)"
                echo "────────────────────────────────────────────────────────"
                echo "Issue: Modules without required sanity tests"
                echo "Required: Every module must have sanity tests for core functionality"
                echo
                for i in "${!missing_sanity_violations[@]}"; do
                    printf "%3d. Module '%s' (create: tests/sanity_%s.rs)\n" $((i + 1)) "${missing_sanity_violations[i]}" "${missing_sanity_violations[i]}"
                done
                echo
                echo "Fix: Create sanity test files for each module"
                echo
            fi

            # Missing UAT tests section
            if [[ ${#missing_uat_violations[@]} -gt 0 ]]; then
                echo "🎭 MISSING UAT TESTS (${#missing_uat_violations[@]} modules)"
                echo "────────────────────────────────────────────────────────"
                echo "Issue: Modules without required visual UAT/ceremony tests"
                echo "Required: Every module must have UAT tests for visual demonstrations"
                echo
                for i in "${!missing_uat_violations[@]}"; do
                    printf "%3d. Module '%s' (create: tests/uat_%s.rs)\n" $((i + 1)) "${missing_uat_violations[i]}" "${missing_uat_violations[i]}"
                done
                echo
                echo "Fix: Create UAT test files with visual demonstrations for each module"
                echo
            fi

            # Missing category entry files section
            if [[ ${#missing_category_entry_violations[@]} -gt 0 ]]; then
                echo "📋 MISSING CATEGORY ENTRY FILES (${#missing_category_entry_violations[@]} categories)"
                echo "────────────────────────────────────────────────────────"
                echo "Issue: Missing category-level test orchestrators"
                echo "Required: Each category needs an entry file (e.g., smoke.rs, unit.rs)"
                echo
                for i in "${!missing_category_entry_violations[@]}"; do
                    printf "%3d. Category '%s' (create: tests/%s.rs)\n" $((i + 1)) "${missing_category_entry_violations[i]}" "${missing_category_entry_violations[i]}"
                done
                echo
                echo "Fix: Create category entry files for cross-module integration tests"
                echo
            fi

            # Unauthorized root files section
            if [[ ${#unauthorized_root_violations[@]} -gt 0 ]]; then
                echo "🚫 UNAUTHORIZED ROOT FILES (${#unauthorized_root_violations[@]} files)"
                echo "────────────────────────────────────────────────────────"
                echo "Issue: Files in tests/ root that don't follow organization rules"
                echo "Allowed: <category>.rs or <category>_<module>.rs only"
                echo
                for i in "${!unauthorized_root_violations[@]}"; do
                    printf "%3d. %s\n" $((i + 1)) "${unauthorized_root_violations[i]}"
                done
                echo
                echo "Fix: Rename to pattern, move to tests/_adhoc/, or move to tests/_archive/"
                echo
            fi

            # Directory violations section
            if [[ ${#directory_violations[@]} -gt 0 ]]; then
                echo "📁 INVALID DIRECTORIES (${#directory_violations[@]} directories)"
                echo "────────────────────────────────────────────────────────"
                echo "Issue: Test directories don't match approved organization"
                echo "Valid: unit/, sanity/, smoke/, integration/, e2e/, uat/, chaos/, bench/, sh/, old/, _archive/"
                echo
                for i in "${!directory_violations[@]}"; do
                    printf "%3d. %s\n" $((i + 1)) "${directory_violations[i]}"
                done
                echo
                echo "Fix: Move tests to approved category directories or rename to _archive/"
                echo
            fi

            # Summary box
            local fix_summary="VIOLATION SUMMARY & FIXES

Total Violations: $total_violations
• Naming issues: ${#naming_violations[@]}
• Missing sanity tests: ${#missing_sanity_violations[@]}
• Invalid directories: ${#directory_violations[@]}

QUICK FIXES:
• Run './bin/test.sh lint' for detailed analysis
• Use './bin/test.sh --override' for emergency bypass
• Follow naming pattern: <category>_<module>.rs
• Create missing sanity tests for all modules"

            boxy_display "$fix_summary" "warning" "📊 Test Organization Fix Guide"
            exit 1
        fi

        if [[ "$STRICT_MODE" == "true" && "$OVERRIDE_MODE" != "true" ]]; then
            # HARD FAIL: Tests cannot run with violations in strict mode
            local error_text="🚫 TEST EXECUTION BLOCKED 🚫

Test organization violations detected ($total_violations total):
• Naming issues: ${#naming_violations[@]}
• Missing sanity tests: ${#missing_sanity_violations[@]}
• Invalid directories: ${#directory_violations[@]}

SOLUTION OPTIONS:
• Fix violations and re-run tests
• Use --violations flag to see complete organized list
• Use --override flag for emergency bypass with warnings
• Use --skip-enforcement to disable validation entirely

Tests cannot proceed until organization is compliant."

            boxy_display "$error_text" "error" "❌ Test Organization Violations"
            exit 1
        elif [[ "$OVERRIDE_MODE" == "true" ]]; then
            # OVERRIDE MODE: Show violations but continue with warning
            local override_text="Proceeding with violations in override mode ($total_violations total):

• Naming issues: ${#naming_violations[@]}
• Missing sanity tests: ${#missing_sanity_violations[@]}
• Invalid directories: ${#directory_violations[@]}

Fix these violations when possible.
Use --violations flag to see complete organized list."

            boxy_display "$override_text" "warning" "⚠️  Organization Violations (Override Active)"
        else
            # PERMISSIVE MODE: Just warn
            echo "⚠️  Test structure warnings ($total_violations total):"
            echo "   • Naming issues: ${#naming_violations[@]}"
            echo "   • Missing sanity tests: ${#missing_sanity_violations[@]}"
            echo "   • Invalid directories: ${#directory_violations[@]}"
            echo "   Use --violations flag for detailed breakdown"
            echo
        fi
    else
        echo "✅ Test structure is compliant"
    fi

    return 0
}

# Lint mode: check compliance only
lint_tests() {
    echo "🧹 Linting test organization..."
    echo

    STRICT_MODE="true"  # Always strict in lint mode
    validate_test_structure

    echo "✅ Test organization lint completed"
}

# Generate test report
report_tests() {
    echo "📊 Test Organization Report"
    echo "=========================="
    echo

    # Count tests by category
    local categories=(unit sanity smoke integration e2e uat chaos bench)

    for category in "${categories[@]}"; do
        local count=$(find tests -name "${category}_*.rs" -o -name "${category}.rs" 2>/dev/null | wc -l)
        echo "$category: $count test files"
    done

    echo
    echo "Test directories:"
    for dir in tests/*/; do
        [[ ! -d "$dir" ]] && continue
        local dir_name=$(basename "$dir")
        local file_count=$(find "$dir" -name "*.rs" 2>/dev/null | wc -l)
        echo "  $dir_name/: $file_count files"
    done

    echo
    validate_test_structure
}


# Parse optional flags (can be anywhere in arguments)
VERBOSE_MODE="false"
QUICK_MODE="true"  # Default to quick mode
COMPREHENSIVE_MODE="false"
STRICT_MODE="true"  # Default to strict - tests fail if disorganized
SKIP_ENFORCEMENT="false"
OVERRIDE_MODE="false"
VIOLATIONS_MODE="false"
ARGS=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        --verbose|-v)
            VERBOSE_MODE="true"
            shift 1
            ;;
        --quick)
            QUICK_MODE="true"
            COMPREHENSIVE_MODE="false"
            shift 1
            ;;
        --comprehensive|--full)
            QUICK_MODE="false"
            COMPREHENSIVE_MODE="true"
            shift 1
            ;;
        --strict)
            STRICT_MODE="true"
            shift 1
            ;;
        --skip-enforcement)
            SKIP_ENFORCEMENT="true"
            STRICT_MODE="false"
            shift 1
            ;;
        --override)
            OVERRIDE_MODE="true"
            STRICT_MODE="false"
            shift 1
            ;;
        --violations)
            VIOLATIONS_MODE="true"
            shift 1
            ;;
        *)
            ARGS+=("$1")
            shift 1
            ;;
    esac
done

# Restore non-flag arguments
set -- "${ARGS[@]}"

# Available tests
declare -A TESTS=(
    # Core functionality tests
    ["sanity"]="sanity_main"                     # Sanity package (core + baseline)
    ["param"]="features_param"                    # Parameter expansion comprehensive tests (wrapper)
    ["param-helpers"]="features_param"            # Param helper layer tests (wrapper)
    ["param-uat"]="uat_main"                      # UAT: param usage demo (wrapper)
    ["macros"]="macro-smoke"                     # Basic macro functionality 
    ["context"]="uat_global.rs"                 # Legacy alias → Global UAT wrapper
    ["global"]="uat_global.rs"                  # Global store/expansion UAT
    ["host-env"]="uat_host_env.rs"             # Host env UAT
    ["host-paths"]="uat_host_paths.rs"         # Host XDG/RSB paths UAT
    ["cli"]="sh/cli_macros_e2e"                 # CLI macros E2E (example-driven)
    ["args"]="args-processing"                   # Command line argument processing
    ["stdopts"]="stdopts"                        # Short-flag expansion behind feature
    ["uat-colors"]="uat_main"                     # UAT: visible color demo (wrapper)
    ["uat-colors-macros"]="uat_main"              # UAT: colored! macro behaviors (wrapper)
    ["uat-stdopts"]="uat_stdopts"                # UAT: visible stdopts demo
    ["uat-glyphs"]="uat_glyphs"                  # UAT: visible glyphs demo
    ["uat-visual"]="uat_visual"                  # UAT: bg + color + glyphs
    ["uat-prompts"]="uat_prompts"                # UAT: prompts (confirm/ask/select)
    # Threads & Bash
    ["threads"]="threads_sanity"                 # Threads sanity wrapper
    ["uat-threads"]="uat_threads"               # Threads UAT wrapper
    ["bash"]="bash_sanity"                       # Bash sanity wrapper
    ["uat-bash"]="uat_bash"                      # Bash UAT wrapper

    # Visual colors
    ["colors"]="features_colors"                  # Rust tests for color registry/API (wrapper)
    ["colors-runtime"]="features_colors"          # Runtime toggles and backgrounds gating (wrapper)
    
    # Integration tests  
    ["bootstrap"]="bootstrap-lifecycle"          # Full bootstrap → options → dispatch flow
    ["integration"]="rsb-integration"            # End-to-end RSB workflows
    
    # Regression tests
    ["regression"]="regression-suite"            # Tests for previously broken functionality
    ["defects"]="known-defects"                  # Verification of fixed defects
    
    # Comprehensive suites
    ["all"]="all-tests"                          # Run all test categories
    ["smoke"]="smoke-tests"                      # Quick smoke test suite
    ["full"]="comprehensive-suite"               # Full validation test suite
    
    # Legacy/compatibility tests (moved to old/)
    ["old"]="old/legacy-tests"                   # Original RSB tests (now in old/)
)

show_help() {
    if [[ -n "$BOXY" ]]; then
        cat <<-EOF | $BOXY --theme info --title "🧪 RSB Test Runner (BASHFX Aligned)" --width max
Available Commands:
  test.sh [options] run <test>      Run specific test
  test.sh list                      List available tests
  test.sh lint                      Check test organization compliance
  test.sh report                    Generate test organization report
  test.sh help                      Show this help

Options:
  --comprehensive        Run full validation test suite
  --quick                Force quick mode (default)
  --verbose              Show detailed test output
  --strict               Fail on test organization violations (DEFAULT)
  --override             Run tests despite violations (shows warnings)
  --violations           Show complete violation list and exit
  --skip-enforcement     Skip test organization validation entirely

Test Categories (BASHFX Organization):
  sanity                 Core functionality validation (REQUIRED for all modules)
  smoke                  Minimal CI tests (<10s total)
  unit                   Fast, isolated module tests
  integration            Cross-module interaction tests
  e2e                    End-to-end user workflow tests
  uat                    User Acceptance Tests (with visual ceremony)
  chaos                  Edge cases, stress tests, property tests
  bench                  Performance benchmarks

Legacy Tests (Transitioning):
  param                  Parameter expansion comprehensive tests
  macros                 Basic macro functionality tests
  context                Global context operations tests
  args                   Command line argument processing tests
  bootstrap              Full bootstrap → options → dispatch flow
  regression             Tests for previously broken functionality
  defects                Verification of fixed defects
  all                    Run all test categories
  full                   Full validation test suite
  old                    Legacy tests (moved to old/ directory)
EOF
    else
        echo "🧪 RSB TEST RUNNER (BASHFX Aligned)"
        echo "===================================="
        echo
        echo "Available Commands:"
        echo "  test.sh [options] run <test>      Run specific test"
        echo "  test.sh list                      List available tests"
        echo "  test.sh lint                      Check test organization compliance"
        echo "  test.sh report                    Generate test organization report"
        echo "  test.sh help                      Show this help"
        echo
        echo "Options:"
        echo "  --comprehensive        Run full validation test suite"
        echo "  --quick                Force quick mode (default)"
        echo "  --verbose              Show detailed test output"
        echo "  --strict               Fail on test organization violations (DEFAULT)"
        echo "  --override             Run tests despite violations (shows warnings)"
        echo "  --violations           Show complete violation list and exit"
        echo "  --skip-enforcement     Skip test organization validation entirely"
        echo
        echo "Test Categories (BASHFX Organization):"
        echo "  sanity                 Core functionality validation (REQUIRED for all modules)"
        echo "  smoke                  Minimal CI tests (<10s total)"
        echo "  unit                   Fast, isolated module tests"
        echo "  integration            Cross-module interaction tests"
        echo "  e2e                    End-to-end user workflow tests"
        echo "  uat                    User Acceptance Tests (with visual ceremony)"
        echo "  chaos                  Edge cases, stress tests, property tests"
        echo "  bench                  Performance benchmarks"
        echo
        echo "Legacy Tests (Transitioning):"
        echo "  param                  Parameter expansion comprehensive tests"
        echo "  macros                 Basic macro functionality tests"
        echo "  context                Global context operations tests"
        echo "  args                   Command line argument processing tests"
        echo "  bootstrap              Full bootstrap → options → dispatch flow"
        echo "  regression             Tests for previously broken functionality"
        echo "  defects                Verification of fixed defects"
        echo "  all                    Run all test categories"
        echo "  full                   Full validation test suite"
        echo "  old                    Legacy tests (moved to old/ directory)"
    fi
}

list_tests() {
    if [[ -n "$BOXY" ]]; then
        {
            echo "Available Tests:"
            echo
            for test_name in $(printf "%s\n" "${!TESTS[@]}" | sort); do
                test_file="${TESTS[$test_name]}"
                
                # Special handling for sanity package
                if [[ "$test_name" == "sanity" ]]; then
                    if [[ -f "$TEST_DIR/sanity_main.rs" ]]; then
                        echo "✅ $test_name → sanity_main.rs (core + baseline)"
                    else
                        echo "❌ $test_name → sanity_main.rs (missing)"
                    fi
                elif [[ -f "$TEST_DIR/$test_file.sh" ]]; then
                    echo "✅ $test_name → $test_file.sh"
                elif [[ -f "$TEST_DIR/$test_file" ]]; then
                    echo "✅ $test_name → $test_file"
                elif [[ -f "$TEST_DIR/$test_file.rs" ]]; then
                    echo "✅ $test_name → $test_file.rs"
                else
                    echo "❌ $test_name → $test_file (missing)"
                fi
            done
            echo
            echo "Auto‑discovered wrappers:"
            for wrap in $(find "$TEST_DIR" -maxdepth 1 -type f -name "*.rs" -printf "%f\n" | sort); do
                base="${wrap%.rs}"
                printf "  • %s\n" "$base"
            done
        } | $BOXY --theme info --title "🗂️ Available RSB Tests" --width max
    else
        echo "🗂️ AVAILABLE RSB TESTS"
        echo "====================="
        for test_name in $(printf "%s\n" "${!TESTS[@]}" | sort); do
            test_file="${TESTS[$test_name]}"
            
            # Special handling for sanity package
            if [[ "$test_name" == "sanity" ]]; then
                if [[ -f "$TEST_DIR/sanity_main.rs" ]]; then
                    echo "✅ $test_name → sanity_main.rs (core + baseline)"
                else
                    echo "❌ $test_name → sanity_main.rs (missing)"
                fi
            elif [[ -f "$TEST_DIR/$test_file.sh" ]]; then
                echo "✅ $test_name → $test_file.sh"
            elif [[ -f "$TEST_DIR/$test_file" ]]; then
                echo "✅ $test_name → $test_file"
            elif [[ -f "$TEST_DIR/$test_file.rs" ]]; then
                echo "✅ $test_name → $test_file.rs"
            else
                echo "❌ $test_name → $test_file (missing)"
            fi
        done
        echo
        echo "Auto‑discovered wrappers:"
        for wrap in $(find "$TEST_DIR" -maxdepth 1 -type f -name "*.rs" -printf "%f\n" | sort); do
            base="${wrap%.rs}"
            echo "  • $base"
        done
    fi
}

run_test() {
    local test_name="$1"
    
    if [[ -z "$test_name" ]]; then
        echo "❌ Error: Test name required"
        echo "Use: test.sh run <test>"
        echo "Available tests: ${!TESTS[*]}"
        exit 1
    fi
    
    if [[ ! "${TESTS[$test_name]+exists}" ]]; then
        # Fallback: run by wrapper filename or shell script name
        if [[ -f "$TEST_DIR/$test_name.rs" ]]; then
            echo "ℹ️  Running auto‑discovered wrapper: $test_name.rs"
            ctest test --test "$test_name" -- --nocapture
            exit 0
        elif [[ -f "$TEST_DIR/sh/$test_name.sh" ]]; then
            echo "ℹ️  Running shell test: tests/sh/$test_name.sh"
            exec bash "$TEST_DIR/sh/$test_name.sh"
        else
            echo "❌ Error: Unknown test '$test_name'"
            echo "Available tests: ${!TESTS[*]}"
            echo "Auto wrappers available:"
            find "$TEST_DIR" -maxdepth 1 -type f -name "*.rs" -printf "  • %f\n" | sed 's/\.rs$//'
            exit 1
        fi
    fi
    
    local test_file="${TESTS[$test_name]}"
    
    # If mapping points to a Rust wrapper (tests/<name>.rs), run as Cargo test
    if [[ "$test_file" == *.rs && -f "$TEST_DIR/$test_file" ]]; then
        local wrapper_name="${test_file%.rs}"
        if [[ -n "$BOXY" ]]; then
            echo "🦀 Running Rust wrapper: $test_file" | $BOXY --theme success --title "🧪 RSB Test Runner" --width max
        else
            echo "🦀 Running Rust wrapper: $test_file"
        fi
        ctest test --test "$wrapper_name" -- --nocapture
        exit 0
    fi
    
    # Header
    if [[ -n "$BOXY" ]]; then
        echo "🚀 Running RSB test: $test_name" | $BOXY --theme success --title "🧪 RSB Test Runner" --width max
    else
        echo "🚀 Running RSB test: $test_name"
        echo "=========================="
    fi
    echo
    
    # Change to project root 
    cd "$ROOT_DIR"
    
    # Export test configuration
    export RSB_TEST_MODE="true"
    export RSB_VERBOSE="${VERBOSE_MODE}"
    export RSB_QUICK_MODE="${QUICK_MODE}"
    export RSB_COMPREHENSIVE="${COMPREHENSIVE_MODE}"
    
    # Handle different test types
    case "$test_name" in
        "all")
            # Run a broad set of tests across categories
            "$0" run sanity
            "$0" run global
            "$0" run param
            "$0" run stdopts
            "$0" run colors
            "$0" run colors-runtime
            "$0" run uat-colors
            "$0" run uat-colors-macros
            "$0" run uat-glyphs
            "$0" run uat-visual
            "$0" run uat-prompts
            ;;
        "smoke")
            # Quick validation: core sanity + param; skip heavy visuals
            "$0" run sanity
            "$0" run global
            "$0" run param
            ;;
        "sanity")
            # Sanity package (core + baseline demos)
            if [[ "$VERBOSE_MODE" == "true" ]]; then
                echo "🦀 Running sanity package with verbose output..."
            ctest test --test sanity_main -- --nocapture
            else
                echo "🦀 Running sanity package..."
                ctest test --test sanity_main
            fi
            ;;
        "param")
            # Comprehensive parameter expansion tests
            if [[ "$VERBOSE_MODE" == "true" ]]; then
                echo "🦀 Running comprehensive parameter expansion tests with verbose output..."
                ctest test --test features_param -- --nocapture
            else
                echo "🦀 Running comprehensive parameter expansion tests..."
                ctest test --test features_param
            fi
            ;;
        "param-helpers")
            if [[ "$VERBOSE_MODE" == "true" ]]; then
                echo "🦀 Running param helper tests with verbose output..."
                ctest test --test param_helpers -- --nocapture
            else
                echo "🦀 Running param helper tests..."
                ctest test --test param_helpers
            fi
            ;;
        "param-uat")
            if [[ "$VERBOSE_MODE" == "true" ]]; then
                echo "🧪 UAT: param! usage demo with verbose output..."
            else
                echo "🧪 UAT: param! usage demo..."
            fi
            ctest test --features visuals --test uat_main -- --nocapture
            ;;
        "colors")
            # Ensure color sets are enabled for the test run
            export RSB_COLORS="simple,status,named"
            if [[ "$VERBOSE_MODE" == "true" ]]; then
                echo "🦀 Running color sanity tests with verbose output..."
                ctest test --features visuals --test features_colors -- --nocapture
            else
                echo "🦀 Running color sanity tests..."
                ctest test --features visuals --test features_colors
            fi
            ;;
        "colors-runtime")
            export RSB_COLORS="simple,status,named"
            export RSB_COLOR="always"
            if [[ "$VERBOSE_MODE" == "true" ]]; then
                echo "🦀 Running color runtime tests with verbose output..."
                ctest test --features visuals --test features_colors -- --nocapture
            else
                echo "🦀 Running color runtime tests..."
                ctest test --features visuals --test features_colors
            fi
            ;;
        "stdopts")
            if [[ "$VERBOSE_MODE" == "true" ]]; then
                echo "🦀 Running stdopts tests (feature-gated) with verbose output..."
                ctest test --features stdopts --test stdopts -- --nocapture
            else
                echo "🦀 Running stdopts tests (feature-gated)..."
                ctest test --features stdopts --test stdopts
            fi
            ;;
        "uat-colors")
            # Force rich color sets and visible output
            export RSB_COLORS="simple,status,named"
            export RSB_COLOR="always"
            if [[ "$VERBOSE_MODE" == "true" ]]; then
                echo "🧪 UAT: Colors demo (visible) with verbose output..."
            else
                echo "🧪 UAT: Colors demo (visible)..."
            fi
            ctest test --features visuals --test uat_main -- --nocapture
            ;;
        "uat-colors-macros")
            export RSB_COLORS="simple,status,named"
            export RSB_COLOR="always"
            if [[ "$VERBOSE_MODE" == "true" ]]; then
                echo "🧪 UAT: Colors macros (colored!) demo with verbose output..."
            else
                echo "🧪 UAT: Colors macros (colored!) demo..."
            fi
            ctest test --features visuals --test uat_main -- --nocapture
            ;;
        "uat-stdopts")
            if [[ "$VERBOSE_MODE" == "true" ]]; then
                echo "🧪 UAT: Stdopts demo (visible) with verbose output..."
            else
                echo "🧪 UAT: Stdopts demo (visible)..."
            fi
            ctest test --features stdopts --test uat_stdopts -- --nocapture
            ;;
        "uat-glyphs")
            if [[ "$VERBOSE_MODE" == "true" ]]; then
                echo "🧪 UAT: Glyphs demo (visible) with verbose output..."
            else
                echo "🧪 UAT: Glyphs demo (visible)..."
            fi
            ctest test --features visuals --test uat_main -- --nocapture
            ;;
        "uat-visual")
            export RSB_COLORS="simple,status,named,bg"
            export RSB_COLOR="always"
            if [[ "$VERBOSE_MODE" == "true" ]]; then
                echo "🧪 UAT: Visual combo (bg + color + glyphs) with verbose output..."
            else
                echo "🧪 UAT: Visual combo (bg + color + glyphs)..."
            fi
            ctest test --features visuals --test uat_main -- --nocapture
            ;;
        "uat-prompts")
            export RSB_COLORS="simple"
            export RSB_COLOR="always"
            if [[ "$VERBOSE_MODE" == "true" ]]; then
                echo "🧪 UAT: Prompts (confirm/ask/select) with verbose output..."
            else
                echo "🧪 UAT: Prompts (confirm/ask/select)..."
            fi
            cargo test --features visuals --test uat_main -- --nocapture
            ;;
        *)
            # Shell-based tests
            local test_path=""
            
            # Try different file extensions and paths
            if [[ -f "$TEST_DIR/$test_file.sh" ]]; then
                test_path="$TEST_DIR/$test_file.sh"
            elif [[ -f "$TEST_DIR/$test_file" ]]; then
                # If this is a Rust wrapper but we didn't catch it above, run via cargo
                if [[ "$test_file" == *.rs ]]; then
                    local wrapper_name="${test_file%.rs}"
                    ctest test --test "$wrapper_name" -- --nocapture
                    exit 0
                fi
                test_path="$TEST_DIR/$test_file"
            elif [[ -f "$TEST_DIR/sh/$test_file.sh" ]]; then
                test_path="$TEST_DIR/sh/$test_file.sh"
            else
                echo "❌ Error: Test file not found for '$test_name'"
                echo "    Checked: $TEST_DIR/$test_file.sh"
                echo "    Checked: $TEST_DIR/$test_file"
                echo "    Checked: $TEST_DIR/sh/$test_file.sh"
                exit 1
            fi
            
            echo "📜 Executing shell test: $test_path"
            exec bash "$test_path"
            ;;
    esac
}

# Main command dispatch
case "${1:-help}" in
    "run")
        # Validate structure before running tests (unless skipped)
        if [[ "$SKIP_ENFORCEMENT" != "true" ]]; then
            validate_test_structure
        fi
        run_test "$2"
        ;;
    "list")
        list_tests
        ;;
    "lint")
        lint_tests
        ;;
    "report")
        report_tests
        ;;
    "help"|"--help"|"-h")
        show_help
        ;;
    *)
        echo "❌ Unknown command: $1"
        echo "Use: test.sh help"
        exit 1
        ;;
esac
