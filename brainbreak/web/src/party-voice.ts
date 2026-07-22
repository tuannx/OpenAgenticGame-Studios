/**
 * Party voice + freeze-dance audio director.
 *
 * Bridges Supernova Freeze Party phase events from the WASM game into:
 *  - Music pause/resume (music stopping IS the freeze cue — the classic game)
 *  - Kid-friendly voice commands via the browser SpeechSynthesis API
 *  - Synthesized celebration / freeze SFX via Web Audio
 *
 * Voice uses SpeechSynthesis so no audio assets are needed and it works
 * offline. Pitch is raised and rate slowed for a friendly, clear delivery
 * that kindergarten and grade-1 kids can follow.
 */

import { musicEngine } from './audio';

/** Supernova event kinds sent from the Rust game (see platform.rs). */
export const SupernovaEvent = {
  Countdown: 1,
  Dance: 2,
  Freeze: 3,
  PerfectFreeze: 4,
  Drop: 5,
  Result: 6,
} as const;

/**
 * Voice-clip manifest key → filename. Clips are pre-generated at build time
 * with the Kokoro small model (see scripts/generate-voice.mjs) and served from
 * /voice/. When a manifest is present the player prefers those consistent,
 * kid-friendly clips; otherwise it falls back to the device SpeechSynthesis API.
 */
const VOICE_MANIFEST_URL = '/voice/manifest.json';

class PartyVoice {
  private voice: SpeechSynthesisVoice | null = null;
  private enabled = true;
  private clipManifest: Record<string, string> | null = null;
  private clipBuffers = new Map<string, AudioBuffer>();
  private clipCheckStarted = false;

  constructor() {
    this.pickVoice();
    if (typeof speechSynthesis !== 'undefined') {
      // Voices load asynchronously in some browsers.
      speechSynthesis.addEventListener?.('voiceschanged', () => this.pickVoice());
    }
  }

  /** Rank available voices so we pick a clean, friendly one rather than the first. */
  private pickVoice(): void {
    if (typeof speechSynthesis === 'undefined') return;
    const voices = speechSynthesis.getVoices();
    const score = (v: SpeechSynthesisVoice): number => {
      const name = v.name.toLowerCase();
      let s = 0;
      if (name.includes('google')) s += 4; // Chrome's Google voices are clear & pleasant
      if (name.includes('natural') || name.includes('premium') || name.includes('enhanced')) s += 3;
      if (name.includes('samantha') || name.includes('ava') || name.includes('allison')) s += 2; // macOS quality
      if (v.lang === 'en-US') s += 1;
      if (v.localService) s += 1;
      return s;
    };
    const english = voices.filter((v) => v.lang.startsWith('en'));
    const ranked = [...english].sort((a, b) => score(b) - score(a));
    this.voice = ranked[0] ?? voices[0] ?? null;
  }

  private audioContext(): AudioContext | undefined {
    return (musicEngine as unknown as { context?: AudioContext }).context;
  }

  /** One-time check for pre-generated voice clips (Kokoro build output). */
  private async ensureClipsChecked(): Promise<void> {
    if (this.clipCheckStarted) return;
    this.clipCheckStarted = true;
    try {
      const res = await fetch(VOICE_MANIFEST_URL);
      if (!res.ok) return; // no clips shipped -> SpeechSynthesis only
      const manifest = (await res.json()) as Record<string, string>;
      this.clipManifest = manifest;
    } catch {
      // Offline or manifest absent; SpeechSynthesis remains the fallback.
    }
  }

  private async clipBuffer(key: string): Promise<AudioBuffer | null> {
    await this.ensureClipsChecked();
    const manifest = this.clipManifest;
    const context = this.audioContext();
    if (!manifest || !context) return null;
    const cached = this.clipBuffers.get(key);
    if (cached) return cached;
    const file = manifest[key];
    if (!file) return null;
    try {
      const res = await fetch(`/voice/${file}`);
      if (!res.ok) return null;
      const buffer = await context.decodeAudioData(await res.arrayBuffer());
      this.clipBuffers.set(key, buffer);
      return buffer;
    } catch {
      return null;
    }
  }

  setEnabled(enabled: boolean): void {
    this.enabled = enabled;
    if (!enabled && typeof speechSynthesis !== 'undefined') {
      speechSynthesis.cancel();
    }
  }

  /**
   * Speak a command by key. Prefers a pre-generated Kokoro clip when available;
   * otherwise falls back to the device SpeechSynthesis voice.
   */
  async speak(key: string, text: string, opts?: { pitch?: number; rate?: number }): Promise<void> {
    if (!this.enabled) return;
    const context = this.audioContext();
    const buffer = await this.clipBuffer(key);
    if (buffer && context && context.state === 'running') {
      if (typeof speechSynthesis !== 'undefined') speechSynthesis.cancel();
      const source = context.createBufferSource();
      source.buffer = buffer;
      source.connect(context.destination);
      source.start();
      return;
    }
    this.say(text, opts);
  }

  /** Speak a short kid-friendly phrase via SpeechSynthesis. Cancels in-flight speech. */
  say(text: string, opts?: { pitch?: number; rate?: number }): void {
    if (!this.enabled || typeof speechSynthesis === 'undefined') return;
    try {
      speechSynthesis.cancel();
      const utterance = new SpeechSynthesisUtterance(text);
      if (this.voice) utterance.voice = this.voice;
      utterance.pitch = opts?.pitch ?? 1.3; // higher = friendlier for kids
      utterance.rate = opts?.rate ?? 0.95; // slightly slower for clarity
      utterance.volume = 1.0;
      speechSynthesis.speak(utterance);
    } catch {
      // Speech is a nice-to-have; never break the game over it.
    }
  }
}

class PartyDirector {
  private voice = new PartyVoice();
  private lastCountdown = -1;

  /** Handle a Supernova phase event from the WASM game. */
  handleEvent(kind: number, value: number): void {
    switch (kind) {
      case SupernovaEvent.Countdown:
        this.onCountdown(value);
        break;
      case SupernovaEvent.Dance:
        this.onDance();
        break;
      case SupernovaEvent.Freeze:
        this.onFreeze();
        break;
      case SupernovaEvent.PerfectFreeze:
        this.onPerfectFreeze();
        break;
      case SupernovaEvent.Drop:
        this.onDrop();
        break;
      case SupernovaEvent.Result:
        this.onResult(value);
        break;
      default:
        break;
    }
  }

  private onCountdown(num: number): void {
    // num = 2, 1 during countdown; 0 = GO!
    if (num === this.lastCountdown) return;
    this.lastCountdown = num;
    if (num > 0) {
      void this.voice.speak(`count_${num}`, String(num), { pitch: 1.4, rate: 1.0 });
      this.playTick(num);
    } else {
      void this.voice.speak('go', 'Go go go!', { pitch: 1.5, rate: 1.1 });
      this.playGo();
    }
  }

  private onDance(): void {
    this.lastCountdown = -1;
    musicEngine.resume();
    void this.voice.speak('dance', 'Dance!', { pitch: 1.4, rate: 1.0 });
  }

  private onFreeze(): void {
    musicEngine.pause(); // music stopping is the freeze signal
    this.playFreezeShimmer();
    void this.voice.speak('freeze', 'Freeze!', { pitch: 1.2, rate: 0.85 });
  }

  private onPerfectFreeze(): void {
    this.playChime();
    void this.voice.speak('great_job', 'Great job!', { pitch: 1.4, rate: 1.0 });
  }

  private onDrop(): void {
    musicEngine.resume();
    this.playCelebration();
    void this.voice.speak('super_nova', 'Super nova!', { pitch: 1.5, rate: 1.05 });
  }

  private onResult(outcome: number): void {
    // outcome: 0 = full supernova, 1 = time expired
    if (outcome === 0) {
      void this.voice.speak('wow', 'Wow! You did it!', { pitch: 1.4, rate: 1.0 });
    } else {
      void this.voice.speak('yay', 'Yay! Nice dancing!', { pitch: 1.4, rate: 1.0 });
    }
  }

  // --- Synthesized SFX (Web Audio, no assets) ---

  private context(): AudioContext | undefined {
    return (musicEngine as unknown as { context?: AudioContext }).context;
  }

  private playTick(num: number): void {
    const ctx = this.context();
    if (!ctx || ctx.state !== 'running') return;
    const at = ctx.currentTime;
    const osc = ctx.createOscillator();
    const gain = ctx.createGain();
    osc.type = 'sine';
    osc.frequency.setValueAtTime(num === 1 ? 660 : 520, at);
    gain.gain.setValueAtTime(0.12, at);
    gain.gain.exponentialRampToValueAtTime(0.001, at + 0.15);
    osc.connect(gain).connect(ctx.destination);
    osc.start(at);
    osc.stop(at + 0.18);
  }

  private playGo(): void {
    const ctx = this.context();
    if (!ctx || ctx.state !== 'running') return;
    const at = ctx.currentTime;
    const osc = ctx.createOscillator();
    const gain = ctx.createGain();
    osc.type = 'triangle';
    osc.frequency.setValueAtTime(523, at);
    osc.frequency.exponentialRampToValueAtTime(1046, at + 0.2);
    gain.gain.setValueAtTime(0.15, at);
    gain.gain.exponentialRampToValueAtTime(0.001, at + 0.3);
    osc.connect(gain).connect(ctx.destination);
    osc.start(at);
    osc.stop(at + 0.32);
  }

  /** Icy descending shimmer for the freeze moment. */
  private playFreezeShimmer(): void {
    const ctx = this.context();
    if (!ctx || ctx.state !== 'running') return;
    const at = ctx.currentTime;
    for (let i = 0; i < 3; i += 1) {
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.type = 'sine';
      const start = 1400 - i * 300;
      osc.frequency.setValueAtTime(start, at + i * 0.06);
      osc.frequency.exponentialRampToValueAtTime(start * 0.5, at + i * 0.06 + 0.25);
      gain.gain.setValueAtTime(0.06, at + i * 0.06);
      gain.gain.exponentialRampToValueAtTime(0.001, at + i * 0.06 + 0.3);
      osc.connect(gain).connect(ctx.destination);
      osc.start(at + i * 0.06);
      osc.stop(at + i * 0.06 + 0.32);
    }
  }

  /** Bright ascending chime for a perfect freeze star. */
  private playChime(): void {
    const ctx = this.context();
    if (!ctx || ctx.state !== 'running') return;
    const at = ctx.currentTime;
    const notes = [523, 659, 784, 1046]; // C5 E5 G5 C6
    notes.forEach((freq, i) => {
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.type = 'sine';
      osc.frequency.setValueAtTime(freq, at + i * 0.07);
      gain.gain.setValueAtTime(0.1, at + i * 0.07);
      gain.gain.exponentialRampToValueAtTime(0.001, at + i * 0.07 + 0.25);
      osc.connect(gain).connect(ctx.destination);
      osc.start(at + i * 0.07);
      osc.stop(at + i * 0.07 + 0.28);
    });
  }

  /** Big celebration arpeggio for the supernova drop. */
  private playCelebration(): void {
    const ctx = this.context();
    if (!ctx || ctx.state !== 'running') return;
    const at = ctx.currentTime;
    const notes = [392, 523, 659, 784, 1046, 1318]; // G4→E6 rising
    notes.forEach((freq, i) => {
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.type = 'triangle';
      osc.frequency.setValueAtTime(freq, at + i * 0.08);
      gain.gain.setValueAtTime(0.13, at + i * 0.08);
      gain.gain.exponentialRampToValueAtTime(0.001, at + i * 0.08 + 0.4);
      osc.connect(gain).connect(ctx.destination);
      osc.start(at + i * 0.08);
      osc.stop(at + i * 0.08 + 0.42);
    });
  }
}

export const partyDirector = new PartyDirector();
