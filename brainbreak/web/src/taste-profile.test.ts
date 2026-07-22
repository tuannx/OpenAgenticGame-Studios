import { describe, expect, it } from 'vitest';
import {
  LocalTasteStore,
  TASTE_STORAGE_KEY,
  defaultTasteProfile,
  parseTasteProfile,
  recommendTaste,
  recordGuideOnlyStart,
  recordOpacityPreference,
  recordPlayStart,
  recordRunOutcome,
  recordSessionStarted,
  recordSetupExit,
  recordThemePreference,
} from './taste-profile';

describe('local taste profile', () => {
  it('falls back to safe defaults for corrupt or incompatible storage', () => {
    expect(parseTasteProfile('{broken')).toEqual(defaultTasteProfile());
    expect(parseTasteProfile(JSON.stringify({ schemaVersion: 99, modeStarts: { mirror: 99 } })))
      .toEqual(defaultTasteProfile());
  });

  it('bounds restored numeric and preference values', () => {
    const parsed = parseTasteProfile(JSON.stringify({
      ...defaultTasteProfile(),
      sessionsStarted: -8,
      gestureStarts: 1,
      gestureReadySamples: 99,
      meanGestureReadyMs: Number.MAX_VALUE,
      preferredCameraOpacity: 2,
      preferredTheme: 'not-a-theme',
      modeCompletions: { mirror: -3, strike: Number.MAX_VALUE, duo: 2 },
      breaksCompleted: Number.MAX_VALUE,
      lastRunOutcome: 'not-an-outcome',
    }));

    expect(parsed.sessionsStarted).toBe(0);
    expect(parsed.gestureReadySamples).toBe(1);
    expect(parsed.meanGestureReadyMs).toBe(120_000);
    expect(parsed.preferredCameraOpacity).toBe(20);
    expect(parsed.preferredTheme).toBe('cyber-trunk');
    expect(parsed.modeCompletions).toEqual({ mirror: 0, strike: 1_000_000, duo: 2, supernova: 0 });
    expect(parsed.breaksCompleted).toBe(1_000_000);
    expect(parsed.lastRunOutcome).toBe('none');
  });

  it('migrates pre-outcome profiles without losing existing taste', () => {
    const legacy = defaultTasteProfile();
    const { modeCompletions: _modeCompletions, breaksCompleted: _breaksCompleted,
      energySpentRuns: _energySpentRuns, lastRunOutcome: _lastRunOutcome, ...preOutcome } = legacy;
    const parsed = parseTasteProfile(JSON.stringify({
      ...preOutcome,
      preferredTheme: 'pink-girl',
      modeStarts: { mirror: 4, strike: 1, duo: 0 },
    }));

    expect(parsed.preferredTheme).toBe('pink-girl');
    expect(parsed.modeStarts.mirror).toBe(4);
    expect(parsed.modeCompletions).toEqual({ mirror: 0, strike: 0, duo: 0, supernova: 0 });
    expect(parsed.lastRunOutcome).toBe('none');
  });

  it('waits for a stable mode signal before recommending it', () => {
    let profile = recordPlayStart(defaultTasteProfile(), 'strike', 'gesture', 4_000);
    expect(recommendTaste(profile).mode).toBeNull();
    profile = recordPlayStart(profile, 'strike', 'gesture', 6_000);

    expect(recommendTaste(profile)).toMatchObject({ mode: 'strike', compactSetup: true });
    expect(profile.meanGestureReadyMs).toBe(5_000);
  });

  it('weights completed runs more strongly than starts', () => {
    let profile = defaultTasteProfile();
    profile = recordPlayStart(profile, 'mirror', 'gesture', 4_000);
    profile = recordPlayStart(profile, 'mirror', 'gesture', 4_000);
    profile = recordPlayStart(profile, 'strike', 'gesture', 4_000);
    profile = recordPlayStart(profile, 'strike', 'gesture', 4_000);
    expect(recommendTaste(profile).mode).toBe('mirror');

    profile = recordRunOutcome(profile, 'strike', 'energy-spent');
    expect(recommendTaste(profile).mode).toBe('strike');
    expect(profile.modeCompletions.strike).toBe(1);
    expect(profile.energySpentRuns).toBe(1);
    expect(profile.lastRunOutcome).toBe('energy-spent');

    profile = recordRunOutcome(profile, 'strike', 'break-complete');
    expect(profile.breaksCompleted).toBe(1);
    expect(profile.lastRunOutcome).toBe('break-complete');
  });

  it('restores full guidance after fallback, guide-only, or setup exit', () => {
    let profile = recordPlayStart(defaultTasteProfile(), 'mirror', 'gesture', 4_000);
    profile = recordPlayStart(profile, 'mirror', 'gesture', 4_000);
    expect(recommendTaste(profile).compactSetup).toBe(true);

    expect(recommendTaste(recordPlayStart(profile, 'mirror', 'touch-fallback', 4_000)).compactSetup).toBe(false);
    expect(recommendTaste(recordGuideOnlyStart(profile, 'mirror')).compactSetup).toBe(false);
    expect(recommendTaste(recordSetupExit(profile)).compactSetup).toBe(false);
  });

  it('serializes only constant-size aggregate data', () => {
    let profile = recordSessionStarted(defaultTasteProfile());
    profile = recordPlayStart(profile, 'duo', 'gesture', 5_500);
    profile = recordThemePreference(profile, 'purple-vaporwave');
    profile = recordOpacityPreference(profile, 500);
    const serialized = JSON.stringify(profile);

    expect(profile.preferredCameraOpacity).toBe(100);
    expect(serialized).not.toMatch(/pose|keypoint|image|video|frame/i);
  });

  it('continues in memory when browser storage reads or writes fail', () => {
    const store = new LocalTasteStore({
      getItem: () => { throw new Error('blocked'); },
      setItem: () => { throw new Error('quota'); },
    });

    expect(store.update(recordSessionStarted).sessionsStarted).toBe(1);
    expect(store.snapshot().sessionsStarted).toBe(1);
    expect(TASTE_STORAGE_KEY).toBe('brainbreak.taste.v1');
  });
});
