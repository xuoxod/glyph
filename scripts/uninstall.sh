#!/bin/sh
# ==============================================================================
# Sovereign Glyph Engine - Uninstaller Script
# Removes `em` and `glyph` from ~/.local/bin
# ==============================================================================
set -eu

INSTALL_DIR="${HOME}/.local/bin"

echo "🧹 Removing glyph binaries from ${INSTALL_DIR}..."
rm -f "${INSTALL_DIR}/em" "${INSTALL_DIR}/glyph"

echo "✓ Uninstalled successfully."
