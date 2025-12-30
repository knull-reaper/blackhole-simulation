#!/usr/bin/env bash
set -euo pipefail

PROFILE="${1:-release}"
if [[ "$PROFILE" != "release" && "$PROFILE" != "debug" ]]; then
  echo "Usage: $0 [release|debug]"
  exit 1
fi

TARGET="wasm32-unknown-unknown"
CRATE="blackhole_web"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

pushd "$REPO_ROOT" >/dev/null
if [[ "$PROFILE" == "release" ]]; then
  cargo build --target "$TARGET" --release --lib
else
  cargo build --target "$TARGET" --lib
fi

WASM_PATH="$REPO_ROOT/target/$TARGET/$PROFILE/$CRATE.wasm"
if [[ ! -f "$WASM_PATH" ]]; then
  echo "Wasm output not found at $WASM_PATH"
  exit 1
fi

wasm-bindgen --out-dir "$REPO_ROOT/web/pkg" --target web "$WASM_PATH"
popd >/dev/null
