#!/usr/bin/env node
// BrainBreak Game Mixer MCP server (stdio).
//
// Lets an agent author and manage config-driven game files stored in
// brainbreak/games/*.game.json. Validation mirrors the Rust GameConfig schema
// in crates/brainbreak-core/src/config.rs so anything saved here loads in-game.
//
// Run manually:  node brainbreak/mcp/server.mjs
// Registered via .mcp.json at the repo root as "brainbreak-games".
import { mkdir, readFile, readdir, unlink, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import { z } from 'zod';

// brainbreak/mcp/../games -> brainbreak/games (independent of the caller cwd).
const GAMES_DIR = resolve(dirname(fileURLToPath(import.meta.url)), '..', 'games');

const MECHANICS = ['dodge', 'pump', 'catch', 'hold', 'pattern'];
const CURVES = ['breath-arc', 'steady-ramp', 'waves'];
const END_MODES = ['time-based', 'life-based', 'goal-based'];
const THEME_STYLES = ['neon', 'cosmic', 'minimal'];
// Recognized action bitmasks (must match web/src/studio.tsx ACTIONS).
const ACTION_MASKS = {
  1: 'Move Left',
  2: 'Move Right',
  4: 'Jump',
  8: 'Squat',
  16: 'Left Hand Up',
  32: 'Right Hand Up',
  64: 'Clap',
};

/**
 * Validate a parsed game config against the GameConfig schema.
 * Returns { ok, errors[] }. Mirrors config.rs serde rules + ranges.
 */
export function validateGameConfig(raw) {
  if (typeof raw !== 'object' || raw === null || Array.isArray(raw)) {
    return { ok: false, errors: ['config must be a JSON object'] };
  }
  const c = raw;
  const errors = [];

  if (c.schema_version !== 1) errors.push('schema_version must be 1');
  if (typeof c.id !== 'string' || c.id.trim() === '') {
    errors.push('id must be a non-empty string');
  } else if (!/^[a-z0-9][a-z0-9-]*$/.test(c.id)) {
    errors.push('id should be a lowercase slug (a-z, 0-9, -)');
  }
  if (typeof c.title !== 'string' || c.title.trim() === '') errors.push('title must be a non-empty string');

  if (!Array.isArray(c.actions)) {
    errors.push('actions must be an array');
  } else {
    if (c.actions.length > 3) errors.push('actions supports at most 3 slots (simplicity)');
    c.actions.forEach((a, i) => {
      if (!Number.isInteger(a) || a < 0) {
        errors.push(`actions[${i}] must be a non-negative integer bitmask`);
      } else if (a !== 0 && !(a in ACTION_MASKS)) {
        errors.push(`actions[${i}] (${a}) is not a recognized action mask`);
      }
    });
  }

  if (typeof c.mechanic !== 'string' || !MECHANICS.includes(c.mechanic)) {
    errors.push(`mechanic must be one of ${MECHANICS.join(', ')}`);
  }

  if (c.scoring != null) {
    for (const k of ['combo_step', 'on_beat_bonus', 'coop_bonus']) {
      if (c.scoring[k] != null && (typeof c.scoring[k] !== 'number' || c.scoring[k] < 0)) {
        errors.push(`scoring.${k} must be a non-negative number`);
      }
    }
  }

  if (c.pacing != null) {
    const p = c.pacing;
    if (p.bpm != null && (typeof p.bpm !== 'number' || p.bpm < 60 || p.bpm > 180)) {
      errors.push('pacing.bpm must be between 60 and 180');
    }
    if (p.session_seconds != null && (typeof p.session_seconds !== 'number' || p.session_seconds < 30 || p.session_seconds > 120)) {
      errors.push('pacing.session_seconds must be between 30 and 120');
    }
    if (p.curve != null && !CURVES.includes(p.curve)) {
      errors.push(`pacing.curve must be one of ${CURVES.join(', ')}`);
    }
  }

  if (c.challenge != null) {
    const ch = c.challenge;
    for (const k of ['spawn_interval', 'speed_ramp', 'difficulty']) {
      if (ch[k] != null && (typeof ch[k] !== 'number' || ch[k] < 0)) {
        errors.push(`challenge.${k} must be a non-negative number`);
      }
    }
    if (typeof ch.difficulty === 'number' && ch.difficulty > 1) {
      errors.push('challenge.difficulty must be <= 1.0');
    }
  }

  if (c.end != null) {
    if (c.end.mode != null && !END_MODES.includes(c.end.mode)) {
      errors.push(`end.mode must be one of ${END_MODES.join(', ')}`);
    }
    if (c.end.lives != null && (!Number.isInteger(c.end.lives) || c.end.lives < 0)) {
      errors.push('end.lives must be a non-negative integer');
    }
    if (c.end.goal_score != null && (!Number.isInteger(c.end.goal_score) || c.end.goal_score < 0)) {
      errors.push('end.goal_score must be a non-negative integer');
    }
  }

  if (c.theme != null) {
    if (c.theme.style != null && !THEME_STYLES.includes(c.theme.style)) {
      errors.push(`theme.style must be one of ${THEME_STYLES.join(', ')}`);
    }
    for (const k of ['primary', 'secondary']) {
      if (c.theme[k] != null && (!Number.isInteger(c.theme[k]) || c.theme[k] < 0)) {
        errors.push(`theme.${k} must be a non-negative integer (0xRRGGBB)`);
      }
    }
  }

  return { ok: errors.length === 0, errors };
}

function isSafeId(id) {
  return typeof id === 'string' && /^[a-z0-9][a-z0-9-]*$/.test(id);
}

function gamePath(id) {
  return resolve(GAMES_DIR, `${id}.game.json`);
}

function parseConfig(configStr) {
  try {
    return { config: JSON.parse(configStr), error: null };
  } catch (error) {
    return { config: null, error: `Invalid JSON: ${error.message}` };
  }
}

async function listGames() {
  let entries;
  try {
    entries = await readdir(GAMES_DIR);
  } catch {
    return [];
  }
  const games = [];
  for (const name of entries.filter((n) => n.endsWith('.game.json'))) {
    try {
      const cfg = JSON.parse(await readFile(resolve(GAMES_DIR, name), 'utf8'));
      games.push({
        id: cfg.id ?? name.replace(/\.game\.json$/, ''),
        title: cfg.title ?? '(untitled)',
        mechanic: cfg.mechanic ?? 'dodge',
      });
    } catch {
      // Skip unreadable/corrupt files.
    }
  }
  return games;
}

const server = new McpServer({ name: 'brainbreak-games', version: '1.0.0' });

server.tool(
  'list_games',
  'List all agent-authored BrainBreak game configs in brainbreak/games/. Returns [{ id, title, mechanic }].',
  {},
  async () => {
    const games = await listGames();
    return { content: [{ type: 'text', text: JSON.stringify(games, null, 2) }] };
  },
);

server.tool(
  'get_game',
  'Read a single BrainBreak game config by id. Returns the full GameConfig JSON.',
  { id: z.string().describe('Game id slug, e.g. "example-catch"') },
  async ({ id }) => {
    if (!isSafeId(id)) return { content: [{ type: 'text', text: `Invalid id "${id}"` }], isError: true };
    try {
      const text = await readFile(gamePath(id), 'utf8');
      return { content: [{ type: 'text', text }] };
    } catch {
      return { content: [{ type: 'text', text: `Game "${id}" not found in ${GAMES_DIR}` }], isError: true };
    }
  },
);

server.tool(
  'validate_game',
  'Validate a BrainBreak game config against the GameConfig schema WITHOUT writing it. Input: the config as a JSON string. Returns { ok, errors[] }.',
  { config: z.string().describe('GameConfig serialized as a JSON string') },
  async ({ config }) => {
    const { config: parsed, error } = parseConfig(config);
    if (error) return { content: [{ type: 'text', text: JSON.stringify({ ok: false, errors: [error] }, null, 2) }] };
    return { content: [{ type: 'text', text: JSON.stringify(validateGameConfig(parsed), null, 2) }] };
  },
);

server.tool(
  'save_game',
  'Validate then write a BrainBreak game config to brainbreak/games/<id>.game.json. Input: the config as a JSON string. Actions are padded to 3 slots for the Rust schema. Returns the saved path.',
  { config: z.string().describe('GameConfig serialized as a JSON string') },
  async ({ config }) => {
    const { config: parsed, error } = parseConfig(config);
    if (error) return { content: [{ type: 'text', text: error }], isError: true };
    const result = validateGameConfig(parsed);
    if (!result.ok) {
      return { content: [{ type: 'text', text: `Validation failed:\n${result.errors.join('\n')}` }], isError: true };
    }
    // Normalize actions to exactly 3 slots for the Rust [u32; 3] schema.
    const actions = [...(Array.isArray(parsed.actions) ? parsed.actions : [])];
    while (actions.length < 3) actions.push(0);
    const normalized = { ...parsed, actions };
    await mkdir(GAMES_DIR, { recursive: true });
    const path = gamePath(normalized.id);
    await writeFile(path, `${JSON.stringify(normalized, null, 2)}\n`, 'utf8');
    return { content: [{ type: 'text', text: `Saved game "${normalized.title}" (${normalized.id}) to ${path}` }] };
  },
);

server.tool(
  'delete_game',
  'Delete a BrainBreak game config by id.',
  { id: z.string().describe('Game id slug, e.g. "example-catch"') },
  async ({ id }) => {
    if (!isSafeId(id)) return { content: [{ type: 'text', text: `Invalid id "${id}"` }], isError: true };
    try {
      await unlink(gamePath(id));
      return { content: [{ type: 'text', text: `Deleted game "${id}"` }] };
    } catch {
      return { content: [{ type: 'text', text: `Game "${id}" not found` }], isError: true };
    }
  },
);

const transport = new StdioServerTransport();
await server.connect(transport);
console.error('brainbreak-games MCP server running on stdio');
