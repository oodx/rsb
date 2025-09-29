#!/bin/bash
# Production Build Script for Boxy
# Builds with maximum optimization and minimal debug strings

set -e

echo "🔨 Building Boxy for PRODUCTION deployment..."
echo

# Clean previous builds
echo "1. Cleaning previous builds..."
cargo clean --release

# Build with production profile (if defined) or release
echo "2. Building with production optimizations..."
if grep -q "\[profile.production\]" Cargo.toml; then
    echo "   Using production profile..."
    cargo build --profile production
    BINARY="target/production/boxy"
else
    echo "   Using release profile..."
    cargo build --release
    BINARY="target/release/boxy"
fi

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo "❌ Build failed - binary not found!"
    exit 1
fi

echo "3. Original binary size: $(ls -lh "$BINARY" | awk '{print $5}')"
echo

# Optional: Additional stripping with GNU strip
echo "4. Applying additional binary stripping..."
cp "$BINARY" "${BINARY}.original"
strip --strip-all "$BINARY" 2>/dev/null || strip "$BINARY"
echo "   After strip: $(ls -lh "$BINARY" | awk '{print $5}')"
echo

# Analyze string leakage
echo "5. Security analysis..."
LEAKED_PATHS=$(strings "$BINARY" | grep -c "^/home/" || true)
TOTAL_STRINGS=$(strings "$BINARY" | wc -l)
DEBUG_STRINGS=$(strings "$BINARY" | grep -c "^/rust\|^src/" || true)

echo "   Total strings in binary: $TOTAL_STRINGS"
echo "   Debug/file path strings: $DEBUG_STRINGS"
echo "   Leaked personal paths: $LEAKED_PATHS"

if [ "$LEAKED_PATHS" -gt 0 ]; then
    echo "   ⚠️  WARNING: Personal paths still present!"
    strings "$BINARY" | grep "^/home/" | head -5
else
    echo "   ✅ No personal paths leaked"
fi
echo

# Optional: UPX compression (if available)
if command -v upx &> /dev/null; then
    echo "6. Compressing with UPX..."
    cp "$BINARY" "${BINARY}.uncompressed"
    upx --best --lzma "$BINARY" 2>&1 | grep -E "compressed|ratio"
    echo "   Compressed: $(ls -lh "$BINARY" | awk '{print $5}')"
    echo
else
    echo "6. UPX not available (skip compression)"
    echo "   Install with: sudo apt install upx"
    echo
fi

# Show final stats
echo "✅ Production build complete!"
echo
echo "Binary location: $BINARY"
echo "Final size: $(ls -lh "$BINARY" | awk '{print $5}')"
echo
echo "To install:"
echo "  cp $BINARY ~/.local/lib/odx/boxylib/boxy"
echo
echo "To test string leakage:"
echo "  strings $BINARY | less"