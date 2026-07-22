import { describe, expect, it } from 'vitest';
import {
  availableModesForCapacity,
  cameraFrameGeometry,
  detectMotionCapability,
} from './motion-capability';

describe('local motion capability', () => {
  it('uses one-pose portrait capture for phones and Android devices', () => {
    const iphone = detectMotionCapability({ userAgent: 'Mozilla/5.0 (iPhone; CPU iPhone OS 18_0)' });
    const android = detectMotionCapability({ userAgent: 'Mozilla/5.0 (Linux; Android 15; Pixel)' });

    expect(iphone).toMatchObject({ deviceClass: 'mobile', localPoseCapacity: 1, idealWidth: 480, idealHeight: 640 });
    expect(android.localPoseCapacity).toBe(1);
  });

  it('recognizes iPadOS when Safari advertises a desktop user agent', () => {
    const capability = detectMotionCapability({
      userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15)',
      platform: 'MacIntel',
      maxTouchPoints: 5,
    });

    expect(capability.deviceClass).toBe('mobile');
    expect(capability.localPoseCapacity).toBe(1);
  });

  it('keeps the measured two-pose profile on desktop', () => {
    const capability = detectMotionCapability({
      userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)',
      platform: 'MacIntel',
      maxTouchPoints: 0,
    });

    expect(capability).toMatchObject({ deviceClass: 'desktop', localPoseCapacity: 2, idealWidth: 640, idealHeight: 480 });
    expect(availableModesForCapacity(capability.localPoseCapacity)).toEqual(['mirror', 'strike', 'duo', 'supernova']);
    expect(availableModesForCapacity(1)).toEqual(['mirror', 'strike', 'supernova']);
  });
});

describe('camera frame geometry', () => {
  const fallback = { idealWidth: 640, idealHeight: 480 };

  it('preserves portrait, landscape, and near-square intrinsic dimensions', () => {
    expect(cameraFrameGeometry(480, 640, fallback)).toEqual({
      aspectRatio: 0.75,
      height: 640,
      orientation: 'portrait',
      width: 480,
    });
    expect(cameraFrameGeometry(1280, 720, fallback).orientation).toBe('landscape');
    expect(cameraFrameGeometry(1000, 990, fallback).orientation).toBe('square');
  });

  it('falls back to the requested profile when metadata is absent or invalid', () => {
    expect(cameraFrameGeometry(0, Number.NaN, fallback)).toEqual({
      aspectRatio: 4 / 3,
      height: 480,
      orientation: 'landscape',
      width: 640,
    });
  });
});
