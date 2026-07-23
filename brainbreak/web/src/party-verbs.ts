/**
 * Everyday movement verbs + light pretend roles for dance-along callouts.
 * Danny Go taste/format inspiration only — original studio phrases.
 *
 * Detection: prefer verbs that map to existing MoveNet Action masks.
 * Flavor: announce-only when sensing is weak (still drives play via voice/UI).
 */
export type VerbSense = 'detect' | 'flavor';

export interface PartyVerb {
  key: string;
  /** Spoken + HUD-friendly prompt. */
  text: string;
  /** Short on-canvas cue. */
  cue: string;
  sense: VerbSense;
  /** Existing Action labels this verb aligns with (empty = flavor-only). */
  actions: readonly string[];
}

/** Small clear set — LÀM ÍT. */
export const PARTY_VERBS: readonly PartyVerb[] = Object.freeze([
  {
    key: 'jump',
    text: 'Jump!',
    cue: 'JUMP!',
    sense: 'detect',
    actions: ['JUMP'],
  },
  {
    key: 'clap',
    text: 'Clap!',
    cue: 'CLAP!',
    sense: 'detect',
    actions: ['CLAP'],
  },
  {
    key: 'paint',
    text: 'Paint the air! Hands up!',
    cue: 'PAINT!',
    sense: 'detect',
    actions: ['LEFT HAND UP', 'RIGHT HAND UP'],
  },
  {
    key: 'reach',
    text: 'Reach up high!',
    cue: 'REACH!',
    sense: 'detect',
    actions: ['LEFT HAND UP', 'RIGHT HAND UP'],
  },
  {
    key: 'tiptoe',
    text: 'Tiptoe rise!',
    cue: 'TIPTOE!',
    sense: 'flavor',
    actions: ['JUMP'],
  },
  {
    key: 'wiggle',
    text: 'Wiggle!',
    cue: 'WIGGLE!',
    sense: 'flavor',
    actions: [],
  },
  {
    key: 'march',
    text: 'March in place!',
    cue: 'MARCH!',
    sense: 'flavor',
    actions: ['MOVE LEFT', 'MOVE RIGHT'],
  },
  {
    key: 'royal_wave',
    text: 'King and queen wave!',
    cue: 'ROYAL WAVE!',
    sense: 'detect',
    actions: ['LEFT HAND UP', 'RIGHT HAND UP'],
  },
]);

export function verbAt(index: number): PartyVerb {
  const safe = Number.isFinite(index) ? Math.max(0, Math.floor(index)) : 0;
  return PARTY_VERBS[safe % PARTY_VERBS.length]!;
}
