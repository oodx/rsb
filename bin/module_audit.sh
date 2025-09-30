#!/bin/bash
echo "=== MODULE_SPEC v3 COMPLIANCE AUDIT ==="
echo
echo "## SPEC VIOLATION: Root-level .rs files (should be in modules)"
echo
for file in src/*.rs; do
  name=$(basename "$file")
  # Allowed files per MODULE_SPEC
  case "$name" in
    lib.rs|prelude.rs|deps.rs|lang.rs) 
      echo "✅ ALLOWED: $name"
      ;;
    *)
      lines=$(wc -l < "$file")
      echo "❌ LEGACY: $name ($lines lines)"
      ;;
  esac
done

echo
echo "## Module directories"
ls -1 src/ | grep -v "\.rs$" | while read dir; do
  if [ -d "src/$dir" ]; then
    has_mod=$([ -f "src/$dir/mod.rs" ] && echo "✅" || echo "❌")
    echo "$has_mod src/$dir/"
  fi
done
