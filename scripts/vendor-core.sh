#!/usr/bin/env bash
# Vendor solana-wasm-core into every PixZClaw plugin.
#
# The plugin CI (tools/ci/validate_components.sh) copies only plugins/<name>
# and wit/v0 into its build snapshot, so a path dependency reaching outside the
# plugin directory does not resolve there. Each plugin therefore carries its own
# copy under vendor/solana-wasm-core, and crates/solana-wasm-core stays the
# single source of truth this script syncs from.
#
# Usage: tools/vendor-core.sh [--check]
#   --check  exit 1 when a vendored copy differs from the source (CI guard)

set -euo pipefail

REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
SOURCE="$REPO_ROOT/crates/solana-wasm-core"
PLUGINS=(brl-usdc-invoice invoice-status pixzclaw-brief)

CHECK_ONLY=false
if [[ ${1:-} == "--check" ]]; then
  CHECK_ONLY=true
fi

# The source crate declares its own `[workspace]` so it builds standalone. The
# vendored copy must not: it lives inside the plugin's workspace root, and a
# second root in the same tree is a cargo error.
strip_workspace() {
  sed '/^\[workspace\]$/,$d' "$1"
}

if [[ ! -d "$SOURCE/src" ]]; then
  echo "error: source crate not found at $SOURCE" >&2
  exit 1
fi

status=0
for plugin in "${PLUGINS[@]}"; do
  dest="$REPO_ROOT/plugins/$plugin/vendor/solana-wasm-core"

  if $CHECK_ONLY; then
    if [[ ! -d "$dest" ]]; then
      echo "drift: $plugin has no vendored core"
      status=1
      continue
    fi
    if ! diff -qr "$SOURCE/src" "$dest/src" >/dev/null \
      || ! diff -q <(strip_workspace "$SOURCE/Cargo.toml") "$dest/Cargo.toml" >/dev/null; then
      echo "drift: $plugin vendored core differs from crates/solana-wasm-core"
      status=1
    else
      echo "ok: $plugin"
    fi
    continue
  fi

  rm -rf "$dest"
  mkdir -p "$dest"
  cp -R "$SOURCE/src" "$dest/src"
  strip_workspace "$SOURCE/Cargo.toml" >"$dest/Cargo.toml"
  [[ -f "$SOURCE/LICENSE" ]] && cp "$SOURCE/LICENSE" "$dest/LICENSE"
  [[ -f "$SOURCE/README.md" ]] && cp "$SOURCE/README.md" "$dest/README.md"
  echo "vendored -> plugins/$plugin/vendor/solana-wasm-core"
done

exit $status
