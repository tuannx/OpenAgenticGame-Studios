import { describe, expect, it } from 'vitest';
import { MotionNavigationController } from './motion-navigation';
import type { PoseSnapshot } from './vision';

function pose({ lean = 0, raised = false, quality = 0.9, id = 1 } = {}): PoseSnapshot {
  const keypoints = Array.from({ length: 17 }, () => ({ x: 0.5, y: 0.5, score: 0.9 }));
  keypoints[5] = { x: 0.45 + lean, y: 0.36, score: 0.9 };
  keypoints[6] = { x: 0.55 + lean, y: 0.36, score: 0.9 };
  keypoints[9] = { x: 0.43, y: raised ? 0.22 : 0.52, score: 0.9 };
  keypoints[10] = { x: 0.57, y: 0.52, score: 0.9 };
  keypoints[11] = { x: 0.46, y: 0.62, score: 0.9 };
  keypoints[12] = { x: 0.54, y: 0.62, score: 0.9 };
  return { id, keypoints, quality };
}

describe('hands-free motion navigation', () => {
  it('changes one mode per lean and rearms only after returning neutral', () => {
    const navigator = new MotionNavigationController('mirror');

    expect(navigator.update([pose({ lean: 0.08 })], 0).mode).toBe('strike');
    expect(navigator.update([pose({ lean: 0.08 })], 100).mode).toBe('strike');
    navigator.update([pose({ lean: 0 })], 200);
    expect(navigator.update([pose({ lean: -0.08 })], 300).mode).toBe('mirror');
  });

  it('confirms after one second of a continuously raised hand', () => {
    const navigator = new MotionNavigationController('mirror');

    expect(navigator.update([pose({ raised: true })], 100).confirmProgress).toBe(0);
    expect(navigator.update([pose({ raised: true })], 600).confirmProgress).toBeCloseTo(0.5);
    expect(navigator.update([pose({ raised: true })], 1_100).confirmed).toBe(true);
    expect(navigator.update([pose({ raised: true })], 1_200).confirmed).toBe(false);
  });

  it('resets confirmation when tracking or the raised-hand pose is lost', () => {
    const navigator = new MotionNavigationController('strike');

    navigator.update([pose({ raised: true })], 0);
    expect(navigator.update([pose({ raised: true })], 700).confirmProgress).toBeCloseTo(0.7);
    expect(navigator.update([], 800).confirmProgress).toBe(0);
    expect(navigator.update([pose({ raised: true })], 1_100).confirmProgress).toBe(0);
  });

  it('requires two tracked players with raised hands for duo mode', () => {
    const navigator = new MotionNavigationController('duo');

    const onePlayer = navigator.update([pose({ raised: true })], 0);
    expect(onePlayer.requiredPlayers).toBe(2);
    expect(onePlayer.confirmProgress).toBe(0);

    navigator.update([pose({ raised: true }), pose({ raised: true, id: 2 })], 100);
    expect(navigator.update([pose({ raised: true }), pose({ raised: true, id: 2 })], 1_100).confirmed).toBe(true);
  });

  it('ignores low-quality poses', () => {
    const navigator = new MotionNavigationController('mirror');
    const state = navigator.update([pose({ lean: 0.1, raised: true, quality: 0.1 })], 1_000);

    expect(state.mode).toBe('mirror');
    expect(state.trackedPlayers).toBe(0);
    expect(state.confirmProgress).toBe(0);
  });

  it('never navigates into a mode excluded by local pose capacity', () => {
    const navigator = new MotionNavigationController('duo', {}, ['mirror', 'strike']);

    expect(navigator.selectedMode()).toBe('mirror');
    expect(navigator.update([pose({ lean: -0.08 })], 0).mode).toBe('strike');
    navigator.update([pose({ lean: 0 })], 100);
    expect(navigator.update([pose({ lean: 0.08 })], 200).mode).toBe('mirror');
  });

  it('auto-invites Duo when a second body enters frame', () => {
    const navigator = new MotionNavigationController('mirror');

    expect(navigator.update([pose()], 0).mode).toBe('mirror');
    const invited = navigator.update([pose(), pose({ id: 2 })], 100);
    expect(invited.mode).toBe('duo');
    expect(invited.requiredPlayers).toBe(2);
    expect(invited.trackedPlayers).toBe(2);
  });

  it('respects lean away from Duo while two bodies remain', () => {
    const navigator = new MotionNavigationController('mirror');
    navigator.update([pose(), pose({ id: 2 })], 0);
    expect(navigator.selectedMode()).toBe('duo');

    navigator.update([pose({ lean: 0 }), pose({ id: 2 })], 50);
    const left = navigator.update([pose({ lean: -0.08 }), pose({ id: 2 })], 100);
    expect(left.mode).not.toBe('duo');
    expect(navigator.update([pose({ lean: 0 }), pose({ id: 2 })], 200).mode).not.toBe('duo');
  });

  it('re-invites Duo after the partner leaves and returns', () => {
    const navigator = new MotionNavigationController('mirror');
    navigator.update([pose(), pose({ id: 2 })], 0);
    navigator.update([pose({ lean: 0 }), pose({ id: 2 })], 50);
    navigator.update([pose({ lean: -0.08 }), pose({ id: 2 })], 100);
    expect(navigator.selectedMode()).not.toBe('duo');

    navigator.update([pose()], 200);
    expect(navigator.update([pose(), pose({ id: 2 })], 300).mode).toBe('duo');
  });
});
