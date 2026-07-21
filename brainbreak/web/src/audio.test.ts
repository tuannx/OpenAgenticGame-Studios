import { describe, expect, it } from 'vitest';
import { beatPulse, normalizedBeatPhase } from './audio';

describe('music clock', () => {
  it('normalizes beat phase at 120 bpm', () => {
    expect(normalizedBeatPhase(0, 120)).toBe(0);
    expect(normalizedBeatPhase(0.25, 120)).toBeCloseTo(0.5);
    expect(normalizedBeatPhase(0.5, 120)).toBe(0);
  });

  it('peaks at the beat boundary and falls between beats', () => {
    expect(beatPulse(0)).toBeCloseTo(1);
    expect(beatPulse(0.5)).toBeLessThan(0.01);
    expect(beatPulse(0.98)).toBeGreaterThan(beatPulse(0.25));
  });
});
