# Design

## One shell owner

A small pure presentation function maps `launcher | ready | guide | gameplay`
to exactly one visible modal and a background-interaction policy. `main.ts`
applies that result in one place instead of independently mutating three modal
classes and three body classes across event handlers.

- Launcher: launcher visible; gameplay canvas, camera card, and settings hidden;
  canvas and settings inert. Canvas visibility preserves its layout box so the
  Macroquad runtime never boots against a `display:none` zero-sized surface.
- Ready: Ready visible; camera card remains the live framing surface; canvas and
  settings inert.
- Guide: truthful guide-only demo visible; camera card and settings hidden;
  canvas and settings inert.
- Gameplay: all modal gates hidden; canvas and settings interactive.

`body.launcher-open`, plus initial `inert` attributes on the canvas and settings,
provides the same boundary before JavaScript finishes booting. Runtime ownership
uses attribute toggles rather than relying on the optional DOM `inert` property,
then becomes authoritative. The launcher closes its three-stop Tab loop without
adding another visible control.

## Invariants

- Exactly one modal gate is visible in each pre-game owner state.
- Canvas and optional settings are inert for every modal owner, not only the
  launcher.
- The launcher hides camera/settings technical chrome without obscuring its
  same-style illustrations or changing its no-touch promise.
- Ready retains the camera framing surface and guide retains its two-action
  focus loop.
- Returning, camera failure, camera stop, and setup exit restore launcher
  ownership; confirmed play restores gameplay ownership.
- Fresh-origin idle does not request vision, camera, or audio.
- No scoring, collision, pose, multiplayer, persistence, Rust, bridge, or ABI
  behavior changes.

## Validation package

Unit-test every owner mapping, then run strict TypeScript/tests, native Rust and
release WASM gates, production build, HTTP MIME probes, and the real production
entry in Chrome. Browser evidence must cover first paint, pointer/keyboard mode
selection, launcher Tab order, launcher-to-guide-to-launcher transitions, and
the absence of background focus or app-origin warnings/errors. Physical camera
and audio remain an explicit attended-QA gap.
