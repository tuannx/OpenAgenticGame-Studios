import { describe, expect, it } from 'vitest';
import { PARTY_VERBS, verbAt } from './party-verbs';

describe('party verbs', () => {
  it('ships a small detect+flavor set including royal wave', () => {
    expect(PARTY_VERBS.length).toBeGreaterThanOrEqual(6);
    expect(PARTY_VERBS.some((v) => v.key === 'paint' && v.sense === 'detect')).toBe(true);
    expect(PARTY_VERBS.some((v) => v.key === 'tiptoe' && v.sense === 'flavor')).toBe(true);
    expect(PARTY_VERBS.some((v) => v.key === 'royal_wave')).toBe(true);
  });

  it('rotates deterministically', () => {
    expect(verbAt(0).key).toBe(PARTY_VERBS[0]!.key);
    expect(verbAt(PARTY_VERBS.length).key).toBe(PARTY_VERBS[0]!.key);
  });
});
