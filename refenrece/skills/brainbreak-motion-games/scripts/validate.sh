#!/usr/bin/env bash
set -euo pipefail

skill_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
repo_dir="$(cd "$skill_dir/../../.." && pwd)"
project_dir="${1:-$repo_dir/brainbreak}"
base_url="${2:-}"

if [[ ! -f "$project_dir/Cargo.toml" || ! -f "$project_dir/package.json" ]]; then
  echo "Expected BrainBreak Cargo.toml and package.json in: $project_dir" >&2
  exit 2
fi

echo "[1/4] Validating Rust and release WASM"
bash "$repo_dir/refenrece/skills/macroquad-rust-wasm/scripts/validate.sh" "$project_dir"

echo "[2/4] Validating TypeScript and browser logic"
npm --prefix "$project_dir" run typecheck
npm --prefix "$project_dir" test

echo "[3/4] Building production web bundle and auditing dependencies"
npm --prefix "$project_dir" run build
npm --prefix "$project_dir" audit --audit-level=high

dist_dir="$project_dir/web/dist"
wasm_file="$(find "$dist_dir" -maxdepth 1 -type f -name 'brainbreak-game-*.wasm' -print -quit)"
js_file="$(find "$dist_dir/assets" -maxdepth 1 -type f -name '*.js' -print -quit)"

if [[ -z "$wasm_file" || ! "$(basename "$wasm_file")" =~ ^brainbreak-game-[a-f0-9]{12}\.wasm$ ]]; then
  echo "Missing fingerprinted BrainBreak WASM in: $dist_dir" >&2
  exit 1
fi
if [[ -z "$js_file" ]]; then
  echo "Missing production JavaScript in: $dist_dir/assets" >&2
  exit 1
fi

audio_file="$(find "$dist_dir/audio" -maxdepth 1 -type f -name '*.mp3' -print -quit 2>/dev/null || true)"
if [[ -n "$audio_file" ]]; then
  if [[ ! "$(basename "$audio_file")" =~ -[a-f0-9]{12}\.mp3$ ]]; then
    echo "Audio must use a 12-character content fingerprint: $audio_file" >&2
    exit 1
  fi
  if [[ ! -f "$dist_dir/audio/ATTRIBUTION.md" ]]; then
    echo "Audio exists without dist/audio/ATTRIBUTION.md" >&2
    exit 1
  fi
fi

echo "[4/4] Checking optional deployed origin"
if [[ -n "$base_url" ]]; then
  base_url="${base_url%/}"
  curl -fsS "$base_url/health" >/dev/null

  check_content_type() {
    local url="$1"
    local expected="$2"
    local content_type
    content_type="$(curl -fsSI "$url" | tr -d '\r' | awk -F': ' 'tolower($1)=="content-type" {print tolower($2); exit}')"
    if [[ "$content_type" != *"$expected"* ]]; then
      echo "Unexpected content-type for $url: $content_type (expected $expected)" >&2
      exit 1
    fi
  }

  check_content_type "$base_url/" "text/html"
  check_content_type "$base_url/$(basename "$wasm_file")" "application/wasm"
  check_content_type "$base_url/assets/$(basename "$js_file")" "javascript"
  if [[ -n "$audio_file" ]]; then
    check_content_type "$base_url/audio/$(basename "$audio_file")" "audio/"
  fi
else
  echo "No production URL supplied; skipped live HTTP probes"
fi

echo "BrainBreak motion-game validation passed"
echo "WASM artifact: $wasm_file"
