# Proposal

## Problem

The guide-only fallback is truthful and uses the selected mode illustration, but
at 390x844 its bottom-aligned card leaves the collapsed Party & settings panel,
inactive camera card, and gameplay canvas visibly competing above it. The same
screen also reintroduces platform-font emoji in the mode title and both actions,
breaking the shape-first language established across launcher and camera setup.

## Proposed change

Make guide-only a single-owner presentation. While its dialog is open, suppress
unrelated technical chrome, center the illustrated card on compact portrait,
use plain mode titles, and replace camera/back emoji glyphs with fixed CSS
geometry. Preserve the dominant 60px camera recovery action, 48px mode-return
action, focus loop, Escape behavior, canonical image source, and honest
non-evaluation copy.

## Why now

M20 made the one required camera touch reachable. The next fallback boundary
must remain equally simple when the player explicitly chooses not to grant
camera access.

## Non-goals

- No scoring, guide-only evaluation, mode, camera, audio, or permission changes.
- No new image, tutorial, state machine, persistence signal, or navigation step.
- No redesign of the technical settings panel outside guide-only presentation.

