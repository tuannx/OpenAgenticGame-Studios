# Validation

## Pre-change evidence

- A same-origin synthetic 1280x720 MediaStream drove the production camera
  intent, lazy vision chunk, TensorFlow backend, MoveNet detector, inference
  loop, and Ready presentation without granting physical camera permission.
- Desktop exposed two status owners: actionable `BƯỚC VÀO KHUNG HÌNH` on the
  canonical Mirror Beat image and generic `Đang tìm người chơi` on the live
  preview. At the 2560px-wide capture, the Ready and camera cards were separated
  by well over 1000px of empty space.
- Portrait 390x844 stacked the live preview at the top and actionable status on
  the lower game-art card, leaving the physical correction far from the body
  view it explains.
- At 667x375, `.ready-visual-container` was `display:none`; the actionable
  status measured 0x0 while still existing in the dialog tree. The visible
  camera status measured 233.45x32 and contained only generic search text.
- The short-landscape Ready card and camera preview were otherwise fully
  contained, so this is an ownership/copy failure rather than overflow.

## Automated evidence

Passed on the final tree on 2026-07-21:

- The entry contract now proves exactly one `#camera-status` node with status
  semantics and no legacy Ready-art status owner.
- `presentReadyCameraStatus()` tests prove actionable permission, initialization,
  and calibration copy owns the preview before tracking, then yields to
  pose-driven guidance without a second mapping in the inference callback.
- Strict TypeScript and all 68 Vitest browser-logic tests passed.
- Strict Rust formatting/Clippy, 41 deterministic core tests, 22 game/layout
  tests, release `wasm32-unknown-unknown`, production Vite build, Macroquad
  loader syntax/strict-mode checks, and `git diff --check` passed.

The complete validator stopped only at the unchanged dependency audit. The
inherited `wrangler -> miniflare -> sharp` chain reports three high findings;
the offered forced fix installs incompatible Wrangler 4.15.2 and was not
applied.

## Browser and responsive evidence

The final HTTP-served production bundle ran its real lazy vision/MoveNet path
against a same-origin synthetic 1280x720 MediaStream:

- At 667x375, exactly one status role was exposed. Its visible 233.45x40 rail
  read `BƯỚC VÀO KHUNG HÌNH` inside the live 253.45x142.56 camera preview. The
  legacy status node count was zero while the intentionally hidden art container
  remained zero-size.
- The permission-pending branch displayed `CHO PHÉP CAMERA ĐỂ TIẾP TỤC` in the
  same 233.45x40 camera rail; the instruction did not depend on video metadata
  or the hidden canonical image.
- At 390x844, the 343.20x193.05 top camera preview owned the same actionable
  cue while the 366px lower Ready card retained clean canonical mode art and the
  complete action compass.
- At the 2560px-wide production viewport, the Ready card was x=690..1230 and
  camera preview x=1350..1870, reducing the measured inter-card gap to 120px.
  Both remained centered as one bounded 1180px stage.
- The final short-landscape body stayed in `gesture-setup`; Ready and camera
  rectangles remained fully inside 667x375. The temporary QA harness was removed.

Synthetic video proves application layout, model startup, inference callback,
and status ownership. It does not prove mirrored human landmarks, camera
rotation, recognition thresholds, or room-distance legibility.

## Capability and resource evidence

Camera constraints, detector/model selection, framing/navigation policy,
evaluation, audio, scoring, collision, multiplayer, Rust, bridge imports, ABI,
storage, and assets are unchanged. The existing camera intent still owns lazy
vision/audio startup; the change only consolidates presentation after that
boundary. The final server trace observed `vision-DkruDn0I.js` and the
fingerprinted MP3 only after camera intent, together with the unchanged
fingerprinted game WASM.

## Release and HTTP evidence

- WASM remains `brainbreak-game-425aa5274bb6.wasm`, 954,575 bytes, with SHA-256
  `425aa5274bb6c9a3766e5e384505d97e65d3817ea163cae9731f218f562ce0f5`.
- Final entry HTML is 10,351 bytes; game JS is 34,764 bytes; CSS is 31,047
  bytes. Fingerprinted audio remains
  `special-spotlight-db0e06528b9a.mp3`, 3,079,985 bytes.
- Built and source attribution documents remain byte-identical with SHA-256
  `f04d45438657db9b3f118dfddcbc3ed2ba9c2c511f4db92f7629cfc2e75d0b3b`.
- Explicit IPv4 HTTP returned `text/html`, `application/wasm`,
  `text/javascript`, `text/css`, `audio/mpeg`, and `text/markdown` for the entry,
  game WASM, game script, stylesheet, MP3, and attribution document.

## Persona review

- **Moving player:** the correction now sits on the mirrored body view rather
  than requiring a glance back to decorative game art.
- **Low/no touch:** no action or tutorial was added; the existing lean/hold
  compass and collapsed fallback are unchanged.
- **Visual continuity:** canonical mode art remains the identity owner and is no
  longer used as a technical status surface.
- **Accessibility:** one polite status owner survives the smallest landscape
  state instead of leaving a hidden zero-size live node.
- **Performance/safety:** the hot status callback maps each typed status once;
  camera evaluation and deterministic gameplay boundaries are unchanged.

## Residual attended QA

A real person/camera session must still verify mirrored landmark alignment,
orientation changes, one/two-person framing, gesture thresholds, live-region
announcement cadence, and legibility at 1.5-2.5 metres. Physical audio playback,
low-brightness/projector appearance, and real-device safe areas also remain
unverified.
