import JSZip from 'jszip';
import { proceduralBeat } from './audio';
import { registerBrainBreakPlugin, setCustomTargets, updatePoseBridge } from './bridge';
import { roomTransport } from './network';
import './style.css';

const video = document.querySelector<HTMLVideoElement>('#camera-video')!;
const overlay = document.querySelector<HTMLCanvasElement>('#pose-overlay')!;
const cameraStatus = document.querySelector<HTMLElement>('#camera-status')!;
const roomStatus = document.querySelector<HTMLElement>('#room-status')!;
const roomCode = document.querySelector<HTMLInputElement>('#room-code')!;

roomTransport.onStatus = (status) => { roomStatus.textContent = status; };

async function unlockAudio(): Promise<void> {
  await proceduralBeat.start();
}

document.querySelector('#camera-button')!.addEventListener('click', async () => {
  try {
    await unlockAudio();
    const { startVision } = await import('./vision');
    await startVision(video, overlay, updatePoseBridge, (status) => { cameraStatus.textContent = status; });
  } catch (error) {
    cameraStatus.textContent = error instanceof Error ? error.message : 'Camera failed';
  }
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
window.load('/brainbreak-game.wasm');
window.setTimeout(() => document.querySelector('#boot-screen')?.classList.add('hidden'), 900);
