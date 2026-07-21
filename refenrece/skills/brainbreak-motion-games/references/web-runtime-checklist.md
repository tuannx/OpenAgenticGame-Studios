# BrainBreak Web Runtime Checklist

Use this checklist after native, WASM, TypeScript, and bundle validation pass.

## Camera and Motion

- Serve from HTTPS or localhost and start `getUserMedia` from an explicit user gesture.
- Verify the default screen asks for camera prominently and presents guide-only mode as a smaller secondary choice.
- Verify denial, missing hardware, suspended tabs, and permission revocation return to guide-only state without stale evaluation.
- Verify mirrored video and skeleton coordinates stay aligned across resize and orientation changes.
- Exercise each action at threshold boundaries, during holds, after release, and after tracking loss.
- Test one person, two people entering in different orders, crossing positions, and one person leaving.

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
- Check reduced-motion behavior, contrast, keyboard focus, touch targets, safe areas, landscape/portrait, and mobile viewport resizing.
- Inspect console errors, network failures, long tasks, memory growth, and frame cadence for several minutes.
- Confirm the game remains playable when pose inference updates more slowly than rendering.

## HTTP and Production

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
