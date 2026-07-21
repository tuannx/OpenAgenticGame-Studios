# Macroquad — Version Reference

Last verified: 2026-07-20

| Field | Value |
| --- | --- |
| Framework | Macroquad |
| Latest verified crate | `0.4.15` |
| Dependency policy | Use `macroquad = "0.4"` for compatible projects or pin an exact patch when reproducible builds require it |
| Language | Stable Rust |
| Browser target | `wasm32-unknown-unknown` |
| Web loader | Macroquad/miniquad `mq_js_bundle.js` |
| Rust 1.96+ link compatibility | `-C link-arg=--allow-undefined` for Macroquad 0.4.15 |

## Verification Sources

- https://docs.rs/crate/macroquad/latest
- https://github.com/not-fl3/macroquad
- https://macroquad.rs/articles/wasm/
- https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html
- https://blog.rust-lang.org/2026/04/04/changes-to-webassembly-targets-and-handling-undefined-symbols/

## Compatibility Contract

- Verify the current crate version before initializing or upgrading a project.
- Treat `macroquad::experimental` as unstable unless the pinned release and
  project tests prove the required behavior.
- Build browser releases with `RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C
  link-arg=--allow-undefined" cargo build --release --target
  wasm32-unknown-unknown` while using Macroquad 0.4.15 on Rust 1.96+.
- Use the loader associated with the selected Macroquad/miniquad version.
- Preserve asset paths in the static web bundle and validate them over HTTP.
- Do not assume browser behavior is equivalent to the native target.

## Required Validation

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C link-arg=--allow-undefined" \
  cargo build --release --target wasm32-unknown-unknown
```

For any browser-facing change, add a served browser smoke test covering console
errors, asset loading, input, resize/orientation, focus loss, audio startup, and
release frame pacing.
