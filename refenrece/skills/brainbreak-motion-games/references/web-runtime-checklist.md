# BrainBreak Web Runtime Checklist

Use this checklist after native, WASM, TypeScript, and bundle validation pass.

## Camera and Motion

- Serve from HTTPS or localhost and start `getUserMedia` from an explicit user gesture.
- Verify the default screen asks for camera prominently and presents guide-only mode as a smaller secondary choice.
- Verify denial, missing hardware, suspended tabs, and permission revocation return to guide-only state without stale evaluation.
- Verify mirrored video and skeleton coordinates stay aligned across resize and orientation changes.
- Exercise each action at threshold boundaries, during holds, after release, and after tracking loss.
- Test one person, two people entering in different orders, crossing positions, and one person leaving.
- In each case, compare the camera status, HUD identities, and stage avatars in
  the same frame. Single-player modes must not show an unevaluated extra body;
  Duo may keep only its two explicitly required placeholders before evaluation.

## Audio and Rhythm

- Confirm the source license permits redistribution and adaptation in a web game.
- Keep the license/attribution source, local fingerprinted asset, visible credit, and modification notice consistent.
- Start or resume `AudioContext` only after a user gesture.
- Verify mute/unmute, focus loss/recovery, looping, analyzer values, and the deterministic visual fallback when audio fails.
- Confirm the audio response uses an audio MIME type and the asset URL never resolves to HTML.

## Multiplayer

- Test host and guest in separate real browser contexts.
- Verify only evaluated actions and minimal metadata cross the data channel; inspect payloads for pose/video leakage.
- Verify disconnect suppresses remote evaluation immediately and reconnect does not replay stale actions.
- Test STUN-only behavior and TURN-configured behavior separately. Report when TURN secrets are absent.
- Verify room expiry, invalid codes, host departure, guest departure, and tab suspension.

## UX, Accessibility, and Performance

- Verify the current action, tracking confidence/state, and next hint are readable while moving at camera distance.
- Check participant presence at desktop and 390x844 portrait; confirm narrow
  layouts do not reveal or overlap a false player slot.
- During Running, verify avatar limbs, foot identity, and the nearest lane remain
  above the persistent HUD deck at desktop, 390x844, and 667x375. Repeat with
  one through four displayed cards where the environment supports those states.
- Confirm Ready, countdown, pause, tracking hold, and result reclaim the full
  canvas instead of retaining an empty Running-only HUD deck.
- Check reduced-motion behavior, contrast, keyboard focus, touch targets, safe areas, landscape/portrait, and mobile viewport resizing.
- When a camera-owned modal opens, verify focus enters it, forward/reverse Tab
  stay within visible actions, and Escape restores both the prior shell and a
  usable focus target. Repeat Escape before permission resolves and confirm a
  late MediaStream is stopped rather than attached.
- Inspect console errors, network failures, long tasks, memory growth, and frame cadence for several minutes.
- Confirm the game remains playable when pose inference updates more slowly than rendering.

## HTTP and Production

For a local browser smoke test, bind an explicit high IPv4 port and probe the
served bytes before opening a browser. A server can successfully bind
`::PORT` while another process already owns `127.0.0.1:PORT`, causing a correct-
looking localhost URL to load the wrong application.

```bash
python3 -m http.server 58921 --bind 127.0.0.1 -d web/dist
curl -fsSI http://127.0.0.1:58921/
curl -fsSI http://127.0.0.1:58921/brainbreak-game-HASH.wasm
```

Confirm the entrypoint title or another build-specific marker, artifact length,
and MIME types before treating browser console or screenshots as project
evidence. Server startup alone is not origin identity proof.

Probe the deployed origin, not only a local preview:

```bash
curl -fsS https://example.workers.dev/health
curl -fsSI https://example.workers.dev/
curl -fsSI https://example.workers.dev/assets/app-HASH.js
curl -fsSI https://example.workers.dev/brainbreak-game-HASH.wasm
curl -fsSI https://example.workers.dev/audio/track-HASH.mp3
```

Confirm:

- `/health` identifies the expected build SHA;
- HTML is `no-cache` or stricter;
- WASM is `application/wasm`;
- JavaScript is a JavaScript MIME type;
- audio is an audio MIME type;
- only fingerprinted, genuine assets receive immutable caching;
- an unknown asset path returning the SPA HTML fallback is never immutable;
- production bytes match the intended local release artifacts when reproducibility matters.

Do not report camera, audio, two-person identity, or P2P visual behavior as verified when an interactive browser or second peer was unavailable. Record that gap explicitly.
