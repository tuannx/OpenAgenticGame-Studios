# Proposal

The launcher correctly gives camera play one dominant full-width action and one
smaller guide-only route. Production CSS currently renders that fallback at
11px, weight 400, `#64748b`, and 0.76 element opacity. Against the two endpoints
of the launcher-card gradient, its effective contrast is only about 2.64-2.80:1.
Responsive screenshots confirm that the action is nearly invisible even though
it is the only honest route for camera denial, missing hardware, or a user who
wants to preview before granting permission.

Keep one secondary action and its existing 48px target, eye shape, focus path,
and guide-only behavior. Raise text to 12px/700 with an opaque muted token that
clears 4.5:1 across the card gradient. Make the label state the consequence in
one line: demo, no camera, no score.

## Non-goals

- Adding a second fallback, another disclosure, or another touch step.
- Making guide-only visually equal to or stronger than camera play.
- Changing guide-demo ownership, audio unlock, camera/vision loading, scoring,
  pose evaluation, multiplayer, taste persistence, Rust, bridge, or ABI.
- Treating the stale boot overlay as a user-visible screen; first-paint evidence
  shows the launcher sits above it for its complete visible lifetime.
