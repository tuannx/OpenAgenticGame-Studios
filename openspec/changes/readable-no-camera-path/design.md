# Design

## Honest secondary hierarchy

The launcher retains exactly three keyboard stops: selected game, dominant
camera CTA, and guide-only fallback. The fallback keeps a transparent background,
compact width, underline, geometric eye, and 48px height. These shape and area
differences preserve camera-first hierarchy without using illegibility as the
hierarchy mechanism.

The final muted foreground is `#94a3b8` at opacity 1, 12px, weight 700. Static
sRGB calculation gives at least 6.87:1 against both opaque endpoints of the
launcher-card gradient (`#0a0e28` and `#210c40`). Hover may brighten the same
action; keyboard focus continues to use the global cyan outline.

The label becomes `Xem demo không camera • không điểm`. It names both the mode
and its evaluation consequence before the user enters the guide-only screen.

## Invariants

- Camera remains the only dominant full-width action and the only path that can
  request vision/camera permission.
- Guide-only stays one 48px secondary target with no nested disclosure.
- The label exposes no-camera and no-score truth before navigation.
- Pointer, forward/reverse Tab, Enter, guide focus loop, and Escape return remain
  unchanged.
- Fresh-origin idle still requests neither vision nor audio; explicit guide-only
  intent may start the existing audio path but must not load vision.
- No camera, audio, scoring, pose, multiplayer, taste, Rust, bridge, or ABI
  behavior changes.

## Validation package

Run strict TypeScript and web tests, production build, native Rust/release WASM
gates, HTTP MIME probes, and the real production entry in Chrome. Browser
evidence must cover computed typography/opacity, target size, contrast math,
desktop/390x844/667x375 layout, keyboard loops, guide transition, lazy resources,
and application-origin logs.
