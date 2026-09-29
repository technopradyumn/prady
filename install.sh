#!/usr/bin/env bash
# Prady Compiler Installer for Linux & macOS
# Usage: curl -fsSL https://raw.githubusercontent.com/technopradyumn/prady/main/install.sh | sh

set -e

PRADY_HOME="$HOME/.prady"
PRADY_BIN="$PRADY_HOME/bin"

echo "=========================================="
echo "  Installing Prady Toolchain (v1.0.0 GA)"
echo "=========================================="

mkdir -p "$PRADY_BIN"

# Check if local release build exists
if [ -f "./target/release/prady" ] && [ -f "./target/release/prady-lsp" ]; then
    echo "Installing from local release build..."
    cp ./target/release/prady "$PRADY_BIN/"
    cp ./target/release/prady-lsp "$PRADY_BIN/"
    chmod +x "$PRADY_BIN/prady" "$PRADY_BIN/prady-lsp"
else
    OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
    ARCH="$(uname -m)"
    
    TARGET=""
    if [ "$OS" = "linux" ]; then
        if [ "$ARCH" = "x86_64" ]; then
            TARGET="x86_64-unknown-linux-gnu"
        elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
            TARGET="aarch64-unknown-linux-gnu"
        fi
    elif [ "$OS" = "darwin" ]; then
        if [ "$ARCH" = "arm64" ]; then
            TARGET="aarch64-apple-darwin"
        else
            TARGET="x86_64-apple-darwin"
        fi
    fi

    if [ -z "$TARGET" ]; then
        echo "Unsupported OS/architecture: $OS $ARCH"
        exit 1
    fi

    URL="https://github.com/technopradyumn/prady/releases/download/v1.0.0/prady-v1.0.0-${TARGET}.tar.gz"
    echo "Downloading Prady package from GitHub..."
    curl -fsSL "$URL" | tar -xz -C "$PRADY_BIN"
    
    # Flatten if directory was packaged inside (handles prady-VERSION-TARGET/ subfolder)
    find "$PRADY_BIN" -name "prady" -not -path "$PRADY_BIN/prady" -type f -exec mv {} "$PRADY_BIN/" \; 2>/dev/null || true
    find "$PRADY_BIN" -name "prady-lsp" -not -path "$PRADY_BIN/prady-lsp" -type f -exec mv {} "$PRADY_BIN/" \; 2>/dev/null || true
    # Remove any leftover subdirectories
    find "$PRADY_BIN" -mindepth 1 -maxdepth 1 -type d -exec rm -rf {} \; 2>/dev/null || true
    chmod +x "$PRADY_BIN/prady"
    chmod +x "$PRADY_BIN/prady-lsp" 2>/dev/null || true
fi

# Configure PATH in shell config
SHELL_CONFIG=""
if [ -n "$ZSH_VERSION" ] || [ -f "$HOME/.zshrc" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [ -f "$HOME/.bashrc" ]; then
    SHELL_CONFIG="$HOME/.bashrc"
else
    SHELL_CONFIG="$HOME/.profile"
fi

if ! grep -q "$PRADY_BIN" "$SHELL_CONFIG" 2>/dev/null; then
    echo "Adding $PRADY_BIN to $SHELL_CONFIG..."
    echo "" >> "$SHELL_CONFIG"
    echo "# Prady programming language" >> "$SHELL_CONFIG"
    echo "export PATH=\"$PRADY_BIN:\$PATH\"" >> "$SHELL_CONFIG"
fi

# Try symlinking to /usr/local/bin so it works system-wide immediately without restarting shell
if [ -w "/usr/local/bin" ]; then
    ln -sf "$PRADY_BIN/prady" /usr/local/bin/prady
    ln -sf "$PRADY_BIN/prady-lsp" /usr/local/bin/prady-lsp
fi

echo ""
echo "=========================================="
echo "  Prady is installed system-wide!"
echo "=========================================="
echo "Prady CLI: $PRADY_BIN/prady"
echo "Prady LSP: $PRADY_BIN/prady-lsp"
echo ""
echo "Run 'source $SHELL_CONFIG' or open a new terminal window."
echo "Verify with:"
echo "    prady version"
echo "    prady run main.pr"
