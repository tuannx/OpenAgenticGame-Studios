# Proposal

M21 made guide-only a single-owner dialog for settings, camera, attribution, and
keyboard focus. The production screen still leaves the inert Macroquad canvas at
full visibility under a scrim whose outer stop is only 94% opaque. As a result,
runtime `BEAT STRIKE` and `GUIDANCE ONLY` labels remain visible in the top corners,
with player silhouettes below the dialog. Those are status signals outside the
dialog and contradict the existing single-owner contract.

Keep the canvas and its central neon sun/lane as ambient context, but make the
outer stop of the existing radial scrim fully opaque. The guide card, canonical
mode illustration, limitation truth, camera recovery, and mode return remain the
only readable message/action layer.

## Non-goals

- Hiding or stopping the Macroquad canvas, changing its render state, or adding a
  Rust/bridge “guide backdrop” mode.
- Changing the guide illustration, title, truth copy, action labels, targets,
  action labels, target sizes, audio behavior, or camera transition. The existing
  return-focus intent may be repaired if browser evidence proves it unreliable.
- Changing scoring, collision, guide-only evaluation, pose, multiplayer, taste
  persistence, Rust, bridge imports, or ABI.
- Replacing the ambient scene with a new asset, animation, or touch step.
