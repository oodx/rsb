#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
now_utc() { date -u +%Y-%m-%dT%H:%M:%SZ; }

cmd=${1:-}
case "$cmd" in
  bump)
    ts="$(now_utc)"
    # Update Date: lines in common session files
    for f in \
      "$root_dir/.session/SESSION_CURRENT.md" \
      "$root_dir/.session/TASKS.txt" \
      "$root_dir"/.session/SESSION_*.md; do
      [ -f "$f" ] || continue
      # only touch files that contain a Date: header
      if grep -q '^Date:' "$f"; then
        sed -i -E "s/^Date: .*/Date: ${ts//\//\/}/" "$f"
        echo "Updated date: $f -> $ts"
      fi
    done
    ;;
  *)
    cat <<USAGE
session.sh — session utilities

Usage:
  $0 bump     # Update 'Date:' headers to current UTC ISO8601 in .session files
USAGE
    ;;
esac
