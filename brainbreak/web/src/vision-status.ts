import type { VisionStatus } from './vision';

export type CameraSetupVisualPhase =
  | 'permission'
  | 'initializing'
  | 'calibrating'
  | 'tracking'
  | 'error';

export interface VisionStatusPresentation {
  phase: CameraSetupVisualPhase;
  camera: string;
  ready?: string;
}

export type CameraStartFailureReason = 'permission' | 'missing' | 'busy' | 'startup';

export interface CameraStartFailurePresentation {
  reason: CameraStartFailureReason;
  message: string;
}

function cameraFailureName(error: unknown): string {
  if (typeof error !== 'object' || error === null || !('name' in error)) return '';
  return typeof error.name === 'string' ? error.name : '';
}

export function presentCameraStartFailure(error: unknown): CameraStartFailurePresentation {
  switch (cameraFailureName(error)) {
    case 'NotAllowedError':
    case 'SecurityError':
      return {
        reason: 'permission',
        message: 'CAMERA BLOCKED • ALLOW ACCESS, THEN TRY AGAIN',
      };
    case 'NotFoundError':
    case 'DevicesNotFoundError':
      return {
        reason: 'missing',
        message: 'NO CAMERA FOUND • TRY THE NO-SCORE PREVIEW',
      };
    case 'NotReadableError':
    case 'TrackStartError':
      return {
        reason: 'busy',
        message: 'CAMERA BUSY • CLOSE OTHER APPS, THEN TRY AGAIN',
      };
    default:
      return {
        reason: 'startup',
        message: 'CAMERA NOT READY • TRY AGAIN OR OPEN THE PREVIEW',
      };
  }
}

export function presentVisionStatus(status: VisionStatus): VisionStatusPresentation {
  switch (status.phase) {
    case 'requesting-camera':
      return {
        phase: 'permission',
        camera: 'Waiting for camera permission',
        ready: 'ALLOW CAMERA TO CONTINUE',
      };
    case 'initializing-ai':
      return {
        phase: 'initializing',
        camera: 'Camera on • starting motion AI',
        ready: 'STARTING MOTION AI',
      };
    case 'calibrating':
      return {
        phase: 'calibrating',
        camera: `${status.backend.toUpperCase()} • calibrating`,
        ready: 'STEP BACK · FULL BODY IN FRAME',
      };
    case 'tracking':
      return {
        phase: 'tracking',
        camera: status.players === 0
          ? 'Looking for a player'
          : `${status.players} player${status.players === 1 ? '' : 's'} • camera evaluated`,
      };
    case 'inference-error':
      return {
        phase: 'error',
        camera: 'Motion tracking paused',
        ready: status.message,
      };
  }
}

export function presentReadyCameraStatus(
  presentation: VisionStatusPresentation,
): string | undefined {
  if (presentation.phase === 'tracking') return undefined;
  return presentation.ready ?? presentation.camera;
}
