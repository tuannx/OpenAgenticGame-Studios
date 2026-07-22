# Proposal

The deterministic core emits a one-frame `RunnerFeedback`, but the renderer only
retains an untyped flash and same-shaped particles. Success and collision are
therefore distinguished mainly by color, sound, and screen shake. The bottom HUD
also spends scarce space on 11-13 px words such as `LIFE` and `COMBO x0`.

The audio path has a related correctness gap: Rust sends only feedback kind, so
the browser always receives combo zero. The existing combo pitch method changes
the backing track playback rate, which would change its BPM while beat metrics
continue to assume 126 BPM.

Introduce a bounded presentation pulse, shape-code outcomes and lives, reveal
combo only from x2, and migrate the feedback import to `(kind, combo)`. Apply the
pitch ratio to short feedback oscillators while leaving music timing untouched.
