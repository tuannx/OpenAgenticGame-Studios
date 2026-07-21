# Validation Evidence

Validated on 2026-07-21.

## Build and tests

- `cargo fmt --all -- --check`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo test --all-targets --all-features`: 5 passed.
- `npm run typecheck`: passed.
- `npm test`: 2 passed.
- `npm run build`: passed; release WASM is 608,513 bytes.
- `npm audit --audit-level=high`: 0 vulnerabilities.
- `wrangler deploy --dry-run`: passed with Assets and Durable Object bindings.

## Runtime smoke

- Local HTTP: game, studio, health, and `application/wasm` MIME passed.
- Local room protocol: host and guest connected to the same Durable Object and
  an offer probe relayed successfully over WebSocket.
- Production health reports build `a8d6e1b` at
  `https://brainbreak-motion-party.tuannx87.workers.dev`.
- Production game, studio, security headers, WASM MIME, room creation, and WSS
  relay passed.
- The in-app Browser backend reported no available browser, so visual/camera QA
  is not represented as passed by this run.

## TURN status

The Worker has a room-gated, short-lived Cloudflare Realtime TURN broker. The
current Wrangler OAuth token cannot create Calls TURN keys (`Authentication
error`); production therefore returns the intentional Cloudflare STUN fallback
until a `Calls Write` API token is used to install `TURN_KEY_ID` and
`TURN_KEY_API_TOKEN` secrets.
