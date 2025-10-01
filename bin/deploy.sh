#!/bin/bash
set -e

# Configuration
LIB_DIR="$HOME/.local/lib/odx/rsbdoc"
BIN_DIR="$HOME/.local/bin/odx"
BINARY_NAME="rsbdoc"

lib_file="$LIB_DIR/$BINARY_NAME"
bin_file="$BIN_DIR/$BINARY_NAME"

# Resolve repository root from bin/
ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
DEPLOYABLE="${BINARY_NAME}"

# Extract version from Cargo.toml at repo root
VERSION=$(grep '^version' "$ROOT_DIR/Cargo.toml" | head -1 | cut -d'"' -f2)

# Display deployment ceremony
echo "╔════════════════════════════════════════════════╗"
echo "║           RSBDOC DEPLOYMENT CEREMONY           ║"
echo "╠════════════════════════════════════════════════╣"
echo "║ Package: $BINARY_NAME                          ║"
echo "║ Version: v$VERSION (Documentation CLI)         ║"
echo "║ Target:  $bin_file             ║"
echo "╚════════════════════════════════════════════════╝"
echo ""

echo "🔨 Building rsbdoc v$VERSION..."
cd "$ROOT_DIR"
if ! cargo build --release --bin rsbdoc; then
    echo "❌ Build failed!"
    exit 1
fi

# Check if binary was created (at repo root)
if [ ! -f "$ROOT_DIR/target/release/${DEPLOYABLE}" ]; then
    echo "❌ Binary not found at target/release/${DEPLOYABLE}"
    exit 1
fi

echo "📦 Deploying rsbdoc to $LIB_DIR..."
mkdir -p "$BIN_DIR" "$LIB_DIR"

if [ -f "$lib_file" ]; then
	echo "📦 Removing previous rsbdoc installation"
	rm -f "$lib_file"
	rm -f "$bin_file"
fi

if ! cp "$ROOT_DIR/target/release/${DEPLOYABLE}" "$lib_file"; then
    echo "❌ Failed to copy binary to $lib_file"
    exit 1
fi

if ! chmod +x "$lib_file"; then
	echo "❌ Failed to make binary executable"
	exit 1
fi

if ! ln -s "$lib_file" "$bin_file"; then
	echo "❌ Failed to create symlink from lib to bin"
	exit 1
fi

# Verify deployment
if [ ! -x "$bin_file" ]; then
    echo "❌ Binary is not executable at $bin_file"
    exit 1
fi

# Test the binary
echo "🧪 Testing binary..."
if ! "$bin_file" list > /dev/null 2>&1; then
    echo "❌ Binary test failed!"
    exit 1
fi

# Set up environment hints
echo "🔧 Environment setup..."
BRAIN_HOME="${BRAIN_HOME:-$HOME/repos/docs/brain}"
RSB_HOME="${RSB_HOME:-$ROOT_DIR}"

if [ ! -d "$BRAIN_HOME/dev" ]; then
    echo "⚠️  BRAIN_HOME not found at $BRAIN_HOME"
    echo "   Set BRAIN_HOME environment variable to your docs location"
else
    echo "✅ BRAIN_HOME detected at $BRAIN_HOME"
fi

if [ ! -d "$RSB_HOME/docs/tech/features" ]; then
    echo "⚠️  RSB features not found at $RSB_HOME"
else
    echo "✅ RSB_HOME detected at $RSB_HOME"
fi

# Create environment config helper
CONFIG_DIR="$HOME/.config/rsbdoc"
CONFIG_FILE="$CONFIG_DIR/rsbdoc.env"
mkdir -p "$CONFIG_DIR"

cat > "$CONFIG_FILE" << EOF
# rsbdoc environment configuration
# Source this file or add these exports to your shell config

export BRAIN_HOME="${BRAIN_HOME:-$HOME/repos/docs/brain}"
export RSB_HOME="$ROOT_DIR"
EOF

echo ""
echo "╔════════════════════════════════════════════════╗"
echo "║          DEPLOYMENT SUCCESSFUL!                ║"
echo "╚════════════════════════════════════════════════╝"
echo "  Deployed: rsbdoc v$VERSION"
echo "  Location: $bin_file"
echo "  Config:   $CONFIG_FILE"
echo ""
echo "📖 Quick start:"
echo "   source $CONFIG_FILE        # Load environment"
echo "   rsbdoc list                # List all topics"
echo "   rsbdoc feat list           # List RSB features"
echo "   rsbdoc feat bash           # View FEATURES_BASH.md"
echo ""
echo "🔧 To make permanent, add to your ~/.bashrc or ~/.zshrc:"
echo "   source $CONFIG_FILE"
echo ""
echo "   Or manually add:"
echo "   export BRAIN_HOME=\"$BRAIN_HOME\""
echo "   export RSB_HOME=\"$ROOT_DIR\""
