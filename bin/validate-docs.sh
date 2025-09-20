#!/bin/bash
#
# RSB Documentation Validation Script
# Meta Process v2 - Self-Hydrating Workflow System
#
# Purpose: Silent success, noisy failure validation
# - File structure integrity (all required docs exist)
# - Internal reference validation (no broken links)
# - Staleness detection (critical docs >1 week, others >1 month)
# - Clean output (only show problems)
#

set -euo pipefail

# Colors for output
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Counters
ERROR_COUNT=0
WARNING_COUNT=0

# Helper functions
error() {
    echo -e "${RED}❌ ERROR: $1${NC}" >&2
    ((ERROR_COUNT++))
}

warning() {
    echo -e "${YELLOW}⚠️  WARNING: $1${NC}" >&2
    ((WARNING_COUNT++))
}

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

# Check if file exists and is not empty
check_file_exists() {
    local file="$1"
    local description="$2"

    if [[ ! -f "$file" ]]; then
        error "Missing required file: $file ($description)"
        return 1
    fi

    if [[ ! -s "$file" ]]; then
        error "Empty file: $file ($description)"
        return 1
    fi

    return 0
}

# Check file staleness
check_staleness() {
    local file="$1"
    local max_days="$2"
    local description="$3"

    if [[ ! -f "$file" ]]; then
        return 0  # Already checked in check_file_exists
    fi

    local file_age_days
    file_age_days=$(( ($(date +%s) - $(stat -c %Y "$file")) / 86400 ))

    if [[ $file_age_days -gt $max_days ]]; then
        warning "$file is stale ($file_age_days days old, max $max_days days) - $description"
    fi
}

# Check for broken internal links
check_internal_links() {
    local file="$1"

    if [[ ! -f "$file" ]]; then
        return 0
    fi

    # Extract markdown links and file references
    grep -n '\[.*\](\..*\.md\|\..*\.txt\|docs/\|bin/\|\./\)' "$file" 2>/dev/null | while read -r line; do
        local line_num=$(echo "$line" | cut -d: -f1)
        local link=$(echo "$line" | sed -n 's/.*\[\([^]]*\)\](\([^)]*\)).*/\2/p')

        # Skip external links (http/https)
        if [[ "$link" =~ ^https?:// ]]; then
            continue
        fi

        # Convert relative paths to absolute
        local target_file
        if [[ "$link" =~ ^\.\/ ]]; then
            target_file="${link#./}"
        elif [[ "$link" =~ ^docs/ ]] || [[ "$link" =~ ^bin/ ]]; then
            target_file="$link"
        else
            target_file="$link"
        fi

        # Check if target exists
        if [[ -n "$target_file" ]] && [[ ! -f "$target_file" ]] && [[ ! -d "$target_file" ]]; then
            error "Broken link in $file:$line_num -> $target_file"
        fi
    done
}

# Main validation function
main() {
    echo "🔍 RSB Documentation Validation (Meta Process v2)"
    echo "=================================================="
    echo

    # Phase 1: Check required Meta Process v2 files
    echo "📋 Phase 1: Core Meta Process Files"

    # Essential entry points
    check_file_exists "START.txt" "Single entry point"
    check_file_exists "README.md" "Project description"
    check_file_exists "LICENSE" "License file"

    # Process documents directory
    if [[ ! -d "docs/procs" ]]; then
        error "Missing required directory: docs/procs"
    else
        check_file_exists "docs/procs/PROCESS.txt" "Master workflow guide"
        check_file_exists "docs/procs/CONTINUE.md" "Session status"
        check_file_exists "docs/procs/QUICK_REF.txt" "30-second context"
        check_file_exists "docs/procs/SPRINT.txt" "Current tasks"
        check_file_exists "docs/procs/ROADMAP.txt" "Strategic overview"
        check_file_exists "docs/procs/TASKS.txt" "Task breakdown"
    fi

    # Technical documentation (preserve existing structure)
    if [[ ! -d "docs/tech" ]]; then
        error "Missing required directory: docs/tech"
    else
        check_file_exists "docs/tech/INDEX.md" "Technical documentation index"
    fi

    # Validation infrastructure
    check_file_exists "bin/validate-docs.sh" "Documentation validator (this script)"
    check_file_exists "bin/test.sh" "RSB test runner"

    echo
    echo "📅 Phase 2: Staleness Detection"

    # Critical files (max 1 week)
    check_staleness "docs/procs/CONTINUE.md" 7 "Session status must be current"
    check_staleness "docs/procs/SPRINT.txt" 7 "Sprint tasks must be current"
    check_staleness "docs/procs/QUICK_REF.txt" 14 "Quick reference should be recent"

    # Important files (max 1 month)
    check_staleness "docs/procs/PROCESS.txt" 30 "Process guide should be current"
    check_staleness "docs/procs/ROADMAP.txt" 30 "Roadmap should be reviewed monthly"
    check_staleness "START.txt" 30 "Entry point should be current"

    echo
    echo "🔗 Phase 3: Internal Link Validation"

    # Check key files for broken links
    for file in START.txt docs/procs/*.md docs/procs/*.txt docs/tech/INDEX.md; do
        if [[ -f "$file" ]]; then
            check_internal_links "$file"
        fi
    done

    echo
    echo "📊 Phase 4: Structure Validation"

    # Check directory structure compliance
    required_dirs=("docs/procs" "docs/tech" "docs/misc" "bin" ".analysis")
    for dir in "${required_dirs[@]}"; do
        if [[ ! -d "$dir" ]]; then
            error "Missing required directory: $dir"
        fi
    done

    # Check for scattered root files (should be minimal)
    root_files_count=$(find . -maxdepth 1 -type f -name "*.md" -o -name "*.txt" | grep -v -E "(README.md|START.txt|LICENSE|CHANGELOG.md|CONTINUE.md|META_PROCESS.txt)" | wc -l)
    if [[ $root_files_count -gt 0 ]]; then
        warning "Found $root_files_count potentially misplaced files in root directory"
        find . -maxdepth 1 -type f -name "*.md" -o -name "*.txt" | grep -v -E "(README.md|START.txt|LICENSE|CHANGELOG.md|CONTINUE.md|META_PROCESS.txt)" | while read -r file; do
            warning "Consider moving: $file"
        done
    fi

    echo
    echo "📈 Phase 5: Content Quality Checks"

    # Check for empty or minimal files
    for file in docs/procs/*.md docs/procs/*.txt; do
        if [[ -f "$file" ]] && [[ $(wc -l < "$file") -lt 5 ]]; then
            warning "Potentially incomplete file: $file (less than 5 lines)"
        fi
    done

    # Check for TODO markers in process docs
    if grep -r "TODO\|FIXME\|XXX" docs/procs/ >/dev/null 2>&1; then
        warning "Found TODO/FIXME markers in process documentation:"
        grep -rn "TODO\|FIXME\|XXX" docs/procs/ | head -5
    fi

    echo
    echo "==============================================="

    # Final report
    if [[ $ERROR_COUNT -eq 0 ]] && [[ $WARNING_COUNT -eq 0 ]]; then
        success "All documentation validation checks passed! 🎉"
        success "Meta Process v2 system is healthy and ready for use."
        exit 0
    elif [[ $ERROR_COUNT -eq 0 ]]; then
        success "No critical errors found."
        echo -e "${YELLOW}⚠️  Found $WARNING_COUNT warnings that should be addressed.${NC}"
        exit 0
    else
        echo -e "${RED}❌ Found $ERROR_COUNT critical errors and $WARNING_COUNT warnings.${NC}" >&2
        echo -e "${RED}Meta Process v2 system requires fixes before use.${NC}" >&2
        exit 1
    fi
}

# Run main function
main "$@"