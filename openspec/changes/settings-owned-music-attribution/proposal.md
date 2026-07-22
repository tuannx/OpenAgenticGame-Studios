# Proposal

The music credit currently floats independently at the bottom-right of portrait
gameplay, occupying 190×52.5px even while settings are closed. Short landscape
hides it entirely. This both competes with camera-distance gameplay and gives the
same required release metadata inconsistent responsive behavior.

Move the existing attribution element into the existing technical control grid.
The collapsed 48px settings shape remains the only optional technical entry
point. Opening settings reveals the unchanged track title, BPM, artist link,
CC BY 4.0 link, and web-compression notice alongside the audio control.

## Non-goals

- Changing, replacing, remixing, or re-encoding the licensed audio.
- Changing music startup, playback, mute, beat timing, or feedback synthesis.
- Removing attribution evidence or weakening external-link safety.
- Adding a new disclosure, touch step, animation, asset, or JavaScript state.
