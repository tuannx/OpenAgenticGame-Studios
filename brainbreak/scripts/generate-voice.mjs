#!/usr/bin/env node
/**
 * Generate the Supernova Freeze Party voice-command clips with the Kokoro
 * small TTS model (~82M params, runs locally — no cloud API key needed).
 *
 * Why build-time clips instead of runtime SpeechSynthesis?
 *  - The command vocabulary is tiny and fixed (~9 phrases), so a handful of
 *    pre-rendered clips give a consistent, warm, kid-friendly voice on every
 *    device instead of whatever TTS voice the browser happens to ship.
 *  - Total payload is well under 1 MB; the player falls back to the device
 *    SpeechSynthesis API automatically when the clips are absent.
 *
 * Usage:
 *   npm i -D kokoro-js            # one-time (downloads onnx runtime)
 *   node scripts/generate-voice.mjs
 *
 * The first run downloads the Kokoro ONNX weights (~80 MB, q8) from HuggingFace
 * into your local cache, then writes WAV clips + a manifest to web/public/voice/.
 * Deploy as usual — the browser client picks the clips up automatically.
 */

import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const OUT_DIR = join(__dirname, '..', 'web', 'public', 'voice');

/** Fixed Supernova command vocabulary. Keys match party-voice.ts speak() calls. */
const PHRASES = [
  { key: 'count_2', text: 'Two!' },
  { key: 'count_1', text: 'One!' },
  { key: 'go', text: 'Go go go!' },
  { key: 'dance', text: 'Dance!' },
  { key: 'freeze', text: 'Freeze!' },
  { key: 'great_job', text: 'Great job!' },
  { key: 'super_nova', text: 'Super nova!' },
  { key: 'wow', text: 'Wow! You did it!' },
  { key: 'yay', text: 'Yay! Nice dancing!' },
];

/** Warm, clear female voice; slightly slower for kindergarten clarity. */
const VOICE = process.env.KOKORO_VOICE || 'af_heart';
const SPEED = Number(process.env.KOKORO_SPEED || '0.95');
const MODEL_ID = process.env.KOKORO_MODEL || 'onnx-community/Kokoro-82M-v1.0-ONNX';
const DTYPE = process.env.KOKORO_DTYPE || 'q8';

async function loadTts() {
  let KokoroTTS;
  try {
    ({ KokoroTTS } = await import('kokoro-js'));
  } catch {
    console.error(
      '\nkokoro-js is not installed. Run:\n\n  npm i -D kokoro-js\n\nthen re-run this script.',
    );
    process.exit(1);
  }
  console.log(`Loading Kokoro model ${MODEL_ID} (${DTYPE})… (first run downloads ~80 MB)`);
  return KokoroTTS.from_pretrained(MODEL_ID, { dtype: DTYPE });
}

async function main() {
  const tts = await loadTts();
  mkdirSync(OUT_DIR, { recursive: true });

  const manifest = {};
  for (const { key, text } of PHRASES) {
    const file = `${key}.wav`;
    const outPath = join(OUT_DIR, file);
    process.stdout.write(`  ${key.padEnd(11)} "${text}" … `);
    const audio = await tts.generate(text, { voice: VOICE, speed: SPEED });
    await audio.save(outPath);
    manifest[key] = file;
    console.log('ok');
  }

  writeFileSync(join(OUT_DIR, 'manifest.json'), `${JSON.stringify(manifest, null, 2)}\n`);
  console.log(`\nWrote ${PHRASES.length} clips + manifest.json to ${OUT_DIR}`);
  console.log('Voice:', VOICE, '| Speed:', SPEED);
  console.log('\nOptional: compress WAV→MP3 with ffmpeg to shrink payload further.');
}

main().catch((err) => {
  console.error('Voice generation failed:', err);
  process.exit(1);
});
