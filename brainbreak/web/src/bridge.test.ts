import { afterEach, describe, expect, it, vi } from 'vitest';
import {
  registerBrainBreakPlugin,
  setCameraEvaluation,
  setLocalPoseCapacity,
  setPageActive,
  setRunOutcomeHandler,
  setRuntimeGameModeHandler,
  setSelectedGameMode,
  updatePoseBridge,
} from './bridge';
import { musicEngine } from './audio';
import type { PoseSnapshot } from './vision';

interface RegisteredPlugin {
  name: string;
  version: number;
  register_plugin(importObject: WebAssembly.Imports): void;
}

afterEach(() => {
  setLocalPoseCapacity(2);
  setCameraEvaluation(false);
  setPageActive(true);
  updatePoseBridge([]);
  setRuntimeGameModeHandler(null);
  setRunOutcomeHandler(null);
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

function registeredEnv(): Record<string, (...args: number[]) => number | void> {
  let registered: RegisteredPlugin | undefined;
  vi.stubGlobal('window', {
    miniquad_add_plugin(plugin: unknown) {
      registered = plugin as RegisteredPlugin;
    },
  });
  registerBrainBreakPlugin();
  const env: Record<string, (...args: number[]) => number | void> = {};
  registered?.register_plugin({ env } as WebAssembly.Imports);
  return env;
}

describe('BrainBreak WASM bridge registration', () => {
  it('registers synchronized v7 game-mode and result imports', () => {
    let registered: RegisteredPlugin | undefined;
    vi.stubGlobal('window', {
      miniquad_add_plugin(plugin: unknown) {
        registered = plugin as RegisteredPlugin;
      },
    });

    registerBrainBreakPlugin();
    expect(registered?.name).toBe('brainbreak_bridge');
    expect(registered?.version).toBe(7);

    const env: Record<string, (...args: number[]) => number | void> = {};
    registered?.register_plugin({ env } as WebAssembly.Imports);
    expect(env.bb_game_mode).toBeTypeOf('function');
    expect(env.bb_set_game_mode).toBeTypeOf('function');
    expect(env.bb_record_run_outcome).toBeTypeOf('function');

    setSelectedGameMode(2);
    expect(env.bb_game_mode?.()).toBe(2);
    setSelectedGameMode(99);
    expect(env.bb_game_mode?.()).toBe(2);
    setSelectedGameMode(-10);
    expect(env.bb_game_mode?.()).toBe(0);
    setSelectedGameMode(1.6);
    expect(env.bb_game_mode?.()).toBe(2);
    setSelectedGameMode(Number.NaN);
    expect(env.bb_game_mode?.()).toBe(0);

    const runtimeModes: number[] = [];
    setRuntimeGameModeHandler((mode) => runtimeModes.push(mode));
    env.bb_set_game_mode?.(1);
    env.bb_set_game_mode?.(99);
    env.bb_set_game_mode?.(-10);
    expect(runtimeModes).toEqual([1, 2, 0]);
    expect(env.bb_game_mode?.()).toBe(0);
  });

  it('forwards only known run outcomes and locally supported modes', () => {
    const env = registeredEnv();
    const outcomes: Array<[number, string]> = [];
    setRunOutcomeHandler((mode, outcome) => outcomes.push([mode, outcome]));

    env.bb_record_run_outcome?.(0, 1);
    env.bb_record_run_outcome?.(1, 2);
    env.bb_record_run_outcome?.(99, 1);
    env.bb_record_run_outcome?.(0, 99);
    expect(outcomes).toEqual([
      [0, 'break-complete'],
      [1, 'energy-spent'],
    ]);

    setLocalPoseCapacity(1);
    env.bb_record_run_outcome?.(2, 1);
    expect(outcomes).toHaveLength(2);
  });

  it('forwards feedback kind and the evaluated player combo to audio', () => {
    const feedback = vi.spyOn(musicEngine, 'playFeedback').mockImplementation(() => undefined);
    const env = registeredEnv();

    env.bb_play_feedback?.(3, 15);

    expect(feedback).toHaveBeenCalledOnce();
    expect(feedback).toHaveBeenCalledWith(3, 15);
  });

  it('requires a fresh pose after page inactivity before evaluation resumes', () => {
    const env = registeredEnv();
    const pose: PoseSnapshot = {
      id: 7,
      quality: 0.9,
      keypoints: Array.from({ length: 17 }, () => ({ x: 0.5, y: 0.5, score: 0.9 })),
    };

    setPageActive(true);
    setCameraEvaluation(true);
    updatePoseBridge([pose]);
    expect(env.bb_evaluation_enabled?.(0)).toBe(1);

    setPageActive(false);
    updatePoseBridge([pose]);
    expect(env.bb_evaluation_enabled?.(0)).toBe(0);

    setPageActive(true);
    expect(env.bb_evaluation_enabled?.(0)).toBe(0);
    updatePoseBridge([pose]);
    expect(env.bb_evaluation_enabled?.(0)).toBe(1);
  });

  it('clamps Duo and the second evaluation slot for one-pose devices', () => {
    const env = registeredEnv();
    const pose = (id: number): PoseSnapshot => ({
      id,
      quality: 0.9,
      keypoints: Array.from({ length: 17 }, () => ({ x: 0.5, y: 0.5, score: 0.9 })),
    });

    setLocalPoseCapacity(1);
    setSelectedGameMode(2);
    expect(env.bb_game_mode?.()).toBe(0);

    const runtimeModes: number[] = [];
    setRuntimeGameModeHandler((mode) => runtimeModes.push(mode));
    env.bb_set_game_mode?.(2);
    expect(runtimeModes).toEqual([0]);

    setPageActive(true);
    setCameraEvaluation(true);
    updatePoseBridge([pose(1), pose(2)]);
    expect(env.bb_evaluation_enabled?.(0)).toBe(1);
    expect(env.bb_evaluation_enabled?.(1)).toBe(0);
  });
});
