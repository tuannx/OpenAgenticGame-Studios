export type PlayableGameMode = 'mirror' | 'strike' | 'duo' | 'supernova';
export type LocalPoseCapacity = 1 | 2;

export interface MotionDeviceHints {
  userAgent: string;
  platform?: string;
  maxTouchPoints?: number;
}

export interface MotionCapability {
  deviceClass: 'mobile' | 'desktop';
  idealWidth: number;
  idealHeight: number;
  idealFrameRate: number;
  localPoseCapacity: LocalPoseCapacity;
  minimumInferenceIntervalMs: number;
}

export interface CameraFrameGeometry {
  aspectRatio: number;
  height: number;
  orientation: 'landscape' | 'portrait' | 'square';
  width: number;
}

export const ALL_PLAYABLE_MODES: readonly PlayableGameMode[] = ['mirror', 'strike', 'duo', 'supernova'];

export function detectMotionCapability(hints: MotionDeviceHints): MotionCapability {
  const mobileUserAgent = /Android|iPhone|iPad|iPod/i.test(hints.userAgent);
  const iPadDesktopUserAgent = hints.platform === 'MacIntel' && (hints.maxTouchPoints ?? 0) > 1;
  if (mobileUserAgent || iPadDesktopUserAgent) {
    return {
      deviceClass: 'mobile',
      idealWidth: 480,
      idealHeight: 640,
      idealFrameRate: 20,
      localPoseCapacity: 1,
      minimumInferenceIntervalMs: 65,
    };
  }
  return {
    deviceClass: 'desktop',
    idealWidth: 640,
    idealHeight: 480,
    idealFrameRate: 30,
    localPoseCapacity: 2,
    minimumInferenceIntervalMs: 38,
  };
}

export function currentMotionCapability(source: MotionDeviceHints = navigator): MotionCapability {
  return detectMotionCapability(source);
}

export function availableModesForCapacity(capacity: LocalPoseCapacity): readonly PlayableGameMode[] {
  return capacity >= 2 ? ALL_PLAYABLE_MODES : ALL_PLAYABLE_MODES.filter((mode) => mode !== 'duo');
}

export function cameraFrameGeometry(
  width: number,
  height: number,
  fallback: Pick<MotionCapability, 'idealWidth' | 'idealHeight'>,
): CameraFrameGeometry {
  const safeWidth = Number.isFinite(width) && width > 0 ? Math.round(width) : fallback.idealWidth;
  const safeHeight = Number.isFinite(height) && height > 0 ? Math.round(height) : fallback.idealHeight;
  const delta = Math.abs(safeWidth - safeHeight);
  const squareTolerance = Math.max(safeWidth, safeHeight) * 0.02;
  return {
    aspectRatio: safeWidth / safeHeight,
    height: safeHeight,
    orientation: delta <= squareTolerance ? 'square' : safeWidth > safeHeight ? 'landscape' : 'portrait',
    width: safeWidth,
  };
}
