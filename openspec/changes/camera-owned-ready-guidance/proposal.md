# Proposal

The camera-intent path reaches the real MoveNet loop safely, but the Ready screen
splits one physical correction across two status surfaces. The actionable
`BƯỚC VÀO KHUNG HÌNH` live region is drawn over canonical game art while the
actual mirrored preview says only `Đang tìm người chơi`. At 667x375 the art is
intentionally hidden, so the actionable status has a zero-size box and the only
visible camera text is generic. On very wide screens the two surfaces are also
separated by more than a thousand pixels.

Make the real preview the only visible/live status owner during camera setup.
Preserve canonical mode art as identity, keep the existing shape-first action
compass, and bound the wide-screen Ready/camera pair to one centered stage.

## Non-goals

- Changing camera constraints, model choice, pose recognition, framing policy,
  navigation thresholds, evaluation, scoring, multiplayer, Rust, bridge ABI,
  audio, storage, or assets.
- Adding a tutorial, dialog, button, touch step, or new image.
- Claiming synthetic video proves physical camera alignment or room-distance
  readability.
