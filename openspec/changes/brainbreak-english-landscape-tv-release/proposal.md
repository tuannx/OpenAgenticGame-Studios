# Proposal

The current product has individually validated UX improvements, but the release
surface still mixes Vietnamese and English, allows portrait gameplay to compete
for scarce overlay space, and records WASM and Cloudflare evidence only as
milestone residuals. Users therefore receive many local changes without a clear,
repeatable improvement gate.

Ship one coherent release contract:

- English is the primary product language for visible UI, status, alternatives,
  and accessibility names.
- Hints, actions, and setup guidance lead with the existing neon image language
  and body/action pictograms; text stays short and secondary.
- Landscape is the canonical play surface. Portrait phone and iPad screens show
  one illustrated rotate state instead of compressing gameplay.
- Phone landscape, iPad landscape, 16:9 desktop, and TV-scale viewports retain
  bounded, safe-area-aware layouts with no clipped primary action.
- Release Rust/WASM and web payload sizes are measured against a checked budget,
  and optimization never weakens deterministic scoring or camera truth.
- Closeout requires a reviewed build deployed to Cloudflare plus live origin
  checks for runtime behavior, content types, and cache policy.

## Non-goals

- Adding a new game mode, camera model, scoring rule, network protocol, touch
  gesture, generated art family, or telemetry service.
- Pretending a browser can universally force device rotation. Unsupported
  orientation-lock attempts must fail safely; portrait remains explicitly
  blocked by the visual rotate owner.
- Treating synthetic camera input as proof of physical distance, real two-person
  identity, or two-peer multiplayer behavior.
