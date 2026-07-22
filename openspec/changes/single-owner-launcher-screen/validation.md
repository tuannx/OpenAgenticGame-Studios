# Validation

## Pre-change evidence

- A fresh explicit-IPv4 origin remained on the launcher for more than six
  seconds without requesting the vision chunk or MP3. The earlier delayed
  request was a prior QA-input artifact, not application auto-start.
- The same fresh entry visibly leaked the collapsed settings control and camera
  card behind the launcher.
- Eight successive Tab inputs all left `document.activeElement` on
  `#glcanvas`, proving the declared modal did not own keyboard interaction.
- Runtime inspection also found that this Chrome surface honors the `inert`
  attribute for focus exclusion without exposing the optional DOM property; the
  owner therefore uses `toggleAttribute` so gameplay can reliably remove the
  first-paint boundary.
- Responsive production screenshots exposed faint runtime labels through the
  translucent launcher even after technical chrome was removed. Launcher-owned
  CSS therefore hides canvas visibility without removing its layout box.

## Automated evidence

Passed on the final tree on 2026-07-21:

- Strict TypeScript typecheck and 58 Vitest tests across 12 files. Four new
  owner-invariant cases prove each pre-game state exposes exactly one modal and
  gameplay exposes none.
- Strict Rust formatting/Clippy, 41 deterministic core tests, and 22 game/layout
  tests remain green.
- Release `wasm32-unknown-unknown`, production Vite build, Macroquad loader
  syntax/patch checks, and `git diff --check` passed.

The complete BrainBreak validator passed Rust/native/WASM, TypeScript, all tests,
and the production build, then stopped at the unchanged dependency audit. The
`wrangler -> miniflare -> sharp` chain reports three inherited high findings;
the offered forced fix installs incompatible Wrangler 4.15.2 and was not applied.

## Ownership and interaction evidence

`presentShellOwner` maps launcher, Ready, guide, and gameplay to one modal and
one background policy. `setShellOwner` is now the only writer for the three gate
visibility classes and their body presentation classes. Initial HTML carries
`launcher-open` plus inert canvas/settings attributes so first paint and runtime
state agree.

The final real production entry reported exactly one visible dialog,
`#motion-gate`. The gameplay canvas was `visibility:hidden` and inert; settings
were hidden and inert; the camera card was hidden. The canvas layout box remains
present for Macroquad initialization and is restored outside launcher ownership.

Before M24, eight consecutive Tab inputs stayed on `#glcanvas`. Final input QA
cycled selected mode -> camera CTA -> guide-only -> selected mode; Shift+Tab from
the selected mode returned to guide-only. ArrowRight moved both focus and checked
radio state from Random to Mirror. Guide-only opened with camera return focused,
cycled camera/back/camera, and Escape restored launcher ownership with the
selected mode focused. No background control entered either loop.

## Resource and runtime evidence

A fresh-origin launcher remained idle for more than six seconds without a
vision chunk or MP3 request. Explicit guide-only intent loaded the existing MP3
but no vision chunk, proving the camera boundary stayed lazy. The earlier M23
delayed request was therefore a QA-input artifact rather than application
auto-start.

Application-origin log filtering contained no warnings or errors. The only
origin messages were the existing Macroquad informational notices about unused
loader plugins; Chrome-extension listener warnings were excluded from app
evidence.

## Responsive visual evidence

The real production entry was rendered at 390x844 and 667x375 through same-origin
QA frames:

- Portrait retained the two-by-two canonical mode art grid, placement image,
  dominant camera CTA, no-touch promise, local taste note, and quiet demo route
  inside one bounded launcher card.
- Short landscape retained the four-art row, dominant camera CTA, promise,
  taste note, and demo route without technical chrome or runtime copy competing
  through the scrim.
- Both final captures removed camera/settings bleed and the faint underlying
  `MIRROR BEAT / GUIDANCE ONLY` canvas labels. No new asset or touch step was
  introduced.

The temporary responsive QA page was removed after evidence capture.

## Release and HTTP evidence

- WASM remains `brainbreak-game-425aa5274bb6.wasm`, 954,575 bytes raw / 313,033
  gzip; its full SHA-256 still starts with the filename fingerprint.
- Final entry HTML is 10,654 bytes raw / 3,459 gzip; game JS is 34,142 raw /
  12,036 gzip; CSS is 30,449 raw / 7,313 gzip.
- Fingerprinted audio remains
  `special-spotlight-db0e06528b9a.mp3`, 3,079,985 bytes raw.
- Built and source attribution documents retain identical SHA-256
  `f04d45438657db9b3f118dfddcbc3ed2ba9c2c511f4db92f7629cfc2e75d0b3b`.
- Explicit IPv4 HTTP returned `text/html`, `application/wasm`,
  `text/javascript`, `text/css`, `audio/mpeg`, and `text/markdown` for the final
  entry, WASM, game script, stylesheet, MP3, and attribution document.

## Persona review

- **Security/privacy:** no new camera, audio, storage, network, or pose path was
  added. Fresh idle remains capability-free and guide-only still cannot load
  vision.
- **Performance:** one four-value pure mapping and bounded DOM attribute/class
  writes replace scattered mutations. No frame-loop, asset, detector, or WASM
  work was added; hidden canvas keeps its rendering surface dimensions.
- **Readability/accessibility:** visual and keyboard modality now agree. The
  illustrated launcher owns the viewport, roving radio navigation remains
  intact, and both forward/reverse Tab loops exclude background chrome.
- **Runtime regression:** Ready retains its camera surface; guide retains its
  truthful two-action loop; confirmed gameplay is the only owner that removes
  background inertness. Scoring, collision, multiplayer, taste, Rust, bridge,
  and ABI owners are unchanged.

## Residual attended QA

Camera permission and physical audio playback were not started. A real device
session must still verify camera-to-Ready-to-gameplay restoration, canvas sizing
after permission, screen-reader modal announcement, device safe areas, audible
mix, and focus behavior in the target assistive technologies. Build, HTTP, and
headful keyboard evidence do not prove those attended surfaces.
