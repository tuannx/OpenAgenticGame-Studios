# Validation

## Automated evidence

Passed on the final tree on 2026-07-21:

- Strict TypeScript typecheck and 54 Vitest tests across 11 files.
- Three new presentation tests prove framing priority, the closed
  `framing/neutral/left/right/holding` visual states, and identical static hold
  copy at 10% and 95% numeric progress.
- Expanded framing tests prove every normal correction is free of emoji and
  bullet-joined prose; capability tests cover simplified permission,
  initialization, calibration, tracking, and dynamic error copy.
- Strict workspace Clippy, 41 deterministic core tests, and 22 game/layout
  tests remain green.
- Release `wasm32-unknown-unknown`, production Vite build, formatting, and
  `git diff --check` passed.

The complete BrainBreak validator passed Rust/native/WASM, TypeScript, all
tests, and the production build, then stopped at the unchanged dependency audit.
The `wrangler -> miniflare -> sharp` chain reports three inherited high findings;
the offered forced fix changes Wrangler incompatibly and was not applied in
this presentation milestone.

## Runtime-truth and hierarchy review

`presentReadyNavigation` receives the existing `MotionNavigationState` and
`PoseFramingPresentation`. Any non-ready framing state returns the one current
physical correction before direction or hold feedback. Once framed, direction,
neutral, and holding map to a closed visual state; numeric progress remains
separate and authoritative in `MotionNavigationController`.

The ready surface keeps the selected canonical 640x640 neon mode image. Two CSS
chevrons carry lean direction, while one fixed raised-hand SVG sits inside a
conic hold ring. Permission and framing states dim the compass so their single
correction owns attention. The visible percentage string and font arrows/hand
emoji were removed; `aria-valuenow` remains for assistive technology.

Camera acquisition, framing thresholds, stable-ID dwell, one-second confirm
hold, mode availability, fallback enablement, Rust evaluation, browser imports,
and ABI are unchanged. The ring derives only from existing `confirmProgress`;
there is no second timer or state machine.

## Release and HTTP evidence

- WASM remains byte-identical to M18:
  `brainbreak-game-425aa5274bb6.wasm`, 954,575 bytes raw / 313,033 gzip, ABI v7.
- Final entry HTML is 10,494 bytes raw / 3,463 gzip; game JS is 33,940 raw /
  11,912 gzip; CSS is 27,263 raw / 6,781 gzip.
- The deferred vision chunk remains 1,348,440 bytes raw (341.46 KB gzip by
  Vite); the milestone changes no model or inference code.
- Explicit IPv4 HTTP returned the BrainBreak title plus
  `application/wasm`, `text/javascript`, `text/css`, and `image/jpeg` for the
  final WASM, game/vision scripts, stylesheet, and canonical image.
- Browser server logs show the vision chunk loads only after camera intent and
  every required JS/WASM/image/audio request returns 200. The inherited browser
  `favicon.ico` request remains the only 404.

## Browser and responsive evidence

At 2560x1289, the permission state rendered a 540x591.9 ready card, 480x240
canonical image, 480x101.4 compass, 76x76 ring, 44 px collapsed fallback, and
520x390 live camera card. All were contained with body scroll dimensions equal
to the viewport. The one-line title measured 42.5 px high; the selected image
decoded at 640x640; computed style exposed the expected conic ring.

A same-origin production-bundle fixture then exercised real CSS viewports:

- At 390x844, the live preview stayed in the upper region and the ready/action
  card stayed in the lower region without overlap or body escape.
- The first 667x375 run exposed a real composition defect: the mobile camera
  rule covered the action card even though both rectangles were individually in
  bounds. The final short-landscape rule now gives the ready card the left
  55vw lane and camera the right 38vw lane; the repeated visual run showed both
  surfaces side by side with the full compass visible.

The temporary responsive fixture was removed after QA. Desktop and both iframe
runs emitted no warning or error from the application origin.

## Persona review

- **Security/privacy:** no permission auto-start, pose persistence, network,
  image, camera, or evaluation boundary changed.
- **Performance:** normal visible copy is static and percentage text formatting
  left the pose-update path. The ring updates one bounded CSS angle and reuses
  fixed DOM/SVG nodes; WASM and deferred vision bytes are unchanged.
- **Readability/accessibility:** canonical art preserves mode identity;
  chevrons and a raised-hand silhouette do not depend on font coverage or color;
  the progressbar and polite live region remain semantic.
- **Runtime regression:** navigation thresholds, hold timing, Duo requirements,
  framing priority, fallback gating, and Rust gameplay remain owned by their
  existing typed components.

## Residual attended QA

Camera permission was not accepted. A physical session must still verify live
reticle/skeleton alignment, real near/far/center corrections, lean mode changes,
the radial ring from zero to full, single/Duo raised-hand confirmation, child and
adult readability at 1.5-2.5 metres, rotation with real video metadata, denial,
retry, and stop/back. Permission-state geometry and synthetic pose tests do not
prove those behaviors.
