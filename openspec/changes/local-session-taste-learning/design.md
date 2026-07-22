# Design

## Boundary

`taste-profile.ts` is a browser-capability adapter with a pure reducer core. The
UI emits meaningful session events; the camera pose callback never writes to
storage. The deterministic Rust runtime remains the only evaluated gameplay
boundary.

## Versioned local schema

Schema version 1 stores:

- bounded counters for session/setup outcomes;
- completed starts per playable mode;
- a bounded running mean for gesture time-to-ready;
- the latest setup outcome;
- validated theme, overlay, and opacity preferences.

The storage key is `brainbreak.taste.v1`. Parsing is fail-closed to defaults,
and both local-storage access and writes are exception-safe.

## Recommendation policy

- A mode becomes recommendable after two completed starts of that mode.
- Ties use a stable Mirror → Strike → Duo order.
- Compact setup requires two gesture-confirmed samples, a mean ready time of at
  most eight seconds, a latest gesture success, and a lifetime difficulty ratio
  no greater than 25%.
- The setup guide remains present in compact form so physical placement is
  never silently assumed.

## UX behavior

- First session: Random selected, full neon placement image, standard camera CTA.
- Stable return: learned mode selected, CTA says “continue”, placement image is
  a compact reminder, and the privacy note names what is and is not remembered.
- Any latest difficulty outcome: full guidance returns automatically.

## Privacy and performance

Writes occur on session start, camera attempt, completed start, setup exit, and
settings changes only. No timer loop or pose frame performs serialization. The
profile is constant-size and local-only.

