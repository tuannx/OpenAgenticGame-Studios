# Single-Owner Launcher Screen

M24 makes the launcher a real visual and interaction boundary. One shell owner
now coordinates launcher, Ready, guide-only demo, and gameplay presentation so
the canvas and optional settings cannot remain visible or keyboard-active behind
an `aria-modal` screen.

Status: implemented and validated on 2026-07-21. Native/WASM/web/browser/HTTP
gates passed; the unchanged inherited `sharp` audit findings and attended camera,
audio, and assistive-technology checks remain documented in `validation.md`.
