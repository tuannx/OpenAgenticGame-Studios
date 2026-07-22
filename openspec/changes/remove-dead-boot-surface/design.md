# Design

## One first-paint owner

The existing `launcher-open` HTML class, launcher dialog, inert canvas/settings,
and launcher-owned canvas visibility already form the first-paint boundary. The
boot screen cannot sit above that boundary without becoming a second visible
screen, and cannot sit below it without becoming dead visual UI. Removing it is
the smallest coherent expression of the current product contract.

The document title remains `Neon Beat Runner • BrainBreak`; only the duplicate
body heading/status are removed. `window.load(__BRAINBREAK_WASM_PATH__)` remains
the sole Macroquad startup call.

## Invariants

- Initial HTML still opens directly on the illustrated launcher.
- `#glcanvas` keeps its real full-viewport layout box and first-paint inert state.
- The production bundle still requests the fingerprinted WASM and local loader.
- Launcher keyboard order, guide transition, and lazy vision/audio behavior do
  not change.
- No camera, detector, Rust, bridge import, ABI, evaluation, or asset change.

## Validation package

Add a static entry contract test, run strict TypeScript and focused web tests,
build production, then inspect immediate and settled HTTP-served accessibility
snapshots plus launcher keyboard/visual behavior in Chrome. Close with the full
native/WASM/web gate, MIME/fingerprint checks, and release size evidence.
