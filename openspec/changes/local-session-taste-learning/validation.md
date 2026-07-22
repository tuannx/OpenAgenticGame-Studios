# Validation

## Automated evidence

- `npm run typecheck` — strict TypeScript passed after final UI refinement.
- `npm test` — 16 tests passed across 5 files; 6 cover taste parsing,
  bounds, recommendation hysteresis, full-guide recovery, privacy-shaped
  serialization, and storage failure.
- `npm run build:web` — release WASM and Vite production bundle passed.
- `bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak`
  — all four gates passed: 19 Rust tests, 16 web tests, release WASM/web build,
  dependency audit with zero vulnerabilities, and loader validation.
- The remaining Vite size warning is the existing dynamically imported vision
  chunk; it is not loaded on the launcher critical path.

## HTTP and browser evidence

- Preview responses were correct: HTML `text/html`, card art `image/jpeg`, and
  fingerprinted WASM `application/wasm`.
- First-time desktop at 1440×784: Random selected, full setup, 980×661 launcher,
  no body overflow, and the local-only note visible.
- Returning desktop: two UI-driven Mirror guide-only starts selected Mirror on
  reload and changed the CTA to `TIẾP TỤC MIRROR BEAT`; the most recent
  guide-only outcome correctly kept `data-taste="full"`.
- Visual theme `purple-vaporwave` and overlay `hologram` persisted across reload.
- Changing from the learned Mirror preference to Strike immediately restored
  `BẬT CAMERA` and updated the privacy/taste note to the current selection.
- Phone portrait at 390×844: 366×731 launcher, two-column card grid, no body
  overflow, and all setup/CTA/privacy content remained visible.
- Keyboard ArrowRight moved checked state and roving focus from Strike to Duo,
  with `tabindex="0"` on the selected card.
- The app origin emitted no browser error or warning. Observed warnings belonged
  to an unrelated Chrome extension content script.

## Sequential persona review

### Security and privacy

- Local storage is versioned, bounded, schema-checked, and exception-safe.
- The constant-size serialized object has no pose, keypoint, frame, image, or
  video field; there is no network path in the adapter.
- Corrupt and relationally inconsistent counters normalize to safe defaults.

### Performance

- Serialization happens only at session/setup/settings boundaries, never per
  camera frame or render tick.
- The profile is constant-size, and recommendation is a fixed three-mode scan.

### Readability and architecture

- Pure reducers and recommendation policy live in `taste-profile.ts`; DOM and
  browser storage remain at the capability edge in `main.ts`.
- Recommendation thresholds and bounds are named constants and covered by tests.

### Runtime regression

- M2 does not change authoritative Rust judgment or bridge ABI.
- Guide-only still disables evaluation, and touch fallback remains explicit.
- Native Rust, release WASM, browser build, responsive layout, persistence, and
  keyboard fallback were independently checked.

## Residual physical-camera gap

Camera permission was deliberately not accepted in browser automation. Real
lean navigation, one-second raised-hand confirmation, compact setup after two
physical gesture successes, mirrored overlay alignment, two-player readiness,
and audio/camera unlock feel still need an attended real-camera session. These
surfaces are not inferred from unit or build success.
