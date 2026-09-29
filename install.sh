#!/usr/bin/env sh
# Install the latest Prady CLI and language server for the current user.
# Usage: curl -fsSL https://raw.githubusercontent.com/technopradyumn/prady/main/install.sh | sh

set -eu

PRADY_HOME="${HOME:?HOME must be set}/.prady"
PRADY_BIN="$PRADY_HOME/bin"
TMP_DIR=""

cleanup() {
    if [ -n "$TMP_DIR" ] && [ -d "$TMP_DIR" ]; then
        rm -rf "$TMP_DIR"
    fi
}
trap cleanup EXIT

mkdir -p "$PRADY_BIN"

# When run from a built source checkout, use its binaries instead of downloading.
if [ -f "compiler/prady-cli/Cargo.toml" ] &&
   [ -f "target/release/prady" ] &&
   [ -f "target/release/prady-lsp" ]; then
    echo "Installing Prady from the local release build..."
    cp "target/release/prady" "$PRADY_BIN/prady"
    cp "target/release/prady-lsp" "$PRADY_BIN/prady-lsp"
else
    OS="$(uname -s)"
    ARCH="$(uname -m)"
    case "${OS}-${ARCH}" in
        Linux-x86_64)   TARGET="x86_64-unknown-linux-gnu" ;;
        Linux-aarch64|Linux-arm64) TARGET="aarch64-unknown-linux-gnu" ;;
        Darwin-x86_64)  TARGET="x86_64-apple-darwin" ;;
        Darwin-arm64|Darwin-aarch64) TARGET="aarch64-apple-darwin" ;;
        *)
            echo "Unsupported OS/architecture: ${OS} ${ARCH}" >&2
            exit 1
            ;;
    esac

    VERSION="$(curl -fsSL --retry 3 \
        -H "Accept: application/vnd.github+json" \
        -H "User-Agent: Prady-Installer" \
        "https://api.github.com/repos/technopradyumn/prady/releases/latest" |
        sed -n 's/.*"tag_name":[[:space:]]*"\([^"]*\)".*/\1/p' | head -n 1)"
    if [ -z "$VERSION" ]; then
        echo "Could not determine the latest Prady release from GitHub." >&2
        exit 1
    fi

    ARCHIVE="prady-${VERSION}-${TARGET}.tar.gz"
    TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/prady-install.XXXXXX")"
    echo "Downloading Prady ${VERSION} for ${TARGET}..."
    curl -fsSL --retry 3 \
        "https://github.com/technopradyumn/prady/releases/download/${VERSION}/${ARCHIVE}" \
        -o "$TMP_DIR/$ARCHIVE"
    tar -xzf "$TMP_DIR/$ARCHIVE" -C "$TMP_DIR"

    CLI="$(find "$TMP_DIR" -type f -name prady -print -quit)"
    LSP="$(find "$TMP_DIR" -type f -name prady-lsp -print -quit)"
    if [ -z "$CLI" ] || [ -z "$LSP" ]; then
        echo "The Prady release archive is missing prady or prady-lsp." >&2
        exit 1
    fi
    cp "$CLI" "$PRADY_BIN/prady"
    cp "$LSP" "$PRADY_BIN/prady-lsp"
fi

chmod +x "$PRADY_BIN/prady" "$PRADY_BIN/prady-lsp"

SHELL_CONFIG=""
if [ -n "${ZSH_VERSION:-}" ] || [ -f "$HOME/.zshrc" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [ -f "$HOME/.bashrc" ]; then
    SHELL_CONFIG="$HOME/.bashrc"
else
    SHELL_CONFIG="$HOME/.profile"
fi

if ! grep -Fq "$PRADY_BIN" "$SHELL_CONFIG" 2>/dev/null; then
    printf '\n# Prady programming language\nexport PATH="%s:$PATH"\n' "$PRADY_BIN" >> "$SHELL_CONFIG"
fi

export PATH="$PRADY_BIN:$PATH"
"$PRADY_BIN/prady" version
echo ""
echo "Prady CLI and language server installed to $PRADY_BIN"
echo "Open a new terminal or run: . \"$SHELL_CONFIG\""
echo "Then try: prady run path/to/hello.pr"
