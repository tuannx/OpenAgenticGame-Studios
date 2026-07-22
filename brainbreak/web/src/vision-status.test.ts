import { describe, expect, it } from 'vitest';
import {
  presentCameraStartFailure,
  presentReadyCameraStatus,
  presentVisionStatus,
} from './vision-status';

describe('vision status presentation', () => {
  it('distinguishes permission, initialization, and calibration', () => {
    expect(presentVisionStatus({ phase: 'requesting-camera' })).toMatchObject({
      phase: 'permission',
      ready: 'ALLOW CAMERA TO CONTINUE',
    });
    expect(presentVisionStatus({ phase: 'initializing-ai' }).phase).toBe('initializing');
    expect(presentVisionStatus({ phase: 'calibrating', backend: 'webgl' })).toEqual({
      phase: 'calibrating',
      camera: 'WEBGL • calibrating',
      ready: 'STAND 1.5–2.5M AWAY • KEEP YOUR FULL BODY IN FRAME',
    });
  });

  it('leaves pose-driven guidance in control after tracking starts', () => {
    expect(presentVisionStatus({ phase: 'tracking', players: 2 })).toEqual({
      phase: 'tracking',
      camera: '2 players • camera evaluated',
    });
  });

  it('gives the camera action copy before tracking and then yields to pose guidance', () => {
    expect(presentReadyCameraStatus(presentVisionStatus({ phase: 'requesting-camera' })))
      .toBe('ALLOW CAMERA TO CONTINUE');
    expect(presentReadyCameraStatus(presentVisionStatus({ phase: 'initializing-ai' })))
      .toBe('STARTING MOTION AI');
    expect(presentReadyCameraStatus(presentVisionStatus({
      phase: 'calibrating',
      backend: 'webgl',
    })))
      .toBe('STAND 1.5–2.5M AWAY • KEEP YOUR FULL BODY IN FRAME');
    expect(presentReadyCameraStatus(presentVisionStatus({ phase: 'tracking', players: 1 })))
      .toBeUndefined();
  });

  it('keeps inference errors visible and recoverable', () => {
    expect(presentVisionStatus({ phase: 'inference-error', message: 'backend reset' }))
      .toEqual({
        phase: 'error',
        camera: 'Motion tracking paused',
        ready: 'backend reset',
      });
  });

  it.each([
    ['NotAllowedError', 'permission', 'CAMERA BLOCKED • ALLOW ACCESS, THEN TRY AGAIN'],
    ['SecurityError', 'permission', 'CAMERA BLOCKED • ALLOW ACCESS, THEN TRY AGAIN'],
    ['NotFoundError', 'missing', 'NO CAMERA FOUND • TRY THE NO-SCORE PREVIEW'],
    ['DevicesNotFoundError', 'missing', 'NO CAMERA FOUND • TRY THE NO-SCORE PREVIEW'],
    ['NotReadableError', 'busy', 'CAMERA BUSY • CLOSE OTHER APPS, THEN TRY AGAIN'],
    ['TrackStartError', 'busy', 'CAMERA BUSY • CLOSE OTHER APPS, THEN TRY AGAIN'],
  ])('maps %s to an actionable %s recovery', (name, reason, message) => {
    expect(presentCameraStartFailure({ name })).toEqual({ reason, message });
  });

  it('keeps unknown startup failures generic and does not expose adapter text', () => {
    expect(presentCameraStartFailure(new Error('private adapter detail'))).toEqual({
      reason: 'startup',
      message: 'CAMERA NOT READY • TRY AGAIN OR OPEN THE PREVIEW',
    });
    expect(presentCameraStartFailure('not an error')).toEqual({
      reason: 'startup',
      message: 'CAMERA NOT READY • TRY AGAIN OR OPEN THE PREVIEW',
    });
  });
});
