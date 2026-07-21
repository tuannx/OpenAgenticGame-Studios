import * as poseDetection from '@tensorflow-models/pose-detection';
import '@tensorflow/tfjs-backend-cpu';
import '@tensorflow/tfjs-backend-wasm';
import '@tensorflow/tfjs-backend-webgl';
import * as tf from '@tensorflow/tfjs-core';
import { setWasmPaths } from '@tensorflow/tfjs-backend-wasm';

export interface PoseSnapshot {
  id: number;
  quality: number;
  keypoints: Array<{ x: number; y: number; score: number }>;
}

const isMobile = /Android|iPhone|iPad|iPod/i.test(navigator.userAgent);
let detector: poseDetection.PoseDetector | undefined;
let running = false;
let activeStream: MediaStream | undefined;

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

function drawPoses(canvas: HTMLCanvasElement, poses: PoseSnapshot[]): void {
  const context = canvas.getContext('2d');
  if (!context) return;
  context.clearRect(0, 0, canvas.width, canvas.height);
  const links = [[5,6],[5,7],[7,9],[6,8],[8,10],[5,11],[6,12],[11,12],[11,13],[13,15],[12,14],[14,16]];
  const colors = ['#22d3ee', '#c4b5fd'];
  context.lineWidth = 3;
  context.font = '600 13px system-ui';
  for (const [player, pose] of poses.entries()) {
    const color = colors[player % colors.length];
    context.strokeStyle = color;
    context.fillStyle = color;
    for (const [from, to] of links) {
      const a = pose.keypoints[from];
      const b = pose.keypoints[to];
      if (!a || !b || a.score < 0.25 || b.score < 0.25) continue;
      context.beginPath();
      context.moveTo(a.x * canvas.width, a.y * canvas.height);
      context.lineTo(b.x * canvas.width, b.y * canvas.height);
      context.stroke();
    }
    for (const keypoint of pose.keypoints) {
      if (keypoint.score < 0.25) continue;
      context.beginPath();
      context.arc(keypoint.x * canvas.width, keypoint.y * canvas.height, 4, 0, Math.PI * 2);
      context.fill();
    }
    const visible = pose.keypoints.filter((keypoint) => keypoint.score >= 0.25);
    if (visible.length > 0) {
      const left = Math.min(...visible.map((keypoint) => keypoint.x)) * canvas.width;
      const top = Math.min(...visible.map((keypoint) => keypoint.y)) * canvas.height;
      const right = Math.max(...visible.map((keypoint) => keypoint.x)) * canvas.width;
      const bottom = Math.max(...visible.map((keypoint) => keypoint.y)) * canvas.height;
      context.globalAlpha = 0.75;
      context.strokeRect(left - 8, top - 8, right - left + 16, bottom - top + 16);
      context.fillText(`P${player + 1} ${Math.round(pose.quality * 100)}%`, left, Math.max(16, top - 13));
      context.globalAlpha = 1;
    }
  }
}

export async function startVision(
  video: HTMLVideoElement,
  overlay: HTMLCanvasElement,
  onPoses: (poses: PoseSnapshot[]) => void,
  onStatus: (status: string) => void,
): Promise<void> {
  if (running) return;
  running = true;
  onStatus('Requesting camera…');
  activeStream = await navigator.mediaDevices.getUserMedia({
    audio: false,
    video: {
      facingMode: 'user',
      width: { ideal: isMobile ? 480 : 640 },
      height: { ideal: isMobile ? 640 : 480 },
      frameRate: { ideal: isMobile ? 20 : 30, max: 30 },
    },
  });
  video.srcObject = activeStream;
  await video.play();
  overlay.width = video.videoWidth || 640;
  overlay.height = video.videoHeight || 480;
  const backend = await configureBackend();
  detector = await poseDetection.createDetector(poseDetection.SupportedModels.MoveNet, {
    modelType: isMobile
      ? poseDetection.movenet.modelType.SINGLEPOSE_LIGHTNING
      : poseDetection.movenet.modelType.MULTIPOSE_LIGHTNING,
    enableTracking: !isMobile,
    trackerType: poseDetection.TrackerType.BoundingBox,
  });
  onStatus(`${backend.toUpperCase()} • calibrating`);

  let inferencePending = false;
  let lastInference = 0;
  const minimumInterval = isMobile ? 65 : 38;
  const infer = async (now: number) => {
    if (!running) return;
    if (!inferencePending && now - lastInference >= minimumInterval && video.readyState >= 2) {
      inferencePending = true;
      lastInference = now;
      try {
        const results = await detector!.estimatePoses(video, { maxPoses: isMobile ? 1 : 2, flipHorizontal: true });
        if (!running) return;
        const poses = results
          .map((pose, index): PoseSnapshot => {
            const points = pose.keypoints.map((point) => ({
              x: 1 - point.x / video.videoWidth,
              y: point.y / video.videoHeight,
              score: point.score ?? 0,
            }));
            const quality = points.length === 0 ? 0 : points.reduce((sum, point) => sum + point.score, 0) / points.length;
            return { id: pose.id ?? index + 1, quality, keypoints: points };
          })
          .sort((a, b) => (a.keypoints[11]?.x ?? 0) - (b.keypoints[11]?.x ?? 0));
        onPoses(poses);
        drawPoses(overlay, poses);
        onStatus(poses.length === 0 ? 'Step into frame' : `${poses.length} player${poses.length > 1 ? 's' : ''} ready`);
      } catch (error) {
        onStatus(error instanceof Error ? error.message : 'Pose inference failed');
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
  detector?.dispose();
  detector = undefined;
  activeStream?.getTracks().forEach((track) => track.stop());
  activeStream = undefined;
}
