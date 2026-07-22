export const MUSIC_TRACK = Object.freeze({
  title: 'Special Spotlight',
  artist: 'Kevin MacLeod',
  bpm: 126,
  beatOffsetSeconds: 0.06,
  url: '/audio/special-spotlight-db0e06528b9a.mp3',
  sourceUrl: 'https://incompetech.com/music/royalty-free/index.html?isrc=USUAN1600067&Search=Search',
  licenseUrl: 'https://creativecommons.org/licenses/by/4.0/',
});
export const MUSIC_PLAYBACK_RATE = 1;

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
  private frequencyData = new Uint8Array(128);
  private muted = false;
  private lastMetrics: MusicMetrics = { phase: 0, pulse: 0, energy: 0, playing: false };
  private metricsFrame = -1;

  async start(): Promise<void> {
    this.context ??= new AudioContext({ latencyHint: 'interactive' });
    if (!this.audio) {
      this.audio = new Audio(MUSIC_TRACK.url);
      this.audio.loop = true;
      this.audio.preload = 'auto';
      const source = this.context.createMediaElementSource(this.audio);
      this.analyser = this.context.createAnalyser();
      this.analyser.fftSize = 256;
      this.analyser.smoothingTimeConstant = 0.72;
      this.gain = this.context.createGain();
      this.gain.gain.value = this.muted ? 0 : 0.72;
      source.connect(this.analyser).connect(this.gain).connect(this.context.destination);
    }
    this.audio.playbackRate = MUSIC_PLAYBACK_RATE;
    await this.context.resume();
    await this.audio.play();
  }

  toggle(): boolean {
    this.muted = !this.muted;
    if (this.gain && this.context) {
      this.gain.gain.setTargetAtTime(this.muted ? 0 : 0.72, this.context.currentTime, 0.025);
    }
    return this.muted;
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
      // Orb collection / Boost / Clap
      if (typeof navigator !== 'undefined' && navigator.vibrate) {
        navigator.vibrate(40);
      }
      oscillator.type = 'sine';
      oscillator.frequency.setValueAtTime(540 * pitch, at);
      oscillator.frequency.exponentialRampToValueAtTime(920 * pitch, at + 0.14);
      gain.gain.setValueAtTime(0.085, at);
      gain.gain.exponentialRampToValueAtTime(0.001, at + 0.18);
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
      MUSIC_TRACK.bpm,
      playing ? MUSIC_TRACK.beatOffsetSeconds : 0,
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
