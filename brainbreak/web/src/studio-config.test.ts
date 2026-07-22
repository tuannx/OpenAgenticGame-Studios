import { describe, expect, it } from 'vitest';
import {
  buildAgentPrompt,
  buildConfig,
  configToNodes,
  normalizeConfig,
  validate,
  validateConfigShape,
  type GameConfigJson,
  type StudioNodeLike,
} from './studio-config';

function node(id: string, category: string, value: string, label = value): StudioNodeLike {
  return { id, position: { x: 0, y: 0 }, data: { label, category, value } };
}

const validNodes: StudioNodeLike[] = [
  node('1', 'mechanic', 'catch', 'Catch'),
  node('2', 'action', '4', 'Jump'),
  node('3', 'action', '8', 'Squat'),
  node('4', 'scoring', 'combo', 'Combo'),
  node('5', 'pacing', 'breath-arc', 'Breath Arc'),
  node('6', 'end', 'time-based', 'Timer'),
];

describe('buildConfig + configToNodes round-trip', () => {
  it('rebuilds an equivalent config from its own node graph', () => {
    const config = buildConfig(validNodes, 'Star Catcher', 120, 60);
    expect(config).not.toBeNull();
    const restored = configToNodes(config as GameConfigJson);
    const rebuilt = buildConfig(restored.nodes, restored.title, restored.bpm, restored.sessionSeconds);
    expect(rebuilt).toEqual(config);
  });

  it('preserves title, bpm and session seconds through the round-trip', () => {
    const config = buildConfig(validNodes, 'Star Catcher', 132, 90) as GameConfigJson;
    const restored = configToNodes(config);
    expect(restored.title).toBe('Star Catcher');
    expect(restored.bpm).toBe(132);
    expect(restored.sessionSeconds).toBe(90);
  });

  it('reconstructs mechanic, action, scoring, pacing and end nodes', () => {
    const config = buildConfig(validNodes, 'Star Catcher', 120, 60) as GameConfigJson;
    const { nodes, edges } = configToNodes(config);
    const byCategory = (c: string) => nodes.filter((n) => n.data.category === c);
    expect(byCategory('mechanic')).toHaveLength(1);
    expect(byCategory('action')).toHaveLength(2);
    expect(byCategory('scoring')).toHaveLength(1);
    expect(byCategory('pacing')).toHaveLength(1);
    expect(byCategory('end')).toHaveLength(1);
    // Every non-mechanic node is linked back to the mechanic node.
    const mechanicId = byCategory('mechanic')[0].id;
    expect(edges).toHaveLength(nodes.length - 1);
    expect(edges.every((e) => e.source === mechanicId)).toBe(true);
  });
});

describe('validate', () => {
  it('accepts a complete graph', () => {
    expect(validate(validNodes)).toEqual([]);
  });

  it('flags missing categories', () => {
    const errors = validate([node('1', 'mechanic', 'dodge')]);
    expect(errors).toContain('Add at least 1 Action node');
    expect(errors).toContain('Add at least 1 Scoring node');
    expect(errors).toContain('Add at least 1 Pacing node');
    expect(errors).toContain('Add at least 1 End node');
  });
});

describe('validateConfigShape', () => {
  it('accepts a well-formed config', () => {
    const config = buildConfig(validNodes, 'Star Catcher', 120, 60) as GameConfigJson;
    expect(validateConfigShape(config)).toEqual([]);
  });

  it('rejects non-objects, bad schema_version and invalid mechanics', () => {
    expect(validateConfigShape(null)).toEqual(['config must be a JSON object']);
    expect(validateConfigShape({ schema_version: 2, id: 'x', title: 'y', actions: [], mechanic: 'catch' })).toContain('schema_version must be 1');
    expect(validateConfigShape({ schema_version: 1, id: 'x', title: 'y', actions: [], mechanic: 'spin' })).toContain('mechanic is invalid');
  });
});

describe('normalizeConfig', () => {
  it('pads actions to three and fills serde-style defaults', () => {
    const cfg = normalizeConfig({ schema_version: 1, id: 'a', title: 'A', actions: [4], mechanic: 'catch' });
    expect(cfg.actions).toEqual([4, 0, 0]);
    expect(cfg.pacing.bpm).toBe(120);
    expect(cfg.pacing.session_seconds).toBe(60);
    expect(cfg.end.mode).toBe('time-based');
    expect(cfg.theme.style).toBe('neon');
  });
});

describe('buildAgentPrompt', () => {
  it('embeds the design, the JSON and the MCP tool instructions', () => {
    const config = buildConfig(validNodes, 'Star Catcher', 120, 60) as GameConfigJson;
    const prompt = buildAgentPrompt(config);
    expect(prompt).toContain('"Star Catcher"');
    expect(prompt).toContain('Mechanic: catch');
    expect(prompt).toContain('validate_game');
    expect(prompt).toContain('save_game');
    expect(prompt).toContain(`brainbreak/games/${config.id}.game.json`);
    expect(prompt).toContain(JSON.stringify(config, null, 2));
  });
});
