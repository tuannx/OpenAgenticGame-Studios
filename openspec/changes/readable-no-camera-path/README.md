# Readable No-Camera Path

M25 keeps the launcher camera-first while making its one honest fallback action
readable at phone and room distance. The secondary path remains visually quieter
than the dominant camera CTA, but no longer disappears below accessible contrast.

Status: implemented and validated on 2026-07-21. Native/WASM/web/browser/HTTP
gates passed; the unchanged inherited `sharp` audit findings and attended camera,
audio, and assistive-technology checks remain documented in `validation.md`.
