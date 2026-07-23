#!/usr/bin/env node
/**
 * Procedural studio music beds for BrainBreak shell rituals.
 *
 * Taste/format inspiration: kids freeze-dance / dance-along party energy
 * (clear beat, call-and-response-friendly phrasing, danger/freeze structure).
 * Melodies, arrangements, and titles are original OpenAgenticGame Studios IP.
 * Danny Go / Kiboomers / commercial catalogs are never ripped or imitated note-for-note.
 *
 * Usage:
 *   node scripts/generate-studio-beds.mjs            # both beds
 *   node scripts/generate-studio-beds.mjs neon-jump   # runner only
 *   node scripts/generate-studio-beds.mjs lava-freeze # supernova only
 */
import { execSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const OUT_DIR = join(__dirname, '../web/public/audio');
const TMP_DIR = '/tmp/brainbreak-studio-beds';
const BPM = 140;
const BEAT = 60 / BPM;
const SR = 44100;

mkdirSync(TMP_DIR, { recursive: true });
mkdirSync(OUT_DIR, { recursive: true });

/** Deterministic noise so fingerprints stay stable across regenerations. */
function makeRng(seed) {
  let s = seed >>> 0;
  return () => {
    s = (Math.imul(1664525, s) + 1013904223) >>> 0;
    return s / 0x100000000;
  };
}

function envADSR(t, a, d, s, r, hold) {
  if (t < 0) return 0;
  if (t < a) return t / a;
  if (t < a + d) return 1 - (1 - s) * ((t - a) / d);
  if (t < a + d + hold) return s;
  const rt = t - (a + d + hold);
  if (rt < r) return s * (1 - rt / r);
  return 0;
}

function allocate(bars) {
  const totalBeats = bars * 4;
  const duration = totalBeats * BEAT;
  const n = Math.floor(duration * SR);
  return {
    totalBeats,
    duration,
    n,
    L: new Float32Array(n),
    R: new Float32Array(n),
  };
}

function writeStereo(buf, i, l, r) {
  if (i < 0 || i >= buf.n) return;
  buf.L[i] += l;
  buf.R[i] += r;
}

function softLimit(buf) {
  for (let i = 0; i < buf.n; i += 1) {
    buf.L[i] = Math.tanh(buf.L[i] * 1.12);
    buf.R[i] = Math.tanh(buf.R[i] * 1.12);
  }
}

function toPcm(buf) {
  const pcm = Buffer.alloc(buf.n * 4);
  for (let i = 0; i < buf.n; i += 1) {
    const ls = Math.max(-1, Math.min(1, buf.L[i]));
    const rs = Math.max(-1, Math.min(1, buf.R[i]));
    pcm.writeInt16LE((ls * 32767) | 0, i * 4);
    pcm.writeInt16LE((rs * 32767) | 0, i * 4 + 2);
  }
  return pcm;
}

function encodeMp3(slug, pcm) {
  const pcmPath = join(TMP_DIR, `${slug}.pcm`);
  const rawMp3 = join(TMP_DIR, `${slug}-raw.mp3`);
  writeFileSync(pcmPath, pcm);
  execSync(
    `ffmpeg -y -f s16le -ar ${SR} -ac 2 -i ${pcmPath} -codec:a libmp3lame -b:a 128k ${rawMp3}`,
    { stdio: 'inherit' },
  );
  const bytes = readFileSync(rawMp3);
  const hash = createHash('sha256').update(bytes).digest('hex');
  const short = hash.slice(0, 12);
  const dest = join(OUT_DIR, `${slug}-${short}.mp3`);
  copyFileSync(rawMp3, dest);
  return { hash, short, dest, bpm: BPM, durationSecs: +(pcm.length / (SR * 4)).toFixed(2) };
}

/** Neon Jump Party — BrainBreak runner: rock-dance pump, punchy jump/clap energy. */
function renderNeonJump() {
  const buf = allocate(16);
  const { totalBeats } = buf;
  const rnd = makeRng(0x524f434b); // 'ROCK'

  // Four-on-the-floor kick + rock side-chain punch
  for (let beat = 0; beat < totalBeats; beat += 1) {
    const t0 = beat * BEAT;
    const len = Math.floor(0.15 * SR);
    const dropBar = Math.floor(beat / 4) % 4 === 0 && beat % 4 === 0;
    for (let i = 0; i < len; i += 1) {
      const t = i / SR;
      const f = (dropBar ? 185 : 155) * Math.exp(-t * 34) + 42;
      const amp = envADSR(t, 0.001, 0.04, 0.16, 0.07, 0.015) * (dropBar ? 0.95 : 0.8);
      const s = Math.sin(2 * Math.PI * f * t) * amp;
      writeStereo(buf, Math.floor(t0 * SR) + i, s, s);
    }
  }

  // Snare/clap on 2 & 4 — dance-rock backbeat kids can bounce to
  for (let beat = 1; beat < totalBeats; beat += 2) {
    const t0 = beat * BEAT;
    const len = Math.floor(0.14 * SR);
    for (let i = 0; i < len; i += 1) {
      const t = i / SR;
      const noise = rnd() * 2 - 1;
      const body = Math.sin(2 * Math.PI * 190 * t) * Math.exp(-t * 22) * 0.22;
      const amp = Math.exp(-t * 26) * 0.5;
      writeStereo(
        buf,
        Math.floor(t0 * SR) + i,
        noise * amp * 0.8 + body,
        noise * amp * 1.2 + body,
      );
    }
  }

  // Power-chord-ish bass: G D C D (rock root motion)
  const bassNotes = [98.0, 73.42, 65.41, 73.42];
  for (let beat = 0; beat < totalBeats; beat += 1) {
    const note = bassNotes[beat % 4];
    const t0 = beat * BEAT;
    const len = Math.floor(BEAT * 0.92 * SR);
    for (let i = 0; i < len; i += 1) {
      const t = i / SR;
      const amp = envADSR(t, 0.006, 0.06, 0.55, 0.06, BEAT * 0.55) * 0.34;
      const s = Math.sin(2 * Math.PI * note * t) * amp
        + Math.sin(2 * Math.PI * note * 2 * t) * amp * 0.35
        + Math.sin(2 * Math.PI * note * 3 * t) * amp * 0.08;
      writeStereo(buf, Math.floor(t0 * SR) + i, s, s);
    }
  }

  // Bright rock-dance lead arpeggio G B D E
  const melody = [392.0, 493.88, 587.33, 659.25, 783.99, 659.25, 587.33, 493.88];
  for (let eighth = 0; eighth < totalBeats * 2; eighth += 1) {
    const bar = Math.floor(eighth / 8);
    if (bar % 8 === 7 && (eighth % 8) >= 6) continue;
    const note = melody[eighth % melody.length];
    const t0 = eighth * (BEAT / 2);
    const len = Math.floor((BEAT / 2) * 0.78 * SR);
    for (let i = 0; i < len; i += 1) {
      const t = i / SR;
      const amp = envADSR(t, 0.003, 0.03, 0.28, 0.06, BEAT * 0.14) * 0.17;
      const s = Math.sin(2 * Math.PI * note * t) * amp
        + Math.sin(2 * Math.PI * note * 2 * t) * amp * 0.18;
      const pan = eighth % 2 === 0 ? -0.28 : 0.28;
      writeStereo(buf, Math.floor(t0 * SR) + i, s * (1 - pan), s * (1 + pan));
    }
  }

  // Busier open hats (16ths on dance bars)
  for (let sixteenth = 0; sixteenth < totalBeats * 4; sixteenth += 1) {
    const t0 = sixteenth * (BEAT / 4);
    const len = Math.floor(0.028 * SR);
    for (let i = 0; i < len; i += 1) {
      const t = i / SR;
      const noise = rnd() * 2 - 1;
      const onBeat = sixteenth % 2 === 0;
      const amp = Math.exp(-t * 85) * (onBeat ? 0.07 : 0.038);
      writeStereo(buf, Math.floor(t0 * SR) + i, noise * amp, noise * amp);
    }
  }

  // Mini riser into every 8th bar
  for (let bar = 7; bar < totalBeats / 4; bar += 8) {
    const t0 = bar * 4 * BEAT + BEAT * 2;
    const len = Math.floor(BEAT * 2 * SR);
    for (let i = 0; i < len; i += 1) {
      const t = i / SR;
      const prog = t / (BEAT * 2);
      const noise = rnd() * 2 - 1;
      const amp = prog * prog * 0.14;
      const f = 220 + prog * 1400;
      const tone = Math.sin(2 * Math.PI * f * t) * amp * 0.4;
      writeStereo(buf, Math.floor(t0 * SR) + i, noise * amp * 0.5 + tone, noise * amp * 0.5 + tone);
    }
  }

  softLimit(buf);
  return encodeMp3('neon-jump-party', toPcm(buf));
}

/**
 * Lava Freeze Party — Supernova elemental journey (~30.5s / 16 bars).
 * Chapters (8-bar cells ≈ 15.2s ≈ Dance 10s + LavaWarning 5s):
 *   bars 0–4  WIND+OCEAN dance (move / sway)
 *   bars 5–6  SMOKE+FIRE danger rise
 *   bar 7     soft ICE breath (music still on; Freeze = pause)
 * Loop again — Drop/Result use SFX + VFX for fire celebrate / ocean cool.
 */
function renderLavaFreeze() {
  const buf = allocate(16);
  const { totalBeats } = buf;
  const rnd = makeRng(0x44414e43); // 'DANC'

  for (let beat = 0; beat < totalBeats; beat += 1) {
    const barIn8 = Math.floor(beat / 4) % 8;
    const fire = barIn8 === 5 || barIn8 === 6;
    const iceBreath = barIn8 === 7;
    if (iceBreath && beat % 4 >= 2) continue;
    const t0 = beat * BEAT;
    const len = Math.floor(0.16 * SR);
    for (let i = 0; i < len; i += 1) {
      const t = i / SR;
      const f = (fire ? 178 : 150) * Math.exp(-t * 30) + 40;
      const amp = envADSR(t, 0.001, 0.045, 0.18, 0.08, 0.015) * (fire ? 0.88 : iceBreath ? 0.5 : 0.78);
      const s = Math.sin(2 * Math.PI * f * t) * amp;
      writeStereo(buf, Math.floor(t0 * SR) + i, s, s);
    }
  }

  // Dance-rock snare/clap call accents
  for (let beat = 1; beat < totalBeats; beat += 2) {
    const barIn8 = Math.floor(beat / 4) % 8;
    if (barIn8 === 7 && beat % 4 === 3) continue;
    const t0 = beat * BEAT;
    const len = Math.floor(0.13 * SR);
    const loud = barIn8 >= 5 ? 0.55 : 0.42;
    for (let i = 0; i < len; i += 1) {
      const t = i / SR;
      const noise = rnd() * 2 - 1;
      const body = Math.sin(2 * Math.PI * 200 * t) * Math.exp(-t * 20) * 0.18;
      const amp = Math.exp(-t * 24) * loud;
      writeStereo(buf, Math.floor(t0 * SR) + i, noise * amp * 0.85 + body, noise * amp * 1.15 + body);
    }
  }

  // Punchy major bass C G A G
  const bassNotes = [130.81, 98.0, 110.0, 98.0];
  for (let beat = 0; beat < totalBeats; beat += 1) {
    const barIn8 = Math.floor(beat / 4) % 8;
    if (barIn8 === 7 && beat % 4 >= 2) continue;
    const note = bassNotes[beat % 4];
    const t0 = beat * BEAT;
    const len = Math.floor(BEAT * 0.9 * SR);
    const level = barIn8 >= 5 ? 0.34 : 0.28;
    for (let i = 0; i < len; i += 1) {
      const t = i / SR;
      const amp = envADSR(t, 0.008, 0.07, 0.52, 0.07, BEAT * 0.5) * level;
      const s = Math.sin(2 * Math.PI * note * t) * amp
        + Math.sin(2 * Math.PI * note * 2 * t) * amp * 0.28;
      writeStereo(buf, Math.floor(t0 * SR) + i, s, s);
    }
  }

  // Dance lead: C E G A with more drive
  const melody = [261.63, 329.63, 392.0, 440.0, 523.25, 440.0, 392.0, 329.63];
  for (let eighth = 0; eighth < totalBeats * 2; eighth += 1) {
    const bar = Math.floor(eighth / 8);
    const barIn8 = bar % 8;
    if (barIn8 === 7 && (eighth % 8) >= 4) continue;
    if (barIn8 >= 5 && (eighth % 8) % 2 === 1) continue;
    const note = melody[eighth % melody.length];
    const t0 = eighth * (BEAT / 2);
    const len = Math.floor((BEAT / 2) * 0.8 * SR);
    const level = barIn8 >= 5 ? 0.12 : 0.17;
    for (let i = 0; i < len; i += 1) {
      const t = i / SR;
      const amp = envADSR(t, 0.004, 0.035, 0.3, 0.07, BEAT * 0.16) * level;
      const s = Math.sin(2 * Math.PI * note * t) * amp
        + Math.sin(2 * Math.PI * note * 2 * t) * amp * 0.14;
      const pan = Math.sin(eighth * 0.7) * 0.35;
      writeStereo(buf, Math.floor(t0 * SR) + i, s * (1 - pan), s * (1 + pan));
    }
  }

  const bells = [784.0, 988.0, 1175.0];
  for (let beat = 0; beat < totalBeats; beat += 4) {
    const barIn8 = Math.floor(beat / 4) % 8;
    if (barIn8 >= 5) continue;
    const note = bells[(beat / 4) % bells.length];
    const t0 = beat * BEAT;
    const len = Math.floor(0.32 * SR);
    for (let i = 0; i < len; i += 1) {
      const t = i / SR;
      const amp = Math.exp(-t * 6) * 0.065;
      const s = Math.sin(2 * Math.PI * note * t) * amp;
      writeStereo(buf, Math.floor(t0 * SR) + i, s * 0.7, s);
    }
  }

  for (let sixteenth = 0; sixteenth < totalBeats * 4; sixteenth += 1) {
    const barIn8 = Math.floor(sixteenth / 16) % 8;
    if (barIn8 === 7 && (sixteenth % 16) >= 8) continue;
    const t0 = sixteenth * (BEAT / 4);
    const len = Math.floor(0.03 * SR);
    for (let i = 0; i < len; i += 1) {
      const t = i / SR;
      const noise = rnd() * 2 - 1;
      const amp = Math.exp(-t * 70) * (sixteenth % 2 === 0 ? 0.065 : 0.035);
      writeStereo(buf, Math.floor(t0 * SR) + i, noise * amp, noise * amp);
    }
  }

  for (let bar = 5; bar < totalBeats / 4; bar += 8) {
    const t0 = (bar - 0.5) * 4 * BEAT;
    const len = Math.floor(2.5 * BEAT * SR);
    for (let i = 0; i < len; i += 1) {
      const t = i / SR;
      const prog = t / (2.5 * BEAT);
      const noise = rnd() * 2 - 1;
      const amp = Math.sin(prog * Math.PI) * 0.09;
      const whoosh = Math.sin(2 * Math.PI * (180 + prog * 1000) * t) * amp * 0.45;
      writeStereo(buf, Math.floor(t0 * SR) + i, noise * amp * 0.55 + whoosh, noise * amp * 0.55 + whoosh);
    }
  }

  softLimit(buf);
  return encodeMp3('lava-freeze-party', toPcm(buf));
}

const want = process.argv[2] ?? 'all';
const results = {};
if (want === 'all' || want === 'neon-jump') results['neon-jump'] = renderNeonJump();
if (want === 'all' || want === 'lava-freeze') results['lava-freeze'] = renderLavaFreeze();
writeFileSync(join(TMP_DIR, 'meta.json'), JSON.stringify(results, null, 2));
console.log(JSON.stringify(results, null, 2));
