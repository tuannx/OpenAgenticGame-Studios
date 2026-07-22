# Validation

## Automated evidence

Passed on the final tree on 2026-07-21:

- `cargo fmt --all -- --check` and strict workspace Clippy across all targets and
  features.
- Workspace tests: 41 core tests and 20 game/layout tests.
- The new pause test covers missing, present, and ready single players;
  remote-only single readiness; remote-only Duo rejection; P1/P2 loss; neither,
  either, and both Duo clap states; static ASCII copy; and removal of the bullet
  glyph.
- The expanded layout test contains panel, bars, title, instruction, centered
  single figure, and separated Duo figures at 390x844, 667x375, and 1440x784.
- `npm run typecheck` and 50 Vitest browser-logic tests.
- Release `wasm32-unknown-unknown` build, production Vite bundle, and
  `git diff --check`.

The complete BrainBreak validator passed Rust/native/WASM, TypeScript, tests,
and production build, then stopped at the unchanged dependency audit. The
`wrangler -> miniflare -> sharp` chain reports three inherited high findings;
the offered forced fix changes Wrangler incompatibly and was not applied in this
renderer milestone.

## Runtime-truth and hierarchy review

`pause_presentation` reads evaluated and ready arrays. Single modes match the
core's `any evaluated` capacity rule; Duo reads required P1/P2 only. Missing
evaluation always maps to a return-to-frame instruction before clap guidance.
Remote P3/P4 evaluation does not satisfy local Duo presentation.

The two-bar pause glyph and `PAUSED` remain the first visual layer. One concise
next action and one/two bounded motion figures now follow it. No photographic
mode art was added: this preserves M12's deliberate safety hierarchy while
sharing the neon vector and body-state language of Ready and TrackingHold.

`RunnerGame::update`, the pause recognizer/dwell, readiness clearing, countdown,
tracking thresholds, scoring, and browser imports are untouched. Presentation
contains only static slices, enums, and fixed arrays.

## Release WASM and HTTP evidence

- Artifact: `brainbreak-game-cf80311d85a2.wasm`, 953,491 bytes raw and 312,589
  bytes under gzip.
- M16 artifact: 952,227 raw / 311,969 gzip. M17 adds 1,264 raw bytes (+0.13%)
  and 620 gzip bytes (+0.20%) with no new asset or decoder.
- `wasm-objdump` confirms `brainbreak_bridge_crate_version()` remains `7`; no
  JavaScript import changed.
- Explicit IPv4 HTTP on `127.0.0.1:58923` returned the correct BrainBreak title,
  final WASM length, and `200 application/wasm`, `200 text/javascript`, and
  `200 image/jpeg` for required artifacts.
- Browser server logs show final JS/WASM, all mode images, phone guide, and the
  Macroquad loader returned 200. Only inherited `favicon.ico` returned 404.

## Browser evidence

Chrome loaded `Neon Beat Runner • BrainBreak`, created a 5120x2578 backing
canvas for a 2560x1289 CSS canvas, and decoded all inspected setup/mode images at
640x640. No warning or error came from the application origin.

This proves the final release and unchanged browser imports start after the
pause renderer change. Without camera permission and an evaluated running
session, startup cannot enter `Paused` through the real one-second gesture, so
it does not prove direct pause composition or recovery timing.

## Persona review

- **Security:** no camera permission, input, asset, decoder, network, or import
  boundary changed.
- **Performance:** the presenter/layout are stack-only and reuse bounded vector
  primitives; measured raw growth is +0.13%.
- **Readability:** intentional pause retains its universal bars while missing,
  present, and ready players gain corner frame, neutral body, and raised arm plus
  diamond. Copy exposes one action without bullet punctuation.
- **Runtime regression:** core pause and resume contracts remain unchanged;
  renderer tests use their authoritative evaluation/readiness facts.

## Residual attended QA

A permissioned physical run must still trigger the one-second pause gesture,
lose/reacquire P1 and P2 while paused, latch claps independently, and verify
bars, figures, copy, and transition timing at 1.5-2.5 metres. Automated mapping,
geometry, and browser startup cannot replace that evidence.
