# Macroquad Current Best Practices

Last verified: 2026-07-20

## Architecture

- Keep deterministic game rules in plain Rust modules that can run without a
  graphics context.
- Keep `#[macroquad::main]` as a thin composition root and async frame boundary.
- Capture input once per frame and convert it into game actions before updating
  domain state.
- Render from game state after update; avoid draw calls that mutate game rules.

## Timing And Physics

- Use `get_frame_time()` for frame-rate-independent behavior.
- Clamp exceptional delta values caused by suspended tabs or debugging.
- Use a fixed-step accumulator when gameplay physics needs deterministic steps;
  interpolate only the rendered presentation when appropriate.

## WASM Delivery

- Build release WASM with `wasm32-unknown-unknown`.
- On Rust 1.96+ with Macroquad 0.4.15, pass
  `-C link-arg=--allow-undefined`; miniquad's WebGL imports are fulfilled by the
  JavaScript host. Re-test and remove this compatibility flag when upstream
  declares the imports directly.
- Use Macroquad/miniquad's loader for the standard path; add wasm-bindgen only
  for an explicit dependency or JavaScript interoperability contract.
- Pin the JavaScript loader locally for production releases.
- Serve builds over HTTP and test browser behavior rather than opening HTML from
  the filesystem.
- Preserve relative asset paths and verify every request in browser developer
  tools.

## Responsive Game UX

- Compute layout from the current viewport instead of fixed desktop dimensions.
- Map touch/mouse coordinates into the same logical coordinate system.
- Define minimum target sizes and safe placement for mobile controls.
- Choose an explicit policy for aspect ratio and orientation changes.
- Resume safely after tab focus loss and start audio from a user gesture when
  required by browser policy.

## Performance

- Measure release builds; debug WASM is not representative.
- Batch compatible draw work and avoid per-frame heap churn.
- Load assets asynchronously and avoid blocking the browser frame loop.
- Track frame-time percentiles and WASM transfer size before adding aggressive
  build or compression complexity.

## Sources

- https://docs.rs/crate/macroquad/latest
- https://github.com/not-fl3/macroquad
- https://macroquad.rs/articles/wasm/
- https://blog.rust-lang.org/2026/04/04/changes-to-webassembly-targets-and-handling-undefined-symbols/
