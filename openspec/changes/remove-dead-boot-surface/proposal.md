# Proposal

The launcher already covers the viewport at z-index 20 and hides the game canvas
while Macroquad loads. A legacy `#boot-screen` remains behind it at z-index 10.
It is therefore not a useful visual loading surface, but Chrome exposes its
`Neon Beat Runner` heading and `Charging the rhythm highway…` copy alongside the
launcher in the accessibility snapshot both before and after its 900ms opacity
transition.

Remove the redundant markup, styling, and timeout. Keep the first-paint launcher,
canvas layout box, WASM load call, explicit capability laziness, and all shell
owner transitions unchanged.

## Non-goals

- Introducing a new loading/progress screen or waiting state.
- Delaying launcher interaction until WASM completion.
- Changing launcher art, copy, actions, camera/audio intent, or taste behavior.
- Changing Rust, Macroquad loading, bridge imports, ABI, evaluation, or assets.
