# Proposal

At 390x844 Running, the centered 280px Macroquad action beacon occupies roughly
`x=55..335`, while the 100px DOM camera PIP occupies `x=276..376` on the same
top row. The optional 48px settings trigger also touches the beacon at the left
edge. The result hides part of the lane diagram and makes three unrelated owners
compete exactly where the player must read the next body action.

Give narrow portrait one explicit top-overlay hierarchy. Keep the camera PIP in
its truthful top-right position, fit the action beacon into the remaining left
corridor using its existing compact presentation, and move the optional settings
trigger below the cue only while the gameplay shell owns the page.

## Non-goals

- Changing camera framing, inference, pose evaluation, scoring, obstacle timing,
  action semantics, HUD deck, assets, audio, networking, or Rust/JavaScript ABI.
- Hiding the live camera preview, shrinking touch targets, or adding a close,
  collapse, tutorial, preference, or touch step.
- Moving the cue based on a transient obstacle with DOM measurement each frame.
- Claiming synthetic camera input proves room-distance readability or physical
  gesture timing.
