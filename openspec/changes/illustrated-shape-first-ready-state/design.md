# Design

## Presentation boundary

`ready_presentation` is a pure mapping from mode-independent runtime facts to a
small value object containing static title/instruction slices, tone, and two
figure states. It does not inspect DOM state, allocate strings, or mutate the
runner. Rendering owns color and geometry; the deterministic core remains the
sole owner of readiness and phase transitions.

The mapping distinguishes:

- motion/camera unavailable;
- no tracked player;
- Duo with only one tracked player;
- one-player signal pending or received;
- Duo with neither, either, or both player signals received.

## Shape-first visual language

The selected mode image occupies a bounded left crop. The right region contains
short copy and one or two procedural figures. A neutral standing figure means
present, a corner-frame silhouette means still needed, and a raised arm plus
diamond means ready. Camera-required uses a crossed camera glyph. These shapes
remain meaningful in grayscale; color is reinforcing information only.

## Layout and fallback

One pure layout helper returns panel, image, content, and figure geometry. The
panel uses a compact height in phone landscape and a taller height elsewhere.
Tests cover 390x844, 667x375, and 1440x784.

Ready borrows `ModeArt::texture` from M14 and the existing center-cover crop
helper. If the texture is absent, a neon procedural art field remains behind the
same copy and body cues. No image is decoded or allocated in the frame loop.

## Performance and compatibility

- All copy is `&'static str`; presentation and layout are stack values.
- The existing three textures remain loaded exactly once and borrowed per frame.
- No gameplay, gesture, bridge ABI, JavaScript, or asset URL changes.
- Release artifact size is measured against M14 rather than assumed unchanged.

## Validation impact

Run strict native Rust checks and semantic/layout tests, then separately build
release WASM and the production web bundle. Serve the bundle over HTTP to verify
WASM/JS/image content types and browser startup. A permissioned physical camera
session is still required to verify room-distance recognition and timing.
