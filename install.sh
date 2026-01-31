#!/bin/bash

# NusaCloud CLI Build and Install Script

set -e

echo "🚀 Building localstacker CLI..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed."
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Build the project
echo "🔨 Compiling (this may take a few minutes)..."
cargo build --release

# Check if build was successful
if [ ! -f "target/release/localstacker" ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"
echo ""

# Install binary
if [ "$EUID" -eq 0 ]; then
    echo "📦 Installing to /usr/local/bin/..."
    cp target/release/localstacker /usr/local/bin/
    chmod +x /usr/local/bin/localstacker
    echo "✅ Installed successfully!"
else
    echo "⚠️  Not running as root."
    echo "   To install system-wide, run:"
    echo "   sudo cp target/release/localstacker /usr/local/bin/"
    echo ""
    echo "   Or install for current user:"
    echo "   mkdir -p ~/.local/bin"
    echo "   cp target/release/localstacker ~/.local/bin/"
    echo "   export PATH=\"\$HOME/.local/bin:\$PATH\"  # Add to ~/.bashrc or ~/.zshrc"
fi

echo ""
echo "🎉 Done! Try running:"
echo "   localstacker --help"
echo ""
