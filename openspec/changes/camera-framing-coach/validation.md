# Validation

## Baseline

- Current tracking presentation treats every pose above aggregate quality
  `0.25` as tracked and immediately enables setup navigation.
- Aggregate quality alone does not prove the shoulders, hips, hand, scale, or
  camera margins needed by the active recognizer.
- Real camera framing and gesture feel remain attended QA surfaces.

## Automated evidence

- `pose-framing.test.ts` covers the 400 ms stable-ID gate, too-near,
  too-far, off-center, cropped/low-quality, identity replacement, one-player
  Duo navigation, and two-player restabilization.
- `pose-framing-status.test.ts` covers recovery, stable, Duo invitation, and
  active-navigation presentation without coupling localized copy to vision.
- The final
  `bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak`
  passed: 26 deterministic core tests, 3 Macroquad layout tests, strict Clippy,
  native/release WASM, strict TypeScript, 38 web tests across 9 files, production
  Vite build, and dependency audit with zero vulnerabilities.
- `git diff --check` passed after the final responsive adjustment.
- Final artifacts were `brainbreak-game-a82f5a4d904c.wasm` (654,008 bytes),
  `game-DygXaRBo.js` (30,191 bytes), `style-BXw-BJ-R.css` (22,152 bytes), and
  deferred `vision-DFHowJYG.js` (1,348,111 bytes; 341.34 KB gzip in Vite).
- Local HTTP probes returned `application/wasm`, `text/javascript`, and
  `text/css` for the corresponding fingerprinted artifacts.

## Browser evidence

- A fresh final launcher observed 3 scripts and no `vision-*` asset. The
  camera-intent state observed 5 scripts including `vision-DFHowJYG.js`; lazy
  capability ownership from M5 remains intact.
- At `390×844`, the camera card (343×257) and ready card (366×372) stayed inside
  the viewport with no body overflow. The reticle was visible, fallback stayed
  collapsed with a 44 px summary target, and start/back were disabled during
  permission.
- At `844×390`, the settled ready card (422×226) and camera card (338×253) stayed
  inside the viewport. The selected image is intentionally hidden at this short
  height so the real camera preview and one status remain dominant.
- At `1440×784`, the ready card (540×601), 480×240 selected neon image, and
  520×390 camera preview stayed in bounds with no body overflow.
- Entering camera setup now removes width/transform transitions from the live
  camera card, so rotation snaps to a safe composition instead of briefly
  sliding the framing target through an edge. Reduced-motion also disables the
  camera-card transition.
- Fresh cold-start and camera-intent tabs emitted no app-origin warning/error.
  Reloading an already initialized Macroquad tab did emit three
  `glBindTexture called with an already deleted texture` messages from
  `mq_js_bundle.js`; a new cold tab did not reproduce them. This is retained as
  a separate reload-lifecycle residual, not hidden as a clean-console claim.

## Persona review

### Security and privacy

- Framing reads only the current local pose snapshot. It adds no camera auto
  start, network call, storage key, raw-pose persistence, screenshot capture, or
  biometric/taste signal.
- Vision acquisition and Rust evaluation boundaries are unchanged; bridge ABI
  remains version 5.

### Performance

- Work is bounded by the existing maximum of two local poses and a fixed set of
  head/shoulder/wrist/hip lookups. No detector, model, render-loop, or network
  work was added.
- The 341.34 KB gzip vision chunk is still deferred rather than reduced. Real
  detector startup and first-pose latency were not measured without permission.

### Readability and architecture

- `PoseFramingStatus` is a closed union, `PoseFramingCoach` owns ephemeral setup
  policy, and presentation mapping owns Vietnamese copy. `vision.ts` remains a
  capability adapter and `MotionNavigationController` remains gesture logic.
- Framing and mode readiness stay separate: one framed player can lean away
  from Duo, but the post-update required-player check keeps touch fallback and
  raised-hand confirmation from starting Duo with P1 alone.

### Runtime regression

- Raw pose bridging after launch, page-visibility freshness, guide-only
  fail-closed behavior, selected-mode art, theme tokens, local taste aggregates,
  and the one-touch camera-intent boundary remain unchanged.
- Capability `tracking` callbacks no longer overwrite fallback state computed
  from the later framing/navigation owner in the same inference frame.

## Residual attended QA

Camera permission was not accepted. Real hardware still needs to tune and prove
the near/far thresholds for children and adults, low-light/cropped recovery,
400 ms stability feel, mirrored reticle/skeleton alignment, P1/P2 identity when
crossing, lean escape from Duo, raised-hand confirmation, denial/retry, and
camera stop/back after tracking. Synthetic fixtures and permission-state browser
screens are intentionally not presented as that evidence.

The same-tab Macroquad WebGL reload warning also remains a separate follow-up.
No production deploy or live-origin probe was performed.
