#!/usr/bin/env bash
#
# RSB Test Ceremony Runner (BASHFX Aligned)
# Visual test orchestration with boxy integration
#
# Usage:
#   ./ceremony.sh sanity                    # Run sanity test suite with ceremony
#   ./ceremony.sh uat                       # Run UAT with visual demonstrations
#   ./ceremony.sh smoke                     # Run smoke tests with timing
#   ./ceremony.sh all                       # Run complete test ceremony
#   ./ceremony.sh --list                    # List available test ceremonies
#   ./ceremony.sh --report                  # Generate test organization report
#
# portable: bash, boxy (optional for enhanced display), cargo
# builtins: printf, test, local, read, basename, dirname, find, sort
#

# Script directory determination
CEREMONY_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$CEREMONY_DIR/../../" && pwd)"
TESTS_DIR="$PROJECT_ROOT/tests"

# Force use of local boxy binary if available
LOCAL_BOXY="$PROJECT_ROOT/target/release/boxy"
if [[ -f "$LOCAL_BOXY" ]]; then
    export PATH="$PROJECT_ROOT/target/release:$PATH"
fi

# Check for boxy availability and test basic functionality
BOXY_AVAILABLE=false
if command -v boxy &> /dev/null; then
    # Test if boxy supports basic usage
    if echo "test" | boxy --theme info --title "test" &> /dev/null; then
        BOXY_AVAILABLE=true
    fi
fi

# ═══════════════════════════════════════════════════════════════════════════════
# CEREMONY UX FUNCTIONS (BASHFX Visual Friendliness)
# ═══════════════════════════════════════════════════════════════════════════════

ux_info() {
    if [[ "$BOXY_AVAILABLE" == "true" ]]; then
        echo "$*" | boxy --theme info
    else
        echo "ℹ️  INFO: $*"
    fi
}

ux_success() {
    if [[ "$BOXY_AVAILABLE" == "true" ]]; then
        echo "$*" | boxy --theme success
    else
        echo "✅ SUCCESS: $*"
    fi
}

ux_warn() {
    if [[ "$BOXY_AVAILABLE" == "true" ]]; then
        echo "$*" | boxy --theme warning
    else
        echo "⚠️  WARN: $*"
    fi
}

ux_error() {
    if [[ "$BOXY_AVAILABLE" == "true" ]]; then
        echo "$*" | boxy --theme error
    else
        echo "❌ ERROR: $*"
    fi
}

ux_ceremony_header() {
    local title="$1"
    local subtitle="${2:-}"

    if [[ "$BOXY_AVAILABLE" == "true" ]]; then
        if [[ -n "$subtitle" ]]; then
            printf "%s\n%s\n" "$title" "$subtitle" | boxy --theme info --title "🎭 RSB Test Ceremony" --width max
        else
            echo "$title" | boxy --theme info --title "🎭 RSB Test Ceremony" --width max
        fi
    else
        echo "🎭 ═══════════════════════════════════════════════════════════════════════════════"
        echo "   $title"
        [[ -n "$subtitle" ]] && echo "   $subtitle"
        echo "   ═══════════════════════════════════════════════════════════════════════════════"
    fi
}

ux_test_step() {
    local step="$1"
    local total="$2"
    local name="$3"
    local status="${4:-RUNNING}"

    local symbol
    case "$status" in
        RUNNING) symbol="🔄" ;;
        PASS) symbol="✅" ;;
        FAIL) symbol="❌" ;;
        SKIP) symbol="⏭️" ;;
        STUB) symbol="🚧" ;;
        *) symbol="❔" ;;
    esac

    printf "  %s [%02d/%02d] %s - %s\n" "$symbol" "$step" "$total" "$name" "$status"
}

ux_ceremony_summary() {
    local suite_name="$1"
    local passed="$2"
    local failed="$3"
    local skipped="${4:-0}"
    local duration="${5:-unknown}"

    local total=$((passed + failed + skipped))
    local success_rate=0
    [[ $total -gt 0 ]] && success_rate=$((passed * 100 / total))

    local summary_text
    printf -v summary_text "Suite: %s\nPassed: %d | Failed: %d | Skipped: %d\nTotal: %d tests | Success Rate: %d%%\nDuration: %s" \
        "$suite_name" "$passed" "$failed" "$skipped" "$total" "$success_rate" "$duration"

    if [[ "$BOXY_AVAILABLE" == "true" ]]; then
        if [[ $failed -eq 0 ]]; then
            echo "$summary_text" | boxy --theme success --title "📊 Test Results"
        else
            echo "$summary_text" | boxy --theme warning --title "📊 Test Results"
        fi
    else
        echo "📊 ═══════════════════════════════════════════════════════════════════════════════"
        echo "$summary_text" | sed 's/^/   /'
        echo "   ═══════════════════════════════════════════════════════════════════════════════"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# TEST DISCOVERY AND EXECUTION
# ═══════════════════════════════════════════════════════════════════════════════

discover_test_categories() {
    local categories=()

    # Discover from wrapper files
    for wrapper in "$TESTS_DIR"/*.rs; do
        [[ ! -f "$wrapper" ]] && continue

        local basename
        basename="$(basename "$wrapper" .rs)"

        # Skip archive files
        [[ "$basename" =~ ^_ ]] && continue

        # Extract category from pattern <category>_<module> or just <category>
        if [[ "$basename" =~ ^([a-z]+)(_.*)?$ ]]; then
            local category="${BASH_REMATCH[1]}"

            # Only add if it's a valid category
            case "$category" in
                unit|sanity|smoke|integration|e2e|uat|chaos|bench)
                    # Add to categories if not already present
                    if [[ ! " ${categories[*]} " =~ " ${category} " ]]; then
                        categories+=("$category")
                    fi
                    ;;
            esac
        fi
    done

    printf "%s\n" "${categories[@]}" | sort -u
}

discover_tests_in_category() {
    local category="$1"
    local tests=()

    # Find wrapper files for this category
    for wrapper in "$TESTS_DIR"/${category}_*.rs "$TESTS_DIR"/${category}.rs; do
        [[ ! -f "$wrapper" ]] && continue

        local basename
        basename="$(basename "$wrapper" .rs)"
        tests+=("$basename")
    done

    printf "%s\n" "${tests[@]}" | sort
}

# Execute a single test with ceremony
execute_test_with_ceremony() {
    local test_name="$1"
    local step="$2"
    local total="$3"

    ux_test_step "$step" "$total" "$test_name" "RUNNING"

    local start_time
    start_time="$(date +%s)"

    local exit_code=0

    # Use the existing test.sh runner with enforcement skipped for ceremony
    if "$PROJECT_ROOT/bin/test.sh" --skip-enforcement run "$test_name" &> /dev/null; then
        exit_code=0
    else
        exit_code=1
    fi

    local end_time
    end_time="$(date +%s)"
    local duration=$((end_time - start_time))

    if [[ $exit_code -eq 0 ]]; then
        ux_test_step "$step" "$total" "$test_name (${duration}s)" "PASS"
    else
        ux_test_step "$step" "$total" "$test_name (${duration}s)" "FAIL"
    fi

    return $exit_code
}

# Execute category ceremony
execute_category_ceremony() {
    local category="$1"

    local tests
    mapfile -t tests < <(discover_tests_in_category "$category")

    if [[ ${#tests[@]} -eq 0 ]]; then
        ux_warn "No tests found for category: $category"
        return 0
    fi

    local category_title
    case "$category" in
        sanity) category_title="Core Functionality Validation" ;;
        smoke) category_title="Minimal CI Tests (<10s)" ;;
        unit) category_title="Fast Isolated Module Tests" ;;
        integration) category_title="Cross-Module Interaction Tests" ;;
        e2e) category_title="End-to-End User Workflows" ;;
        uat) category_title="User Acceptance Tests (Visual Ceremony)" ;;
        chaos) category_title="Edge Cases & Stress Tests" ;;
        bench) category_title="Performance Benchmarks" ;;
        *) category_title="Test Category: $category" ;;
    esac

    ux_ceremony_header "$category_title" "Running ${#tests[@]} test suites"

    local start_time
    start_time="$(date +%s)"

    local passed=0
    local failed=0
    local step=1

    for test in "${tests[@]}"; do
        if execute_test_with_ceremony "$test" "$step" "${#tests[@]}"; then
            ((passed++))
        else
            ((failed++))
        fi
        ((step++))
    done

    local end_time
    end_time="$(date +%s)"
    local duration=$((end_time - start_time))

    ux_ceremony_summary "$category" "$passed" "$failed" 0 "${duration}s"

    return $([[ $failed -eq 0 ]] && echo 0 || echo 1)
}

# Execute complete ceremony (all categories)
execute_complete_ceremony() {
    local categories
    mapfile -t categories < <(discover_test_categories)

    if [[ ${#categories[@]} -eq 0 ]]; then
        ux_error "No test categories discovered"
        return 1
    fi

    ux_ceremony_header "Complete RSB Test Ceremony" "Progressive testing across ${#categories[@]} categories"

    local total_passed=0
    local total_failed=0
    local total_start_time
    total_start_time="$(date +%s)"

    # Execute in order of complexity: smoke -> sanity -> unit -> integration -> e2e -> uat -> chaos -> bench
    local ordered_categories=(smoke sanity unit integration e2e uat chaos bench)

    for category in "${ordered_categories[@]}"; do
        # Only run if category exists
        if [[ " ${categories[*]} " =~ " ${category} " ]]; then
            echo
            if execute_category_ceremony "$category"; then
                ux_success "Category passed: $category"
            else
                ux_error "Category failed: $category"
                ((total_failed++))
            fi
            ((total_passed++))
        fi
    done

    local total_end_time
    total_end_time="$(date +%s)"
    local total_duration=$((total_end_time - total_start_time))

    echo
    ux_ceremony_summary "Complete Test Ceremony" "$total_passed" "$total_failed" 0 "${total_duration}s"

    return $([[ $total_failed -eq 0 ]] && echo 0 || echo 1)
}

# ═══════════════════════════════════════════════════════════════════════════════
# LISTING AND REPORTING
# ═══════════════════════════════════════════════════════════════════════════════

list_ceremonies() {
    local categories
    mapfile -t categories < <(discover_test_categories)

    local list_content="Available Test Ceremonies:\n\n"

    for category in "${categories[@]}"; do
        local tests
        mapfile -t tests < <(discover_tests_in_category "$category")

        list_content+="📁 $category (${#tests[@]} tests)\n"
        for test in "${tests[@]}"; do
            list_content+="  • $test\n"
        done
        list_content+="\n"
    done

    list_content+="Usage Examples:\n"
    list_content+="  ./ceremony.sh sanity         # Run sanity test ceremony\n"
    list_content+="  ./ceremony.sh uat            # Run UAT with visual demonstrations\n"
    list_content+="  ./ceremony.sh all            # Run complete test ceremony\n"

    if [[ "$BOXY_AVAILABLE" == "true" ]]; then
        printf "$list_content" | boxy --theme info --title "🎭 RSB Test Ceremonies" --width max
    else
        printf "$list_content"
    fi
}

generate_ceremony_report() {
    ux_ceremony_header "Test Organization Report" "Current RSB test structure analysis"

    # Use existing test.sh report functionality
    "$PROJECT_ROOT/bin/test.sh" report
}

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN DISPATCH
# ═══════════════════════════════════════════════════════════════════════════════

show_usage() {
    local usage_text="RSB Test Ceremony Runner (BASHFX Aligned)\n\n"
    usage_text+="USAGE:\n"
    usage_text+="    ceremony.sh <target>\n\n"
    usage_text+="TARGETS:\n"
    usage_text+="    sanity               Run sanity test ceremony\n"
    usage_text+="    smoke                Run smoke test ceremony\n"
    usage_text+="    unit                 Run unit test ceremony\n"
    usage_text+="    integration          Run integration test ceremony\n"
    usage_text+="    e2e                  Run end-to-end test ceremony\n"
    usage_text+="    uat                  Run UAT with visual demonstrations\n"
    usage_text+="    chaos                Run chaos test ceremony\n"
    usage_text+="    bench                Run benchmark ceremony\n"
    usage_text+="    all                  Run complete test ceremony\n"
    usage_text+="    --list               List available test ceremonies\n"
    usage_text+="    --report             Generate test organization report\n\n"
    usage_text+="ENVIRONMENT:\n"
    usage_text+="    BOXY                 Path to boxy binary for enhanced display\n\n"
    usage_text+="EXAMPLES:\n"
    usage_text+="    ./ceremony.sh sanity\n"
    usage_text+="    ./ceremony.sh uat\n"
    usage_text+="    ./ceremony.sh all\n"

    if [[ "$BOXY_AVAILABLE" == "true" ]]; then
        printf "$usage_text" | boxy --theme info --title "🎭 Ceremony Runner Help" --width max
    else
        printf "$usage_text"
    fi
}

# Main execution
main() {
    local target="${1:-}"

    case "$target" in
        --help|-h|help)
            show_usage
            ;;
        --list|list)
            list_ceremonies
            ;;
        --report|report)
            generate_ceremony_report
            ;;
        all|complete)
            execute_complete_ceremony
            ;;
        sanity|smoke|unit|integration|e2e|uat|chaos|bench)
            execute_category_ceremony "$target"
            ;;
        "")
            ux_warn "No target specified"
            show_usage
            exit 1
            ;;
        *)
            ux_error "Unknown target: $target"
            show_usage
            exit 1
            ;;
    esac
}

# Execute if run directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi