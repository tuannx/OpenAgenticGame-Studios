# Proposal

The current mobile vision profile runs MoveNet SinglePose with `maxPoses: 1`,
while the launcher, Random choice, lean navigation, bridge, and Rust result flow
still expose Duo. This creates an unreachable success path: the UI promises two-
player evaluation that the active detector cannot produce.

The camera card also forces every stream into a landscape 4:3 box with `cover`.
A portrait mobile stream is therefore cropped before the framing coach evaluates
what the player sees, weakening the truthfulness of its guidance.

Introduce a small capability value before loading TensorFlow, propagate its
local pose capacity to every selection boundary, and let intrinsic video
geometry own the preview aspect. The one-pose profile fails closed. Duo can be
restored only by selecting a measured two-pose capability, not by viewport size.
