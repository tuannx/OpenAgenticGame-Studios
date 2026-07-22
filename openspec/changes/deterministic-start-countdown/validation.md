# Validation

## Automated evidence

- `cargo fmt --all -- --check`, workspace tests, and strict workspace Clippy
  passed: 28 deterministic core tests and 11 game/layout tests.
- Core coverage proves single-player and Duo readiness, replay preservation,
  exact countdown freeze/completion, ignored countdown actions, and distinct
  tracking-loss destinations for `Starting` and `Resuming`.
- `bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak`
  passed Rust, release WASM, TypeScript, 47 web tests, and production bundle
  construction. It then stopped at the known dependency audit: three high
  findings in `wrangler -> miniflare -> sharp`; the offered automatic fix would
  install a breaking Wrangler version and was not applied.
- Final artifacts are `brainbreak-game-e7a98206a46b.wasm` (656,833 bytes) and
  `assets/game-BOosuH2A.js` (33,000 bytes). `wasm-objdump` confirms the existing
  `bb_play_feedback` import and bridge crate version `6`; M11 adds no ABI change.

## HTTP/browser evidence

- The final production bundle loaded from `http://127.0.0.1:4199/` in a clean
  browser tab. The only warnings came from an unrelated browser extension;
  application-origin warning/error count was zero.
- At 390x844, 844x390, and 1440x784, the canvas, setup gate, body scroll bounds,
  and viewport were identical, with boot canvas opacity `1`.
- The final WASM returned `application/wasm`; the final game chunk returned
  `text/javascript`.

## Sequential persona review

### Safety and privacy

- Readiness still requires runtime evaluation; guide-only input cannot enter
  `Starting`. M11 adds no camera storage, network payload, analytics, permission,
  or browser import.

### Performance

- Countdown work is fixed-time scalar state. No dynamic collection, allocation,
  browser timer, or render-loop string formatting was added.

### Readability and architecture

- `Starting` and `Resuming` encode different interruption destinations while
  sharing presentation. Two narrow helpers prevent callers from constructing an
  invalid generic countdown phase.
- The DOM continues to own permission/framing/mode selection; Rust remains the
  sole owner of authoritative run entry.

### Runtime regression

- The ready gesture is consumed as readiness only. World progression begins on
  the frame after countdown completion, while tracking resume retains its
  hazard-clearing hold behavior.
- HUD, action cue, feedback markers, and beat meter remain `Running`-only.

## Residual attended QA

A permissioned physical camera run is still required to judge the two-second
preparation beat at 1.5-2.5 metres, one- and two-person readiness, audio feel,
and real tracking interruption during the countdown.
