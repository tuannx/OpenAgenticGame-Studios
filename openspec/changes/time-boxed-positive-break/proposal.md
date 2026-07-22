# Proposal

## Problem

The current runner has a result screen but no positive completion boundary. A
skilled player can continue indefinitely because results appear only after all
evaluated players lose their energy. That weakens the product promise of a short
brain break and makes the user either fail deliberately or return to the device.

Taste learning also records entry success but not whether a run reached a
bounded completion or ended early. Recommending from starts alone misses the
stronger aggregate signal available at a real session boundary.

## Proposed behavior

Add a deterministic 90-second active-time budget. Pause, tracking hold, and all
countdowns preserve the remaining time. When the budget is exhausted, complete
the run before any further world judgment, show a positive result, and retain
the existing lean-to-choose plus clap-to-replay interaction.

Expose a one-shot Rust result event to the browser. Store only aggregate counts
per mode plus the last coarse outcome, then weight future mode taste with that
completion signal. No pose-derived or per-frame data crosses this boundary.

## Success

- A strong player is guaranteed a positive result after 90 active seconds.
- Safety holds never consume the break budget.
- The final boundary cannot inflict a simultaneous collision.
- Outcome learning stays constant-size, local, reversible, and non-biometric.
