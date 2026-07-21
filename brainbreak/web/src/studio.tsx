import { useCallback, useMemo, useState } from 'react';
import { createRoot } from 'react-dom/client';
import JSZip from 'jszip';
import {
  Background,
  Controls,
  MiniMap,
  ReactFlow,
  applyEdgeChanges,
  applyNodeChanges,
  addEdge,
  type Connection,
  type Edge,
  type EdgeChange,
  type Node,
  type NodeChange,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import './style.css';

const ACTIONS = [
  ['Move left', 1], ['Move right', 2], ['Jump', 4], ['Squat', 8],
  ['Left hand up', 16], ['Right hand up', 32], ['Clap', 64],
] as const;

interface StudioManifest {
  schemaVersion: 1;
  title: string;
  locale: string;
  bpm: number;
}

function Studio() {
  const [title, setTitle] = useState('My Motion Mix');
  const [bpm, setBpm] = useState(112);
  const [targets, setTargets] = useState<number[]>([16, 32, 64, 8, 1, 2, 4, 64]);
  const [selectedAction, setSelectedAction] = useState(16);
  const [nodes, setNodes] = useState<Node[]>([
    { id: 'pose', position: { x: 70, y: 120 }, data: { label: 'Pose landmarks' }, type: 'input' },
    { id: 'hold', position: { x: 310, y: 80 }, data: { label: 'Hold ≥ 180 ms' } },
    { id: 'confidence', position: { x: 310, y: 190 }, data: { label: 'Confidence ≥ 0.55' } },
    { id: 'action', position: { x: 580, y: 130 }, data: { label: 'Emit action' }, type: 'output' },
  ]);
  const [edges, setEdges] = useState<Edge[]>([
    { id: 'pose-hold', source: 'pose', target: 'hold' },
    { id: 'pose-confidence', source: 'pose', target: 'confidence' },
    { id: 'hold-action', source: 'hold', target: 'action' },
    { id: 'confidence-action', source: 'confidence', target: 'action' },
  ]);
  const [message, setMessage] = useState('Pack is valid');
  const onNodesChange = useCallback((changes: NodeChange[]) => setNodes((current) => applyNodeChanges(changes, current)), []);
  const onEdgesChange = useCallback((changes: EdgeChange[]) => setEdges((current) => applyEdgeChanges(changes, current)), []);
  const onConnect = useCallback((connection: Connection) => setEdges((current) => addEdge(connection, current)), []);
  const manifest = useMemo<StudioManifest>(() => ({ schemaVersion: 1, title: title.trim(), locale: 'en', bpm }), [title, bpm]);

  const appendBeat = () => setTargets((current) => [...current, selectedAction].slice(0, 64));
  const exportPack = async () => {
    if (!manifest.title || bpm < 60 || bpm > 180 || targets.length === 0) {
      setMessage('Title, BPM 60–180, and at least one beat are required');
      return;
    }
    const zip = new JSZip();
    zip.file('manifest.json', JSON.stringify(manifest, null, 2));
    zip.file('beatmap.json', JSON.stringify({ schemaVersion: 1, targets }, null, 2));
    zip.file('actions.json', JSON.stringify({ schemaVersion: 1, graph: { nodes, edges } }, null, 2));
    zip.file('LICENSES.json', JSON.stringify({ tracks: [], note: 'Add rights-cleared audio before distribution.' }, null, 2));
    const blob = await zip.generateAsync({ type: 'blob', compression: 'DEFLATE' });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement('a');
    anchor.href = url;
    anchor.download = `${title.toLowerCase().replace(/[^a-z0-9]+/g, '-') || 'brainbreak'}.brainbreak.zip`;
    anchor.click();
    URL.revokeObjectURL(url);
    setMessage('Pack exported successfully');
  };

  return (
    <main className="studio-shell">
      <header className="studio-header">
        <div><h1>BrainBreak Studio</h1><small>Visual motion and beat authoring</small></div>
        <nav><a href="/">Play</a><button onClick={() => void exportPack()}>Export pack</button></nav>
      </header>
      <section className="studio-grid">
        <aside className="studio-panel">
          <h2>Action palette</h2>
          <div className="action-list">
            {ACTIONS.map(([label, mask]) => (
              <button key={mask} className="action-chip" onClick={() => setSelectedAction(mask)}>
                <span>{label}</span><strong>{selectedAction === mask ? '●' : '+'}</strong>
              </button>
            ))}
          </div>
          <button onClick={appendBeat} style={{ marginTop: 16, width: '100%' }}>Add selected beat</button>
        </aside>
        <section>
          <div className="studio-canvas">
            <ReactFlow nodes={nodes} edges={edges} onNodesChange={onNodesChange} onEdgesChange={onEdgesChange} onConnect={onConnect} fitView>
              <MiniMap /><Controls /><Background gap={20} color="#26324a" />
            </ReactFlow>
          </div>
          <div className="timeline">
            {targets.map((target, index) => (
              <button className="beat-cell" key={`${index}-${target}`} onClick={() => setTargets((current) => current.filter((_, beat) => beat !== index))}>
                {index + 1}<small>{ACTIONS.find(([, mask]) => mask === target)?.[0]}</small>
              </button>
            ))}
          </div>
        </section>
        <aside className="studio-panel">
          <h2>Pack properties</h2>
          <label>Title<input value={title} onChange={(event) => setTitle(event.target.value)} /></label>
          <label>BPM<input type="number" min="60" max="180" value={bpm} onChange={(event) => setBpm(Number(event.target.value))} /></label>
          <label>Hint style<select defaultValue="ghost"><option value="ghost">Ghost silhouette + text</option><option value="icon">Icon + audio</option></select></label>
          <label>Minimum confidence<input type="number" min="0.2" max="0.95" step="0.05" defaultValue="0.55" /></label>
          <label>Hold duration (ms)<input type="number" min="0" max="2000" defaultValue="180" /></label>
          <p className={message.includes('valid') || message.includes('success') ? 'validation-ok' : 'validation-error'}>{message}</p>
          <small>Exported packs stay local and can be imported from the game control panel.</small>
        </aside>
      </section>
    </main>
  );
}

createRoot(document.querySelector('#studio-root')!).render(<Studio />);
