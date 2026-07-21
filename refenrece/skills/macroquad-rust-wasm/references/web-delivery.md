# Macroquad Web Delivery

Last verified: 2026-07-20

## Build Contract

Install the standard Rust WASM target once:

```bash
rustup target add wasm32-unknown-unknown
```

Build a release artifact:

```bash
RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C link-arg=--allow-undefined" \
  cargo build --release --target wasm32-unknown-unknown
```

Rust 1.96 stopped passing `--allow-undefined` to `wasm-ld`. Macroquad 0.4.15
and miniquad intentionally leave browser/WebGL functions for the JavaScript
host to import, so current stable Rust needs the explicit linker compatibility
flag. Remove it only after the pinned Macroquad/miniquad release declares those
imports directly and a smoke test proves the new path.

The artifact is emitted under:

```text
target/wasm32-unknown-unknown/release/<crate_name>.wasm
```

Macroquad uses miniquad's JavaScript loader for its normal browser path. It does
not require wasm-bindgen unless the project adds a separate dependency or custom
interop contract that requires it.

## Static Bundle

Produce a self-contained directory:

```text
dist/
├── index.html
├── game.wasm
├── mq_js_bundle.js
└── assets/
```

Prefer copying `mq_js_bundle.js` from the Macroquad/miniquad source associated
with the pinned crate version. A remote loader URL is acceptable for a quick
prototype but is not a reproducible production dependency.

Minimal HTML shell:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width,initial-scale=1,viewport-fit=cover">
    <title>Macroquad Game</title>
    <style>
      html, body, canvas {
        width: 100%;
        height: 100%;
        margin: 0;
        overflow: hidden;
        background: #000;
      }
      canvas { display: block; touch-action: none; }
    </style>
  </head>
  <body>
    <canvas id="glcanvas" tabindex="1"></canvas>
    <script src="mq_js_bundle.js"></script>
    <script>load("game.wasm");</script>
  </body>
</html>
```

## Local Browser Validation

Serve the bundle through HTTP:

```bash
python3 -m http.server 4000 --directory dist
```

Open `http://localhost:4000` and verify console output, network status, input,
resize/orientation behavior, audio startup, asset paths, and frame pacing.

## CI Baseline

A Macroquad CI job should:

1. Install the stable Rust toolchain with `rustfmt` and `clippy`.
2. Add `wasm32-unknown-unknown`.
3. Run formatting, clippy, and native tests.
4. Build release WASM.
5. Assemble the static bundle.
6. Upload the bundle as an artifact before any environment-specific deploy job.

Keep deployment separate from validation so pull requests can prove the bundle
without receiving production credentials.

## Official Sources

- Macroquad crate and build instructions: https://docs.rs/crate/macroquad/latest
- Macroquad repository: https://github.com/not-fl3/macroquad
- Macroquad WASM interoperability notes: https://macroquad.rs/articles/wasm/
- Rust WASM target documentation:
  https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html
- Rust undefined-symbol compatibility change:
  https://blog.rust-lang.org/2026/04/04/changes-to-webassembly-targets-and-handling-undefined-symbols/
