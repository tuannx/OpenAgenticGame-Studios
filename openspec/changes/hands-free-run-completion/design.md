# Design

## Ownership

- `brainbreak-core` owns `result_selection`, mode cycling, evaluated-input
  filtering, and a one-shot `next_run_request`.
- `brainbreak-game` consumes the request, resets MotionRuntime and RunnerGame,
  draws the HUD/result, and invokes the browser import on WASM.
- `bridge.ts` owns the selected-mode browser value and reports runtime-started
  runs to the DOM/taste adapter. It does not own scoring or result decisions.

## Result state machine

```text
Running -> GameOver(current mode selected)
GameOver + exclusive Left/Right -> cycle result selection
GameOver + Clap from evaluated player -> next_run_request(selection)
composition root consumes request -> start_run(selection, evaluation mask)
```

The request is a one-shot `Option<GameMode>` consumed with `take`. The core does
not call browser APIs. `start_run` preserves best scores and requires two
evaluated players before directly starting Duo.

## Presentation

- Replace the generic header with the active mode and evaluation/audio state.
- Show player HUD only while running; use larger score/life/combo typography.
- Result uses the existing neon panels and renderer, not a new art style.
- Result content: positive title, score/best/combo, three mode chips, and one
  instruction line: lean to choose, clap to play.
- The active mode chip uses border, fill, and scale—not color alone.

## ABI

Add `bb_set_game_mode(mode)` and bump both plugin and Rust bridge versions to 5.
The import updates the browser-selected mode and invokes an optional runtime-mode
handler. `main.ts` uses that handler to align the launcher card and record a
gesture-start aggregate; no pose data crosses this boundary.

## Fallback

Keyboard arrows and C/Enter/R map to the same deterministic actions. If the
browser callback is unavailable, the Rust run still transitions locally; the
release ABI validation prevents silently shipping a missing import.

