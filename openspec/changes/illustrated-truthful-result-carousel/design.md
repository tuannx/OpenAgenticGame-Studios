# Design

## Canonical asset ownership

The browser public directory remains the only source for mode artwork:

- `/assets/game_mode_mirror.jpg`
- `/assets/game_mode_strike.jpg`
- `/assets/game_mode_duo.jpg`

WASM loads these URLs once at composition startup. Native uses absolute paths
derived from `CARGO_MANIFEST_DIR`. Macroquad 0.4.15 does not enable JPEG decoding
by default, so the game crate explicitly enables only `image`'s JPEG feature and
decodes through a fallible boundary before constructing a GPU texture. Image
bytes are not embedded in the Rust binary and no copied result-specific
thumbnails are introduced. The boundary caps encoded bytes, image dimensions,
and decoder allocation before GPU upload. A fetch, decode, or limit failure
produces `None` and the renderer keeps a styled procedural card, so art
availability never becomes a gameplay dependency.

## Presentation contract

Each result choice is an image-backed card with a centered cover crop. A bounded
dark label shelf protects text contrast. Selection uses a thicker outline and a
diamond marker, making it recognizable in grayscale. An unavailable Duo card is
desaturated through tint/dimming and receives a procedural lock glyph plus a
short `P2 NEEDED` label.

The card geometry grows only within the existing result panel. Pure layout tests
cover 390x844, 667x375, and 1440x784. Image crop math accepts numeric source and
destination geometry, so it can be tested without a GPU or browser.

## Capability truth

Result availability is derived from evaluated state for players 0 and 1, the same
frame facts that `RunnerGame::update` uses to decide whether Duo can be selected.
Rendering does not mutate or replace deterministic navigation. If P2 leaves the
frame, Duo looks unavailable; when P2 returns, it becomes available again.

## Performance and failure behavior

- Three textures load once before the frame loop and are borrowed while drawing.
- No image decoding, path construction, collections, or texture allocation occur
  per frame.
- Existing launcher requests make browser cache reuse likely, but correctness
  does not depend on cache timing.
- The release WASM contains no JPG bytes, but the explicit JPEG decoder is still
  code. Measurement increased raw WASM from 660,409 to 948,983 bytes (+43.7%);
  the final artifact is 310,703 bytes under gzip. This remains below 1 MB raw and
  avoids adding three duplicate PNG/TGA transfers, so the measured tradeoff is
  accepted for the user's explicit same-style image priority.

## Validation impact

This changes Macroquad rendering and browser asset consumption but not the ABI.
Run strict Rust checks, tests, release WASM build, production web build, an
HTTP-served asset/MIME smoke test, and browser console/network inspection. A real
camera-backed result state is still required for physical-distance visual proof.
