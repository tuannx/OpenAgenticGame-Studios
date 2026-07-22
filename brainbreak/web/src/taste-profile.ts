import type { PlayableGameMode } from './motion-navigation';
import type { CameraOverlayMode, VisualTheme } from './vision';

export const TASTE_STORAGE_KEY = 'brainbreak.taste.v1';
const SCHEMA_VERSION = 1 as const;
const MAX_COUNTER = 1_000_000;
const MAX_READY_MS = 120_000;
const COMPACT_READY_MS = 8_000;

export type TasteOutcome = 'none' | 'gesture-success' | 'touch-fallback' | 'guide-only' | 'setup-exit';
export type RunOutcome = 'none' | 'break-complete' | 'energy-spent';

export interface TasteProfile {
  schemaVersion: typeof SCHEMA_VERSION;
  sessionsStarted: number;
  cameraAttempts: number;
  gestureStarts: number;
  touchFallbackStarts: number;
  guideOnlyStarts: number;
  setupExits: number;
  modeStarts: Record<PlayableGameMode, number>;
  modeCompletions: Record<PlayableGameMode, number>;
  breaksCompleted: number;
  energySpentRuns: number;
  lastRunOutcome: RunOutcome;
  gestureReadySamples: number;
  meanGestureReadyMs: number;
  lastOutcome: TasteOutcome;
  preferredTheme: VisualTheme;
  preferredOverlay: CameraOverlayMode;
  preferredCameraOpacity: number;
}

export interface TasteRecommendation {
  mode: PlayableGameMode | null;
  compactSetup: boolean;
  isReturning: boolean;
}

export interface TasteStorage {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
}

export function defaultTasteProfile(): TasteProfile {
  return {
    schemaVersion: SCHEMA_VERSION,
    sessionsStarted: 0,
    cameraAttempts: 0,
    gestureStarts: 0,
    touchFallbackStarts: 0,
    guideOnlyStarts: 0,
    setupExits: 0,
    modeStarts: { mirror: 0, strike: 0, duo: 0, supernova: 0 },
    modeCompletions: { mirror: 0, strike: 0, duo: 0, supernova: 0 },
    breaksCompleted: 0,
    energySpentRuns: 0,
    lastRunOutcome: 'none',
    gestureReadySamples: 0,
    meanGestureReadyMs: 0,
    lastOutcome: 'none',
    preferredTheme: 'cyber-trunk',
    preferredOverlay: 'glass_pip',
    preferredCameraOpacity: 80,
  };
}

function boundedInteger(value: unknown, fallback = 0, max = MAX_COUNTER): number {
  return typeof value === 'number' && Number.isFinite(value)
    ? Math.min(max, Math.max(0, Math.round(value)))
    : fallback;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function isOutcome(value: unknown): value is TasteOutcome {
  return value === 'none' || value === 'gesture-success' || value === 'touch-fallback'
    || value === 'guide-only' || value === 'setup-exit';
}

function isRunOutcome(value: unknown): value is RunOutcome {
  return value === 'none' || value === 'break-complete' || value === 'energy-spent';
}

function isTheme(value: unknown): value is VisualTheme {
  return value === 'cyber-trunk' || value === 'pink-girl' || value === 'purple-vaporwave';
}

function isOverlay(value: unknown): value is CameraOverlayMode {
  return value === 'glass_pip' || value === 'cutout' || value === 'hologram' || value === 'skeleton';
}

export function parseTasteProfile(raw: string | null): TasteProfile {
  if (!raw) return defaultTasteProfile();

  try {
    const value: unknown = JSON.parse(raw);
    if (!isRecord(value) || value.schemaVersion !== SCHEMA_VERSION) return defaultTasteProfile();

    const defaults = defaultTasteProfile();
    const rawModes = isRecord(value.modeStarts) ? value.modeStarts : {};
    const rawCompletions = isRecord(value.modeCompletions) ? value.modeCompletions : {};
    const gestureStarts = boundedInteger(value.gestureStarts);
    const gestureReadySamples = Math.min(boundedInteger(value.gestureReadySamples), gestureStarts);
    const preferredCameraOpacity = boundedInteger(value.preferredCameraOpacity, defaults.preferredCameraOpacity, 100);
    return {
      schemaVersion: SCHEMA_VERSION,
      sessionsStarted: boundedInteger(value.sessionsStarted),
      cameraAttempts: boundedInteger(value.cameraAttempts),
      gestureStarts,
      touchFallbackStarts: boundedInteger(value.touchFallbackStarts),
      guideOnlyStarts: boundedInteger(value.guideOnlyStarts),
      setupExits: boundedInteger(value.setupExits),
      modeStarts: {
        mirror: boundedInteger(rawModes.mirror),
        strike: boundedInteger(rawModes.strike),
        duo: boundedInteger(rawModes.duo),
        supernova: boundedInteger(rawModes.supernova),
      },
      modeCompletions: {
        mirror: boundedInteger(rawCompletions.mirror),
        strike: boundedInteger(rawCompletions.strike),
        duo: boundedInteger(rawCompletions.duo),
        supernova: boundedInteger(rawCompletions.supernova),
      },
      breaksCompleted: boundedInteger(value.breaksCompleted),
      energySpentRuns: boundedInteger(value.energySpentRuns),
      lastRunOutcome: isRunOutcome(value.lastRunOutcome)
        ? value.lastRunOutcome
        : defaults.lastRunOutcome,
      gestureReadySamples,
      meanGestureReadyMs: gestureReadySamples === 0 ? 0 : boundedInteger(value.meanGestureReadyMs, 0, MAX_READY_MS),
      lastOutcome: isOutcome(value.lastOutcome) ? value.lastOutcome : defaults.lastOutcome,
      preferredTheme: isTheme(value.preferredTheme) ? value.preferredTheme : defaults.preferredTheme,
      preferredOverlay: isOverlay(value.preferredOverlay) ? value.preferredOverlay : defaults.preferredOverlay,
      preferredCameraOpacity: Math.max(20, preferredCameraOpacity),
    };
  } catch {
    return defaultTasteProfile();
  }
}

function increment(value: number): number {
  return Math.min(MAX_COUNTER, value + 1);
}

export function recordSessionStarted(profile: TasteProfile): TasteProfile {
  return { ...profile, sessionsStarted: increment(profile.sessionsStarted) };
}

export function recordCameraAttempt(profile: TasteProfile): TasteProfile {
  return { ...profile, cameraAttempts: increment(profile.cameraAttempts) };
}

export function recordPlayStart(
  profile: TasteProfile,
  mode: PlayableGameMode,
  entry: 'gesture' | 'touch-fallback',
  readyMs?: number,
): TasteProfile {
  const next: TasteProfile = {
    ...profile,
    gestureStarts: entry === 'gesture' ? increment(profile.gestureStarts) : profile.gestureStarts,
    touchFallbackStarts: entry === 'touch-fallback' ? increment(profile.touchFallbackStarts) : profile.touchFallbackStarts,
    modeStarts: { ...profile.modeStarts, [mode]: increment(profile.modeStarts[mode]) },
    lastOutcome: entry === 'gesture' ? 'gesture-success' : 'touch-fallback',
  };

  if (entry !== 'gesture' || typeof readyMs !== 'number' || !Number.isFinite(readyMs)) return next;
  const sample = Math.min(MAX_READY_MS, Math.max(0, Math.round(readyMs)));
  const priorSampleCount = Math.min(profile.gestureReadySamples, MAX_COUNTER - 1);
  const sampleCount = priorSampleCount + 1;
  const weightedTotal = profile.meanGestureReadyMs * priorSampleCount + sample;
  return {
    ...next,
    gestureReadySamples: sampleCount,
    meanGestureReadyMs: Math.round(weightedTotal / sampleCount),
  };
}

export function recordGuideOnlyStart(profile: TasteProfile, mode: PlayableGameMode): TasteProfile {
  return {
    ...profile,
    guideOnlyStarts: increment(profile.guideOnlyStarts),
    modeStarts: { ...profile.modeStarts, [mode]: increment(profile.modeStarts[mode]) },
    lastOutcome: 'guide-only',
  };
}

export function recordSetupExit(profile: TasteProfile): TasteProfile {
  return { ...profile, setupExits: increment(profile.setupExits), lastOutcome: 'setup-exit' };
}

export function recordRunOutcome(
  profile: TasteProfile,
  mode: PlayableGameMode,
  outcome: Exclude<RunOutcome, 'none'>,
): TasteProfile {
  return {
    ...profile,
    modeCompletions: {
      ...profile.modeCompletions,
      [mode]: increment(profile.modeCompletions[mode]),
    },
    breaksCompleted: outcome === 'break-complete'
      ? increment(profile.breaksCompleted)
      : profile.breaksCompleted,
    energySpentRuns: outcome === 'energy-spent'
      ? increment(profile.energySpentRuns)
      : profile.energySpentRuns,
    lastRunOutcome: outcome,
  };
}

export function recordThemePreference(profile: TasteProfile, theme: VisualTheme): TasteProfile {
  return { ...profile, preferredTheme: theme };
}

export function recordOverlayPreference(profile: TasteProfile, overlay: CameraOverlayMode): TasteProfile {
  return { ...profile, preferredOverlay: overlay };
}

export function recordOpacityPreference(profile: TasteProfile, opacity: number): TasteProfile {
  const safeOpacity = Math.min(100, Math.max(20, Math.round(Number.isFinite(opacity) ? opacity : 80)));
  return { ...profile, preferredCameraOpacity: safeOpacity };
}

export function recommendTaste(profile: TasteProfile): TasteRecommendation {
  const modeOrder: readonly PlayableGameMode[] = ['mirror', 'strike', 'duo'];
  const mode = modeOrder.reduce<PlayableGameMode | null>((best, candidate) => {
    if (profile.modeStarts[candidate] < 2) return best;
    const candidateWeight = profile.modeStarts[candidate] + profile.modeCompletions[candidate] * 3;
    const bestWeight = best === null
      ? -1
      : profile.modeStarts[best] + profile.modeCompletions[best] * 3;
    if (candidateWeight > bestWeight) return candidate;
    return best;
  }, null);
  const difficultOutcomes = profile.touchFallbackStarts + profile.guideOnlyStarts + profile.setupExits;
  const completedOutcomes = profile.gestureStarts + difficultOutcomes;
  const difficultyRatio = completedOutcomes === 0 ? 0 : difficultOutcomes / completedOutcomes;

  return {
    mode,
    compactSetup: profile.gestureReadySamples >= 2
      && profile.meanGestureReadyMs <= COMPACT_READY_MS
      && profile.lastOutcome === 'gesture-success'
      && difficultyRatio <= 0.25,
    isReturning: profile.sessionsStarted > 0,
  };
}

export class LocalTasteStore {
  private profile: TasteProfile;

  constructor(private readonly storage: TasteStorage | null) {
    let raw: string | null = null;
    try {
      raw = storage?.getItem(TASTE_STORAGE_KEY) ?? null;
    } catch {
      // Storage may be blocked by browser privacy policy; in-memory defaults remain usable.
    }
    this.profile = parseTasteProfile(raw);
  }

  snapshot(): TasteProfile {
    return {
      ...this.profile,
      modeStarts: { ...this.profile.modeStarts },
      modeCompletions: { ...this.profile.modeCompletions },
    };
  }

  update(reducer: (profile: TasteProfile) => TasteProfile): TasteProfile {
    this.profile = reducer(this.profile);
    try {
      this.storage?.setItem(TASTE_STORAGE_KEY, JSON.stringify(this.profile));
    } catch {
      // A failed persistence write must never block camera setup or gameplay.
    }
    return this.snapshot();
  }
}

export function createLocalTasteStore(): LocalTasteStore {
  try {
    return new LocalTasteStore(window.localStorage);
  } catch {
    return new LocalTasteStore(null);
  }
}
