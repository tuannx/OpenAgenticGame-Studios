/**
 * Spoken coach cue map — short kid lines only (LÀM ÍT).
 * Keys match optional /voice/manifest.json clips from scripts/generate-voice.mjs.
 */

export type VoiceCueId =
  | 'ritual_jump'
  | 'ritual_lava'
  | 'raise_hand'
  | 'hold'
  | 'duo_invite'
  | 'lets_go'
  | 'jump'
  | 'clap_replay'
  | 'count_3'
  | 'count_2'
  | 'count_1'
  | 'go'
  | 'dance'
  | 'lava'
  | 'freeze'
  | 'perfect'
  | 'drop'
  | 'celebrate'
  | 'nice_try';

export interface VoiceCue {
  id: VoiceCueId;
  /** Spoken English line — one short sentence max. */
  text: string;
  pitch?: number;
  rate?: number;
  /** When true, skip music duck (music already paused / silent). */
  silenceBed?: boolean;
}

export const VOICE_CUES: Record<VoiceCueId, VoiceCue> = Object.freeze({
  ritual_jump: { id: 'ritual_jump', text: 'Jump party!', pitch: 1.35, rate: 0.95 },
  ritual_lava: { id: 'ritual_lava', text: 'Lava freeze!', pitch: 1.3, rate: 0.92 },
  raise_hand: { id: 'raise_hand', text: 'Hands up!', pitch: 1.35, rate: 0.95 },
  hold: { id: 'hold', text: 'Hold!', pitch: 1.25, rate: 0.9 },
  duo_invite: { id: 'duo_invite', text: 'Bring a friend!', pitch: 1.3, rate: 0.95 },
  lets_go: { id: 'lets_go', text: "Let's go!", pitch: 1.4, rate: 1.0 },
  jump: { id: 'jump', text: 'Jump!', pitch: 1.4, rate: 1.0 },
  clap_replay: { id: 'clap_replay', text: 'Clap to play!', pitch: 1.3, rate: 0.95 },
  count_3: { id: 'count_3', text: 'Three!', pitch: 1.4, rate: 1.0 },
  count_2: { id: 'count_2', text: 'Two!', pitch: 1.4, rate: 1.0 },
  count_1: { id: 'count_1', text: 'One!', pitch: 1.4, rate: 1.0 },
  go: { id: 'go', text: 'Go!', pitch: 1.5, rate: 1.05 },
  dance: { id: 'dance', text: 'Move!', pitch: 1.4, rate: 1.0 },
  lava: { id: 'lava', text: 'Lava!', pitch: 1.35, rate: 0.95 },
  freeze: { id: 'freeze', text: 'Freeze!', pitch: 1.15, rate: 0.85, silenceBed: true },
  perfect: { id: 'perfect', text: 'Perfect!', pitch: 1.4, rate: 1.0 },
  drop: { id: 'drop', text: 'Drop!', pitch: 1.45, rate: 1.0 },
  celebrate: { id: 'celebrate', text: 'You did it!', pitch: 1.4, rate: 0.95 },
  nice_try: { id: 'nice_try', text: 'Nice try!', pitch: 1.3, rate: 0.95 },
});

export function countdownCueId(num: number): VoiceCueId | null {
  if (num === 3) return 'count_3';
  if (num === 2) return 'count_2';
  if (num === 1) return 'count_1';
  if (num === 0) return 'go';
  return null;
}
