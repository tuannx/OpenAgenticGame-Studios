<!-- input: recurring implementation mistakes, repo-specific hot paths, and recent delivery incidents -->
<!-- output: compact lessons that should influence future Quick and BMM work -->
<!-- pos: repo-local lessons learned for Codex execution -->
# Lessons Learned

Use these lessons when working in this repository.

## 1. Treat Generated Pipelines As Real Product Surface

- Changes to content, routes, sitemap inputs, prerender inputs, or snapshots often require generator validation, not just code review.
- Prefer `npm run prebuild` or the relevant generator commands before assuming a change is isolated.
- If generated output affects runtime or audits, finish with `npm run build:dev`.

## 2. SSR Head And SEO Behavior Need Explicit Proof

- Metadata changes are easy to get partially right and still ship broken output.
- Use `npx vitest run tests/ssr-head.test.ts` plus the SEO audit scripts when head behavior changes.
- Do not assume audit failures are caused by the current change until the report is inspected.

## 3. I18n And Navigation Changes Can Have History-Semantics Regressions

- Locale and language-entry behavior is not just text replacement; history behavior and SSR output matter too.
- Run targeted i18n tests when touching language detection, switching, or prompt flows.
- Use `npm run build:dev` when locale behavior changes feed routing or SSR.

## 4. Audit Failures Must Be Attributed Before Fixing

- `build:dev` includes post-build and audit stages, so a non-zero exit can come from late-stage checks rather than the new code path itself.
- Read the failing report or tail symptom first.
- Distinguish between a new regression and an existing repository-level warning or gate failure.

## 5. Keep One Change Record Truth Source

- New non-trivial work should start under `openspec/changes/{change-name}/`.
- If a task already lives under `_bmad-output/changes/{change-name}/`, keep it coherent rather than splitting records mid-task.
- A weak or drifting change record causes BMM work to lose continuity faster than code changes do.

## 6. Reference Docs Are Not The Runtime Workflow

- `.codex/reference/` is useful background, but it should not override the repo-local execution docs.
- The active workflow lives in `.codex/workflows/` and `.codex/core/`; repo-specific hot-path maps now live in `docs/system-maps/`.
- If a reference file says something different, treat it as upstream context until the local docs are intentionally updated.

## 7. Continuous Skillization Across Sessions

- In every task/session, distill new patterns, user directives, and taste preferences into persistent files under `refenrece/skills/` and `refenrece/rules/`.
- Never leave learned lessons solely in chat transcripts. Codifying them in repository rules ensures future agent sessions automatically inherit and enforce them.

## 8. Zero-Touch UI & Audio-Lighting Synergy for Motion Games

- Motion game players standing 1.5m - 2.5m away must NEVER be forced to touch the screen.
- Every menu choice, ready check, start, and retry must be 100% hands-free via pose gestures (High Hand Hold 1s, Double Clap, Body Lean).
- Multi-player games require BOTH players detected & ready before launch.
- Motion actions must be quantized to music downbeats with audio pitch scaling (+1 semitone per 5 combo) and reactive neon lighting.

## 9. Ready Input Must Not Become First-Frame Gameplay

- In camera-controlled games, the gesture that confirms readiness leaves the
  body in motion and must not also move, score, collide, or advance the world.
- Route first launch and hands-free replay through one deterministic countdown;
  progression begins on the frame after the countdown completes.
- Keep initial-start and tracking-resume interruption destinations explicit even
  when both states share the same countdown presentation.
- Do not duplicate authoritative timers in DOM setup and the Rust runtime.

## 10. Motion Controls Must Not Leak Into Scoring

- A camera gesture used for pause, navigation, or another control action should
  remain in the evaluated action stream but be filtered before score, miss, and
  combo evaluation.
- Give high-impact control gestures stronger evidence than gameplay verbs: use
  a dwell plus geometry that excludes overlapping actions.
- Reserve the full dwell for the control channel from its first matching frame.
  Filtering only the final meta-action bit still permits constituent gameplay
  actions to score while the player is signaling control intent.
- Intentional pause and tracking failure need separate typed states even when
  both freeze the world; their recovery copy and loss destinations differ.
- Enter pause before gameplay progression and clear stale hazards before the
  later countdown resume.

## 11. Session Completion Is a Gameplay Boundary

- A “brain break” without a positive time limit forces skilled users to fail or
  touch the device. Own the active-time budget in deterministic game state.
- Advance session time only while required evaluation keeps the run active;
  setup, pause, tracking loss, and recovery are safety time, not play time.
- Finish before any other boundary-frame judgment and emit one typed event.
  Browser learning may count the result, but must not infer it from DOM timing
  or persist pose-derived data.

## 12. Visual Continuity Must Reuse Asset Ownership

- Reusing the same art direction is strongest when launcher, setup, and result
  consume one canonical asset URL rather than visually similar file copies.
- Do not embed images in a WASM artifact merely to redraw assets the browser
  already serves; load once, reuse cache ownership, and measure the final binary.
- Illustration is presentation, not a gameplay dependency. Keep a procedural
  fallback and keep capability truth visible when a beautiful choice is disabled.

## 13. Modal Semantics Need Runtime Ownership

- `aria-modal` does not hide siblings or remove a WebGL canvas from sequential
  focus. Prove the boundary with repeated forward/reverse keyboard input.
- One shell owner should coordinate launcher, Ready, guide, and gameplay classes
  plus background inertness; scattered add/remove calls create split-brain UI.
- Preserve first-paint inert attributes, toggle them explicitly for gameplay,
  and hide canvas visibility without removing the Macroquad layout surface.
- Reproduce suspected lazy-resource violations on a fresh origin before editing
  camera/audio code; prior automation input can explain delayed requests.

## 14. Quiet Actions Still Need Readable Truth

- Establish primary/secondary hierarchy with fill, area, placement, and weight;
  do not use sub-threshold contrast as the hierarchy mechanism.
- Compute effective contrast after whole-element opacity, not only from the raw
  foreground token.
- Put fallback consequences such as no camera or no score in the action label
  before navigation, while keeping one bounded 48px path and no extra touch.

## 15. Scrim Geometry Defines Screen Ownership

- A translucent live canvas is acceptable ambience only when runtime labels,
  player shapes, and HUD silhouettes cannot be read outside the owning dialog.
- Split center ambience, lower-world fade, and side-label masks into independent
  gradient layers; one radial alpha behaves differently across aspect ratios.
- When revealing a formerly hidden launcher, restore focus on the next animation
  frame and inspect the real active element in a browser.

## 16. Opacity Does Not Remove Semantic Ownership

- A lower-z screen can be visually covered and still expose duplicate headings
  and status copy to assistive technology, even after opacity reaches zero.
- When the launcher already owns first paint, canvas layout, and WASM startup,
  remove dead boot markup, CSS, animation, and timers instead of adding more
  hiding attributes.
- Validate immediate and settled accessibility snapshots separately, then prove
  the canvas dimensions and real loader/WASM requests remain intact.

## 17. Camera Failure Needs Cause Plus Recovery

- Convert browser camera exceptions into a bounded product reason set and never
  expose raw adapter messages in the interface.
- Keep one status rail and reuse the existing retry and no-score preview paths;
  recovery should not add another dialog or touch decision.
- After guide-originated failure, focus the revealed retry action on the next
  animation frame and test an actual second attempt.
- Validate the longest localized failure in short landscape while preserving
  action target sizes, privacy truth, and an explicit no-stream/evaluation-off
  boundary. Mocked rejections do not replace attended OS permission QA.

## 18. Put Framing Truth On The Camera Preview

- Mode art owns identity; the mirrored preview owns permission, calibration,
  framing, and pose-navigation status.
- Keep one live status node and let pose guidance take ownership after tracking;
  a later generic detector callback must not overwrite the physical correction.
- When compact CSS hides decorative art, measure the actionable status itself.
  A live node that still exists at 0x0 is not usable recovery guidance.
- Bound Ready and camera cards as one wide-screen stage, then validate portrait
  and short landscape independently. Synthetic streams prove layout/model
  handoff, not real-person alignment or camera-distance readability.

## 19. Modal Exit Must Cancel Pending Device Work

- `aria-modal` and correct visuals do not move focus. Inspect the active node
  after each shell transition and trap Tab only across currently visible,
  enabled controls.
- Escape from camera setup must cancel publication as well as hide the dialog.
  Otherwise a delayed permission result can attach a stream and revive hidden
  tracking state after the launcher has returned.
- Race non-abortable `getUserMedia()` against a session AbortSignal and dispose
  every late track. Prove immediate focus recovery and delayed disposal as two
  separate assertions.

## 20. Participant Truth Must Reach The Renderer

- Camera status and HUD correctness are insufficient when the gameplay renderer
  uses a different implicit player-count rule. Compare all three surfaces in one
  camera-backed frame.
- Encode player presence as a pure mode/evaluation mask: P1 is the setup anchor,
  Duo reserves its required P2, and all other slots require authoritative
  evaluation.
- Validate desktop and portrait separately because overlapping false avatars are
  much more damaging in a narrow center lane.
- Treat real two-person entry order, crossing, departure, and identity stability
  as attended evidence. A synthetic detected body proves wiring, not household
  multi-person behavior.

## 21. Reserve Pixels For Persistent HUD Before Drawing The World

- A fixed bottom HUD and a bottom-anchored body avatar will overlap at every
  aspect ratio; translucency or corner placement does not remove the ownership
  conflict.
- Derive stage, passive deck, and cards from one responsive layout. Paint the
  opaque deck after the world so pose overflow and crash shake cannot leak into
  persistent state.
- Keep the deck Running-only and preserve at least 60 percent stage height on the
  shortest supported viewport. Frozen/result screens already own full-canvas
  hierarchy without HUD cards.
- Pair geometry tests across one to four players with camera-backed desktop and
  portrait screenshots; neither proof covers the other's failure mode.
