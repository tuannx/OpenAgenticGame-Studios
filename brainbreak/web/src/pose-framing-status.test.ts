import { describe, expect, it } from 'vitest';
import { presentPoseFraming } from './pose-framing-status';

describe('pose framing presentation', () => {
  it('keeps each recovery action singular and camera-distance readable', () => {
    expect(presentPoseFraming({ kind: 'move-back' }).instruction).toContain('STEP BACK');
    expect(presentPoseFraming({ kind: 'center' }).instruction).toContain('CENTER');
    expect(presentPoseFraming({ kind: 'show-body' }).instruction).toContain('HEAD');
  });

  it('distinguishes stable setup, Duo invitation, and active navigation', () => {
    expect(presentPoseFraming({ kind: 'steady' }).instruction).toContain('HOLD STILL');
    expect(presentPoseFraming({ kind: 'add-player', current: 1, required: 2 }).instruction).toContain('P2');
    expect(presentPoseFraming({ kind: 'ready', players: 1 }).instruction).toContain('READY');
  });

  it('uses static shape-companion copy without emoji or bullet joins', () => {
    const cases = [
      { kind: 'searching' } as const,
      { kind: 'move-closer' } as const,
      { kind: 'move-back' } as const,
      { kind: 'center' } as const,
      { kind: 'show-body' } as const,
      { kind: 'steady' } as const,
      { kind: 'add-player', current: 1, required: 2 } as const,
      { kind: 'ready', players: 1 } as const,
    ];
    for (const status of cases) {
      expect(presentPoseFraming(status).instruction).not.toMatch(/[•↔⬅➡🙌👀✨🔎🎯🙆👤]/u);
    }
  });
});
