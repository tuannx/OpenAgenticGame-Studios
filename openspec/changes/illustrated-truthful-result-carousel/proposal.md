# Proposal

## Problem

The launcher and camera setup consistently use a neon illustrated mode-art set,
but the deterministic result screen falls back to three flat text chips. The
visual discontinuity weakens mode recognition precisely where a player is
standing far away and deciding whether to replay or switch. On a one-player
result, Duo is also drawn like an ordinary choice even though the runtime skips
it, so presentation and behavior disagree.

## Proposed change

Reuse the already shipped Mirror, Strike, and Duo JPGs as Macroquad textures for
the result choices. Load them once, crop them through a pure cover calculation,
and retain a procedural fallback. Derive result availability from the same
evaluated-player facts already used by deterministic navigation, then combine
art dimming with an explicit lock shape for unavailable Duo.

## Why now

M13 established a positive bounded result as a normal endpoint rather than a
failure-only edge case. That makes the result screen a frequent primary surface,
and therefore the next highest-value screen for style continuity and no-touch
choice clarity.

## Non-goals

- No new art generation or asset copies.
- No change to gameplay, gesture recognition, bridge ABI, or browser learning.
- No claim that automated geometry proves readability at physical camera range.
