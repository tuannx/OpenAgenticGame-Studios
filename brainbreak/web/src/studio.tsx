import { useCallback, useEffect, useMemo, useState, type ChangeEvent } from 'react';
import { createRoot } from 'react-dom/client';
import {
  Background,
  Controls,
  MiniMap,
  ReactFlow,
  applyNodeChanges,
  applyEdgeChanges,
  addEdge,
  type Connection,
  type Edge,
  type EdgeChange,
  type Node,
  type NodeChange,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import './style.css';
import { readDeepLink, writeDeepLink } from './deep-link';
import {
  ACTIONS,
  CATEGORY_COLORS,
  PALETTE,
  buildAgentPrompt,
  buildConfig,
  configToNodes,
  normalizeConfig,
  validate,
  validateConfigShape,
  type GameConfigJson,
  type NodeCategory,
  type PaletteItem,
  type StudioEdgeLike,
  type StudioNodeData,
} from './studio-config';

// --- localStorage persistence ---
const STORAGE_KEY = 'brainbreak-custom-games';

function loadSavedGames(): GameConfigJson[] {
  try {
    return JSON.parse(localStorage.getItem(STORAGE_KEY) ?? '[]');
  } catch { return []; }
}

function saveGame(config: GameConfigJson): void {
  const games = loadSavedGames().filter((g) => g.id !== config.id);
  games.push(config);
  localStorage.setItem(STORAGE_KEY, JSON.stringify(games));
}

// --- Main Studio Component ---
let nodeId = 10;

function Studio() {
  const [title, setTitle] = useState('My Motion Game');
  const [bpm, setBpm] = useState(120);
  const [sessionSeconds, setSessionSeconds] = useState(60);
  const [message, setMessage] = useState('');
  const [importText, setImportText] = useState('');
  const [nodes, setNodes] = useState<Node<StudioNodeData>[]>([
    { id: '1', position: { x: 60, y: 160 }, data: { label: 'Dodge', category: 'mechanic', value: 'dodge' } },
    { id: '2', position: { x: 300, y: 60 }, data: { label: 'Jump', category: 'action', value: '4' } },
    { id: '3', position: { x: 300, y: 160 }, data: { label: 'Squat', category: 'action', value: '8' } },
    { id: '4', position: { x: 300, y: 260 }, data: { label: 'Combo', category: 'scoring', value: 'combo' } },
    { id: '5', position: { x: 540, y: 110 }, data: { label: 'Breath Arc', category: 'pacing', value: 'breath-arc' } },
    { id: '6', position: { x: 540, y: 230 }, data: { label: 'Timer', category: 'end', value: 'time-based' } },
  ]);
  const [edges, setEdges] = useState<StudioEdgeLike[]>([
    { id: 'e1-2', source: '1', target: '2' },
    { id: 'e1-3', source: '1', target: '3' },
    { id: 'e1-4', source: '1', target: '4' },
    { id: 'e2-5', source: '2', target: '5' },
    { id: 'e3-6', source: '3', target: '6' },
  ]);

  const onNodesChange = useCallback((changes: NodeChange<Node<StudioNodeData>>[]) => setNodes((cur) => applyNodeChanges<Node<StudioNodeData>>(changes, cur)), []);
  const onEdgesChange = useCallback((changes: EdgeChange<StudioEdgeLike>[]) => setEdges((cur) => applyEdgeChanges<StudioEdgeLike>(changes, cur)), []);
  const onConnect = useCallback((conn: Connection) => setEdges((cur) => addEdge(conn, cur)), []);

  const addNode = (item: PaletteItem) => {
    const id = String(++nodeId);
    setNodes((cur) => [...cur, {
      id,
      position: { x: 100 + Math.random() * 400, y: 80 + Math.random() * 250 },
      data: { label: item.label, category: item.type, value: item.value },
    }]);
  };

  const errors = useMemo(() => validate(nodes), [nodes]);
  const config = useMemo(() => buildConfig(nodes, title, bpm, sessionSeconds), [nodes, title, bpm, sessionSeconds]);

  const exportGame = () => {
    if (!config) {
      setMessage('Fix validation errors before exporting');
      return;
    }
    saveGame(config);
    writeDeepLink({ game: config.id });
    const blob = new Blob([JSON.stringify(config, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement('a');
    anchor.href = url;
    anchor.download = `${config.id}.game.json`;
    anchor.click();
    URL.revokeObjectURL(url);
    setMessage(`"${config.title}" saved + exported!`);
  };

  // Load a game into the editor from a ?game=<id> deep link (localStorage-backed).
  useEffect(() => {
    const link = readDeepLink();
    if (!link.game) return;
    const match = loadSavedGames().find((g) => g.id === link.game);
    if (!match) return;
    const restored = configToNodes(match);
    setNodes(restored.nodes);
    setEdges(restored.edges);
    setTitle(restored.title);
    setBpm(restored.bpm);
    setSessionSeconds(restored.sessionSeconds);
    setMessage(`Loaded "${match.title}" from link`);
  }, []);

  const importGame = (raw: string) => {
    let parsed: unknown;
    try {
      parsed = JSON.parse(raw);
    } catch {
      setMessage('Import failed: invalid JSON');
      return;
    }
    const shapeErrors = validateConfigShape(parsed);
    if (shapeErrors.length > 0) {
      setMessage(`Import failed: ${shapeErrors.join(', ')}`);
      return;
    }
    const config = normalizeConfig(parsed as Record<string, unknown>);
    const restored = configToNodes(config);
    setNodes(restored.nodes);
    setEdges(restored.edges);
    setTitle(restored.title);
    setBpm(restored.bpm);
    setSessionSeconds(restored.sessionSeconds);
    saveGame(config);
    writeDeepLink({ game: config.id });
    setImportText('');
    setMessage(`Imported "${config.title}" — now playable under MY GAMES`);
  };

  const onImportFile = (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = () => importGame(String(reader.result ?? ''));
    reader.readAsText(file);
    event.target.value = '';
  };

  const copyAgentPrompt = async () => {
    if (!config) return;
    try {
      await navigator.clipboard.writeText(buildAgentPrompt(config));
      setMessage('Agent prompt copied to clipboard');
    } catch {
      setMessage('Copy failed — select the Preview JSON manually');
    }
  };

  const savedGames = useMemo(() => loadSavedGames(), [message]);

  return (
    <main className="studio-shell">
      <header className="studio-header">
        <div><h1>BrainBreak Studio</h1><small>Mix components to create new games</small></div>
        <nav>
          <a href="/">Play</a>
          <button onClick={copyAgentPrompt} disabled={errors.length > 0}>Copy Agent Prompt</button>
          <button onClick={exportGame} disabled={errors.length > 0}>Export Game</button>
        </nav>
      </header>
      <section className="studio-grid">
        {/* Left palette */}
        <aside className="studio-panel">
          <h2>Components</h2>
          {(['mechanic', 'action', 'scoring', 'pacing', 'end'] as NodeCategory[]).map((cat) => (
            <div key={cat}>
              <h3 style={{ color: CATEGORY_COLORS[cat], margin: '12px 0 4px', textTransform: 'capitalize' }}>{cat}</h3>
              <div className="action-list">
                {PALETTE.filter((p) => p.type === cat).map((item) => (
                  <button key={item.value} className="action-chip" onClick={() => addNode(item)}>
                    <span>{item.label}</span><strong>+</strong>
                  </button>
                ))}
              </div>
            </div>
          ))}
        </aside>

        {/* Center canvas */}
        <section>
          <div className="studio-canvas">
            <ReactFlow
              nodes={nodes.map((n) => ({
                ...n,
                style: {
                  background: CATEGORY_COLORS[n.data.category as NodeCategory] ?? '#333',
                  color: '#fff',
                  border: 'none',
                  borderRadius: 8,
                  padding: '8px 14px',
                  fontSize: 13,
                },
              }))}
              edges={edges}
              onNodesChange={onNodesChange}
              onEdgesChange={onEdgesChange}
              onConnect={onConnect}
              deleteKeyCode={['Backspace', 'Delete']}
              fitView
            >
              <MiniMap /><Controls /><Background gap={20} color="#26324a" />
            </ReactFlow>
          </div>
          {/* Validation */}
          <div className="timeline" style={{ padding: '8px 12px', gap: 8 }}>
            {errors.length === 0
              ? <span style={{ color: '#69f0ae' }}>Valid game config</span>
              : errors.map((e) => <span key={e} style={{ color: '#ff5252' }}>{e}</span>)}
          </div>
        </section>

        {/* Right properties */}
        <aside className="studio-panel">
          <h2>Game Properties</h2>
          <label>Title<input value={title} onChange={(e) => setTitle(e.target.value)} /></label>
          <label>BPM<input type="number" min="60" max="180" value={bpm} onChange={(e) => setBpm(Number(e.target.value))} /></label>
          <label>Session (s)<input type="number" min="30" max="120" value={sessionSeconds} onChange={(e) => setSessionSeconds(Number(e.target.value))} /></label>

          {config && (
            <details style={{ marginTop: 12 }}>
              <summary>Preview JSON</summary>
              <pre style={{ fontSize: 10, maxHeight: 200, overflow: 'auto' }}>{JSON.stringify(config, null, 2)}</pre>
            </details>
          )}

          <p className={message.includes('saved') ? 'validation-ok' : 'validation-error'}>{message}</p>

          {savedGames.length > 0 && (
            <>
              <h3 style={{ marginTop: 16 }}>Saved Games</h3>
              <ul style={{ fontSize: 12, paddingLeft: 16 }}>
                {savedGames.map((g) => <li key={g.id}>{g.title} ({g.mechanic})</li>)}
              </ul>
            </>
          )}

          <h3 style={{ marginTop: 16 }}>Import Game</h3>
          <textarea
            value={importText}
            onChange={(e) => setImportText(e.target.value)}
            placeholder="Paste a .game.json config here…"
            style={{ width: '100%', minHeight: 80, fontSize: 10, marginTop: 4 }}
          />
          <div style={{ display: 'flex', gap: 8, marginTop: 6 }}>
            <button onClick={() => importGame(importText)} disabled={importText.trim() === ''}>Import text</button>
            <label style={{ fontSize: 12 }}>
              Upload .game.json
              <input type="file" accept=".json,.game.json,application/json" onChange={onImportFile} style={{ display: 'none' }} />
            </label>
          </div>

          <small style={{ marginTop: 12, display: 'block' }}>
            Exported games save to localStorage and appear in the game launcher under "MY GAMES".
          </small>
        </aside>
      </section>
    </main>
  );
}

createRoot(document.querySelector('#studio-root')!).render(<Studio />);
