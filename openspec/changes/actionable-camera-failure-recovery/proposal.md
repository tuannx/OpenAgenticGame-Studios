# Proposal

The camera adapter correctly fails closed and returns to the launcher when
`getUserMedia()` rejects. The launcher currently reduces every failure to
`Không thể bật camera`. That copy does not tell a family whether to grant
permission, connect a camera, close another camera app, retry, or use the honest
no-score preview. A guide-originated attempt can also return while focus remains
on a now-hidden guide button.

Introduce a pure, cause-aware camera-start failure presenter. Show its one-line
recovery message in a compact same-style status rail, keep the existing full-width
camera retry and guide-only preview actions, and restore focus to the retry CTA on
the next animation frame.

## Non-goals

- Adding a new dialog, action, permission workflow, or touch step.
- Auto-opening browser settings or requesting permission outside user intent.
- Claiming that a permission change can be completed hands-free.
- Changing camera constraints, detector/model lifecycle, pose evaluation,
  scoring, multiplayer, Rust, bridge imports, ABI, or assets.
