#!/usr/bin/env bash
set -euo pipefail

# Config: set these to your repo
GITHUB_OWNER="olashina201"
GITHUB_REPO="php-vm"

BIN_NAME="phpvm"
INSTALL_ROOT="${HOME}/.phpvm"
INSTALL_BIN_DIR="${INSTALL_ROOT}/bin"

usage() {
  cat <<EOF
${BIN_NAME} installer

Environment overrides:
  PHPVM_VERSION   Install this version (e.g. v0.1.0). Defaults to latest release
  PHPVM_BIN_DIR   Install directory for the binary (default: ${INSTALL_BIN_DIR})
  PHPVM_OWNER     GitHub owner/org (default: ${GITHUB_OWNER})
  PHPVM_REPO      GitHub repo name (default: ${GITHUB_REPO})
EOF
}

command -v curl >/dev/null 2>&1 || command -v wget >/dev/null 2>&1 || {
  echo "Error: curl or wget is required" >&2
  exit 1
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

GITHUB_OWNER=${PHPVM_OWNER:-$GITHUB_OWNER}
GITHUB_REPO=${PHPVM_REPO:-$GITHUB_REPO}
INSTALL_BIN_DIR=${PHPVM_BIN_DIR:-$INSTALL_BIN_DIR}

os="$(uname -s | tr '[:upper:]' '[:lower:]')"
arch="$(uname -m)"

case "$os" in
  linux) out_os="linux" ;;
  darwin) out_os="darwin" ;;
  msys*|mingw*|cygwin*) echo "Windows not supported by this script. Use PowerShell installer." >&2; exit 1 ;;
  *) echo "Unsupported OS: $os" >&2; exit 1 ;;
esac

case "$arch" in
  x86_64|amd64) out_arch="amd64" ;;
  aarch64|arm64) out_arch="arm64" ;;
  *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;
esac

api_base="https://api.github.com/repos/${GITHUB_OWNER}/${GITHUB_REPO}"

fetch() {
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$1"
  else
    wget -qO- "$1"
  fi
}

if [[ -n "${PHPVM_VERSION:-}" ]]; then
  tag="$PHPVM_VERSION"
else
  echo "Resolving latest release…"
  tag="$(fetch "${api_base}/releases/latest" | grep '"tag_name"' | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')"
  if [[ -z "$tag" ]]; then
    echo "Failed to determine latest release tag" >&2
    exit 1
  fi
fi

asset_prefix="${BIN_NAME}-${out_os}-${out_arch}"
tar_name="${asset_prefix}.tar.gz"
zip_name="${asset_prefix}.zip"

echo "Fetching release assets for ${tag}…"
release_json="$(fetch "${api_base}/releases/tags/${tag}")"

asset_url="$(printf "%s" "$release_json" | grep -A 1 "${tar_name}" | grep "browser_download_url" | sed 's/.*"browser_download_url": *"\([^"]*\)".*/\1/')"
if [[ -z "$asset_url" ]]; then
  # try zip (unlikely on unix, but fallback)
  asset_url="$(printf "%s" "$release_json" | grep -A 1 "${zip_name}" | grep "browser_download_url" | sed 's/.*"browser_download_url": *"\([^"]*\)".*/\1/')"
fi

if [[ -z "$asset_url" ]]; then
  echo "Could not find matching asset for ${out_os}/${out_arch} in ${tag}" >&2
  exit 1
fi

checksum_url="$(printf "%s" "$release_json" | sed -n 's|.*"browser_download_url"\s*:\s*"\(.*SHA256SUMS\)".*|\1|p' | head -n1)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

echo "Downloading: $asset_url"
fetch "$asset_url" >"${tmpdir}/asset"

if [[ -n "$checksum_url" ]]; then
  echo "Verifying checksum…"
  fetch "$checksum_url" >"${tmpdir}/SHA256SUMS"
  if command -v sha256sum >/dev/null 2>&1; then
    (cd "$tmpdir" && sha256sum -c --ignore-missing SHA256SUMS)
  else
    (cd "$tmpdir" && shasum -a 256 -c --ignore-missing SHA256SUMS)
  fi
else
  echo "Warning: SHA256SUMS not found. Skipping verification." >&2
fi

mkdir -p "$INSTALL_BIN_DIR"

if [[ "$asset_url" == *.tar.gz ]]; then
  tar -C "$tmpdir" -xzf "${tmpdir}/asset"
  src_bin="$(find "$tmpdir" -type f -name "${BIN_NAME}" | head -n1)"
elif [[ "$asset_url" == *.zip ]]; then
  unzip -q "${tmpdir}/asset" -d "$tmpdir"
  src_bin="$(find "$tmpdir" -type f -name "${BIN_NAME}" -o -name "${BIN_NAME}.exe" | head -n1)"
else
  echo "Unknown archive format: $asset_url" >&2
  exit 1
fi

if [[ -z "$src_bin" ]]; then
  echo "Binary not found inside archive" >&2
  exit 1
fi

install_path="${INSTALL_BIN_DIR}/${BIN_NAME}"
cp "$src_bin" "$install_path"
chmod +x "$install_path"

echo "Installed ${BIN_NAME} to ${install_path}"

# Add to PATH and init
shell_rc=""
case "${SHELL:-}" in
  *zsh) shell_rc="${HOME}/.zshrc" ;;
  *bash) shell_rc="${HOME}/.bashrc" ;;
  *fish) shell_rc="${HOME}/.config/fish/config.fish" ;;
esac

export PATH="${INSTALL_BIN_DIR}:${PATH}"

if [[ -n "$shell_rc" ]]; then
  if [[ "$shell_rc" == *fish ]]; then
    if ! grep -q "${INSTALL_BIN_DIR}" "$shell_rc" 2>/dev/null; then
      echo "set -Ux PATH ${INSTALL_BIN_DIR} \$PATH" >> "$shell_rc"
    fi
    if ! grep -q "${BIN_NAME} init" "$shell_rc" 2>/dev/null; then
      echo "${BIN_NAME} init --shell=fish | source" >> "$shell_rc"
    fi
  else
    if ! grep -q "${INSTALL_BIN_DIR}" "$shell_rc" 2>/dev/null; then
      echo "export PATH=\"${INSTALL_BIN_DIR}:\$PATH\"" >> "$shell_rc"
    fi
    if ! grep -q "${BIN_NAME} init" "$shell_rc" 2>/dev/null; then
      echo "eval \"\$(${BIN_NAME} init)\"" >> "$shell_rc"
    fi
  fi
fi

echo
echo "${BIN_NAME} installed. Restart your shell or run:"
echo "  export PATH=\"${INSTALL_BIN_DIR}:\$PATH\""
echo "Then verify with:"
echo "  ${BIN_NAME} doctor"


