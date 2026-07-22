# Validation

## Automated evidence

Passed on 2026-07-21:

- `cargo fmt --all -- --check`.
- Strict workspace Clippy across all targets and features.
- Workspace tests: 41 core tests and 13 game/layout tests.
- Web typecheck and Vitest: 50 tests, including five bridge tests and eight
  taste-profile tests.
- Release `wasm32-unknown-unknown` build and production Vite bundle.
- Core coverage proves phase freezes, pre-progression timeout, one-shot typed
  outcomes, distinct energy depletion, replay reset, and best-score retention.
- Layout coverage proves the shape-first session rail and positive result title
  selection without adding a touch requirement.
- Web unit tests prove bridge v7 registration, invalid/unsupported outcome
  rejection, backward-compatible taste migration, bounded aggregates, and
  completion-weighted recommendations.

The complete BrainBreak validator passed every gate through the production
bundle. Its final dependency-audit gate reported three inherited high findings
on `wrangler -> miniflare -> sharp`; npm offered only a forced breaking change.
No force-upgrade was applied as part of this bounded UX milestone.

## Release WASM and HTTP evidence

- Artifact: `brainbreak-game-9869f081b421.wasm`, 660,409 bytes.
- `wasm-objdump` confirms import `env.bb_record_run_outcome`.
- Disassembly confirms `brainbreak_bridge_crate_version()` returns `7`, matching
  the JavaScript plugin version.
- HTTP smoke test returned `200 application/wasm` for the release WASM and
  `200 text/javascript` for `assets/game-7iZ5BRXz.js`.
- Server access logs confirm the browser requested the final JavaScript, WASM,
  launcher art, phone-placement guide, and audio asset.

## Browser visual evidence

The production bundle was served over HTTP and inspected in the browser:

- Phone portrait `390x844`: launcher card `366x738`, no document or card
  overflow; canvas filled the viewport.
- Phone landscape `844x390`: launcher stayed within the document; its bounded
  card used 59 px of internal overflow while the primary action remained a
  clear `771x58` target.
- Desktop `1440x784`: launcher card `980x694`, no overflow; canvas filled the
  viewport.
- Guide-only phone-placement flow used the same Mirror artwork style. At
  `390x844` its `366x542` card needed no scroll; at `844x390` its `760x277`
  card and two actions remained fully contained.
- No application-origin console error or warning appeared. Observed warnings
  were emitted only by an installed Chrome extension.

This browser pass validates launcher and guide presentation plus actual release
asset loading. It does not claim that the 90-second camera-backed result screen
was physically played through.

## Persona review

- **Runtime correctness:** the active clock is owned by deterministic Rust,
  freezes outside evaluated `Running`, and completes before action, beat,
  distance, passive-score, or collision progression on the boundary frame.
- **Security and privacy:** the bridge accepts only validated numeric mode and
  outcome codes. Learning stores constant-size coarse counters, never frames,
  landmarks, body geometry, pose history, or identity.
- **Performance:** runtime cost is one scalar clock update per active frame and
  one bounded event per run; the rail creates no per-frame collection growth.
- **Readability and maintenance:** typed outcomes and one `finish_run` boundary
  keep timeout and depleted-energy semantics explicit while retaining a shared,
  positive result path.

## Residual attended QA

A permissioned physical camera run must still judge whether 90 active seconds
feels appropriate across ages, whether the progress rail is readable at
1.5-2.5 metres, and whether the positive result transition feels immediate.
Those claims remain intentionally unverified rather than inferred from builds,
automated layout tests, or launcher-only browser inspection.
