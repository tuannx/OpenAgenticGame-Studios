#!/usr/bin/env node
/**
 * Generate coach VO clips with Kokoro small TTS (optional build-time assets).
 *
 * Runtime prefers /voice clips when present; otherwise SpeechSynthesis.
 *
 * Usage:
 *   npm i -D kokoro-js
 *   npm run voice:gen
 */

import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const OUT_DIR = join(__dirname, '..', 'web', 'public', 'voice');

/** Keep in sync with web/src/party-voice-cues.ts */
const PHRASES = [
  { key: 'ritual_jump', text: 'Jump party!' },
  { key: 'ritual_lava', text: 'Lava freeze!' },
  { key: 'raise_hand', text: 'Hands up!' },
  { key: 'hold', text: 'Hold!' },
  { key: 'duo_invite', text: 'Bring a friend!' },
  { key: 'lets_go', text: "Let's go!" },
  { key: 'jump', text: 'Jump!' },
  { key: 'clap_replay', text: 'Clap to play!' },
  { key: 'count_3', text: 'Three!' },
  { key: 'count_2', text: 'Two!' },
  { key: 'count_1', text: 'One!' },
  { key: 'go', text: 'Go!' },
  { key: 'dance', text: 'Move!' },
  { key: 'lava', text: 'Lava!' },
  { key: 'freeze', text: 'Freeze!' },
  { key: 'perfect', text: 'Perfect!' },
  { key: 'drop', text: 'Drop!' },
  { key: 'celebrate', text: 'You did it!' },
  { key: 'nice_try', text: 'Nice try!' },
];

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
    process.stdout.write(`  ${key.padEnd(14)} "${text}" … `);
    const audio = await tts.generate(text, { voice: VOICE, speed: SPEED });
    await audio.save(outPath);
    manifest[key] = file;
    console.log('ok');
  }

  writeFileSync(join(OUT_DIR, 'manifest.json'), `${JSON.stringify(manifest, null, 2)}\n`);
  console.log(`\nWrote ${PHRASES.length} clips + manifest.json to ${OUT_DIR}`);
  console.log('Voice:', VOICE, '| Speed:', SPEED);
}

main().catch((err) => {
  console.error('Voice generation failed:', err);
  process.exit(1);
});
