#!/bin/bash
# ╔══════════════════════════════════════════════════════════════════╗
# ║              ForgeLang Neovim Setup Script                        ║
# ║                  Run this to install Tree-sitter grammar          ║
# ╚══════════════════════════════════════════════════════════════════╝

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GRAMMAR_DIR="$SCRIPT_DIR/tree-sitter-forgelang"
NVIM_DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/nvim"
PARSER_DIR="$NVIM_DATA_DIR/lazy/nvim-treesitter/parser"

echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║           ForgeLang Neovim Tree-sitter Setup                     ║"
echo "╚══════════════════════════════════════════════════════════════════╝"
echo

# Check for required tools
echo "🔍 Checking dependencies..."

if ! command -v node &> /dev/null; then
    echo "❌ Node.js is required but not installed."
    echo "   Install from: https://nodejs.org/"
    exit 1
fi
echo "   ✓ Node.js: $(node --version)"

if ! command -v npm &> /dev/null; then
    echo "❌ npm is required but not installed."
    exit 1
fi
echo "   ✓ npm: $(npm --version)"

if ! command -v tree-sitter &> /dev/null; then
    echo "   ⚠ Installing tree-sitter CLI..."
    npm install -g tree-sitter-cli
fi
echo "   ✓ tree-sitter: $(tree-sitter --version)"

echo

# Install grammar dependencies
echo "📦 Installing grammar dependencies..."
cd "$GRAMMAR_DIR"
npm install
echo "   ✓ Dependencies installed"
echo

# Generate parser
echo "🔨 Generating parser..."
tree-sitter generate
echo "   ✓ Parser generated"
echo

# Build parser
echo "🏗 Building parser..."
tree-sitter build
echo "   ✓ Parser built"
echo

# Create parser directory if it doesn't exist
echo "📁 Setting up Neovim parser directory..."
mkdir -p "$PARSER_DIR"
echo "   ✓ Directory: $PARSER_DIR"
echo

# Copy parser library
echo "📋 Copying parser library..."
if [[ -f "$GRAMMAR_DIR/build/Release/tree-sitter_forgelang.so" ]]; then
    cp "$GRAMMAR_DIR/build/Release/tree-sitter_forgelang.so" "$PARSER_DIR/forgelang.so"
    echo "   ✓ Copied to: $PARSER_DIR/forgelang.so"
elif [[ -f "$GRAMMAR_DIR/build/tree-sitter-forgelang.wasm" ]]; then
    cp "$GRAMMAR_DIR/build/tree-sitter-forgelang.wasm" "$PARSER_DIR/forgelang.wasm"
    echo "   ✓ Copied WASM to: $PARSER_DIR/forgelang.wasm"
else
    echo "   ⚠ No parser library found, trying alternative locations..."
    find "$GRAMMAR_DIR" -name "*.so" -o -name "*.wasm" 2>/dev/null | head -1 | while read lib; do
        cp "$lib" "$PARSER_DIR/forgelang.${lib##*.}"
        echo "   ✓ Copied to: $PARSER_DIR/forgelang.${lib##*.}"
    done
fi
echo

# Copy queries
echo "📋 Copying highlight queries..."
mkdir -p "$NVIM_DATA_DIR/lazy/nvim-treesitter/queries/forgelang"
cp "$GRAMMAR_DIR/queries/"*.scm "$NVIM_DATA_DIR/lazy/nvim-treesitter/queries/forgelang/" 2>/dev/null || true
echo "   ✓ Queries copied"
echo

# Add to nvim-treesitter config
echo "⚙ Updating nvim-treesitter configuration..."
NVIM_TREESITTER_CONFIG="$HOME/.config/nvim/lua/plugins/treesitter.lua"
if [[ -f "$NVIM_TREESITTER_CONFIG" ]]; then
    if ! grep -q "forgelang" "$NVIM_TREESITTER_CONFIG"; then
        sed -i "/ensure_installed = {/a\\    'forgelang'," "$NVIM_TREESITTER_CONFIG"
        echo "   ✓ Added 'forgelang' to ensure_installed"
    else
        echo "   ✓ 'forgelang' already in config"
    fi
else
    echo "   ⚠ nvim-treesitter config not found at: $NVIM_TREESITTER_CONFIG"
    echo "   Please manually add 'forgelang' to ensure_installed in your treesitter config"
fi
echo

# Create filetype detection
echo "📝 Setting up filetype detection..."
NVIM_FILETYPE="$HOME/.config/nvim/after/ftdetect/forgelang.lua"
mkdir -p "$(dirname "$NVIM_FILETYPE")"
cat > "$NVIM_FILETYPE" << 'EOF'
vim.filetype.add({
  extension = { fl = 'forgelang' },
  pattern = { ['.*%.fl$'] = 'forgelang' },
})
EOF
echo "   ✓ Created: $NVIM_FILETYPE"
echo

# Summary
echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║                      Setup Complete!                             ║"
echo "╚══════════════════════════════════════════════════════════════════╝"
echo
echo "Next steps:"
echo "  1. Restart Neovim"
echo "  2. Open a .fl file"
echo "  3. Run :checkhealth nvim-treesitter to verify"
echo
echo "If highlighting doesn't work, run in Neovim:"
echo "  :TSUpdate forgelang"
echo
echo "Enjoy coding in ForgeLang! 🦀"
