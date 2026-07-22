# Design

## Single presentation owner

`body.guide-demo-open` already marks the exact dialog lifetime. CSS uses that
existing state to hide the control panel, inactive camera card, and music credit
while the guide dialog owns the viewport. The canvas may remain as a dimmed
ambient background under the dialog scrim, but no competing action or status
chrome remains visible or focusable through the modal.

The compact portrait rule centers the guide card instead of pinning it to the
bottom. Short landscape keeps its current two-column art/action composition.

## Shape and text language

Game-mode presentation titles become plain text. `modeLabel` no longer strips a
trailing emoji and instead uppercases the canonical plain title. The guide camera
and return actions retain semantic button labels while fixed CSS camera and
back-chevron geometry provide first-glance shapes independent of platform fonts.

## Invariants

- Guide-only continues to set Rust evaluation false and cannot score, collide,
  combo, or emit multiplayer actions.
- Camera start still requires the existing explicit button gesture.
- The same canonical mode asset URL is reused; no bytes or duplicate art are
  added.
- Focus remains trapped between the two guide actions and Escape returns to
  mode selection.
- The camera action remains at least 60px and the return action at least 48px.

## Validation package

Run strict TypeScript/tests, native Rust and release WASM gates, production build,
HTTP MIME probes, and real CSS browser QA at 390x844 and 667x375. Browser evidence
must show no control/settings/camera/music chrome while the guide dialog is open,
both actions visible, and no application-origin warnings or errors.

