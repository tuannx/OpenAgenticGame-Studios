# Proposal

## Problem

The runner already protects tracking recovery with a deterministic two-second
countdown, but initial readiness and result replay transition directly into
`Running`. A player standing 1.5-2.5 metres from the screen can therefore finish
a gesture and immediately face motion, hazards, and beat timing without a clear
preparation beat.

The web setup gate must not solve this with its own timer: it selects a mode and
establishes camera capability, while authoritative progression belongs to the
Rust runtime.

## Scope

- Route initial evaluated readiness and hands-free replay through a two-second
  deterministic start phase.
- Keep all gameplay state frozen until that countdown completes.
- Preserve distinct recovery behavior when tracking is lost before a run versus
  during a resumed run.
- Reuse the existing no-touch countdown overlay and responsive layout.

## Non-goals

- No extra ready gesture, DOM modal, tutorial copy, recognizer threshold, or
  countdown duration change.
- No scoring, collision, obstacle pattern, network, audio, or ABI change.
- No production deployment.

## Success criteria

The same core state machine owns first launch, replay, and tracking-resume
countdowns; each path is deterministic, camera-gated, and covered by tests, and
the release WASM still loads over HTTP without browser-origin errors.
