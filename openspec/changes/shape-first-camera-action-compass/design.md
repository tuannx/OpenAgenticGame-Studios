# Design

## Presentation boundary

`presentReadyNavigation` consumes the existing `MotionNavigationState` plus the
typed `PoseFramingPresentation`. It returns only `visualState` and one static
status string. Framing states always win, so a cropped or unstable player never
sees confirmation guidance before the physical correction they need.

The closed visual states are `framing`, `neutral`, `left`, `right`, and
`holding`. Numeric progress remains a separate bounded value already owned by
`MotionNavigationController`; presentation does not mirror its timer.

## Shape and image hierarchy

The selected canonical image and live camera preview retain their current
owners. Below the image, a three-part compass uses CSS chevrons for lean direction
and a fixed inline SVG raised-hand figure inside a conic progress ring. Text is
redundant and short. Font arrows, emoji, and visible percentages are removed.

Direction-specific `data-action` values reinforce a real one-frame selection
event, while the changed mode title/image provide persistent feedback. Holding
persists for the authoritative progress duration and accents the central ring.

## Performance and accessibility

- All normal presenter copy is static; pose updates no longer format visible
  percentage strings.
- CSS geometry is fixed and allocation-free; no image or SVG node is created on
  pose updates.
- The existing progressbar keeps `aria-valuenow`, and the status remains one
  polite live region.
- Reduced-motion removes action animations while preserving all shapes.

## Compatibility

Camera acquisition, framing coach, navigation controller, selected-mode mapping,
touch fallback, Rust bridge imports, and ABI are unchanged. The milestone is
reversible at the DOM/presentation layer.
