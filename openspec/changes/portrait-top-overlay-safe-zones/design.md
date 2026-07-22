# Design

## Narrow-portrait action corridor

`action_cue_layout` detects a narrow portrait canvas independently of hazard
state. It retains the existing compact icon/text geometry but derives the panel
from a bounded left corridor:

- 12px canvas margin;
- a 148px reserved right zone for the 100px PIP, safe-area inset, and breathing
  gap;
- 190–252px panel width, subject to the available corridor.

Desktop and landscape retain their current centered layouts. The cue remains a
pure renderer concern and adds no runtime allocation or DOM dependency.

## Gameplay-only settings position

`ShellPresentation` exposes explicit `gameplayActive` truth. `setShellOwner`
maps that truth to one body class. Narrow portrait CSS moves the existing 48px
settings trigger to `top: 184px`, below the 94px cue/PIP row; launcher, Ready,
guide, landscape, and desktop placement remain unchanged.

The body class is presentation state only. It does not authorize scoring,
camera evaluation, or gameplay input.

## Invariants

- At 390x844 the cue ends before the PIP's reserved zone and the settings trigger
  begins below the cue.
- The action verb, pictogram, lane slots, proximity rail, font floor, and touch
  target sizes do not regress.
- 667x375 and 1280x720 keep centered cue and existing overlay placement.
- No browser import changes; both bridge versions remain at 7.

## Fallback

If the compact panel cannot preserve the lane diagram on a narrower supported
device, increase the right reserve or stack lane slots below the verb. Do not
hide the camera PIP or return the cue to an overlapping centered card.
