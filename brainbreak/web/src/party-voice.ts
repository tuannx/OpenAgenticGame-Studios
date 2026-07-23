/**
 * Party audio director — voice + music + SFX (triple channel with pictograms).
 *
 * Kids at 1.5–4m cannot read. Key moments speak short coach lines while music
 * beds/stingers keep rhythm. Prefers optional /voice clips; falls back to
 * SpeechSynthesis; SFX+pictograms still work if speech is unavailable.
 */

import { musicEngine } from './audio';
import {
  VOICE_CUES,
  countdownCueId,
  type VoiceCue,
  type VoiceCueId,
} from './party-voice-cues';

/** Supernova event kinds sent from the Rust game (see platform.rs). */
export const SupernovaEvent = {
  Countdown: 1,
  Dance: 2,
  Freeze: 3,
  PerfectFreeze: 4,
  Drop: 5,
  Result: 6,
  DropImminent: 7,
  LavaWarning: 8,
} as const;

const VOICE_MANIFEST_URL = '/voice/manifest.json';
const SHELL_THROTTLE_MS = 2_400;
const DUCK_SAFETY_MS = 2_800;

class PartyVoice {
  private voice: SpeechSynthesisVoice | null = null;
  private enabled = true;
  private clipManifest: Record<string, string> | null = null;
  private clipBuffers = new Map<string, AudioBuffer>();
  private clipCheckStarted = false;
  private duckTimer: ReturnType<typeof setTimeout> | null = null;
  private speaking = false;

  constructor() {
    this.pickVoice();
    if (typeof speechSynthesis !== 'undefined') {
      speechSynthesis.addEventListener?.('voiceschanged', () => this.pickVoice());
    }
  }

  private pickVoice(): void {
    if (typeof speechSynthesis === 'undefined') return;
    const voices = speechSynthesis.getVoices();
    const score = (v: SpeechSynthesisVoice): number => {
      const name = v.name.toLowerCase();
      let s = 0;
      if (name.includes('google')) s += 4;
      if (name.includes('natural') || name.includes('premium') || name.includes('enhanced')) s += 3;
      if (name.includes('samantha') || name.includes('ava') || name.includes('allison')) s += 2;
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

  private async ensureClipsChecked(): Promise<void> {
    if (this.clipCheckStarted) return;
    this.clipCheckStarted = true;
    try {
      const res = await fetch(VOICE_MANIFEST_URL);
      if (!res.ok) return;
      this.clipManifest = (await res.json()) as Record<string, string>;
    } catch {
      // Clips optional — SpeechSynthesis remains the path.
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
    if (!enabled) {
      if (typeof speechSynthesis !== 'undefined') speechSynthesis.cancel();
      this.clearDuck();
    }
  }

  private beginDuck(cue: VoiceCue): void {
    if (cue.silenceBed) return;
    musicEngine.duck(0.26, 0.07);
    if (this.duckTimer) clearTimeout(this.duckTimer);
    this.duckTimer = setTimeout(() => this.clearDuck(), DUCK_SAFETY_MS);
  }

  private clearDuck(): void {
    if (this.duckTimer) {
      clearTimeout(this.duckTimer);
      this.duckTimer = null;
    }
    musicEngine.unduck(0.2);
    this.speaking = false;
  }

  /** Prefer clip, then Web Speech; never throws. */
  async speakCue(cue: VoiceCue): Promise<void> {
    if (!this.enabled) return;
    const context = this.audioContext();
    const buffer = await this.clipBuffer(cue.id);
    if (buffer && context && context.state === 'running') {
      if (typeof speechSynthesis !== 'undefined') speechSynthesis.cancel();
      this.beginDuck(cue);
      this.speaking = true;
      const source = context.createBufferSource();
      source.buffer = buffer;
      source.connect(context.destination);
      source.onended = () => this.clearDuck();
      source.start();
      return;
    }
    this.say(cue.text, cue);
  }

  say(text: string, cue?: VoiceCue): void {
    if (!this.enabled || typeof speechSynthesis === 'undefined') return;
    try {
      speechSynthesis.cancel();
      const utterance = new SpeechSynthesisUtterance(text);
      if (this.voice) utterance.voice = this.voice;
      utterance.pitch = cue?.pitch ?? 1.3;
      utterance.rate = cue?.rate ?? 0.95;
      utterance.volume = 1;
      if (cue) this.beginDuck(cue);
      this.speaking = true;
      utterance.onend = () => this.clearDuck();
      utterance.onerror = () => this.clearDuck();
      speechSynthesis.speak(utterance);
    } catch {
      this.clearDuck();
    }
  }

  get isSpeaking(): boolean {
    return this.speaking;
  }
}

export type ShellGuideMoment =
  | 'ritual_jump'
  | 'ritual_lava'
  | 'raise_hand'
  | 'hold'
  | 'duo_invite'
  | 'lets_go'
  | 'jump'
  | 'clap_replay';

class PartyDirector {
  private voice = new PartyVoice();
  private lastCountdown = -1;
  private bassSwellActive = false;
  private bassSwellOsc?: OscillatorNode;
  private bassSwellGain?: GainNode;
  private lastShellAt = 0;
  private lastShellId: VoiceCueId | null = null;
  private holdAnnounced = false;
  private duoInviteAnnounced = false;

  handleEvent(kind: number, value: number): void {
    switch (kind) {
      case SupernovaEvent.Countdown:
        this.onCountdown(value);
        break;
      case SupernovaEvent.Dance:
        this.onDance();
        break;
      case SupernovaEvent.LavaWarning:
        this.onLavaWarning();
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
      case SupernovaEvent.DropImminent:
        this.onDropImminent();
        break;
      default:
        break;
    }
  }

  /** Shell / Ready / BrainBreak key-moment VO — throttled, not tip spam. */
  guide(moment: ShellGuideMoment, opts?: { force?: boolean }): void {
    const cue = VOICE_CUES[moment];
    if (!cue) return;
    const now = performance.now();
    if (!opts?.force) {
      if (this.lastShellId === cue.id && now - this.lastShellAt < SHELL_THROTTLE_MS * 2) return;
      if (now - this.lastShellAt < SHELL_THROTTLE_MS) return;
    }
    this.lastShellAt = now;
    this.lastShellId = cue.id;
    void this.voice.speakCue(cue);
  }

  resetReadyGuides(): void {
    this.holdAnnounced = false;
    this.duoInviteAnnounced = false;
  }

  onReadyOpened(family: 'brainbreak' | 'ar'): void {
    this.resetReadyGuides();
    this.guide(family === 'ar' ? 'ritual_lava' : 'ritual_jump', { force: true });
    // Space before hands-up so lines do not stack.
    setTimeout(() => this.guide('raise_hand', { force: true }), 900);
  }

  onReadyHoldProgress(progress: number): void {
    if (progress < 0.18 || this.holdAnnounced) return;
    this.holdAnnounced = true;
    this.guide('hold', { force: true });
  }

  onDuoNeedsPartner(needsPartner: boolean): void {
    if (!needsPartner) {
      this.duoInviteAnnounced = false;
      return;
    }
    if (this.duoInviteAnnounced) return;
    this.duoInviteAnnounced = true;
    this.guide('duo_invite', { force: true });
  }

  onGameplayStart(family: 'brainbreak' | 'ar'): void {
    this.guide(family === 'ar' ? 'lets_go' : 'jump', { force: true });
  }

  onRitualSelected(family: 'brainbreak' | 'ar'): void {
    this.guide(family === 'ar' ? 'ritual_lava' : 'ritual_jump');
  }

  private speakId(id: VoiceCueId): void {
    void this.voice.speakCue(VOICE_CUES[id]);
  }

  private onCountdown(num: number): void {
    if (num === this.lastCountdown) return;
    this.lastCountdown = num;
    const cueId = countdownCueId(num);
    if (cueId && cueId !== 'go') {
      this.speakId(cueId);
      this.playTick(num);
    } else if (num === 0) {
      this.speakId('go');
      this.playGo();
    }
  }

  private onDance(): void {
    this.lastCountdown = -1;
    this.stopBassSwell();
    musicEngine.resume();
    this.playDanceStinger();
    this.speakId('dance');
  }

  private onLavaWarning(): void {
    this.lastCountdown = -1;
    this.stopBassSwell();
    musicEngine.resume();
    this.playWarnStinger();
    this.speakId('lava');
  }

  private onFreeze(): void {
    this.stopBassSwell();
    musicEngine.pause();
    this.playFreezeShimmer();
    // Brief beat of silence, then spoken Freeze — music already stopped.
    setTimeout(() => this.speakId('freeze'), 120);
  }

  private onPerfectFreeze(): void {
    this.playChime();
    this.speakId('perfect');
  }

  private onDropImminent(): void {
    this.startBassSwell();
  }

  private onDrop(): void {
    this.stopBassSwell();
    musicEngine.resume();
    this.playCelebration();
    this.playBassDrop();
    this.speakId('drop');
    if (typeof navigator !== 'undefined' && navigator.vibrate) {
      navigator.vibrate([80, 40, 160]);
    }
  }

  private onResult(outcome: number): void {
    this.stopBassSwell();
    this.playAfterglowChime();
    this.speakId(outcome === 0 ? 'celebrate' : 'nice_try');
    setTimeout(() => this.guide('clap_replay', { force: true }), 1_100);
    if (typeof navigator !== 'undefined' && navigator.vibrate) {
      navigator.vibrate([40, 30, 40]);
    }
  }

  private playAfterglowChime(): void {
    const ctx = this.context();
    if (!ctx || ctx.state !== 'running') return;
    const at = ctx.currentTime;
    [523, 659, 784].forEach((freq, i) => {
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.type = 'sine';
      osc.frequency.setValueAtTime(freq, at + i * 0.09);
      gain.gain.setValueAtTime(0.001, at + i * 0.09);
      gain.gain.exponentialRampToValueAtTime(0.09, at + i * 0.09 + 0.04);
      gain.gain.exponentialRampToValueAtTime(0.001, at + i * 0.09 + 0.55);
      osc.connect(gain).connect(ctx.destination);
      osc.start(at + i * 0.09);
      osc.stop(at + i * 0.09 + 0.6);
    });
  }

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

  private playDanceStinger(): void {
    const ctx = this.context();
    if (!ctx || ctx.state !== 'running') return;
    const at = ctx.currentTime;
    [392, 523, 659].forEach((freq, i) => {
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.type = 'triangle';
      osc.frequency.setValueAtTime(freq, at + i * 0.05);
      gain.gain.setValueAtTime(0.001, at + i * 0.05);
      gain.gain.exponentialRampToValueAtTime(0.1, at + i * 0.05 + 0.03);
      gain.gain.exponentialRampToValueAtTime(0.001, at + i * 0.05 + 0.22);
      osc.connect(gain).connect(ctx.destination);
      osc.start(at + i * 0.05);
      osc.stop(at + i * 0.05 + 0.25);
    });
  }

  private playWarnStinger(): void {
    const ctx = this.context();
    if (!ctx || ctx.state !== 'running') return;
    const at = ctx.currentTime;
    const osc = ctx.createOscillator();
    const gain = ctx.createGain();
    osc.type = 'sawtooth';
    osc.frequency.setValueAtTime(220, at);
    osc.frequency.exponentialRampToValueAtTime(440, at + 0.35);
    gain.gain.setValueAtTime(0.001, at);
    gain.gain.exponentialRampToValueAtTime(0.12, at + 0.05);
    gain.gain.exponentialRampToValueAtTime(0.001, at + 0.4);
    osc.connect(gain).connect(ctx.destination);
    osc.start(at);
    osc.stop(at + 0.42);
  }

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

  private playChime(): void {
    const ctx = this.context();
    if (!ctx || ctx.state !== 'running') return;
    const at = ctx.currentTime;
    const notes = [523, 659, 784, 1046];
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

  private playCelebration(): void {
    const ctx = this.context();
    if (!ctx || ctx.state !== 'running') return;
    const at = ctx.currentTime;
    const notes = [392, 523, 659, 784, 1046, 1318];
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

  private playBassDrop(): void {
    const ctx = this.context();
    if (!ctx || ctx.state !== 'running') return;
    const at = ctx.currentTime;
    const osc = ctx.createOscillator();
    const gain = ctx.createGain();
    osc.type = 'sawtooth';
    osc.frequency.setValueAtTime(95, at);
    osc.frequency.exponentialRampToValueAtTime(28, at + 0.55);
    gain.gain.setValueAtTime(0.34, at);
    gain.gain.exponentialRampToValueAtTime(0.001, at + 0.7);
    osc.connect(gain).connect(ctx.destination);
    osc.start(at);
    osc.stop(at + 0.72);

    const thud = ctx.createOscillator();
    const thudGain = ctx.createGain();
    thud.type = 'sine';
    thud.frequency.setValueAtTime(58, at);
    thud.frequency.exponentialRampToValueAtTime(22, at + 0.35);
    thudGain.gain.setValueAtTime(0.28, at);
    thudGain.gain.exponentialRampToValueAtTime(0.001, at + 0.4);
    thud.connect(thudGain).connect(ctx.destination);
    thud.start(at);
    thud.stop(at + 0.42);
  }

  private startBassSwell(): void {
    if (this.bassSwellActive) return;
    musicEngine.setDropTension(true);
    const ctx = this.context();
    if (!ctx || ctx.state !== 'running') return;
    this.bassSwellActive = true;
    const at = ctx.currentTime;
    const osc = ctx.createOscillator();
    const gain = ctx.createGain();
    osc.type = 'sawtooth';
    osc.frequency.setValueAtTime(48, at);
    osc.frequency.linearRampToValueAtTime(72, at + 2.4);
    gain.gain.setValueAtTime(0.001, at);
    gain.gain.exponentialRampToValueAtTime(0.16, at + 0.35);
    gain.gain.linearRampToValueAtTime(0.22, at + 2.8);
    osc.connect(gain).connect(ctx.destination);
    osc.start(at);
    osc.stop(at + 8.0);
    this.bassSwellOsc = osc;
    this.bassSwellGain = gain;
  }

  private stopBassSwell(): void {
    musicEngine.setDropTension(false);
    if (!this.bassSwellActive) return;
    this.bassSwellActive = false;
    const ctx = this.context();
    const gain = this.bassSwellGain;
    const osc = this.bassSwellOsc;
    this.bassSwellGain = undefined;
    this.bassSwellOsc = undefined;
    if (ctx && gain) {
      try {
        gain.gain.cancelScheduledValues(ctx.currentTime);
        gain.gain.setValueAtTime(Math.max(0.001, gain.gain.value), ctx.currentTime);
        gain.gain.exponentialRampToValueAtTime(0.001, ctx.currentTime + 0.08);
      } catch {
        // Ignore teardown races.
      }
    }
    if (osc) {
      try {
        osc.stop((ctx?.currentTime ?? 0) + 0.1);
      } catch {
        // Already stopped.
      }
    }
  }
}

export const partyDirector = new PartyDirector();
