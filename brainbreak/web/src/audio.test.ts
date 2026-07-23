import { describe, expect, it } from 'vitest';
import {
  LAVA_FREEZE_TRACK,
  MUSIC_PLAYBACK_RATE,
  RUNNER_TRACK,
  beatPulse,
  comboPitchRatio,
  normalizedBeatPhase,
  trackForProductFamily,
} from './audio';

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

  it('raises feedback pitch one semitone per five combo and caps at four', () => {
    expect(MUSIC_PLAYBACK_RATE).toBe(1);
    expect(comboPitchRatio(0)).toBe(1);
    expect(comboPitchRatio(4)).toBe(1);
    expect(comboPitchRatio(5)).toBeCloseTo(2 ** (1 / 12));
    expect(comboPitchRatio(19)).toBeCloseTo(2 ** (3 / 12));
    expect(comboPitchRatio(20)).toBeCloseTo(2 ** (4 / 12));
    expect(comboPitchRatio(500)).toBeCloseTo(2 ** (4 / 12));
    expect(comboPitchRatio(Number.NaN)).toBe(1);
  });

  it('selects lava-freeze bed for AR and runner bed for BrainBreak', () => {
    expect(trackForProductFamily('ar')).toBe(LAVA_FREEZE_TRACK);
    expect(trackForProductFamily('brainbreak')).toBe(RUNNER_TRACK);
    expect(LAVA_FREEZE_TRACK.bpm).toBe(140);
    expect(RUNNER_TRACK.bpm).toBe(140);
    expect(RUNNER_TRACK.title).toBe('Neon Jump Party');
    expect(LAVA_FREEZE_TRACK.title).toBe('Lava Freeze Party');
    expect(RUNNER_TRACK.url).toContain('neon-jump-party-');
    expect(LAVA_FREEZE_TRACK.url).toContain('lava-freeze-party-');
  });
});
