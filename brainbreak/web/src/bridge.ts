import type { PoseSnapshot } from './vision';
import { roomTransport } from './network';

const KEYPOINTS = 17;
const BUFFER_SIZE = 16 + 2 * (8 + KEYPOINTS * 12);
let latestPoses: PoseSnapshot[] = [];
let customTargets: number[] = [];
const previousGamepadMasks = [0, 0];

const ACTION = {
  left: 1 << 0,
  right: 1 << 1,
  jump: 1 << 2,
  squat: 1 << 3,
  leftUp: 1 << 4,
  rightUp: 1 << 5,
  clap: 1 << 6,
} as const;

declare global {
  interface Window {
    wasm_memory?: WebAssembly.Memory;
    miniquad_add_plugin: (plugin: unknown) => void;
    load: (path: string) => void;
  }
}

export function updatePoseBridge(poses: PoseSnapshot[]): void {
  latestPoses = poses.slice(0, 2);
}

export function setCustomTargets(targets: number[]): void {
  customTargets = targets.filter((target) => Number.isInteger(target) && target > 0);
}

function copyPose(destination: number, capacity: number): number {
  const memory = window.wasm_memory;
  if (!memory || capacity < BUFFER_SIZE) return 0;
  const view = new DataView(memory.buffer, destination, BUFFER_SIZE);
  let offset = 0;
  view.setUint32(offset, 1, true); offset += 4;
  view.setUint32(offset, latestPoses.length, true); offset += 4;
  view.setFloat64(offset, performance.timeOrigin + performance.now(), true); offset += 8;
  for (const pose of latestPoses) {
    view.setUint32(offset, pose.id, true); offset += 4;
    view.setFloat32(offset, pose.quality, true); offset += 4;
    for (let index = 0; index < KEYPOINTS; index += 1) {
      const keypoint = pose.keypoints[index] ?? { x: 0, y: 0, score: 0 };
      view.setFloat32(offset, keypoint.x, true); offset += 4;
      view.setFloat32(offset, keypoint.y, true); offset += 4;
      view.setFloat32(offset, keypoint.score, true); offset += 4;
    }
  }
  return offset;
}

function takeGamepadActions(player: number): number {
  if (player < 0 || player > 1 || !navigator.getGamepads) return 0;
  const gamepad = navigator.getGamepads()[player];
  if (!gamepad) {
    previousGamepadMasks[player] = 0;
    return 0;
  }
  const pressed = (index: number) => gamepad.buttons[index]?.pressed === true;
  let active = 0;
  const horizontal = gamepad.axes[0] ?? 0;
  if (horizontal < -0.55 || pressed(14)) active |= ACTION.left;
  if (horizontal > 0.55 || pressed(15)) active |= ACTION.right;
  if (pressed(0) || pressed(12)) active |= ACTION.jump;
  if (pressed(1) || pressed(13)) active |= ACTION.squat;
  if (pressed(4)) active |= ACTION.leftUp;
  if (pressed(5)) active |= ACTION.rightUp;
  if (pressed(2) || pressed(3)) active |= ACTION.clap;
  const triggered = active & ~previousGamepadMasks[player];
  previousGamepadMasks[player] = active;
  return triggered >>> 0;
}

export function registerBrainBreakPlugin(): void {
  window.miniquad_add_plugin({
    name: 'brainbreak_bridge',
    version: 1,
    register_plugin(importObject: WebAssembly.Imports) {
      const env = importObject.env as Record<string, (...args: number[]) => number | void>;
      env.bb_copy_pose = copyPose;
      env.bb_take_remote_actions = (player: number) => roomTransport.takeRemoteAction(player);
      env.bb_send_local_action = (player: number, mask: number) => roomTransport.sendAction(player, mask);
      env.bb_network_status = () => roomTransport.statusCode;
      env.bb_custom_target = (beatIndex: number) => customTargets.length === 0 ? 0 : customTargets[beatIndex % customTargets.length] ?? 0;
      env.bb_take_gamepad_actions = takeGamepadActions;
    },
  });
}
