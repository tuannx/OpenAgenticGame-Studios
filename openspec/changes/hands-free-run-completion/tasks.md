# Tasks

## M3.1 — Deterministic completion state

- [x] Add result mode selection and one-shot next-run request.
- [x] Preserve best score/combo and correct replay/switch readiness semantics.
- [x] Cover evaluated input, cycling, replay, mode switch, and Duo invariants.

## M3.2 — No-touch bridge and taste loop

- [x] Add synchronized v5 `bb_set_game_mode` import.
- [x] Align launcher selection and local aggregate taste on runtime run starts.
- [x] Extend bridge tests for clamping, callback behavior, and v5 registration.

## M3.3 — Distance-readable HUD and result

- [x] Simplify the header and in-run player HUD.
- [x] Add positive score/best/combo result framing.
- [x] Draw three mode choices with non-color-only selection feedback.
- [x] Keep cue, beat meter, HUD, and result layouts separated on small screens.

## M3.4 — Validation and learning

- [x] Run focused Rust/TypeScript checks and the full BrainBreak gate.
- [x] Perform HTTP/browser responsive and runtime smoke without claiming camera proof.
- [x] Run sequential persona review and update validation evidence.
- [x] Distill the validated result-navigation pattern into the taste guide.
