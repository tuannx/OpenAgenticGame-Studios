# Validation

## Automated evidence

Passed on the final tree on 2026-07-21:

- Strict TypeScript typecheck and 54 Vitest tests across 11 files.
- Strict Rust formatting/Clippy, 41 deterministic core tests, and 22 game/layout
  tests remain green.
- Release `wasm32-unknown-unknown`, production Vite build, Macroquad loader
  syntax/patch checks, and `git diff --check` passed.

The complete BrainBreak validator passed Rust/native/WASM, TypeScript, all tests,
and the production build, then stopped at the unchanged dependency audit. The
`wrangler -> miniflare -> sharp` chain reports three inherited high findings;
the offered forced fix installs an incompatible Wrangler version and was not
applied in this presentation milestone.

## Ownership and license review

`#music-credit` is now the final section of `.control-grid`, whose native
`details` owner is `#control-panel`. `#track-status` remains unchanged and the
production entry booted with `Special Spotlight • 126 BPM`, proving the existing
non-null DOM query still resolves.

The artist and CC BY 4.0 links retain their authoritative URLs, `_blank` targets,
and `noreferrer`. The track title, BPM, artist, license, and `web-compressed`
notice all remain visible in the expanded panel. The audio button, `audio.ts`,
fingerprinted MP3, attribution source, playback rate, beat timing, and feedback
engine are unchanged.

## Release and HTTP evidence

- WASM remains byte-identical to M22:
  `brainbreak-game-425aa5274bb6.wasm`, 954,575 bytes raw / 313,033 gzip.
- Final entry HTML is 10,620 bytes raw / 3,442 gzip; game JS is 33,887 raw /
  11,863 gzip; CSS is 30,323 raw / 7,280 gzip.
- Fingerprinted audio remains
  `special-spotlight-db0e06528b9a.mp3`, 3,079,985 bytes raw / 3,069,365 gzip.
- Built `audio/ATTRIBUTION.md` is 869 bytes raw / 534 gzip and has the same
  SHA-256 as its source:
  `f04d45438657db9b3f118dfddcbc3ed2ba9c2c511f4db92f7629cfc2e75d0b3b`.
- Explicit IPv4 HTTP returned `text/html`, `application/wasm`,
  `text/javascript`, `text/css`, `audio/mpeg`, and `text/markdown` for the final
  entry, WASM, game script, stylesheet, MP3, and attribution document.

## Browser and responsive evidence

Production CSS and the real production entry were served over explicit IPv4
HTTP and exercised at 390x844 and 667x375:

- Before M23, portrait rendered a standalone 190x52.5 credit at the bottom-right
  whether settings were closed or open; short landscape hid it entirely.
- Final native visibility was false with settings closed in both viewports and
  true with settings open. The only closed technical chrome is the existing
  48x48 settings shape.
- Portrait open rendered the complete 206x89 attribution inside the panel with
  no vertical scroll. Short-landscape rendered the complete 191x89 attribution
  inside the same 230px panel using bounded grid scroll: 253px client height for
  643px content with `overflow-y: auto`.
- Both artist and license links measured 44px tall. The semantic snapshot exposed
  one named `Music attribution` region, track status, both links, and the
  modification notice only in the open states.
- Pointer opened the native disclosure; Enter closed it; Space reopened it.
  No JavaScript disclosure state was added.
- The real entry preserved exact source/license URLs, `_blank`, `noreferrer`,
  and `#track-status` ownership. Application-origin warning/error filtering was
  empty for both the entry and responsive fixture.

Temporary QA fixtures were removed after evidence capture.

## Persona review

- **Security/privacy:** no camera, pose, storage, network, or evaluation boundary
  changed. External links retain the existing safe new-tab relationship.
- **Performance:** the floating composited/backdrop-filtered element and three
  responsive overrides were removed. No asset, listener, state machine, audio
  node, or frame-loop work was added.
- **Readability/accessibility:** one technical surface now owns controls and
  attribution; the credit keeps a named region, complete text, 44px links,
  native keyboard access, and responsive scroll.
- **Runtime regression:** audio status lookup, source/license evidence, playback,
  launcher, Ready, guide-only, camera, taste, multiplayer, Rust, and ABI owners
  remain intact.

## Residual attended QA

Camera permission and audio playback were not started. A physical session must
still verify screen-reader announcement, device safe areas, external-link opening,
audio unlock/playback, and target-device readability. HTTP audio delivery and
unchanged engine source do not prove attended playback or audible mix quality.
