# Design

## Quiet disclosure

The native `details` and `summary` remain the semantic and keyboard owners. The
summary contains a semantic label plus a decorative CSS sliders glyph. In the
closed state, the label is visually hidden and the disclosure is exactly 48px
square. In the open state, the panel returns to its responsive full width and
the same label becomes visible beside the glyph.

The glyph uses CSS lines and dots instead of an emoji or external icon so it is
stable across platform fonts and adds no asset request. It is decorative only;
the summary's text remains its accessible name.

## Invariants

- The closed target is 48×48 at desktop, compact portrait, and short landscape.
- The open panel retains its current 230px responsive width and scroll behavior.
- Camera, audio, opacity, theme, reduced motion, room, and Studio controls remain
  present with unchanged identifiers and event owners.
- Native Enter/Space disclosure, pointer activation, and focus-visible outline
  remain available.
- Guide-only continues to hide the entire panel while its modal owns the screen.
- No JavaScript, asset, camera, scoring, multiplayer, persistence, Rust, or ABI
  behavior changes.

## Validation package

Run strict TypeScript/tests, native Rust and release WASM gates, production build,
HTTP MIME probes, and real production-CSS browser QA at 390×844 and 667×375.
Evidence must cover closed dimensions, open content, pointer and keyboard toggle,
focus outline availability, no clipping, and no application-origin warnings or
errors.
