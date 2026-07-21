# Design

## Decision: Layer Specialized Knowledge on Macroquad

Keep `macroquad-rust-wasm` authoritative for generic engine, toolchain, native/WASM, and browser loader guidance. Keep `brainbreak-motion-games` authoritative for the camera-to-evaluation-to-gameplay product stack. Future tasks load both, preventing knowledge drift and duplicated version-sensitive instructions.

## Decision: Express Contracts Before Recipes

The skill prioritizes invariants that survive game variants:

- deterministic core owns evaluation and scoring;
- camera/pose/audio/network remain adapters;
- untracked players cannot receive evaluated outcomes;
- raw camera and pose data remain local;
- bridge ABI versions change together;
- licensed audio has attributable, fingerprinted delivery;
- only verified non-HTML assets receive immutable caching;
- native, WASM, browser, interactive camera, P2P, and production evidence remain distinct.

Implementation details that may change, such as thresholds or exact game visuals, stay in source/configuration rather than agent instructions.

## Decision: Progressive Disclosure

Keep `SKILL.md` as the routing and workflow entrypoint. Load `architecture-contract.md` for cross-layer or motion-evaluation work and `web-runtime-checklist.md` for browser/deploy work. Keep automation in `scripts/validate.sh` so the same gate can be executed without loading its implementation into context.

## Decision: Validator Is Read-Only with Respect to Deployment

The validator formats/checks/tests/builds and optionally probes a supplied origin. It never deploys, edits secrets, or mutates Cloudflare state. Deployment remains an explicit user-authorized operation.

## Known Limitation Captured

The current adapter can combine fallback input with camera actions once evaluation is active. The skill records this provenance limitation so future agents do not overclaim that every evaluated action was camera-derived.
