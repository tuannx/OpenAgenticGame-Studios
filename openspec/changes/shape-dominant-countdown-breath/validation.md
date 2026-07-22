# Validation

## Automated evidence

Passed on the final tree on 2026-07-21:

- `cargo fmt --all -- --check` and strict workspace Clippy across all targets and
  features.
- Workspace tests: 41 core tests and 22 game/layout tests.
- The new presentation test covers values above two, both exact stage
  boundaries, fractions within each stage, zero, negative, `NaN`, and positive
  infinity. It proves bounded ticks and the single `2 -> 1` reset.
- The new stroke test proves distinct, non-empty fixed geometry for `2` and `1`.
- The expanded layout test contains panel, title, outer ring, ticks, and numeral
  strokes at 390x844, 667x375, and 1440x784.
- `npm run typecheck` and 50 Vitest browser-logic tests.
- Release `wasm32-unknown-unknown` build, production Vite bundle, and
  `git diff --check`.

The complete BrainBreak validator passed Rust/native/WASM, TypeScript, tests,
and production build, then stopped at the unchanged dependency audit. The
`wrangler -> miniflare -> sharp` chain reports three inherited high findings;
the offered forced fix changes Wrangler incompatibly and was not applied in
this presentation milestone.

## Runtime-truth and hierarchy review

`countdown_presentation` projects only the core's public
`runner.countdown_remaining`. Values above one select `Two` and subtract one for
stage-local progress; values at or below one select `One`. The ring derives its
bounded twelve-tick count from that same fraction, so it drains monotonically
within a numeral and resets only when the authoritative value crosses one.

Starting, Resuming, and PauseResuming share the same renderer. The vector `2`
and `1`, time ring, and `GET READY` are the only dominant layers. No photographic
mode image was added: the two-second safety breath intentionally has one timing
shape, while canonical art remains present on surrounding Ready, recovery, and
result surfaces.

`RunnerGame::update`, countdown duration and transitions, tracking-loss
destinations, gesture consumption, scoring, and browser imports are untouched.
Presentation contains only copyable enums, fixed arrays, and bounded drawing
loops; it creates no second timer and performs no heap allocation per frame.

## Release WASM and HTTP evidence

- Artifact: `brainbreak-game-425aa5274bb6.wasm`, 954,575 bytes raw and 313,033
  bytes under gzip.
- M17 artifact: 953,491 raw / 312,589 gzip. M18 adds 1,084 raw bytes (+0.11%)
  and 444 gzip bytes (+0.14%) with no new asset or decoder.
- `wasm-objdump` confirms `brainbreak_bridge_crate_version()` remains `7`; no
  JavaScript import changed.
- Explicit IPv4 HTTP on `127.0.0.1:58924` returned the correct BrainBreak title,
  final WASM length, and `200 application/wasm`, `200 text/javascript`, and
  `200 image/jpeg` for representative required artifacts.
- Browser-driven server logs show final JS/WASM, all mode images, phone guide,
  and the Macroquad loader returned 200. Only inherited `favicon.ico` returned
  404 during browser startup.

## Browser evidence

Chrome loaded `Neon Beat Runner • BrainBreak`, created a 5120x2578 backing
canvas for a 2560x1289 CSS canvas, and decoded every inspected setup/mode image
at 640x640. No warning or error came from the application origin.

This proves the final release and unchanged browser imports start after the
countdown renderer change. Without camera permission and an evaluated start or
resume path, startup cannot prove the numeral transition, visible drain rhythm,
or the physical preparation breath.

## Persona review

- **Security:** no camera permission, input, asset, decoder, network, or import
  boundary changed.
- **Performance:** the renderer is stack-only, uses fixed twelve- and five-item
  bounds, and increases compressed WASM by only +0.14%.
- **Readability:** font-dependent numerals are replaced by large vector strokes;
  the draining ring provides redundant time shape without adding prose, emoji,
  or another action.
- **Runtime regression:** all three run-entry phases reuse a projection of the
  authoritative remaining time; core countdown and interruption semantics stay
  unchanged.

## Residual attended QA

A permissioned physical run must still enter the initial countdown, hands-free
replay, tracking resume, and intentional-pause resume; observe the `2 -> 1`
reset and drain at 1.5-2.5 metres; interrupt each countdown with tracking loss;
and verify the breath feels calm without hiding the next gameplay cue.
Automated mapping, geometry, and browser startup cannot replace that evidence.
