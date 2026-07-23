import type { PoseSnapshot } from './vision';
import { roomTransport } from './network';
import { musicEngine } from './audio';
import { partyDirector } from './party-voice';
import type { RunOutcome } from './taste-profile';

const KEYPOINTS = 17;
const BUFFER_SIZE = 16 + 2 * (8 + KEYPOINTS * 12);
let latestPoses: PoseSnapshot[] = [];
let customTargets: number[] = [];
let activeGameConfig: Uint8Array | null = null;
let cameraEvaluationEnabled = false;
let pageActive = true;
let reduceMotion = false;
let localPoseCapacity: 1 | 2 = 2;
const previousGamepadMasks = [0, 0];

// Cached DataView — invalidated only when WASM memory is re-grown
let cachedView: DataView | null = null;
let cachedBuffer: ArrayBuffer | null = null;

const ACTION = {
  left: 1 << 0,
  right: 1 << 1,
  jump: 1 << 2,
  squat: 1 << 3,
  leftUp: 1 << 4,
  rightUp: 1 << 5,
  clap: 1 << 6,
  handsDown: 1 << 8,
} as const;

declare global {
  interface Window {
    wasm_memory?: WebAssembly.Memory;
    miniquad_add_plugin: (plugin: unknown) => void;
    load: (path: string) => void;
  }
}

export function updatePoseBridge(poses: PoseSnapshot[]): void {
  latestPoses = pageActive ? poses.slice(0, localPoseCapacity) : [];
}

export function setCameraEvaluation(enabled: boolean): void {
  cameraEvaluationEnabled = enabled;
  if (!enabled) latestPoses = [];
}

export function setPageActive(active: boolean): void {
  pageActive = active;
  if (!active) latestPoses = [];
}

export function setReduceMotion(enabled: boolean): void {
  reduceMotion = enabled;
}

export function setCustomTargets(targets: number[]): void {
  customTargets = targets.filter((target) => Number.isInteger(target) && target > 0);
}

export function setGameConfig(json: string): void {
  activeGameConfig = new TextEncoder().encode(json);
}

export function clearGameConfig(): void {
  activeGameConfig = null;
}

function copyPose(destination: number, capacity: number): number {
  const memory = window.wasm_memory;
  if (!memory || capacity < BUFFER_SIZE) return 0;
  // Reuse DataView unless the underlying buffer was re-grown
  if (cachedBuffer !== memory.buffer) {
    cachedBuffer = memory.buffer;
    cachedView = new DataView(memory.buffer);
  }
  const view = cachedView!;
  let offset = destination;
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
  return offset - destination;
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
  if (pressed(6) || pressed(7)) active |= ACTION.handsDown;
  const triggered = active & ~previousGamepadMasks[player];
  previousGamepadMasks[player] = active;
  return triggered >>> 0;
}

export type GameModeValue = 0 | 1 | 2 | 3 | 4;

let selectedGameMode: GameModeValue = 0; // 0 = MirrorBeat, 1 = BeatStrike, 2 = DuoGroove, 3 = Supernova, 4 = Custom
let runtimeGameModeHandler: ((mode: GameModeValue) => void) | null = null;
let runOutcomeHandler: ((mode: GameModeValue, outcome: Exclude<RunOutcome, 'none'>) => void) | null = null;

function normalizeGameMode(mode: number): GameModeValue {
  const normalized = Number.isFinite(mode) ? Math.round(mode) : 0;
  if (normalized <= 0) return 0;
  if (normalized === 1) return 1;
  if (normalized === 3) return 3; // Supernova — no 2-player requirement
  if (normalized === 4) return 4; // Custom config-driven game
  return localPoseCapacity >= 2 ? 2 : 0;
}

export function setLocalPoseCapacity(capacity: number): void {
  localPoseCapacity = capacity >= 2 ? 2 : 1;
  latestPoses = latestPoses.slice(0, localPoseCapacity);
  selectedGameMode = normalizeGameMode(selectedGameMode);
}

export function setSelectedGameMode(mode: number): void {
  selectedGameMode = normalizeGameMode(mode);
}

export function getSelectedGameMode(): GameModeValue {
  return selectedGameMode;
}

export function hasCustomGameConfig(): boolean {
  return activeGameConfig !== null;
}

export function setRuntimeGameModeHandler(handler: ((mode: GameModeValue) => void) | null): void {
  runtimeGameModeHandler = handler;
}

export function setRunOutcomeHandler(
  handler: ((mode: GameModeValue, outcome: Exclude<RunOutcome, 'none'>) => void) | null,
): void {
  runOutcomeHandler = handler;
}

export function registerBrainBreakPlugin(): void {
  window.miniquad_add_plugin({
    name: 'brainbreak_bridge',
    version: 7,
    register_plugin(importObject: WebAssembly.Imports) {
      const env = importObject.env as Record<string, (...args: number[]) => number | void>;
      env.bb_copy_pose = copyPose;
      env.bb_take_remote_actions = (player: number) => roomTransport.takeRemoteAction(player);
      env.bb_send_local_action = (player: number, mask: number) => roomTransport.sendAction(player, mask);
      env.bb_network_status = () => roomTransport.statusCode;
      env.bb_custom_target = (beatIndex: number) => customTargets.length === 0 ? 0 : customTargets[beatIndex % customTargets.length] ?? 0;
      env.bb_take_gamepad_actions = takeGamepadActions;
      env.bb_evaluation_enabled = (player: number) => pageActive && cameraEvaluationEnabled && (latestPoses[player]?.quality ?? 0) >= 0.25 ? 1 : 0;
      env.bb_audio_beat_phase = () => musicEngine.metrics().phase;
      env.bb_audio_pulse = () => musicEngine.metrics().pulse;
      env.bb_audio_energy = () => musicEngine.metrics().energy;
      env.bb_audio_playing = () => musicEngine.metrics().playing ? 1 : 0;
      env.bb_reduce_motion = () => reduceMotion ? 1 : 0;
      env.bb_play_feedback = (kind: number, combo: number) => musicEngine.playFeedback(kind, combo);
      env.bb_supernova_event = (kind: number, value: number) => partyDirector.handleEvent(kind, value);
      env.bb_game_mode = () => selectedGameMode;
      env.bb_set_game_mode = (mode: number) => {
        setSelectedGameMode(mode);
        runtimeGameModeHandler?.(selectedGameMode);
      };
      env.bb_record_run_outcome = (mode: number, outcome: number) => {
        if (!Number.isInteger(mode) || mode < 0 || mode > 2) return;
        if (mode === 2 && localPoseCapacity < 2) return;
        if (outcome !== 1 && outcome !== 2) return;
        runOutcomeHandler?.(
          mode as GameModeValue,
          outcome === 1 ? 'break-complete' : 'energy-spent',
        );
      };
      env.bb_copy_game_config = (dest: number, capacity: number) => {
        if (!activeGameConfig || !window.wasm_memory) return 0;
        const bytes = activeGameConfig.subarray(0, capacity);
        new Uint8Array(window.wasm_memory.buffer, dest, bytes.length).set(bytes);
        return bytes.length;
      };
    },
  });
}
