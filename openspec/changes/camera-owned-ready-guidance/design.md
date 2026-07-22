# Design

## One camera-owned live status

Remove the Ready-card status badge and make `#camera-status` the sole polite
status node. Before tracking, it shows the actionable setup copy already emitted
by `presentVisionStatus`; technical backend copy remains a fallback only when no
action copy exists. Once tracking begins, `updateReadyNavigation()` owns the
camera status so framing correction, mode-change feedback, and hand-hold state
cannot be overwritten by the later generic tracking callback.

Outside Ready, normal camera/gameplay status continues to update. Startup
failure still returns to the launcher and uses the existing recovery rail.

## Spatial hierarchy

During Ready, enlarge and wrap camera status over the actual preview while
keeping `LOCAL ONLY` visible. Canonical mode art becomes a clean identity image
without a conflicting instruction. On desktop/tablet landscapes taller than the
compact breakpoint, compute matching left/right gutters from a bounded 1180px
stage so the Ready card and camera preview remain a coherent pair. Portrait and
short-landscape ownership regions remain stacked/split as before.

## Invariants

- One visible/live setup status exists at every Ready viewport.
- The 667x375 path retains actionable framing copy even though canonical art is
  hidden for height.
- Pose-driven navigation status wins after tracking begins.
- Camera/audio still begin only after explicit intent.
- No new action, listener, timer, persistent field, asset, Rust change, bridge
  import, or ABI version change.

## Validation package

Lock the single status owner in the entry contract and the tracking ownership
policy in a pure presenter test. Exercise production builds with a same-origin
synthetic MediaStream at desktop, 390x844, and 667x375. Measure status visibility,
text, camera/card geometry, status containment, resource requests, and logs;
then run the complete native/WASM/web/HTTP gate.
