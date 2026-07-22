# Design

## Ready owns focus

Make the Ready dialog programmatically focusable and focus it on the next frame
after `setShellOwner('ready')`. Trap Tab only across focusable elements that are
actually visible; the collapsed fallback summary remains the sole stop until
its enabled buttons are revealed.

## Escape is cancellation, not only presentation

Give each camera start an `AbortController`. Escape aborts the active attempt,
records the setup exit, stops loaded vision resources, reveals the launcher,
and restores focus to the primary camera action. Startup code must check the
signal before publishing `cameraRunning` or later Ready state.

The explicit `Chọn lại game` fallback returns focus to the selected mode card;
it must not leave focus on the button that became hidden with Ready.

`getUserMedia()` itself has no portable abort signal. Race the request against
the session signal; if the browser resolves after cancellation, immediately
stop every returned track. This keeps late permission resolution from turning
the hidden camera back on or leaking a device track.

## Invariants

- The one visible modal owns focus.
- Forward and reverse Tab cannot enter hidden launcher, guide, canvas, or
  settings surfaces.
- Escape returns to launcher without adding a visible touch control.
- A late camera stream is stopped and never assigned to the video element.
- Camera evaluation stays off and deterministic Rust gameplay is unchanged.
