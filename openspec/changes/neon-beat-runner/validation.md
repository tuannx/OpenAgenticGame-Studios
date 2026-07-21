# Validation: Neon Beat Runner

Date: 2026-07-21

## Automated Gates

- `cargo fmt --all -- --check`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo test --all-targets --all-features`: passed, 16 Rust tests.
- `refenrece/skills/macroquad-rust-wasm/scripts/validate.sh brainbreak`: passed native checks and release `wasm32-unknown-unknown` build.
- `npm run typecheck`: passed.
- `npm test`: passed, 4 web tests.
- `npm run build`: passed.
- `npm audit --audit-level=high`: passed, 0 vulnerabilities.
- `npx wrangler deploy --dry-run`: passed, 19 static assets found.

## Bundle Evidence

- Release WASM: `brainbreak-game-12115446f734.wasm` before the final release commit.
- Music: `special-spotlight-db0e06528b9a.mp3`, 3,079,985 bytes, 128 kbps, 192.444 seconds.
- Music SHA-256: `db0e06528b9ac754e771d933e5bcf68d8408b99ee4cdb52fcc2988cb9232df61`.
- Static distribution size: approximately 6.7 MB.
- Vite keeps vision in a lazy chunk; its minified size warning is known and does not block initial game-shell loading.

## HTTP Smoke

Using local Wrangler at `http://localhost:8787`:

- `/health` returned game `neon-beat-runner`, schema version 2.
- fingerprinted JavaScript returned `text/javascript` with immutable caching.
- fingerprinted release WASM returned `application/wasm` with immutable caching.
- fingerprinted music returned `audio/mpeg` with immutable caching.
- root HTML contained the camera-first gate and visible CC BY 4.0 attribution.
- release WASM imports included beat phase, pulse, energy, playback, reduced-motion, and feedback sound bridges.

## Persona Review

- Security: camera/privacy and evaluated-action boundaries remain intact; music is same-origin and CSP `media-src` remains self-only.
- Performance: gameplay uses fixed-capacity obstacle and particle arrays; analyser work is cached per ~12 ms frame; the audio asset is web-compressed and fingerprinted.
- Readability: runner rules stay in `brainbreak-core`; browser audio and Macroquad rendering remain adapters.
- Runtime regression: production asset fallback caching is still content-type guarded; remote players are evaluated only while a peer connection exists.

## Known Validation Gap

The configured in-app browser reported no available browser instances. Therefore
interactive canvas inspection, real camera permission, audible playback,
resize/orientation, tab focus recovery, and release frame pacing could not be
automated in this environment. HTTP delivery and build-time contracts passed,
but physical/browser camera playtesting remains required.

## Production

- Deployment and production probes pending.
