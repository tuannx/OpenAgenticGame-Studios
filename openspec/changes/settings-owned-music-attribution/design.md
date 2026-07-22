# Design

## One technical owner

`#music-credit` moves from a fixed sibling of `#control-panel` to the final
section of `.control-grid`. Its `#track-status` identifier stays stable for the
existing audio presentation write in `main.ts`. Its source and license links,
new-tab behavior, `rel=noreferrer`, and modification notice remain unchanged.

The attribution becomes a compact, left-aligned section separated by a quiet
top rule. It adds no nested disclosure: one native settings toggle owns all
advanced controls and release metadata. When settings are closed, neither the
credit nor any other control-grid content enters the gameplay hierarchy.

## Invariants

- Expanded settings show title, 126 BPM, artist, CC BY 4.0, and web-compressed
  notice with the original authoritative URLs.
- `web/public/audio/ATTRIBUTION.md`, the fingerprinted MP3, and `audio.ts` remain
  unchanged.
- The existing audio button and `#track-status` query continue to resolve.
- The 48×48 collapsed settings target, native pointer/keyboard path, guide-only
  ownership, and short-landscape bounded scroll remain intact.
- No scoring, camera, pose, multiplayer, taste persistence, Rust, bridge, or ABI
  behavior changes.

## Validation package

Run strict TypeScript/tests, native Rust and release WASM gates, production build,
HTTP MIME probes for HTML/WASM/JS/CSS/audio, and real production-CSS browser QA
at 390×844 and 667×375. Evidence must show no standalone credit while settings
are closed, full linked attribution while open, bounded landscape scrolling,
native keyboard access, and no application-origin warnings or errors.
