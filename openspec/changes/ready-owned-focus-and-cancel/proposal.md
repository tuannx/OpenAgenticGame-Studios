# Proposal

The illustrated Ready screen owns the visual shell after camera intent, but it
does not own focus. The launcher camera action becomes hidden/disabled and the
accessibility snapshot exposes no active control inside Ready. Escape is also a
no-op, so keyboard users cannot leave the modal while permission or tracking is
active.

Move focus to Ready when it becomes the shell owner, contain forward/reverse Tab
inside its currently visible actions, and make Escape cancel camera startup,
stop any late stream, return to the launcher, and restore focus to the camera
action.

## Non-goals

- Adding a visible close button, tutorial, touch step, image, timer, or setting.
- Changing pose recognition, framing/navigation thresholds, evaluation,
  scoring, multiplayer, Rust, bridge imports, ABI, audio, or storage.
- Claiming a synthetic stream proves operating-system permission UI or
  screen-reader announcement timing.
