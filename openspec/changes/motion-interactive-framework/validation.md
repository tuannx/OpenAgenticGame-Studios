# Validation Evidence

Validated on 2026-07-21.

## Framework and build

- `cargo fmt --all -- --check`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo test --all-targets --all-features`: 10 passed.
- Macroquad validation script: native gates and release WASM passed.
- `npm run typecheck`: passed.
- `npm test`: 2 passed.
- `npm run build`: passed.
- `npm audit --audit-level=high`: 0 vulnerabilities.
- `wrangler deploy --dry-run`: Assets and Durable Object bindings passed.

## Runtime and production

- Local game HTML, security headers, and WASM MIME passed over HTTP.
- Local two-client Durable Object signaling relay passed.
- Production game HTML and privacy/camera controls returned the new build.
- Production two-client WSS signaling relay passed.
- Release WASM is 649,545 bytes and is served as fingerprinted
  `brainbreak-game-69f41264c82f.wasm` with immutable caching.

## Release defect found and fixed

The first deploy served new HTML with a cached fixed-name WASM artifact. The
build now fingerprints the WASM filename, injects that exact URL into the game
bundle, and reserves immutable caching for content-addressed assets.

## Residual validation

The in-app Browser reported no available browser instance. Camera permission,
real-person pose alignment, mobile orientation, and visual frame pacing were
therefore not represented as passed. They remain explicit device QA gates.
