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


# Parse optional flags (can be anywhere in arguments)
VERBOSE_MODE="false"
QUICK_MODE="true"  # Default to quick mode
COMPREHENSIVE_MODE="false"
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
        cat <<-EOF | $BOXY --theme info --title "🧪 RSB Test Runner" --width max
Available Commands:
  test.sh [--comprehensive|--verbose] run <test>    Run specific test
  test.sh list                                      List available tests
  test.sh help                                      Show this help

Options:
  --comprehensive        Run full validation test suite
  --quick                Force quick mode (default)
  --verbose              Show detailed test output

Available Tests:
  sanity                 Core functionality unit tests (Rust)
  param                  Parameter expansion comprehensive tests
  macros                 Basic macro functionality tests
  context                Global context operations tests
  args                   Command line argument processing tests
  bootstrap              Full bootstrap → options → dispatch flow
  integration            End-to-end RSB workflow tests
  regression             Tests for previously broken functionality
  defects                Verification of fixed defects (param! fixes)
  all                    Run all test categories
  smoke                  Quick smoke test suite
  full                   Full validation test suite
  old                    Legacy tests (moved to old/ directory)
EOF
    else
        echo "🧪 RSB TEST RUNNER"
        echo "=================="
        echo
        echo "Available Commands:"
        echo "  test.sh [--comprehensive|--verbose] run <test>    Run specific test"
        echo "  test.sh list                                      List available tests" 
        echo "  test.sh help                                      Show this help"
        echo
        echo "Options:"
        echo "  --comprehensive        Run full validation test suite"
        echo "  --quick                Force quick mode (default)"
        echo "  --verbose              Show detailed test output"
        echo
        echo "Available Tests:"
        echo "  sanity                 Core functionality unit tests (Rust)"
        echo "  param                  Parameter expansion comprehensive tests"
        echo "  macros                 Basic macro functionality tests"
        echo "  context                Global context operations tests"
        echo "  args                   Command line argument processing tests"
        echo "  bootstrap              Full bootstrap → options → dispatch flow"
        echo "  integration            End-to-end RSB workflow tests"
        echo "  regression             Tests for previously broken functionality"
        echo "  defects                Verification of fixed defects (param! fixes)"
        echo "  all                    Run all test categories"
        echo "  smoke                  Quick smoke test suite"
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
        run_test "$2"
        ;;
    "list")
        list_tests
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
