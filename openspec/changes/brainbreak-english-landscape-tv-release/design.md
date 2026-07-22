# Design

## Release owner model

Add `orientation` as a first-class shell owner layered over the existing
launcher, ready, guide, and gameplay owners. A pure presentation function takes
the logical owner plus `portraitBlocked` and guarantees:

- the rotate gate is the only visible modal owner in portrait;
- canvas, settings, and underlying modal content are inert while blocked;
- rotating back to landscape restores the prior logical owner and focus target;
- gameplay state, camera state, scoring, and session progress are not reset.

The gate uses a CSS-drawn rotating device pictogram plus the canonical
`phone_placement_guide.jpg` visual. It has no action and requires no touch.
Where the browser exposes `screen.orientation.lock`, camera-start user intent may
request landscape and ignore rejection; correctness always comes from the gate.

## Responsive surfaces

- Portrait phone/iPad: full-screen rotate owner, safe-area padding, no hidden
  gameplay interaction.
- Short phone landscape: four compact image cards, setup art can yield vertical
  space, and the primary camera action remains at least 52px high.
- iPad 4:3 landscape: launcher and Ready use bounded two-surface composition;
  gameplay retains camera/action/HUD safe zones.
- Desktop/TV 16:9: content is centered with maximum readable widths instead of
  stretching copy or controls across the display.

## English and visual hierarchy

Translate every user-visible Vietnamese source string and accessibility name.
No runtime localization framework is added for this release. Guidance order is:
image or body/action shape, one short English command, then optional truthful
detail. Camera failures remain one-action recovery messages.

## Performance contract

Release builds use one codegen unit, full LTO, abort-on-panic, and stripped
symbols. The build script fingerprints the actual delivered WASM and fails when
it exceeds the checked maximum. The first accepted budget is the measured
pre-change artifact (962,127 bytes); the release must be no larger, and any
profile change must pass native, WASM, web, and browser validation.

The web build records key entry and runtime asset sizes. Model/runtime code stays
intent-loaded after camera action; no TensorFlow dependency moves into boot.

## Cloudflare delivery

The existing Worker remains the delivery boundary. It must inspect the real
asset response content type before assigning immutable cache policy. Live checks
cover `/health`, HTML, hashed JS/CSS, fingerprinted WASM, audio, security
headers, and SPA-fallback resistance for missing typed assets.
