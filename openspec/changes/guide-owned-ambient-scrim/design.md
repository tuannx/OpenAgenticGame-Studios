# Design

## Ambient, not instructional

`#guide-demo-gate` already owns a radial scrim centered near the neon horizon.
Its cyan center remains translucent so the sun and upper lane can read as ambient
art around the selected canonical illustration. The outer `rgba(3,5,20,.94)` stop
becomes opaque `#030514` at 52%, and its geometry becomes an ellipse so the
horizontal horizon stays broad while the lower translucent band ends before the
player silhouettes.

A second vertical layer stays transparent through 48% and reaches opaque at 78%.
It leaves the horizon and upper lane intact, then closes the lower-third reveal
where player heads and HUD silhouettes sit. The opaque guide card covers the
transition band.

A symmetric horizontal layer is opaque through the outer 30% on each side and
opens between 44% and 56%. This suppresses corner runtime labels on narrow
viewports while keeping the centered horizon available as ambience.

Returning to the launcher restores focus on the next animation frame. Its mode
cards have just changed from `display: none`, so same-tick focus is not reliable
in a real browser even though the target card already exists.

This preserves the existing art direction without making the WebGL canvas a
second content owner. Corner status labels and lower player/HUD silhouettes sit
in the opaque bands. The guide card itself is unchanged.

## Invariants

- The canvas remains laid out, rendering, inert, and available for the existing
  guide-to-Ready transition.
- The canonical selected-mode image remains the dominant same-style visual.
- Exactly two guide actions remain: 60px camera recovery and 48px mode return.
- Forward/reverse Tab stays contained, and Escape/action return restores focus
  to the selected launcher card.
- Guide-only remains non-evaluated and cannot score, collide, combo, or emit
  multiplayer actions.
- Explicit guide intent may unlock existing audio but must not load vision.
- No asset, state, Rust, bridge import, or ABI change is introduced.

## Validation package

Run strict TypeScript/tests, production build, native Rust/release WASM gates,
HTTP MIME probes, and the real production guide screen in Chrome. Browser
evidence must show central ambience retained, outer runtime labels/silhouettes
suppressed, both actions visible at desktop/390x844/667x375, keyboard recovery,
lazy vision behavior, and no application-origin warnings/errors.
