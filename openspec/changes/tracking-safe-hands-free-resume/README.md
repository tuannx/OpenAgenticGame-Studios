# Tracking-Safe Hands-Free Resume

**Date:** 2026-07-21  
**Target:** BrainBreak runner state, Macroquad overlays, and browser visibility freshness  
**Status:** M4 implementation, automated gates, and responsive browser smoke complete; attended camera/focus QA pending

This milestone makes camera loss and tab suspension safe during an active run.
The deterministic runtime freezes before another obstacle can be judged, then
returns through a short, gesture-confirmed countdown without requiring touch.

## Records

- `proposal.md` — problem, scope, and acceptance criteria
- `design.md` — deterministic states and browser freshness boundary
- `tasks.md` — milestone execution and validation status
- `validation.md` — evidence and residual attended-QA gaps
