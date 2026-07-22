# Proposal

The collapsed `Party & settings` control currently occupies 145–150px on mobile
and presents technical prose precisely when the player should read body-scale
camera actions from a distance. The full panel still contains important camera,
audio, reduced-motion, room, and Studio controls and must remain discoverable.

Replace only the collapsed presentation with a fixed 48×48 sliders shape. When
opened, the native disclosure expands to its existing width, shows a plain
heading, and exposes the unchanged controls. This removes prose from the active
gameplay hierarchy without deleting recovery or accessibility paths.

## Non-goals

- Redesigning, translating, or reordering settings content.
- Changing camera, audio, multiplayer, taste, or Rust runtime behavior.
- Adding touch gestures, assets, JavaScript state, or browser imports.
