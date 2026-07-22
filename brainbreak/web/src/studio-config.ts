// Pure, framework-free GameConfig <-> node-graph logic shared by the Studio UI
// and unit tests. No React or DOM side effects live here so it can be imported
// safely in a node test environment.

// --- Action definitions (bitmask values) ---
export const ACTIONS = [
  { label: 'Move Left', mask: 1 },
  { label: 'Move Right', mask: 2 },
  { label: 'Jump', mask: 4 },
  { label: 'Squat', mask: 8 },
  { label: 'Left Hand Up', mask: 16 },
  { label: 'Right Hand Up', mask: 32 },
  { label: 'Clap', mask: 64 },
] as const;

// --- Node categories ---
export type NodeCategory = 'mechanic' | 'action' | 'scoring' | 'pacing' | 'end';

export interface PaletteItem {
  type: NodeCategory;
  label: string;
  value: string;
}

export const PALETTE: PaletteItem[] = [
  // Mechanics
  { type: 'mechanic', label: 'Dodge', value: 'dodge' },
  { type: 'mechanic', label: 'Pump', value: 'pump' },
  { type: 'mechanic', label: 'Catch', value: 'catch' },
  { type: 'mechanic', label: 'Hold', value: 'hold' },
  { type: 'mechanic', label: 'Pattern', value: 'pattern' },
  // Actions
  ...ACTIONS.map((a) => ({ type: 'action' as const, label: a.label, value: String(a.mask) })),
  // Scoring
  { type: 'scoring', label: 'Combo', value: 'combo' },
  { type: 'scoring', label: 'On-Beat Bonus', value: 'on-beat' },
  { type: 'scoring', label: 'Co-Op Bonus', value: 'coop' },
  // Pacing
  { type: 'pacing', label: 'Breath Arc', value: 'breath-arc' },
  { type: 'pacing', label: 'Steady Ramp', value: 'steady-ramp' },
  { type: 'pacing', label: 'Waves', value: 'waves' },
  // End
  { type: 'end', label: 'Timer', value: 'time-based' },
  { type: 'end', label: 'Lives', value: 'life-based' },
  { type: 'end', label: 'Goal Score', value: 'goal-based' },
];

export const CATEGORY_COLORS: Record<NodeCategory, string> = {
  mechanic: '#7c4dff',
  action: '#00e5ff',
  scoring: '#ffab00',
  pacing: '#69f0ae',
  end: '#ff4081',
};

// --- Config generation ---
export interface GameConfigJson {
  schema_version: 1;
  id: string;
  title: string;
  actions: [number, number, number];
  mechanic: string;
  scoring: { combo_step: number; on_beat_bonus: number; coop_bonus: number };
  pacing: { bpm: number; session_seconds: number; curve: string };
  challenge: { spawn_interval: number; speed_ramp: number; difficulty: number };
  end: { mode: string; lives: number; goal_score: number };
  theme: { primary: number; secondary: number; style: string };
}

/** Minimal node shape these helpers rely on (avoids a hard @xyflow/react dependency). */
export interface StudioNodeData {
  label: string;
  category: string;
  value: string;
  [key: string]: unknown;
}

export interface StudioNodeLike<D extends StudioNodeData = StudioNodeData> {
  id: string;
  position: { x: number; y: number };
  data: D;
}

export interface StudioEdgeLike {
  id: string;
  source: string;
  target: string;
}

export function buildConfig<D extends StudioNodeData>(nodes: StudioNodeLike<D>[], title: string, bpm: number, sessionSeconds: number): GameConfigJson | null {
  const mechanicNodes = nodes.filter((n) => n.data.category === 'mechanic');
  const actionNodes = nodes.filter((n) => n.data.category === 'action');
  const scoringNodes = nodes.filter((n) => n.data.category === 'scoring');
  const pacingNodes = nodes.filter((n) => n.data.category === 'pacing');
  const endNodes = nodes.filter((n) => n.data.category === 'end');

  if (mechanicNodes.length !== 1) return null;
  if (actionNodes.length < 1 || actionNodes.length > 3) return null;
  if (endNodes.length < 1) return null;

  const actions: [number, number, number] = [0, 0, 0];
  actionNodes.slice(0, 3).forEach((n, i) => { actions[i] = Number(n.data.value); });

  const scoring = {
    combo_step: scoringNodes.some((n) => n.data.value === 'combo') ? 0.15 : 0.0,
    on_beat_bonus: scoringNodes.some((n) => n.data.value === 'on-beat') ? 1.5 : 1.0,
    coop_bonus: scoringNodes.some((n) => n.data.value === 'coop') ? 1.5 : 1.0,
  };

  const curve = (pacingNodes[0]?.data.value as string | undefined) ?? 'breath-arc';
  const endMode = (endNodes[0]?.data.value as string | undefined) ?? 'time-based';

  return {
    schema_version: 1,
    id: title.toLowerCase().replace(/[^a-z0-9]+/g, '-') || 'custom-game',
    title: title.trim() || 'My Game',
    actions,
    mechanic: mechanicNodes[0].data.value as string,
    scoring,
    pacing: { bpm, session_seconds: sessionSeconds, curve },
    challenge: { spawn_interval: 1.2, speed_ramp: 1.5, difficulty: 0.5 },
    end: { mode: endMode, lives: endMode === 'life-based' ? 3 : 0, goal_score: endMode === 'goal-based' ? 100 : 0 },
    theme: { primary: 0x00E5FF, secondary: 0xFF4081, style: 'neon' },
  };
}

// --- Validation ---
export function validate<D extends StudioNodeData>(nodes: StudioNodeLike<D>[]): string[] {
  const errors: string[] = [];
  const mechanicNodes = nodes.filter((n) => n.data.category === 'mechanic');
  const actionNodes = nodes.filter((n) => n.data.category === 'action');
  const scoringNodes = nodes.filter((n) => n.data.category === 'scoring');
  const pacingNodes = nodes.filter((n) => n.data.category === 'pacing');
  const endNodes = nodes.filter((n) => n.data.category === 'end');

  if (mechanicNodes.length === 0) errors.push('Add exactly 1 Mechanic node');
  if (mechanicNodes.length > 1) errors.push('Only 1 Mechanic node allowed');
  if (actionNodes.length === 0) errors.push('Add at least 1 Action node');
  if (actionNodes.length > 3) errors.push('Max 3 Action nodes (simplicity)');
  if (scoringNodes.length === 0) errors.push('Add at least 1 Scoring node');
  if (pacingNodes.length === 0) errors.push('Add at least 1 Pacing node');
  if (endNodes.length === 0) errors.push('Add at least 1 End node');
  return errors;
}

// --- Config <-> node graph mapping (for Import and ?game= deep links) ---
const MECHANIC_VALUES = ['dodge', 'pump', 'catch', 'hold', 'pattern'];

export function validateConfigShape(raw: unknown): string[] {
  if (typeof raw !== 'object' || raw === null || Array.isArray(raw)) return ['config must be a JSON object'];
  const cfg = raw as Record<string, unknown>;
  const errors: string[] = [];
  if (cfg.schema_version !== 1) errors.push('schema_version must be 1');
  if (typeof cfg.id !== 'string' || cfg.id.trim() === '') errors.push('id is required');
  if (typeof cfg.title !== 'string' || cfg.title.trim() === '') errors.push('title is required');
  if (!Array.isArray(cfg.actions) || cfg.actions.length > 3) errors.push('actions must be an array of up to 3 masks');
  if (typeof cfg.mechanic !== 'string' || !MECHANIC_VALUES.includes(cfg.mechanic)) errors.push('mechanic is invalid');
  return errors;
}

/** Fill in serde-style defaults so an imported config is complete and Rust-compatible. */
export function normalizeConfig(raw: Record<string, unknown>): GameConfigJson {
  const cfg = raw as Partial<GameConfigJson>;
  const actions: number[] = Array.isArray(cfg.actions) ? cfg.actions.slice(0, 3) : [];
  while (actions.length < 3) actions.push(0);
  return {
    schema_version: 1,
    id: cfg.id ?? 'custom-game',
    title: cfg.title ?? 'My Game',
    actions: [actions[0] ?? 0, actions[1] ?? 0, actions[2] ?? 0],
    mechanic: cfg.mechanic ?? 'dodge',
    scoring: {
      combo_step: cfg.scoring?.combo_step ?? 0.15,
      on_beat_bonus: cfg.scoring?.on_beat_bonus ?? 1.5,
      coop_bonus: cfg.scoring?.coop_bonus ?? 1.0,
    },
    pacing: {
      bpm: cfg.pacing?.bpm ?? 120,
      session_seconds: cfg.pacing?.session_seconds ?? 60,
      curve: cfg.pacing?.curve ?? 'breath-arc',
    },
    challenge: {
      spawn_interval: cfg.challenge?.spawn_interval ?? 1.2,
      speed_ramp: cfg.challenge?.speed_ramp ?? 1.5,
      difficulty: cfg.challenge?.difficulty ?? 0.5,
    },
    end: {
      mode: cfg.end?.mode ?? 'time-based',
      lives: cfg.end?.lives ?? 0,
      goal_score: cfg.end?.goal_score ?? 0,
    },
    theme: {
      primary: cfg.theme?.primary ?? 0x00e5ff,
      secondary: cfg.theme?.secondary ?? 0xff4081,
      style: cfg.theme?.style ?? 'neon',
    },
  };
}

/** Reconstruct the editor node graph from a saved config. */
export function configToNodes(config: GameConfigJson): { nodes: StudioNodeLike[]; edges: StudioEdgeLike[]; title: string; bpm: number; sessionSeconds: number } {
  const nodes: StudioNodeLike[] = [];
  const edges: StudioEdgeLike[] = [];
  let seq = 0;
  const newId = () => String(++seq);

  const mechanicItem = PALETTE.find((p) => p.type === 'mechanic' && p.value === config.mechanic);
  const mechanicId = newId();
  nodes.push({ id: mechanicId, position: { x: 60, y: 180 }, data: { label: mechanicItem?.label ?? config.mechanic, category: 'mechanic', value: config.mechanic } });

  let slot = 0;
  const addLinked = (category: NodeCategory, value: string, label: string) => {
    const id = newId();
    nodes.push({ id, position: { x: 320 + (slot % 2) * 240, y: 60 + Math.floor(slot / 2) * 100 }, data: { label, category, value } });
    edges.push({ id: `e${mechanicId}-${id}`, source: mechanicId, target: id });
    slot += 1;
  };

  config.actions.filter((mask) => mask !== 0).forEach((mask) => {
    const action = ACTIONS.find((a) => a.mask === mask);
    addLinked('action', String(mask), action?.label ?? `Action ${mask}`);
  });
  if (config.scoring.combo_step > 0) addLinked('scoring', 'combo', 'Combo');
  if (config.scoring.on_beat_bonus > 1) addLinked('scoring', 'on-beat', 'On-Beat Bonus');
  if (config.scoring.coop_bonus > 1) addLinked('scoring', 'coop', 'Co-Op Bonus');
  const curveItem = PALETTE.find((p) => p.type === 'pacing' && p.value === config.pacing.curve);
  addLinked('pacing', config.pacing.curve, curveItem?.label ?? config.pacing.curve);
  const endItem = PALETTE.find((p) => p.type === 'end' && p.value === config.end.mode);
  addLinked('end', config.end.mode, endItem?.label ?? config.end.mode);

  return { nodes, edges, title: config.title, bpm: config.pacing.bpm, sessionSeconds: config.pacing.session_seconds };
}

/** Build a ready-to-paste prompt that asks an agent to author this game via the BrainBreak MCP. */
export function buildAgentPrompt(config: GameConfigJson): string {
  const actionLabels = config.actions
    .filter((mask) => mask !== 0)
    .map((mask) => ACTIONS.find((a) => a.mask === mask)?.label ?? `mask ${mask}`);
  const primary = `#${config.theme.primary.toString(16).padStart(6, '0')}`;
  const secondary = `#${config.theme.secondary.toString(16).padStart(6, '0')}`;
  const endDetail = config.end.mode === 'life-based'
    ? `, lives=${config.end.lives}`
    : config.end.mode === 'goal-based'
      ? `, goal_score=${config.end.goal_score}`
      : '';
  return [
    'You are an agent working on the BrainBreak Motion Party project: a camera-controlled, config-driven HTML5 game (Rust/WASM engine + React Studio).',
    'The user designed a new game in the BrainBreak Studio and wants you to author it into the project.',
    '',
    `Game design — "${config.title}" (id: ${config.id})`,
    `- Mechanic: ${config.mechanic}`,
    `- Actions (max 3): ${actionLabels.join(', ') || 'none'}`,
    `- Scoring: combo_step=${config.scoring.combo_step}, on_beat_bonus=${config.scoring.on_beat_bonus}, coop_bonus=${config.scoring.coop_bonus}`,
    `- Pacing: ${config.pacing.bpm} BPM, ${config.pacing.session_seconds}s session, curve=${config.pacing.curve}`,
    `- Challenge: spawn_interval=${config.challenge.spawn_interval}, speed_ramp=${config.challenge.speed_ramp}, difficulty=${config.challenge.difficulty}`,
    `- End: mode=${config.end.mode}${endDetail}`,
    `- Theme: style=${config.theme.style}, primary=${primary}, secondary=${secondary}`,
    '',
    'Full GameConfig JSON:',
    '```json',
    JSON.stringify(config, null, 2),
    '```',
    '',
    'Instructions:',
    '1. Call the BrainBreak MCP tool "validate_game" with this JSON to confirm it is valid.',
    `2. Call the BrainBreak MCP tool "save_game" with this JSON to persist it to brainbreak/games/${config.id}.game.json.`,
    '3. Optionally author tasteful variations (different pacing/theme/end) with additional "save_game" calls, or suggest matching resources.',
    '4. Playback is per-device (localStorage): instruct the user to open the Studio, use Import, and paste this JSON to play it under "MY GAMES".',
    '5. Do not modify the Rust engine — games are fully data-driven through this config schema.',
  ].join('\n');
}
