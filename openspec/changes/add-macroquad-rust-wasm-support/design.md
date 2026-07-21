# Design

## Decision

Represent Macroquad support as one engine lead agent plus one reusable workflow
skill. Keep detailed, version-sensitive commands in engine reference docs so the
skill remains compact and future updates have one primary source.

## Support Contract

- Language: stable Rust, edition declared by the generated Cargo project.
- Framework: Macroquad `0.4` series; exact latest version must be verified before
  pinning a new project.
- Native loop: `cargo run`, `cargo test`, `cargo clippy`.
- Web target: `wasm32-unknown-unknown` using Macroquad/miniquad's JavaScript
  loader, without assuming wasm-bindgen. Macroquad 0.4.15 on Rust 1.96+ needs
  `-C link-arg=--allow-undefined` because the JavaScript host supplies miniquad's
  browser imports.
- Web validation: release WASM build plus an HTTP-served browser smoke test.
- Architecture: domain/game logic remains independent from Macroquad rendering,
  input, audio, and asset adapters where practical.

## UX And Performance Rules

- Use delta time and clamp large frame gaps before physics integration.
- Scale layout and hit targets from the current canvas dimensions.
- Treat touch, mouse, keyboard, focus loss, resize, and browser audio policy as
  explicit cross-platform concerns.
- Avoid blocking loops and blocking I/O in the WASM frame loop.
- Profile release WASM size and frame time before adding optimization flags.

## Validation Strategy

- Validate skill frontmatter with the repository's skill validator.
- Parse the JSON skill manifest and compare its declared count to its entries.
- Check all newly referenced files exist.
- Check documentation for stale Claude-only paths introduced in the modified
  setup workflow.
