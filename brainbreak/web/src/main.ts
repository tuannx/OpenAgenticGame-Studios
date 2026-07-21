import JSZip from 'jszip';
import { proceduralBeat } from './audio';
import { registerBrainBreakPlugin, setCameraEvaluation, setCustomTargets, updatePoseBridge } from './bridge';
import { roomTransport } from './network';
import './style.css';

declare const __BRAINBREAK_WASM_PATH__: string;
const CAMERA_GATE_SCHEMA = 2;

const video = document.querySelector<HTMLVideoElement>('#camera-video')!;
const overlay = document.querySelector<HTMLCanvasElement>('#pose-overlay')!;
const cameraStatus = document.querySelector<HTMLElement>('#camera-status')!;
const roomStatus = document.querySelector<HTMLElement>('#room-status')!;
const roomCode = document.querySelector<HTMLInputElement>('#room-code')!;
const cameraButton = document.querySelector<HTMLButtonElement>('#camera-button')!;
const motionGate = document.querySelector<HTMLElement>('#motion-gate')!;
const primaryCameraButton = document.querySelector<HTMLButtonElement>('#primary-camera-button')!;
const guideOnlyButton = document.querySelector<HTMLButtonElement>('#guide-only-button')!;
const guideWarning = document.querySelector<HTMLElement>('#guide-warning')!;
const gateError = document.querySelector<HTMLElement>('#gate-error')!;
motionGate.dataset.schema = String(CAMERA_GATE_SCHEMA);
let cameraRunning = false;

roomTransport.onStatus = (status) => { roomStatus.textContent = status; };

async function unlockAudio(): Promise<void> {
  await proceduralBeat.start();
}

function setGuideOnly(enabled: boolean): void {
  document.body.classList.toggle('guide-only', enabled);
  guideWarning.hidden = !enabled;
  setCameraEvaluation(!enabled && cameraRunning);
}

async function stopCamera(): Promise<void> {
  const { stopVision } = await import('./vision');
  stopVision();
  updatePoseBridge([]);
  setCameraEvaluation(false);
  video.srcObject = null;
  overlay.getContext('2d')?.clearRect(0, 0, overlay.width, overlay.height);
  cameraRunning = false;
  cameraButton.textContent = 'Enable motion';
  cameraStatus.textContent = 'Camera off • local only';
}

async function enableCamera(): Promise<void> {
  primaryCameraButton.disabled = true;
  primaryCameraButton.textContent = 'Starting camera…';
  gateError.textContent = '';
  try {
    await unlockAudio();
    const { startVision } = await import('./vision');
    await startVision(video, overlay, updatePoseBridge, (status) => { cameraStatus.textContent = status; });
    cameraRunning = true;
    setCameraEvaluation(true);
    setGuideOnly(false);
    motionGate.classList.add('hidden');
    cameraButton.textContent = 'Stop camera';
  } catch (error) {
    await stopCamera();
    const message = error instanceof Error ? error.message : 'Camera failed';
    cameraStatus.textContent = message;
    gateError.textContent = message;
  } finally {
    primaryCameraButton.disabled = false;
    primaryCameraButton.textContent = 'Enable camera & play';
  }
}

cameraButton.addEventListener('click', async () => {
  if (cameraRunning) {
    await stopCamera();
    setGuideOnly(true);
    motionGate.classList.remove('hidden');
  } else {
    await enableCamera();
  }
});

primaryCameraButton.addEventListener('click', () => void enableCamera());
guideOnlyButton.addEventListener('click', () => {
  motionGate.classList.add('hidden');
  setGuideOnly(true);
});

document.querySelector('#create-room')!.addEventListener('click', async () => {
  try {
    await unlockAudio();
    const code = await roomTransport.createRoom();
    roomCode.value = code;
    roomStatus.textContent = `Room ${code} • waiting for peer`;
    await navigator.clipboard?.writeText(code);
  } catch (error) {
    roomStatus.textContent = error instanceof Error ? error.message : 'Could not create room';
  }
});

document.querySelector('#join-room')!.addEventListener('click', async () => {
  const code = roomCode.value.trim().toUpperCase();
  if (code.length !== 8) {
    roomStatus.textContent = 'Enter an 8-character room code';
    return;
  }
  try {
    await unlockAudio();
    await roomTransport.joinRoom(code);
  } catch (error) {
    roomStatus.textContent = error instanceof Error ? error.message : 'Could not join room';
  }
});

document.querySelector('#audio-button')!.addEventListener('click', async (event) => {
  await unlockAudio();
  const muted = proceduralBeat.toggle();
  (event.currentTarget as HTMLButtonElement).textContent = muted ? 'Sound off' : 'Sound on';
});

document.querySelector<HTMLInputElement>('#pack-input')!.addEventListener('change', async (event) => {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (!file) return;
  try {
    const zip = await JSZip.loadAsync(file);
    const beatmapFile = zip.file('beatmap.json');
    if (!beatmapFile) throw new Error('Pack is missing beatmap.json');
    const beatmap = JSON.parse(await beatmapFile.async('text')) as { targets?: number[] };
    if (!Array.isArray(beatmap.targets) || beatmap.targets.length === 0) throw new Error('Pack has no targets');
    setCustomTargets(beatmap.targets);
    roomStatus.textContent = `Loaded ${file.name}`;
  } catch (error) {
    roomStatus.textContent = error instanceof Error ? error.message : 'Invalid content pack';
  }
});

registerBrainBreakPlugin();
setCameraEvaluation(false);
window.load(__BRAINBREAK_WASM_PATH__);
window.setTimeout(() => document.querySelector('#boot-screen')?.classList.add('hidden'), 900);
