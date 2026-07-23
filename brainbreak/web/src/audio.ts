export type MusicTrackId = 'runner' | 'lava-freeze';

export interface MusicTrack {
  id: MusicTrackId;
  title: string;
  artist: string;
  bpm: number;
  beatOffsetSeconds: number;
  url: string;
  sourceUrl: string;
  licenseUrl: string;
  statusLabel: string;
}

/** Neon Jump Party — BrainBreak runner bed (studio original, CC0). */
export const RUNNER_TRACK: MusicTrack = Object.freeze({
  id: 'runner',
  title: 'Neon Jump Party',
  artist: 'OpenAgenticGame Studios',
  bpm: 140,
  beatOffsetSeconds: 0.0,
  url: '/audio/neon-jump-party-ba01a8912e1b.mp3',
  sourceUrl: 'studio://neon-jump-party',
  licenseUrl: 'https://creativecommons.org/publicdomain/zero/1.0/',
  statusLabel: 'Neon Jump Party • 140 BPM',
});

/**
 * Supernova Floor-is-Lava / elemental Freeze Party bed.
 * Studio-owned procedural composition (Danny Go taste/format only — original IP).
 */
export const LAVA_FREEZE_TRACK: MusicTrack = Object.freeze({
  id: 'lava-freeze',
  title: 'Lava Freeze Party',
  artist: 'OpenAgenticGame Studios',
  bpm: 140,
  beatOffsetSeconds: 0.0,
  url: '/audio/lava-freeze-party-3ee285325456.mp3',
  sourceUrl: 'studio://lava-freeze-party',
  licenseUrl: 'https://creativecommons.org/publicdomain/zero/1.0/',
  statusLabel: 'Lava Freeze Party • 140 BPM',
});

/** @deprecated Prefer RUNNER_TRACK / activeTrack(); kept for existing imports. */
export const MUSIC_TRACK = RUNNER_TRACK;

export const MUSIC_PLAYBACK_RATE = 1;

export function trackForProductFamily(family: 'brainbreak' | 'ar'): MusicTrack {
  return family === 'ar' ? LAVA_FREEZE_TRACK : RUNNER_TRACK;
}

export interface MusicMetrics {
  phase: number;
  pulse: number;
  energy: number;
  playing: boolean;
}

export function normalizedBeatPhase(
  timeSeconds: number,
  bpm: number,
  offsetSeconds = 0,
): number {
  if (!Number.isFinite(timeSeconds) || !Number.isFinite(bpm) || bpm <= 0) return 0;
  const beats = (timeSeconds - offsetSeconds) * bpm / 60;
  return ((beats % 1) + 1) % 1;
}

export function beatPulse(phase: number): number {
  const normalized = ((phase % 1) + 1) % 1;
  const distance = Math.min(normalized, 1 - normalized);
  return Math.exp(-distance * 13);
}

export function comboPitchRatio(combo: number): number {
  const safeCombo = Number.isFinite(combo) ? Math.max(0, Math.floor(combo)) : 0;
  const semitones = Math.min(4, Math.floor(safeCombo / 5));
  return 2 ** (semitones / 12);
}

class MusicEngine {
  private context?: AudioContext;
  private audio?: HTMLAudioElement;
  private analyser?: AnalyserNode;
  private gain?: GainNode;
  private bassShelf?: BiquadFilterNode;
  private frequencyData = new Uint8Array(128);
  private muted = false;
  private dropTension = false;
  private duckLevel = 1;
  private track: MusicTrack = RUNNER_TRACK;
  private lastMetrics: MusicMetrics = { phase: 0, pulse: 0, energy: 0, playing: false };
  private metricsFrame = -1;

  private bedGainTarget(): number {
    if (this.muted) return 0;
    return 0.72 * this.duckLevel;
  }

  private applyBedGain(rampSeconds = 0.06): void {
    if (!this.gain || !this.context) return;
    const at = this.context.currentTime;
    this.gain.gain.cancelScheduledValues(at);
    this.gain.gain.setValueAtTime(this.gain.gain.value, at);
    this.gain.gain.linearRampToValueAtTime(this.bedGainTarget(), at + Math.max(0.02, rampSeconds));
  }

  activeTrack(): MusicTrack {
    return this.track;
  }

  /** Select bed before start(), or swap URL if already wired. */
  selectTrack(track: MusicTrack): void {
    if (this.track.id === track.id) return;
    this.track = track;
    if (this.audio) {
      const wasPlaying = !this.audio.paused;
      this.audio.src = track.url;
      this.audio.load();
      if (wasPlaying) {
        void this.audio.play().catch(() => undefined);
      }
    }
  }

  async start(track: MusicTrack = this.track): Promise<void> {
    this.selectTrack(track);
    this.context ??= new AudioContext({ latencyHint: 'interactive' });
    if (!this.audio) {
      this.audio = new Audio(this.track.url);
      this.audio.loop = true;
      this.audio.preload = 'auto';
      const source = this.context.createMediaElementSource(this.audio);
      this.analyser = this.context.createAnalyser();
      this.analyser.fftSize = 256;
      this.analyser.smoothingTimeConstant = 0.72;
      this.bassShelf = this.context.createBiquadFilter();
      this.bassShelf.type = 'lowshelf';
      this.bassShelf.frequency.value = 140;
      this.bassShelf.gain.value = 0;
      this.gain = this.context.createGain();
      this.gain.gain.value = this.bedGainTarget();
      // Shelf before analyser so Drop tension lifts both heard bass and neon energy.
      source
        .connect(this.bassShelf)
        .connect(this.analyser)
        .connect(this.gain)
        .connect(this.context.destination);
    }
    this.audio.playbackRate = MUSIC_PLAYBACK_RATE;
    await this.context.resume();
    await this.audio.play();
  }

  /** Pre-Drop tension: lift the music's low shelf so Analyser bass energy swells. */
  setDropTension(active: boolean): void {
    this.dropTension = active;
    if (!this.bassShelf || !this.context) return;
    const at = this.context.currentTime;
    this.bassShelf.gain.cancelScheduledValues(at);
    this.bassShelf.gain.setValueAtTime(this.bassShelf.gain.value, at);
    this.bassShelf.gain.linearRampToValueAtTime(active ? 9 : 0, at + (active ? 0.35 : 0.12));
  }

  toggle(): boolean {
    this.muted = !this.muted;
    this.applyBedGain(0.05);
    return this.muted;
  }

  /**
   * Duck the music bed under spoken coach lines (Danny Go call-and-response mix).
   * level 0 = silent bed, 1 = full. Freeze pauses music separately.
   */
  duck(level = 0.28, rampSeconds = 0.08): void {
    this.duckLevel = Math.min(1, Math.max(0, level));
    this.applyBedGain(rampSeconds);
  }

  unduck(rampSeconds = 0.18): void {
    this.duckLevel = 1;
    this.applyBedGain(rampSeconds);
  }

  /** Pause the music (used by Freeze Dance — music stopping IS the freeze cue). */
  pause(): void {
    if (this.audio && !this.audio.paused) {
      this.audio.pause();
    }
  }

  /** Resume the music from where it paused. */
  resume(): void {
    if (this.audio && this.audio.paused && this.context?.state === 'running') {
      void this.audio.play().catch(() => undefined);
    }
  }

  get isPlaying(): boolean {
    return Boolean(this.audio && !this.audio.paused && !this.audio.ended);
  }

  get isMuted(): boolean {
    return this.muted;
  }

  playFeedback(kind: number, comboStreak = 0): void {
    const context = this.context;
    if (!context || context.state !== 'running') return;
    const at = context.currentTime;
    const oscillator = context.createOscillator();
    const gain = context.createGain();
    const pitch = kind === 2 ? 1 : comboPitchRatio(comboStreak);
    if (kind === 2) {
      // Miss / Hazard collision
      if (typeof navigator !== 'undefined' && navigator.vibrate) {
        navigator.vibrate([100, 50, 100]);
      }
      oscillator.type = 'sawtooth';
      oscillator.frequency.setValueAtTime(150, at);
      oscillator.frequency.exponentialRampToValueAtTime(52, at + 0.18);
      gain.gain.setValueAtTime(0.11, at);
      gain.gain.exponentialRampToValueAtTime(0.001, at + 0.2);
    } else if (kind === 3) {
      // Orb collection / Boost / Clap — punchier downbeat reward
      if (typeof navigator !== 'undefined' && navigator.vibrate) {
        navigator.vibrate(40);
      }
      oscillator.type = 'sine';
      oscillator.frequency.setValueAtTime(540 * pitch, at);
      oscillator.frequency.exponentialRampToValueAtTime(980 * pitch, at + 0.14);
      gain.gain.setValueAtTime(0.11, at);
      gain.gain.exponentialRampToValueAtTime(0.001, at + 0.2);
    } else {
      oscillator.type = 'triangle';
      oscillator.frequency.setValueAtTime(290 * pitch, at);
      oscillator.frequency.exponentialRampToValueAtTime(430 * pitch, at + 0.08);
      gain.gain.setValueAtTime(0.04, at);
      gain.gain.exponentialRampToValueAtTime(0.001, at + 0.1);
    }
    oscillator.connect(gain).connect(context.destination);
    oscillator.start(at);
    oscillator.stop(at + 0.22);
  }

  metrics(): MusicMetrics {
    const frame = Math.floor(performance.now() / 12);
    if (frame === this.metricsFrame) return this.lastMetrics;
    this.metricsFrame = frame;

    const audio = this.audio;
    const playing = Boolean(audio && !audio.paused && !audio.ended);
    const time = playing && audio ? audio.currentTime : performance.now() / 1000;
    const phase = normalizedBeatPhase(
      time,
      this.track.bpm,
      playing ? this.track.beatOffsetSeconds : 0,
    );
    let energy = 0;
    if (playing && this.analyser) {
      this.analyser.getByteFrequencyData(this.frequencyData);
      let weighted = 0;
      let weights = 0;
      for (let index = 0; index < this.frequencyData.length; index += 1) {
        const weight = index < 18 ? 1.7 : index < 52 ? 1 : 0.35;
        weighted += (this.frequencyData[index] ?? 0) * weight;
        weights += weight;
      }
      energy = Math.min(1, weighted / Math.max(1, weights * 170));
      if (this.dropTension) {
        energy = Math.min(1, energy * 1.18 + 0.08);
      }
    }
    this.lastMetrics = {
      phase,
      pulse: beatPulse(phase),
      energy,
      playing,
    };
    return this.lastMetrics;
  }
}

export const musicEngine = new MusicEngine();
