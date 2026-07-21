# Neon Beat Runner

## Status

- in_progress

## Harness Alignment

- follows `.codex/core/work-breakdown.md`
- follows `.codex/core/task-sizing.md`
- follows `.codex/core/milestone-design.md`
- follows `.codex/core/validation-matrix.md`
- follows `.codex/core/adr-rules.md`
- follows `.codex/core/persona-review.md`
- follows `.codex/core/closeout-loop.md`
- BMM-sized because gameplay, rendering, browser audio, attribution, and deployment cross multiple boundaries

## Source Context

- `brainbreak-base-framework-v0.1.0`
- `openspec/changes/motion-interactive-framework/`
- Macroquad Rust/WASM validation package
- “Special Spotlight” by Kevin MacLeod, CC BY 4.0, 126 BPM

## Why This Change Exists

- The framework proves camera motion input but its first app is still an abstract reaction stage.
- The next vertical slice must feel like a complete game: readable stakes, continuous movement, audiovisual rhythm, failure/retry, and attractive presentation.

## Relationship To Existing Changes

- Reuses the camera-first evaluation boundary from `motion-interactive-framework`.
- Preserves the renderer-independent motion runtime and adds an application-specific runner domain.
- Uses an original neon runner identity; no Temple Run branding or assets are copied.

## Execution Model

- Implement milestone by milestone, keeping deterministic runner rules separate from Macroquad rendering and browser audio.
- Preserve the tagged base framework as the rollback point.

## Validation Strategy

- Macroquad Rust/WASM package from `.codex/core/validation-matrix.md`
- TypeScript typecheck, Vitest, production Vite build, HTTP-served browser smoke, Cloudflare production verification
- Music attribution and deployed media MIME/cache verification

## Milestone Shape

1. Deterministic runner gameplay and motion-action mapping
2. Beat-synchronized visual/audio experience and responsive UX
3. Browser validation, documentation, deployment, and production verification
