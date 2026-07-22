# Validation

## Baseline

- Fresh origin: `http://127.0.0.1:4181/?m5=baseline`.
- Browser `pageAssets`: 13 observed assets, including
  `vision-BoLXREif.js` before camera interaction.
- Production build reported the vision chunk at about 1.35 MB uncompressed and
  341 KB gzip.

Camera permission, model initialization, and physical gesture timing remain
attended-browser surfaces and are not inferred from asset or unit tests.

## Automated evidence

- `VisionModuleLoader` tests prove preference changes do not import, concurrent
  loads deduplicate, importer rejection retries, cleanup never imports, and a
  module whose initial preference application fails is not cached.
- Typed status tests cover permission, initialization, backend calibration,
  tracking ownership, and recoverable inference-error presentation.
- `bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak`
  passed after final self-review: 26 deterministic core tests, 3 Macroquad
  layout tests, strict Clippy, release WASM, strict TypeScript, 24 web tests,
  production Vite build, and dependency audit with zero vulnerabilities.
- Final artifacts: `brainbreak-game-a82f5a4d904c.wasm` (654,008 bytes),
  `game-CuRPuQU0.js` (26,842 bytes), and deferred
  `vision-DFHowJYG.js` (1,348,111 bytes; 341.34 KB gzip in Vite output).
- HTTP probes returned `application/wasm` for the fingerprinted WASM and
  `text/javascript` for both game and deferred vision chunks.

## Before/after browser asset evidence

| Fresh launcher state | Observed scripts | Vision chunk |
|---|---:|---|
| Baseline on port 4181 | 5 | `vision-BoLXREif.js` present |
| Final release on port 4184 | 3 | absent |
| Final after camera intent | 5 | `vision-DFHowJYG.js` present |

The final fresh launcher booted with no app-origin warning/error log. The
measurement proves removal from the initial launcher path; it does not claim the
deferred camera-start cost disappeared or supply field Core Web Vitals.

## Responsive camera-intent smoke

- At 390×844, camera intent entered typed `permission` state and showed
  `📷 Chọn Cho phép camera để tiếp tục` without granting camera access.
- The selected neon mode image remained visible; the final random selection used
  `/assets/game_mode_strike.jpg`, demonstrating the same mode-art pipeline.
- The stance card measured 366×372 within the viewport with zero body overflow.
- Touch fallback stayed collapsed; its summary target measured 44 px, and both
  start/back actions remained disabled until tracking.
- At 1440×784, the stance card and 474×234 mode art remained in bounds with zero
  horizontal or vertical body overflow.

## Sequential persona review

### Security and privacy

- No camera auto-start, idle prefetch, new storage, pose persistence, network
  payload, or bridge ABI change was introduced.
- Camera permission remained behind the explicit user action and was not granted
  during automation.

### Performance

- Launcher preference restoration is now constant-size data mutation and causes
  no TensorFlow/MoveNet import.
- One pending promise owns capability loading; cleanup is no-load and import or
  adapter-initialization failures remain retryable.
- The 341.34 KB gzip vision chunk is deferred, not eliminated. Backend/model
  download and inference-start latency remain the next measurement surface.

### Readability and architecture

- A discriminated `VisionStatus` union keeps capability state UI-agnostic and
  makes presentation mapping exhaustive.
- `vision-loader.ts` isolates import lifecycle from `main.ts`; `vision.ts` keeps
  camera/backend/detector ownership.
- The camera setup promise deduplicates the UI flow. Fallback actions cannot race
  permission/model setup, and Back stops an active camera before returning.

### Runtime regression

- Theme and overlay changes still reach an already loaded module immediately and
  the latest preferences apply on first load.
- Pose-navigation status regains ownership after tracking begins, so backend
  messages cannot overwrite lean or raised-hand feedback.
- Existing selected-mode assets, camera/audio gesture gates, guide-only path,
  local taste aggregates, and bridge v5 remain intact.

## Residual attended QA

Camera permission was not accepted. Real hardware still needs to verify backend
initialization, model download latency, calibration copy, first-pose transition,
gesture confirmation, denial/retry, and camera stop/back after tracking. No
production deploy or live-origin performance probe was requested.
