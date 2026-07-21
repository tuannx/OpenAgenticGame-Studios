#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PUBLIC_DIR="$PROJECT_DIR/web/public"

mkdir -p "$PUBLIC_DIR/tfjs-wasm"

RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C link-arg=--allow-undefined" \
  cargo build --manifest-path "$PROJECT_DIR/Cargo.toml" \
  --release --target wasm32-unknown-unknown -p brainbreak-game

cp "$PROJECT_DIR/target/wasm32-unknown-unknown/release/brainbreak-game.wasm" \
  "$PUBLIC_DIR/brainbreak-game.wasm"

MACROQUAD_LOADER="$(find "$HOME/.cargo/registry/src" -path '*/macroquad-0.4.15/js/mq_js_bundle.js' -print -quit)"
if [[ -z "$MACROQUAD_LOADER" ]]; then
  echo "Macroquad 0.4.15 loader was not found in the Cargo registry" >&2
  exit 1
fi
cp "$MACROQUAD_LOADER" "$PUBLIC_DIR/mq_js_bundle.js"

cp "$PROJECT_DIR/node_modules/@tensorflow/tfjs-backend-wasm/dist/"*.wasm \
  "$PUBLIC_DIR/tfjs-wasm/"
