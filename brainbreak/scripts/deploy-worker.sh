#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$PROJECT_DIR/web/dist"
shopt -s nullglob
WASM_ARTIFACTS=("$DIST_DIR"/brainbreak-game-*.wasm)
if (( ${#WASM_ARTIFACTS[@]} != 1 )); then
  echo "Expected exactly one fingerprinted release WASM in $DIST_DIR" >&2
  exit 1
fi

WASM_FILENAME="$(basename "${WASM_ARTIFACTS[0]}")"
BUILD_ID="wasm-${WASM_FILENAME#brainbreak-game-}"
BUILD_ID="${BUILD_ID%.wasm}"

echo "Deploying Cloudflare build $BUILD_ID"
cd "$PROJECT_DIR"
exec npx wrangler deploy --var "BUILD_SHA:$BUILD_ID"
