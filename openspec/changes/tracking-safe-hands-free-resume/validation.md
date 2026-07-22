# Validation

## Automated evidence

- Focused core tests covered tracking-loss freeze, hazard clearing, ignored
  unevaluated input, evaluated readiness, exact countdown completion, loss
  during countdown, and Duo two-person readiness.
- `bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak`
  passed twice after implementation and after the allocation-free render pass.
- Rust gate: formatting, strict Clippy, 26 core tests, 3 Macroquad layout tests,
  and release `wasm32-unknown-unknown` build passed.
- Layout coverage includes hold/countdown panels at 390×844, 667×375, and
  1440×784 in addition to existing HUD/result matrices.
- Web gate: strict TypeScript and 17 Vitest tests passed. Bridge coverage proves
  an inactive page clears poses, ignores background callbacks, and requires a
  fresh pose after visibility returns.
- Production web build and dependency audit passed with zero vulnerabilities.
  Final artifact: `brainbreak-game-a82f5a4d904c.wasm` (654,008 bytes).

## HTTP and browser smoke

- Final HTML loaded in a fresh Chrome tab with boot opacity 0 and no app-origin
  warning/error logs.
- Canvas exactly matched 390×844 and 1440×784 viewports; the setup dialog stayed
  within the phone viewport with zero horizontal or vertical body overflow.
- Final fingerprinted WASM returned `application/wasm`; the final game chunk
  returned `text/javascript` and referenced that WASM fingerprint.
- An in-place reload reproduced the known Macroquad stale-context deleted-texture
  warning. It did not appear in either clean fresh tab, including the tab opened
  after the final build, so it is not attributed to the final artifact.
- Opening another agent tab did not move the app document to `hidden`, so the
  browser run did not provide real focus-loss evidence. The deterministic bridge
  test is the current proof for visibility freshness.

## Sequential persona review

### Security and privacy

- Inactivity clears the in-memory pose snapshot and adds no storage, analytics,
  network payload, or permission request.
- Evaluation still requires camera opt-in, page activity, and current pose
  quality. Keyboard/gamepad input cannot bypass that mask.

### Performance

- Core phase checks and obstacle clearing are fixed-size O(4)/O(12) operations.
- Hold and countdown copy uses static strings, avoiding per-frame allocations
  during an indefinitely long pause.
- The dynamically imported vision chunk remains about 1.35 MB uncompressed; it
  is outside the launcher path but remains a future camera-start optimization.

### Readability and architecture

- Typed `TrackingHold` and `Resuming` phases make the safety transitions
  exhaustive and keep browser lifecycle APIs outside the deterministic core.
- Small helpers centralize required evaluation and readiness semantics for first
  start and resume without duplicating Duo rules.
- Bridge version remains 5 because behavior changed behind an existing import;
  no Rust/JavaScript ABI function was added or removed.

### Runtime regression

- Running returns before action, beat, distance, scoring, and collision updates
  on tracking loss; in-flight hazards are removed once.
- Tracking loss during countdown restarts hold, and the world advances only on
  the frame after countdown completion.
- HUD, next cue, and beat meter remain `Running`-only, so frozen states have one
  dominant instruction instead of conflicting motion prompts.

## Residual attended QA

Camera permission was not accepted in automation. A real session still needs to
observe loss/reacquire while running, two-person Duo recovery, physical
camera-distance readability, and actual hidden/visible tab behavior with camera
and audio active. No production deployment or live-origin probe was requested.
