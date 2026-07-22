# Proposal

The illustrated launcher declares `aria-modal=true`, but the 48px settings
disclosure and camera card remain faintly visible behind it. More importantly,
fresh-origin browser evidence shows repeated Tab presses remain trapped on the
background Macroquad canvas instead of reaching launcher choices. The semantic
claim and runtime interaction boundary therefore disagree.

Introduce one explicit shell-presentation owner with four states: launcher,
Ready, guide-only demo, and gameplay. The owner synchronizes modal visibility,
body presentation classes, and background inertness. Mark the initial HTML as
launcher-owned so technical chrome cannot flash or receive focus before the
module initializes.

## Non-goals

- Changing launcher copy, illustrations, mode recommendation, or fold layout.
- Starting camera, vision, audio, scoring, or multiplayer earlier.
- Adding another touch, disclosure, focus-trap listener, or animation.
- Changing pose evaluation, game rules, Rust, the JavaScript bridge, or ABI.
- Claiming physical camera, audio, or screen-reader acceptance without an
  attended device session.
