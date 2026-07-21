#!/usr/bin/env bash
set -euo pipefail

project_dir="${1:-.}"

if [[ ! -f "$project_dir/Cargo.toml" ]]; then
  echo "Cargo.toml not found in: $project_dir" >&2
  exit 2
fi

cd "$project_dir"

if ! rustup target list --installed | grep -qx 'wasm32-unknown-unknown'; then
  echo "Missing Rust target: wasm32-unknown-unknown" >&2
  echo "Install it with: rustup target add wasm32-unknown-unknown" >&2
  exit 2
fi

cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features

# Macroquad/miniquad intentionally imports browser functions from the JS host.
# Rust 1.96+ stopped passing --allow-undefined to wasm-ld by default, so retain
# that behavior explicitly until the pinned Macroquad/miniquad version declares
# its imports with wasm_import_module annotations.
wasm_rustflags="${RUSTFLAGS:-}"
if [[ "$wasm_rustflags" != *"--allow-undefined"* ]]; then
  wasm_rustflags="${wasm_rustflags:+$wasm_rustflags }-C link-arg=--allow-undefined"
fi

RUSTFLAGS="$wasm_rustflags" cargo build --release --target wasm32-unknown-unknown

wasm_dir="target/wasm32-unknown-unknown/release"
wasm_file="$(find "$wasm_dir" -maxdepth 1 -type f -name '*.wasm' -print -quit)"

if [[ -z "$wasm_file" ]]; then
  echo "No release WASM artifact found in: $wasm_dir" >&2
  exit 1
fi

echo "Macroquad validation passed"
echo "WASM artifact: $wasm_file"
