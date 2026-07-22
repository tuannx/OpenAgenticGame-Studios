# Validation

## Pre-change baseline

- Product entry declares `lang="vi"` and mixes Vietnamese launcher/Ready/status
  copy with English settings and runtime labels.
- Portrait media queries continue laying out gameplay, camera PIP, and settings;
  no single rotate/orientation owner exists.
- Release WASM `brainbreak-game-78fbe2afc3c0.wasm` is 962,127 bytes.
- `Cargo.toml` has no explicit release optimization profile and the build has no
  checked payload budget.
- Cloudflare account `05130ca1f0bdabbab4d08a5d75544e92` is authenticated with
  Workers write access. The configured worker is `brainbreak-motion-party`.

## Required final evidence

- Source and production UI contain no Vietnamese user-facing copy.
- Automated ownership tests cover portrait blocking and landscape restoration.
- HTTP-served QA covers 390x844 and 1024x1366 rotate states plus 844x390,
  1366x1024, 1920x1080, and 3840x2160 landscape surfaces.
- Native tests, strict Clippy, release WASM, TypeScript, web tests, and production
  build pass; payload sizes are recorded and within budget.
- Cloudflare deploy output identifies the version and live URL. Live probes prove
  correct HTML, JavaScript, CSS, WASM, audio, security, and cache headers; an
  unknown `.wasm` path must not be served as immutable HTML.

## Unavailable-surface policy

If Chrome, physical camera, iPad/phone hardware, TV casting, or a second peer is
unavailable, record that exact gap. Builds or synthetic input cannot substitute
for those claims.

## Final evidence

- Product source now declares `lang="en"`; launcher, Ready, guide-only recovery,
  camera framing/status, failures, dynamic taste copy, and accessibility names
  are English. The production HTML contains `YOUR BODY IS THE CONTROLLER` and
  `ROTATE TO PLAY` and contains no Vietnamese diacritics.
- `orientation` is a first-class modal presentation. Portrait hides and inerts
  launcher, Ready, guide, gameplay canvas, and settings without resetting the
  logical owner. Returning to landscape restores owner-specific focus. The gate
  leads with a rotating-device shape and canonical placement art and has no
  action or touch requirement.
- Release Rust uses optimization level 3, full LTO, one codegen unit,
  abort-on-panic, stripped symbols, and no incremental release state. Delivered
  WASM fell from 962,127 to 628,180 bytes (34.7 percent) and the build now fails
  above the checked 962,127-byte ceiling.
- `brainbreak-game-50b8e2d9f27e.wasm` has SHA-256
  `50b8e2d9f27e2c8dbfd038979d64f461e7bfd32ca247aa4179a5974024808466`.
  The production response is byte-identical. Boot game JavaScript is 36,853
  bytes (12.60 kB gzip); the 1,348,518-byte vision chunk remains intent-loaded.
- Rust formatting, strict Clippy, 41 core tests, 25 game/layout tests, release
  WASM, TypeScript, 80 web tests, Vite production build, loader syntax, and
  `git diff --check` pass.
- Local and live Worker probes show HTML `no-cache`; hashed JS/CSS/WASM/audio
  with their correct MIME types and immutable caching; security headers; and
  missing hashed WASM/JS as HTML plus `no-cache`. The cache policy now evaluates
  actual response content type before assigning immutable caching.
- Cloudflare worker `brainbreak-motion-party` is live at
  `https://brainbreak-motion-party.tuannx87.workers.dev`, version
  `7e2c3d1c-b82b-460f-b450-125fb2ffd296`. `/health` identifies the deployed
  artifact as `wasm-50b8e2d9f27e`.
- The validator still reports the inherited Wrangler/Miniflare/Sharp audit chain
  (3 high). `npm audit fix --force` proposes the breaking Wrangler 4.15.2
  downgrade and was not applied. This is build-tool exposure, not shipped Worker
  runtime code.

## Residual attended validation

- No in-app or Chrome browser session was available after the production build,
  so the 390x844, 1024x1366, 844x390, 1366x1024, 1920x1080, and 3840x2160 visual
  matrix was not claimed. Pure owner tests prove portrait blocking semantics,
  but they do not replace rendered viewport inspection.
- Physical camera distance, iPhone/iPad hardware rotation, TV casting/readability,
  real two-person identity, and two-peer multiplayer remain unverified here.
