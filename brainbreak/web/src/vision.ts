import * as poseDetection from '@tensorflow-models/pose-detection';
import '@tensorflow/tfjs-backend-cpu';
import '@tensorflow/tfjs-backend-wasm';
import '@tensorflow/tfjs-backend-webgl';
import * as tf from '@tensorflow/tfjs-core';
import { setWasmPaths } from '@tensorflow/tfjs-backend-wasm';
import {
  throwIfCameraStartCancelled,
  waitForCameraStream,
} from './camera-start-cancellation';
import { cameraFrameGeometry, currentMotionCapability } from './motion-capability';
import type { CameraFrameGeometry } from './motion-capability';

export interface PoseSnapshot {
  id: number;
  quality: number;
  keypoints: Array<{ x: number; y: number; score: number }>;
}

export type VisionStatus =
  | { phase: 'requesting-camera' }
  | { phase: 'initializing-ai' }
  | { phase: 'calibrating'; backend: string }
  | { phase: 'tracking'; players: number }
  | { phase: 'inference-error'; message: string };

const motionCapability = currentMotionCapability();
let detector: poseDetection.PoseDetector | undefined;
let running = false;
let activeStream: MediaStream | undefined;
let activeVideo: HTMLVideoElement | undefined;
let activeResizeHandler: (() => void) | undefined;

async function configureBackend(): Promise<string> {
  setWasmPaths('/tfjs-wasm/');
  for (const backend of ['webgl', 'wasm', 'cpu']) {
    try {
      if (await tf.setBackend(backend)) {
        await tf.ready();
        return backend;
      }
    } catch {
      // Continue through the explicit fallback chain.
    }
  }
  throw new Error('No TensorFlow.js backend is available');
}

export type CameraOverlayMode = 'glass_pip' | 'cutout' | 'hologram' | 'skeleton';
export type VisualTheme = 'cyber-trunk' | 'pink-girl' | 'purple-vaporwave';

export interface OverlayConfig {
  mode: CameraOverlayMode;
  theme: VisualTheme;
}

let activeConfig: OverlayConfig = {
  mode: 'glass_pip',
  theme: 'cyber-trunk',
};

export function setOverlayConfig(config: Partial<OverlayConfig>): void {
  if (config.mode) activeConfig.mode = config.mode;
  if (config.theme) activeConfig.theme = config.theme;
}

function getThemeColors(theme: VisualTheme): string[] {
  switch (theme) {
    case 'pink-girl':
      return ['#f472b6', '#fbcfe8', '#fb7185'];
    case 'purple-vaporwave':
      return ['#c084fc', '#38bdf8', '#e879f9'];
    case 'cyber-trunk':
    default:
      return ['#00f3ff', '#f472b6', '#ffe600'];
  }
}

const SKELETON_LINKS = [[5,6],[5,7],[7,9],[6,8],[8,10],[5,11],[6,12],[11,12],[11,13],[13,15],[12,14],[14,16]] as const;
const KEYPOINT_THRESHOLD = 0.25;
const HAND_THRESHOLD = 0.3;

function drawPoses(canvas: HTMLCanvasElement, poses: PoseSnapshot[]): void {
  const context = canvas.getContext('2d');
  if (!context) return;
  const w = canvas.width;
  const h = canvas.height;
  context.clearRect(0, 0, w, h);

  const colors = getThemeColors(activeConfig.theme);
  const isGlowMode = activeConfig.mode === 'skeleton' || activeConfig.mode === 'hologram';
  const dotRadius = isGlowMode ? 6 : 4;

  for (const [player, pose] of poses.entries()) {
    const color = colors[player % colors.length];
    const kps = pose.keypoints;

    // --- Batch skeleton links (single path) ---
    context.strokeStyle = color;
    context.lineWidth = isGlowMode ? 4 : 3;
    if (isGlowMode) { context.shadowBlur = 12; context.shadowColor = color; }
    else { context.shadowBlur = 0; }

    context.beginPath();
    for (const [from, to] of SKELETON_LINKS) {
      const a = kps[from];
      const b = kps[to];
      if (!a || !b || a.score < KEYPOINT_THRESHOLD || b.score < KEYPOINT_THRESHOLD) continue;
      context.moveTo(a.x * w, a.y * h);
      context.lineTo(b.x * w, b.y * h);
    }
    context.stroke();

    // --- Batch keypoint dots (single path) ---
    context.fillStyle = color;
    context.beginPath();
    let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
    let visibleCount = 0;
    for (let i = 0; i < kps.length; i++) {
      const kp = kps[i];
      if (!kp || kp.score < KEYPOINT_THRESHOLD) continue;
      const px = kp.x * w;
      const py = kp.y * h;
      context.moveTo(px + dotRadius, py);
      context.arc(px, py, dotRadius, 0, Math.PI * 2);
      if (px < minX) minX = px;
      if (py < minY) minY = py;
      if (px > maxX) maxX = px;
      if (py > maxY) maxY = py;
      visibleCount++;
    }
    context.fill();

    // --- AR Hand Gauge (raised wrists) ---
    context.shadowBlur = 0;
    const leftWrist = kps[9];
    const rightWrist = kps[10];
    const leftShoulder = kps[5];
    const rightShoulder = kps[6];
    if (leftWrist && leftShoulder && leftWrist.score > HAND_THRESHOLD && leftWrist.y < leftShoulder.y) {
      context.beginPath();
      context.arc(leftWrist.x * w, leftWrist.y * h, 20, 0, Math.PI * 2);
      context.strokeStyle = '#34d399';
      context.lineWidth = 4;
      context.stroke();
    }
    if (rightWrist && rightShoulder && rightWrist.score > HAND_THRESHOLD && rightWrist.y < rightShoulder.y) {
      context.beginPath();
      context.arc(rightWrist.x * w, rightWrist.y * h, 20, 0, Math.PI * 2);
      context.strokeStyle = '#34d399';
      context.lineWidth = 4;
      context.stroke();
    }

    // --- Bounding box + label (no spread allocation) ---
    if (visibleCount > 0) {
      context.globalAlpha = 0.85;
      context.strokeRect(minX - 8, minY - 8, maxX - minX + 16, maxY - minY + 16);
      context.font = '600 13px system-ui';
      context.fillStyle = color;
      context.fillText(`P${player + 1} ${Math.round(pose.quality * 100)}%`, minX, Math.max(16, minY - 13));
      context.globalAlpha = 1;
    }
  }
}

export async function startVision(
  video: HTMLVideoElement,
  overlay: HTMLCanvasElement,
  onPoses: (poses: PoseSnapshot[]) => void,
  onStatus: (status: VisionStatus) => void,
  onFrameGeometry: (geometry: CameraFrameGeometry) => void,
  signal?: AbortSignal,
): Promise<void> {
  if (running) return;
  throwIfCameraStartCancelled(signal);
  running = true;
  onStatus({ phase: 'requesting-camera' });
  activeStream = await waitForCameraStream(navigator.mediaDevices.getUserMedia({
    audio: false,
    video: {
      facingMode: 'user',
      width: { ideal: motionCapability.idealWidth },
      height: { ideal: motionCapability.idealHeight },
      frameRate: { ideal: motionCapability.idealFrameRate, max: 30 },
    },
  }), signal);
  throwIfCameraStartCancelled(signal);
  video.srcObject = activeStream;
  await video.play();
  throwIfCameraStartCancelled(signal);
  const syncFrameGeometry = () => {
    const frameGeometry = cameraFrameGeometry(video.videoWidth, video.videoHeight, motionCapability);
    if (overlay.width !== frameGeometry.width) overlay.width = frameGeometry.width;
    if (overlay.height !== frameGeometry.height) overlay.height = frameGeometry.height;
    onFrameGeometry(frameGeometry);
  };
  activeVideo = video;
  activeResizeHandler = syncFrameGeometry;
  video.addEventListener('resize', syncFrameGeometry);
  syncFrameGeometry();
  onStatus({ phase: 'initializing-ai' });
  const backend = await configureBackend();
  throwIfCameraStartCancelled(signal);
  detector = await poseDetection.createDetector(poseDetection.SupportedModels.MoveNet, {
    modelType: motionCapability.localPoseCapacity === 1
      ? poseDetection.movenet.modelType.SINGLEPOSE_LIGHTNING
      : poseDetection.movenet.modelType.MULTIPOSE_LIGHTNING,
    enableTracking: motionCapability.localPoseCapacity > 1,
    trackerType: poseDetection.TrackerType.BoundingBox,
  });
  if (signal?.aborted) {
    stopVision();
    throwIfCameraStartCancelled(signal);
  }
  onStatus({ phase: 'calibrating', backend });

  let inferencePending = false;
  let lastInference = 0;
  const minimumInterval = motionCapability.minimumInferenceIntervalMs;
  const videoWidth = () => video.videoWidth || 1;
  const videoHeight = () => video.videoHeight || 1;

  // Pre-allocated pose buffer to avoid per-frame GC (max 2 players × 17 keypoints)
  const poseBuffer: PoseSnapshot[] = Array.from({ length: 2 }, () => ({
    id: 0,
    quality: 0,
    keypoints: Array.from({ length: 17 }, () => ({ x: 0, y: 0, score: 0 })),
  }));

  const infer = async (now: number) => {
    if (!running) return;
    if (!inferencePending && now - lastInference >= minimumInterval && video.readyState >= 2) {
      inferencePending = true;
      lastInference = now;
      try {
        const results = await detector!.estimatePoses(video, {
          maxPoses: motionCapability.localPoseCapacity,
          flipHorizontal: true,
        });
        if (!running) return;
        const vw = videoWidth();
        const vh = videoHeight();
        const count = Math.min(results.length, poseBuffer.length);
        for (let p = 0; p < count; p++) {
          const raw = results[p];
          const slot = poseBuffer[p];
          slot.id = raw.id ?? p + 1;
          let scoreSum = 0;
          for (let k = 0; k < 17; k++) {
            const src = raw.keypoints[k];
            const dst = slot.keypoints[k];
            dst.x = 1 - src.x / vw;
            dst.y = src.y / vh;
            dst.score = src.score ?? 0;
            scoreSum += dst.score;
          }
          slot.quality = scoreSum / 17;
        }
        // Sort by left-hip x for stable player ordering (in-place, max 2 elements)
        const poses = poseBuffer.slice(0, count);
        if (count === 2 && (poses[0].keypoints[11]?.x ?? 0) > (poses[1].keypoints[11]?.x ?? 0)) {
          const tmp = poses[0]; poses[0] = poses[1]; poses[1] = tmp;
        }
        onPoses(poses);
        drawPoses(overlay, poses);
        onStatus({ phase: 'tracking', players: count });
      } catch (error) {
        onStatus({
          phase: 'inference-error',
          message: error instanceof Error ? error.message : 'Pose inference failed',
        });
      } finally {
        inferencePending = false;
      }
    }
    requestAnimationFrame(infer);
  };
  requestAnimationFrame(infer);
}

export function stopVision(): void {
  running = false;
  if (activeVideo && activeResizeHandler) {
    activeVideo.removeEventListener('resize', activeResizeHandler);
  }
  activeVideo = undefined;
  activeResizeHandler = undefined;
  detector?.dispose();
  detector = undefined;
  activeStream?.getTracks().forEach((track) => track.stop());
  activeStream = undefined;
}
