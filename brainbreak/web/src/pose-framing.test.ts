import { describe, expect, it } from 'vitest';
import { PoseFramingCoach } from './pose-framing';
import type { PoseSnapshot } from './vision';

function pose({
  center = 0.5,
  id = 1,
  quality = 0.9,
  shoulderWidth = 0.2,
  noseY = 0.14,
  hipY = 0.67,
  hidden = [],
}: {
  center?: number;
  id?: number;
  quality?: number;
  shoulderWidth?: number;
  noseY?: number;
  hipY?: number;
  hidden?: number[];
} = {}): PoseSnapshot {
  const keypoints = Array.from({ length: 17 }, () => ({ x: center, y: 0.5, score: 0.9 }));
  keypoints[0] = { x: center, y: noseY, score: 0.9 };
  keypoints[5] = { x: center - shoulderWidth / 2, y: 0.34, score: 0.9 };
  keypoints[6] = { x: center + shoulderWidth / 2, y: 0.34, score: 0.9 };
  keypoints[9] = { x: center - shoulderWidth / 2, y: 0.52, score: 0.9 };
  keypoints[10] = { x: center + shoulderWidth / 2, y: 0.52, score: 0.9 };
  keypoints[11] = { x: center - shoulderWidth / 3, y: hipY, score: 0.9 };
  keypoints[12] = { x: center + shoulderWidth / 3, y: hipY, score: 0.9 };
  hidden.forEach((index) => { keypoints[index] = { ...keypoints[index]!, score: 0.05 }; });
  return { id, keypoints, quality };
}

describe('camera framing coach', () => {
  it('blocks navigation until one usable pose remains stable for 400ms', () => {
    const coach = new PoseFramingCoach();

    expect(coach.update([pose()], 1, 100).status.kind).toBe('steady');
    expect(coach.update([pose()], 1, 499).navigationPoses).toHaveLength(0);
    const ready = coach.update([pose()], 1, 500);
    expect(ready.status.kind).toBe('ready');
    expect(ready.navigationPoses).toHaveLength(1);
  });

  it.each([
    ['move-back', pose({ shoulderWidth: 0.48 })],
    ['move-back', pose({ noseY: 0.02 })],
    ['move-back', pose({ hipY: 0.95 })],
    ['move-closer', pose({ shoulderWidth: 0.05 })],
    ['center', pose({ center: 0.06 })],
    ['show-body', pose({ hidden: [5, 9] })],
    ['show-body', pose({ hidden: [9, 10] })],
    ['show-body', pose({ quality: 0.2 })],
  ] as const)('returns %s and never forwards an unusable pose', (kind, sample) => {
    const state = new PoseFramingCoach().update([sample], 1, 1_000);
    expect(state.status.kind).toBe(kind);
    expect(state.navigationPoses).toHaveLength(0);
  });

  it('resets stability immediately when framing or tracked identity changes', () => {
    const coach = new PoseFramingCoach();
    coach.update([pose()], 1, 0);
    expect(coach.update([pose()], 1, 400).navigationPoses).toHaveLength(1);
    expect(coach.update([pose({ center: 0.04 })], 1, 410).navigationPoses).toHaveLength(0);
    expect(coach.update([pose({ id: 2 })], 1, 420).status.kind).toBe('steady');
    expect(coach.update([pose({ id: 2 })], 1, 819).navigationPoses).toHaveLength(0);
  });

  it('lets one stable Duo player navigate while asking for P2', () => {
    const coach = new PoseFramingCoach();
    coach.update([pose()], 2, 0);
    const state = coach.update([pose()], 2, 400);

    expect(state.status).toEqual({ kind: 'add-player', current: 1, required: 2 });
    expect(state.navigationPoses).toHaveLength(1);
  });

  it('requires both Duo identities to stabilize after the second player enters', () => {
    const coach = new PoseFramingCoach();
    coach.update([pose()], 2, 0);
    coach.update([pose()], 2, 400);
    expect(coach.update([pose(), pose({ id: 2, center: 0.72 })], 2, 500).status.kind).toBe('steady');
    const ready = coach.update([pose(), pose({ id: 2, center: 0.72 })], 2, 900);
    expect(ready.status.kind).toBe('ready');
    expect(ready.navigationPoses).toHaveLength(2);
  });
});
