#!/usr/bin/env bash
#
# Ceremony Runner - Boxy API Testing Orchestration
# Executes numbered ceremony batches for comprehensive API validation
#
# Usage:
#   ./ceremony_runner.sh batch_01                  # Run specific batch
#   ./ceremony_runner.sh ceremony_05               # Run specific ceremony  
#   ./ceremony_runner.sh ceremony_01-05            # Run ceremony range
#   ./ceremony_runner.sh --list                    # List all ceremonies
#   ./ceremony_runner.sh --generate-docs           # Generate API reference
#
# portable: bash, boxy (optional for enhanced display)
# builtins: printf, test, local, read, basename, dirname
#

# Script directory determination
CEREMONY_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCRIPT_DIR="$(cd "$CEREMONY_DIR/../" && pwd)"
PROJECT_ROOT="$(cd "$CEREMONY_DIR/../../" && pwd)"

# Force use of local project boxy binary (not global)
LOCAL_BOXY="$PROJECT_ROOT/target/release/boxy"
if [[ -f "$LOCAL_BOXY" ]]; then
    export PATH="$PROJECT_ROOT/target/release:$PATH"
fi

# Essential UX functions - always defined for ceremony runner
ux_info() { echo "INFO: $*"; }
ux_success() { echo "SUCCESS: $*"; }
ux_warn() { echo "WARN: $*"; }
ux_error() { echo "ERROR: $*"; }
ux_separator() { echo "═══════════════════════════════════════════════════════════════════════════════"; }

# Source UX kit for enhanced ceremony display (optional)
UX_SOURCED=false
if [[ -f "$SCRIPT_DIR/ux-kit.sh" ]]; then
    source "$SCRIPT_DIR/ux-kit.sh"
    UX_SOURCED=true
elif [[ -f "$SCRIPT_DIR/misc/ux-kit.sh" ]]; then
    source "$SCRIPT_DIR/misc/ux-kit.sh"
    UX_SOURCED=true
fi

# Source UAT ceremonies if available
if [[ -f "$SCRIPT_DIR/misc/uat-ceremonies.sh" ]]; then
    source "$SCRIPT_DIR/misc/uat-ceremonies.sh"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# CEREMONY RUNNER CORE
# ═══════════════════════════════════════════════════════════════════════════════

# Configuration
CEREMONY_AUTO="${CEREMONY_AUTO:-false}"
CEREMONY_VERBOSE="${CEREMONY_VERBOSE:-false}"
CEREMONY_RESULTS_DIR="${CEREMONY_RESULTS_DIR:-/tmp/ceremony_results}"

# Ensure results directory exists
mkdir -p "$CEREMONY_RESULTS_DIR"

# Ceremony discovery and metadata
discover_ceremonies() {
    local pattern="${1:-ceremony_*.sh}"
    find "$CEREMONY_DIR" -name "$pattern" -type f | sort -V
}

discover_batches() {
    find "$CEREMONY_DIR" -name "batch_*" -type d | sort -V
}

# Ceremony execution core
execute_ceremony() {
    local ceremony_file="$1"
    local ceremony_name
    ceremony_name="$(basename "$ceremony_file" .sh)"
    
    if [[ ! -f "$ceremony_file" ]]; then
        ux_error "Ceremony file not found: $ceremony_file"
        return 1
    fi
    
    if [[ ! -x "$ceremony_file" ]]; then
        chmod +x "$ceremony_file"
    fi
    
    local start_time
    start_time="$(date +%s)"
    
    # Enhanced ceremony display if UAT functions available
    if command -v uat_suite_start &> /dev/null; then
        uat_suite_start "$ceremony_name" "Boxy API Ceremony" "$(date)"
    else
        ux_info "Starting ceremony: $ceremony_name"
        ux_separator
    fi
    
    # Execute ceremony with result capture
    local result_file="$CEREMONY_RESULTS_DIR/${ceremony_name}_$(date +%Y%m%d_%H%M%S).log"
    local exit_code=0
    
    if [[ "$CEREMONY_VERBOSE" == "true" ]]; then
        "$ceremony_file" 2>&1 | tee "$result_file"
        exit_code=${PIPESTATUS[0]}
    else
        "$ceremony_file" > "$result_file" 2>&1
        exit_code=$?
    fi
    
    local end_time
    end_time="$(date +%s)"
    local duration=$((end_time - start_time))
    
    # Results display
    if [[ $exit_code -eq 0 ]]; then
        if command -v uat_suite_end &> /dev/null; then
            uat_suite_end "$ceremony_name" "$(wc -l < "$result_file")" "${duration}s"
        else
            ux_success "Ceremony completed: $ceremony_name (${duration}s)"
        fi
    else
        ux_error "Ceremony failed: $ceremony_name (exit code: $exit_code)"
        if [[ "$CEREMONY_VERBOSE" != "true" ]]; then
            ux_warn "See results: $result_file"
        fi
    fi
    
    return $exit_code
}

# Batch execution
execute_batch() {
    local batch_name="$1"
    local batch_dir="$CEREMONY_DIR/$batch_name"
    
    # Handle both short names (batch_01) and full names (batch_01_foundation)
    if [[ ! -d "$batch_dir" ]]; then
        # Try to find directory with matching prefix
        local full_batch_dir=$(find "$CEREMONY_DIR" -maxdepth 1 -type d -name "${batch_name}*" | head -1)
        if [[ -n "$full_batch_dir" ]]; then
            batch_dir="$full_batch_dir"
        else
            ux_error "Batch directory not found: $batch_dir"
            return 1
        fi
    fi
    
    local ceremonies
    ceremonies=($(find "$batch_dir" -name "ceremony_*.sh" -type f | sort -V))
    
    if [[ ${#ceremonies[@]} -eq 0 ]]; then
        ux_warn "No ceremonies found in batch: $batch_name"
        return 0
    fi
    
    ux_info "Executing batch: $batch_name (${#ceremonies[@]} ceremonies)"
    
    local failed_count=0
    local total_count=${#ceremonies[@]}
    
    for ceremony in "${ceremonies[@]}"; do
        execute_ceremony "$ceremony" || ((failed_count++))
    done
    
    # Batch summary
    local success_count=$((total_count - failed_count))
    if [[ $failed_count -eq 0 ]]; then
        ux_success "Batch completed: $batch_name ($success_count/$total_count passed)"
    else
        ux_error "Batch completed: $batch_name ($success_count/$total_count passed, $failed_count failed)"
    fi
    
    return $([[ $failed_count -eq 0 ]] && echo 0 || echo 1)
}

# Range execution (ceremony_01-05)
execute_range() {
    local range_spec="$1"
    local start_num end_num
    
    if [[ "$range_spec" =~ ceremony_([0-9]+)-([0-9]+) ]]; then
        start_num="${BASH_REMATCH[1]}"
        end_num="${BASH_REMATCH[2]}"
    else
        ux_error "Invalid range format: $range_spec (use: ceremony_01-05)"
        return 1
    fi
    
    # Find matching ceremonies in range
    local ceremonies=()
    for ((i=start_num; i<=end_num; i++)); do
        local ceremony_pattern
        printf -v ceremony_pattern "ceremony_%02d_*.sh" "$i"
        local found_ceremonies
        found_ceremonies=($(find "$CEREMONY_DIR" -name "$ceremony_pattern" -type f))
        ceremonies+=("${found_ceremonies[@]}")
    done
    
    if [[ ${#ceremonies[@]} -eq 0 ]]; then
        ux_warn "No ceremonies found in range: $range_spec"
        return 0
    fi
    
    ux_info "Executing range: $range_spec (${#ceremonies[@]} ceremonies)"
    
    local failed_count=0
    for ceremony in "${ceremonies[@]}"; do
        execute_ceremony "$ceremony" || ((failed_count++))
    done
    
    return $([[ $failed_count -eq 0 ]] && echo 0 || echo 1)
}

# Listing functions
list_ceremonies() {
    echo "Available Ceremonies:"
    echo "═══════════════════════════════════════════════════════════════════════════════"
    
    local batches
    batches=($(discover_batches))
    
    for batch_dir in "${batches[@]}"; do
        local batch_name
        batch_name="$(basename "$batch_dir")"
        echo
        echo "📁 $batch_name"
        
        local ceremonies
        ceremonies=($(find "$batch_dir" -name "ceremony_*.sh" -type f | sort -V))
        
        for ceremony in "${ceremonies[@]}"; do
            local ceremony_name
            ceremony_name="$(basename "$ceremony" .sh)"
            local description="No description"
            
            # Extract description from ceremony file if available
            if grep -q "^# Description:" "$ceremony" 2>/dev/null; then
                description="$(grep "^# Description:" "$ceremony" | cut -d: -f2- | sed 's/^ *//')"
            fi
            
            printf "  %-25s - %s\n" "$ceremony_name" "$description"
        done
    done
    
    echo
    echo "Usage Examples:"
    echo "  ./ceremony_runner.sh batch_01_foundation    # Run foundation batch"
    echo "  ./ceremony_runner.sh ceremony_01            # Run specific ceremony"
    echo "  ./ceremony_runner.sh ceremony_01-05         # Run ceremony range"
}

# Documentation generation
generate_docs() {
    local docs_file="$CEREMONY_DIR/../docs/API_REFERENCE.md"
    
    cat > "$docs_file" << 'EOF'
# Boxy API Reference - Generated from Ceremonies

This document is automatically generated from working ceremony tests.

## API Coverage

EOF
    
    local batches
    batches=($(discover_batches))
    
    for batch_dir in "${batches[@]}"; do
        local batch_name
        batch_name="$(basename "$batch_dir")"
        echo "### $batch_name" >> "$docs_file"
        echo >> "$docs_file"
        
        local ceremonies
        ceremonies=($(find "$batch_dir" -name "ceremony_*.sh" -type f | sort -V))
        
        for ceremony in "${ceremonies[@]}"; do
            local ceremony_name
            ceremony_name="$(basename "$ceremony" .sh)"
            
            # Extract command examples from ceremony
            echo "#### $ceremony_name" >> "$docs_file"
            echo >> "$docs_file"
            echo '```bash' >> "$docs_file"
            grep -E "echo.*\|.*boxy|boxy.*--" "$ceremony" | head -3 >> "$docs_file" 2>/dev/null || echo "# No examples found" >> "$docs_file"
            echo '```' >> "$docs_file"
            echo >> "$docs_file"
        done
    done
    
    ux_success "API reference generated: $docs_file"
}

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN DISPATCH
# ═══════════════════════════════════════════════════════════════════════════════

show_usage() {
    cat << 'EOF'
Ceremony Runner - Boxy API Testing Orchestration

USAGE:
    ceremony_runner.sh <target>

TARGETS:
    batch_01               Run specific batch
    ceremony_05            Run specific ceremony  
    ceremony_01-05         Run ceremony range
    --list                 List all available ceremonies
    --generate-docs        Generate API reference from ceremonies

ENVIRONMENT:
    CEREMONY_AUTO=true     Auto-mode (no user interaction)
    CEREMONY_VERBOSE=true  Show ceremony output in real-time
    CEREMONY_RESULTS_DIR   Directory for result logs (default: /tmp/ceremony_results)

EXAMPLES:
    ./ceremony_runner.sh batch_01_foundation
    ./ceremony_runner.sh ceremony_01
    CEREMONY_VERBOSE=true ./ceremony_runner.sh batch_01_foundation

EOF
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
        --generate-docs|generate-docs)
            generate_docs
            ;;
        batch_*)
            execute_batch "$target"
            ;;
        ceremony_*-*)
            execute_range "$target"
            ;;
        ceremony_*)
            # Find ceremony by name pattern - support both exact match and prefix match
            local ceremony_file
            ceremony_file="$(find "$CEREMONY_DIR" -name "${target}.sh" -type f | head -1)"
            if [[ -z "$ceremony_file" ]]; then
                # Try prefix matching (ceremony_01 matches ceremony_01_basic_boxes.sh)
                ceremony_file="$(find "$CEREMONY_DIR" -name "${target}_*.sh" -type f | head -1)"
            fi
            if [[ -n "$ceremony_file" ]]; then
                execute_ceremony "$ceremony_file"
            else
                ux_error "Ceremony not found: $target"
                exit 1
            fi
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