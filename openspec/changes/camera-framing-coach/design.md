# Design

## Decision: frame quality is a setup-domain contract

`vision.ts` owns camera acquisition and pose estimation. It must continue to
emit raw, current-frame `PoseSnapshot` values without localized UI policy.
`pose-framing.ts` converts those values into a typed setup assessment and returns
only stable, usable poses to `MotionNavigationController`. Rust remains the
authoritative gameplay evaluation boundary after launch.

## Framing contract

A usable player must satisfy all of the following:

- aggregate pose quality meets the existing `0.25` evaluation floor;
- both shoulders, both hips, the nose, and at least one wrist meet confidence;
- shoulder scale is neither too small nor too large for reliable movement;
- head and hip remain inside safe vertical margins;
- torso center remains inside the horizontal camera safe zone.

The coach reports the highest-actionability failure. It does not retain frames,
landmark coordinates, images, or per-session biometric measurements.

## Stability and Duo behavior

The current framed pose IDs must remain unchanged for 400 ms before setup
navigation receives them. Any framing failure returns an empty navigation input
immediately, resetting lean/raised-hand dwell.

Framing and mode confirmation are separate responsibilities. In Duo, one stable
player is enough to operate lean navigation, but the existing motion-navigation
controller still requires two raised hands to confirm Duo.

## Presentation

The ready status has one owner: typed framing presentation after pose tracking
starts. The camera preview gets a non-interactive reticle whose color reflects
searching/adjusting/steady/ready state and derives from the active theme tokens.
No extra setup page or touch action is introduced.

