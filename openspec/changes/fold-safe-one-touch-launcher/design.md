# Design

## Responsive ownership

The default desktop and portrait launcher keep their current illustrated grid
and placement guide. Only `max-height: 620px` landscape receives the compact
composition. That composition overrides the mobile two-column rule with four
equal columns, so the selected state and every canonical image remain visible
without introducing a carousel or pagination.

The camera CTA remains the dominant full-width control. The zero-touch promise,
local-only privacy statement, error live region, and guide-only action retain
their DOM order and semantics. Compact rules reduce spacing and illustration
height; they do not hide truthful state.

## Invariants

- The launcher never starts camera or audio without the existing explicit CTA.
- All available modes remain keyboard-selectable and capability filtering is
  unchanged.
- Duo unavailability remains visible through the existing card state.
- Canonical image URLs and initial asset inventory are unchanged.
- The primary CTA and secondary guide-only escape are both visible at 667x375
  without scrolling.

## Evidence

Use the production bundle through HTTP in real CSS viewports at 390x844 and
667x375. Visual evidence must show the complete primary action region and no
card/CTA overlap. TypeScript, native Rust, release WASM, and the full BrainBreak
gate protect the unchanged cross-layer contracts.

