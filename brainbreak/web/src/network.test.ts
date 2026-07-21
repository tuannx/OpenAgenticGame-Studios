import { describe, expect, it } from 'vitest';

describe('room protocol invariants', () => {
  it('keeps action payloads bounded and numeric', () => {
    const payload = JSON.stringify({ type: 'action', player: 1, mask: 64, at: 1234 });
    expect(new TextEncoder().encode(payload).byteLength).toBeLessThan(256);
    expect(JSON.parse(payload)).toMatchObject({ player: 1, mask: 64 });
  });

  it('uses the seven-bit public action mask', () => {
    const actions = [1, 2, 4, 8, 16, 32, 64];
    expect(actions.reduce((mask, action) => mask | action, 0)).toBe(127);
  });
});
