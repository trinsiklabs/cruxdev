#!/bin/bash
# CruxDev installer — detects OS/arch, downloads correct binary from GitHub Releases.
# Usage: curl -fsSL https://cruxdev.dev/install.sh | sh
set -e

REPO="trinsiklabs/cruxdev"
INSTALL_DIR="${CRUXDEV_INSTALL_DIR:-$HOME/.local/bin}"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  darwin) OS_TARGET="apple-darwin" ;;
  linux)  OS_TARGET="unknown-linux-gnu" ;;
  *)      echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
  x86_64|amd64)   ARCH_TARGET="x86_64" ;;
  arm64|aarch64)   ARCH_TARGET="aarch64" ;;
  *)               echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

TARGET="${ARCH_TARGET}-${OS_TARGET}"
BINARY_NAME="cruxdev-${TARGET}"

echo "Detected platform: ${TARGET}"
echo "Install directory: ${INSTALL_DIR}"

# Get latest release tag
LATEST=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')
if [ -z "$LATEST" ]; then
  echo "Error: Could not determine latest release."
  echo "Check: https://github.com/${REPO}/releases"
  exit 1
fi

echo "Latest release: ${LATEST}"

# Download binary
URL="https://github.com/${REPO}/releases/download/${LATEST}/${BINARY_NAME}"
echo "Downloading ${URL}..."
curl -fsSL -o /tmp/cruxdev "$URL"

# Download checksum
CHECKSUM_URL="${URL}.sha256"
if curl -fsSL -o /tmp/cruxdev.sha256 "$CHECKSUM_URL" 2>/dev/null; then
  echo "Verifying checksum..."
  EXPECTED=$(cat /tmp/cruxdev.sha256 | awk '{print $1}')
  ACTUAL=$(shasum -a 256 /tmp/cruxdev | awk '{print $1}')
  if [ "$EXPECTED" != "$ACTUAL" ]; then
    echo "Checksum verification FAILED!"
    echo "Expected: $EXPECTED"
    echo "Actual:   $ACTUAL"
    rm -f /tmp/cruxdev /tmp/cruxdev.sha256
    exit 1
  fi
  echo "Checksum verified."
  rm -f /tmp/cruxdev.sha256
fi

# Install
mkdir -p "$INSTALL_DIR"
mv /tmp/cruxdev "$INSTALL_DIR/cruxdev"
chmod +x "$INSTALL_DIR/cruxdev"

echo ""
echo "CruxDev ${LATEST} installed to ${INSTALL_DIR}/cruxdev"

# Check PATH
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  echo ""
  echo "Add to your PATH:"
  echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
fi

# Verify
if command -v cruxdev >/dev/null 2>&1 || [ -x "$INSTALL_DIR/cruxdev" ]; then
  echo ""
  "$INSTALL_DIR/cruxdev" status . 2>/dev/null || echo "Run 'cruxdev status' to verify."
fi
