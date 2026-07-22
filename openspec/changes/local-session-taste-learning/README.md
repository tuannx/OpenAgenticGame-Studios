# Local Session Taste Learning

**Date:** 2026-07-21  
**Target:** `brainbreak` web launcher, ready flow, and visual preferences  
**Status:** M2 implementation and automated/browser validation complete; physical-camera gesture QA pending

This M2 change makes repeat BrainBreak sessions faster without adding another
touch step. It learns only from local aggregate interaction outcomes and never
stores camera frames, images, video, keypoints, or pose snapshots.

## Records

- `proposal.md` — scope, non-goals, and acceptance criteria
- `design.md` — local profile schema, recommendation policy, and privacy boundary
- `tasks.md` — implementation and validation checklist
- `validation.md` — evidence, persona review, and residual gap
