#!/bin/bash
# Linea Quick Start Installation Script

set -e

echo "🚀 Linea Language Compiler Installation"
echo "========================================"

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed"
    echo "Install Rust with: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✓ Rust toolchain found: $(rustc --version)"

# Build the compiler
echo "📦 Building Linea compiler..."
cd "$(dirname "$0")/compiler"
cargo build --release

# Create symlink
BINARY="$(pwd)/target/release/linea-compiler"
INSTALL_PATH="/usr/local/bin/linea"

if command -v linea &> /dev/null && [ -L "$(command -v linea)" ]; then
    echo "Removing old Linea installation..."
    sudo rm -f "$INSTALL_PATH"
fi

echo "Installing Linea compiler to $INSTALL_PATH..."
sudo ln -sf "$BINARY" "$INSTALL_PATH"

echo ""
echo "✅ Installation complete!"
echo ""
echo "Quick test:"
echo '  echo "var x = 42" > hello.ln'
echo '  echo "display x" >> hello.ln'
echo "  linea compile hello.ln -o hello"
echo "  ./hello"
echo ""
