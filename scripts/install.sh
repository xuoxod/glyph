#!/bin/sh
# ==============================================================================
# Sovereign Glyph Engine - Installer Script
# Installs `em` and `glyph` to ~/.local/bin
# ==============================================================================
set -eu

INSTALL_DIR="${HOME}/.local/bin"
mkdir -p "${INSTALL_DIR}"

SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

if command -v cargo >/dev/null 2>&1; then
    echo "📦 Building glyph (em) in release mode..."
    cargo build --release --manifest-path "${SCRIPT_DIR}/Cargo.toml"
    cp "${SCRIPT_DIR}/target/release/em" "${INSTALL_DIR}/em"
    cp "${SCRIPT_DIR}/target/release/glyph" "${INSTALL_DIR}/glyph"
    chmod +x "${INSTALL_DIR}/em" "${INSTALL_DIR}/glyph"
    echo "✓ Successfully installed 'em' and 'glyph' to ${INSTALL_DIR}"
else
    echo "❌ cargo is required to build from source."
    exit 1
fi

case ":${PATH}:" in
    *:"${INSTALL_DIR}":*) ;;
    *)
        echo ""
        echo "⚠️ Note: ${INSTALL_DIR} is not currently in your PATH."
        echo "  Add it to your shell profile (~/.bashrc or ~/.zshrc):"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
        ;;
esac

echo ""
echo "🎉 Ready! Try typing: em smile  or  em shield"
