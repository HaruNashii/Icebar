#!/usr/bin/env bash
set -euo pipefail

BINARY_NAME="icebar"
SOURCE_BIN="target/release/${BINARY_NAME}"
DEST_DIR="${HOME}/.local/bin"
DEST_BIN="${DEST_DIR}/${BINARY_NAME}"

# Verify the binary exists
if [[ ! -f "${SOURCE_BIN}" ]]; then
    echo "Error: ${SOURCE_BIN} does not exist."
    echo "Build it first with: cargo build --release"
    exit 1
fi

# Create ~/.local/bin if needed
mkdir -p "${DEST_DIR}"

# Install/update the binary
install -m 755 "${SOURCE_BIN}" "${DEST_BIN}"

# Ensure ~/.local/bin is in PATH
if [[ ":$PATH:" != *":${DEST_DIR}:"* ]]; then
    SHELL_RC=""

    if [[ -n "${ZSH_VERSION:-}" ]]; then
        SHELL_RC="${HOME}/.zshrc"
    elif [[ -n "${BASH_VERSION:-}" ]]; then
        SHELL_RC="${HOME}/.bashrc"
    elif [[ -f "${HOME}/.zshrc" ]]; then
        SHELL_RC="${HOME}/.zshrc"
    else
        SHELL_RC="${HOME}/.bashrc"
    fi

    if ! grep -Fq 'export PATH="$HOME/.local/bin:$PATH"' "${SHELL_RC}" 2>/dev/null; then
        echo '' >> "${SHELL_RC}"
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "${SHELL_RC}"
        echo "Added ~/.local/bin to PATH in ${SHELL_RC}"
    fi

    export PATH="${DEST_DIR}:${PATH}"
fi

echo "Installed ${BINARY_NAME} to ${DEST_BIN}"
echo "Current binary: $(command -v ${BINARY_NAME})"
