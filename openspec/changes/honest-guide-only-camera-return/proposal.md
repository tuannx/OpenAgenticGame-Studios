# Proposal

The launcher currently offers `Chơi thử không camera`, hides itself, and enters
the Rust Ready phase with evaluation disabled. The result is a canvas message
asking for camera while the actual camera action is hidden inside a collapsed
settings panel. On a 390×844 phone, there is no visible primary action.

Replace this with a DOM-owned demo handoff. The deterministic runtime remains in
guide-only safety, while the browser presents the selected mode illustration and
an honest next choice. This does not make guide-only gameplay scoreable; it makes
the fallback understandable and recoverable.
