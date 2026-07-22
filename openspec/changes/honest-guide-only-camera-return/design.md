# Design

## Ownership

The browser owns capability explanation and camera permission, so the demo
dialog belongs in `index.html`, `main.ts`, and `style.css`. Rust continues to
enforce that guide-only input cannot change score, collision judgment, combo, or
multiplayer emission. No JavaScript/WASM import changes, so bridge ABI stays v5.

## State flow

```text
launcher --demo--> guide demo --camera--> ready/camera setup
                         |
                         +--back--> launcher with resolved mode selected
```

Opening the demo resolves Random once, synchronizes the selected Rust mode,
records the existing aggregate guide-only outcome, and enables guide-only
safety. Camera return hides the demo and clears guide-only presentation before
requesting camera. A camera error follows the existing recovery path back to the
launcher.

## Presentation

The card reuses the active mode artwork rather than adding another visual
language. Copy is deliberately short: demo status, mode title, and one safety
sentence. The primary camera button is full-width and at least 60 px tall. The
back action is visually quiet but remains at least 48 px tall and keyboard-
focusable.

The dialog sits above the settings, warning, camera PIP, and canvas so the user
does not need to discover a hidden control. Portrait uses a bottom-aligned card;
landscape uses a compact image-and-action grid.
