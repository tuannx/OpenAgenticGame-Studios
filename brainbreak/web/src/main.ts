import { musicEngine } from './audio';
import {
  registerBrainBreakPlugin,
  setCameraEvaluation,
  setGameConfig,
  setLocalPoseCapacity,
  setPageActive,
  setReduceMotion,
  setSelectedGameMode,
} from './bridge';
import { readDeepLink, writeDeepLink, type DeepLink } from './deep-link';
import {
  availableModesForCapacity,
  currentMotionCapability,
} from './motion-capability';
import { roomTransport } from './network';
import { presentShellOwner } from './shell-presentation';
import type { ShellOwner } from './shell-presentation';
import {
  createLocalTasteStore,
  recommendTaste,
  recordOpacityPreference,
  recordOverlayPreference,
  recordSessionStarted,
  recordThemePreference,
} from './taste-profile';
import type { CameraOverlayMode, VisualTheme } from './vision';
import { VisionModuleLoader } from './vision-loader';
import { LauncherController } from './launcher-controller';
import { ReadyController } from './ready-controller';
import './style.css';

declare const __BRAINBREAK_WASM_PATH__: string;
const CAMERA_GATE_SCHEMA = 5;

// --- Shared DOM ---
const video = document.querySelector<HTMLVideoElement>('#camera-video')!;
const overlay = document.querySelector<HTMLCanvasElement>('#pose-overlay')!;
const cameraStatus = document.querySelector<HTMLElement>('#camera-status')!;
const cameraCard = document.querySelector<HTMLElement>('#camera-card')!;
const gameCanvas = document.querySelector<HTMLCanvasElement>('#glcanvas')!;
const controlPanel = document.querySelector<HTMLDetailsElement>('#control-panel')!;
const roomStatus = document.querySelector<HTMLElement>('#room-status')!;
const roomCode = document.querySelector<HTMLInputElement>('#room-code')!;
const cameraButton = document.querySelector<HTMLButtonElement>('#camera-button')!;
const motionGate = document.querySelector<HTMLElement>('#motion-gate')!;
const primaryCameraButton = document.querySelector<HTMLButtonElement>('#primary-camera-button')!;
const guideOnlyButton = document.querySelector<HTMLButtonElement>('#guide-only-button')!;
const guideDemoGate = document.querySelector<HTMLElement>('#guide-demo-gate')!;
const guideDemoImage = document.querySelector<HTMLImageElement>('#guide-demo-image')!;
const guideDemoTitle = document.querySelector<HTMLElement>('#guide-demo-title')!;
const guideDemoCameraButton = document.querySelector<HTMLButtonElement>('#guide-demo-camera-button')!;
const guideDemoBackButton = document.querySelector<HTMLButtonElement>('#guide-demo-back-button')!;
const guideWarning = document.querySelector<HTMLElement>('#guide-warning')!;
const orientationGate = document.querySelector<HTMLElement>('#orientation-gate')!;
const gateError = document.querySelector<HTMLElement>('#gate-error')!;
const audioButton = document.querySelector<HTMLButtonElement>('#audio-button')!;
const reduceMotionButton = document.querySelector<HTMLButtonElement>('#reduce-motion-button')!;
const trackStatus = document.querySelector<HTMLElement>('#track-status')!;
const readyGate = document.querySelector<HTMLElement>('#ready-gate')!;
const readyModeTitle = document.querySelector<HTMLElement>('#ready-mode-title')!;
const readyPoseImg = document.querySelector<HTMLImageElement>('#ready-pose-img')!;
const startGameNowButton = document.querySelector<HTMLButtonElement>('#start-game-now-button')!;
const backToModesButton = document.querySelector<HTMLButtonElement>('#back-to-modes-button')!;
const readyHoldRing = document.querySelector<HTMLElement>('#ready-hold-ring')!;
const readyGestureHint = document.querySelector<HTMLElement>('#ready-gesture-hint')!;
const modeCards = document.querySelectorAll<HTMLElement>('.mode-card');

// --- Custom games from Studio (localStorage) ---
const CUSTOM_GAMES_KEY = 'brainbreak-custom-games';

/** Deselect every mode card (built-in + custom) and activate the matching custom game card. */
function markCustomGameActive(configId: string): void {
  document.querySelectorAll<HTMLElement>('.mode-card').forEach((card) => {
    const selected = card.dataset.mode === 'custom' && card.dataset.configId === configId;
    card.classList.toggle('active', selected);
    card.setAttribute('aria-checked', String(selected));
  });
}

function initCustomGames(): void {
  const section = document.querySelector<HTMLElement>('#custom-games-section');
  if (!section) return;
  let games: { id: string; title: string; mechanic: string }[] = [];
  try { games = JSON.parse(localStorage.getItem(CUSTOM_GAMES_KEY) ?? '[]'); } catch { /* empty */ }
  if (games.length === 0) return;
  section.style.display = '';
  section.innerHTML = games.map((g) => `
    <div class="mode-card" data-mode="custom" data-config-id="${g.id}" role="radio" aria-checked="false" tabindex="-1">
      <div class="mode-badge">MY GAME</div>
      <div class="mode-info"><h3>${g.title}</h3><small>${g.mechanic}</small></div>
    </div>
  `).join('');
  section.querySelectorAll<HTMLElement>('.mode-card').forEach((card) => {
    card.addEventListener('click', () => {
      const configId = card.dataset.configId;
      const all: { id: string }[] = JSON.parse(localStorage.getItem(CUSTOM_GAMES_KEY) ?? '[]');
      const config = all.find((c) => c.id === configId);
      if (!config || !configId) return;
      markCustomGameActive(configId);
      setGameConfig(JSON.stringify(config));
      setSelectedGameMode(4);
      writeDeepLink({ game: configId });
    });
  });
}
initCustomGames();
// Capture the incoming deep link before the launcher's taste recommendation
// rewrites the address bar, so an explicit ?game= / ?mode= always wins.
const initialDeepLink: DeepLink = readDeepLink();

// --- Shared state ---
motionGate.dataset.schema = String(CAMERA_GATE_SCHEMA);
const motionCapability = currentMotionCapability();
const availablePlayableModes = availableModesForCapacity(motionCapability.localPoseCapacity);
motionGate.dataset.poseCapacity = String(motionCapability.localPoseCapacity);
setLocalPoseCapacity(motionCapability.localPoseCapacity);

let cameraRunning = false;
let musicStarted = false;
let reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
const tasteStore = createLocalTasteStore();
const previousTasteProfile = tasteStore.snapshot();
const tasteRecommendation = recommendTaste(previousTasteProfile);
tasteStore.update(recordSessionStarted);
const visionLoader = new VisionModuleLoader({
  mode: previousTasteProfile.preferredOverlay,
  theme: previousTasteProfile.preferredTheme,
});
const portraitOrientation = window.matchMedia('(orientation: portrait)');
let currentShellOwner: ShellOwner = 'launcher';

// --- Shell ownership ---
function setShellOwner(owner: ShellOwner): void {
  currentShellOwner = owner;
  const presentation = presentShellOwner(owner, portraitOrientation.matches);
  motionGate.classList.toggle('hidden', !presentation.launcherVisible);
  readyGate.classList.toggle('hidden', !presentation.readyVisible);
  guideDemoGate.classList.toggle('hidden', !presentation.guideVisible);
  orientationGate.classList.toggle('hidden', !presentation.orientationVisible);
  document.body.classList.toggle('launcher-open', presentation.launcherVisible);
  document.body.classList.toggle('gesture-setup', presentation.readyVisible);
  document.body.classList.toggle('guide-demo-open', presentation.guideVisible);
  document.body.classList.toggle('orientation-blocked', presentation.orientationVisible);
  gameCanvas.toggleAttribute('inert', presentation.backgroundInert);
  controlPanel.toggleAttribute('inert', presentation.backgroundInert);
  motionGate.toggleAttribute('inert', !presentation.launcherVisible);
  readyGate.toggleAttribute('inert', !presentation.readyVisible);
  guideDemoGate.toggleAttribute('inert', !presentation.guideVisible);
  orientationGate.toggleAttribute('inert', !presentation.orientationVisible);
  if (presentation.orientationVisible) requestAnimationFrame(() => orientationGate.focus());
}
setShellOwner('launcher');

portraitOrientation.addEventListener('change', () => {
  setShellOwner(currentShellOwner);
  if (portraitOrientation.matches) return;
  requestAnimationFrame(() => {
    if (currentShellOwner === 'launcher') primaryCameraButton.focus();
    else if (currentShellOwner === 'ready') readyGate.focus();
    else if (currentShellOwner === 'guide') guideDemoCameraButton.focus();
    else gameCanvas.focus();
  });
});

// --- Shared helpers ---
function setGuideOnly(enabled: boolean): void {
  document.body.classList.toggle('guide-only', enabled);
  guideWarning.hidden = !enabled;
  setCameraEvaluation(!enabled && cameraRunning);
}

async function unlockAudio(): Promise<boolean> {
  if (musicStarted) return true;
  try {
    await musicEngine.start();
    musicStarted = true;
    audioButton.textContent = 'Music on';
    trackStatus.textContent = 'Special Spotlight • 126 BPM';
    return true;
  } catch (error) {
    trackStatus.textContent = 'Music unavailable • visual beat active';
    console.warn('Music playback could not start', error);
    return false;
  }
}

// --- Controllers ---
let ready: ReadyController;

const launcher = new LauncherController({
  modeCards,
  primaryCameraButton,
  guideOnlyButton,
  motionGate,
  guideDemoGate,
  guideDemoImage,
  guideDemoTitle,
  guideDemoCameraButton,
  guideDemoBackButton,
  tasteStore,
  tasteRecommendation,
  availablePlayableModes,
  onOpenReady: () => ready.openReadyStanceCheck(),
  onGuideOnly: () => {
    void unlockAudio();
    setGuideOnly(true);
    setShellOwner('guide');
    guideDemoCameraButton.focus();
  },
  onGuideOnlyReturn: () => {
    setGuideOnly(false);
    setShellOwner('launcher');
  },
});

ready = new ReadyController({
  readyGate,
  readyModeTitle,
  readyPoseImg,
  startGameNowButton,
  backToModesButton,
  readyHoldRing,
  readyGestureHint,
  cameraCard,
  cameraStatus,
  cameraButton,
  gateError,
  video,
  overlay,
  gameCanvas,
  visionLoader,
  tasteStore,
  availablePlayableModes,
  getSelectedModeKey: () => launcher.selectedModeKey,
  resolvedLaunchMode: () => launcher.resolvedLaunchMode(),
  selectModeByKey: (key) => launcher.selectModeByKey(key),
  setShellOwner,
  setGuideOnly,
  unlockAudio,
  isCameraRunning: () => cameraRunning,
  setCameraRunning: (running) => { cameraRunning = running; },
  setCameraEvaluation,
  renderPrimaryCameraCta: () => launcher.renderPrimaryCameraCta(),
});

// --- Deep-link: apply an explicit ?game= / ?mode= from the URL on load ---
function applyDeepLink(link: DeepLink): void {
  if (link.game) {
    let all: { id: string }[] = [];
    try { all = JSON.parse(localStorage.getItem(CUSTOM_GAMES_KEY) ?? '[]'); } catch { /* empty */ }
    const config = all.find((c) => c.id === link.game);
    if (config) {
      markCustomGameActive(link.game);
      setGameConfig(JSON.stringify(config));
      setSelectedGameMode(4);
      return;
    }
  }
  if (link.mode) launcher.selectModeByKey(link.mode);
}
applyDeepLink(initialDeepLink);

// --- Room ---
roomTransport.onStatus = (status) => { roomStatus.textContent = status; };

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

// --- Audio toggle ---
audioButton.addEventListener('click', async () => {
  if (!musicStarted) {
    await unlockAudio();
    return;
  }
  const muted = musicEngine.toggle();
  audioButton.textContent = muted ? 'Music off' : 'Music on';
});

// --- Settings: overlay, opacity, theme ---
function isCameraOverlayMode(value: string): value is CameraOverlayMode {
  return value === 'glass_pip' || value === 'cutout' || value === 'hologram' || value === 'skeleton';
}

function isVisualTheme(value: string): value is VisualTheme {
  return value === 'cyber-trunk' || value === 'pink-girl' || value === 'purple-vaporwave';
}

function applyOverlayMode(mode: CameraOverlayMode): void {
  visionLoader.updatePreferences({ mode });
  const card = document.querySelector<HTMLElement>('#camera-card');
  if (!card) return;
  card.classList.remove('mode-glass-pip', 'mode-cutout', 'mode-hologram', 'mode-skeleton');
  card.classList.add(`mode-${mode.replace(/_/g, '-')}`);
}

function applyCameraOpacity(val: number): void {
  const opacityVal = document.querySelector<HTMLElement>('#opacity-val');
  document.documentElement.style.setProperty('--user-cam-opacity', (val / 100).toFixed(2));
  if (opacityVal) opacityVal.textContent = `${val}%`;
}

function applyTheme(theme: VisualTheme): void {
  visionLoader.updatePreferences({ theme });
  document.body.classList.remove('theme-cyber-trunk', 'theme-pink-girl', 'theme-purple-vaporwave');
  document.body.classList.add(`theme-${theme}`);
}

const modeSelect = document.querySelector('#overlay-mode-select') as HTMLSelectElement | null;
if (modeSelect) {
  modeSelect.value = previousTasteProfile.preferredOverlay;
  modeSelect.addEventListener('change', (e: Event) => {
    const value = (e.target as HTMLSelectElement).value;
    if (isCameraOverlayMode(value)) {
      applyOverlayMode(value);
      tasteStore.update((profile) => recordOverlayPreference(profile, value));
    }
  });
  if (isCameraOverlayMode(modeSelect.value)) applyOverlayMode(modeSelect.value);
}

const opacitySlider = document.querySelector('#camera-opacity-slider') as HTMLInputElement | null;
if (opacitySlider) {
  opacitySlider.value = String(previousTasteProfile.preferredCameraOpacity);
  opacitySlider.addEventListener('input', (e: Event) => {
    applyCameraOpacity(Number((e.target as HTMLInputElement).value));
  });
  opacitySlider.addEventListener('change', (e: Event) => {
    tasteStore.update((profile) => recordOpacityPreference(profile, Number((e.target as HTMLInputElement).value)));
  });
  applyCameraOpacity(Number(opacitySlider.value));
}

const themeSelect = document.querySelector('#theme-select') as HTMLSelectElement | null;
if (themeSelect) {
  themeSelect.value = previousTasteProfile.preferredTheme;
  themeSelect.addEventListener('change', (e: Event) => {
    const value = (e.target as HTMLSelectElement).value;
    if (isVisualTheme(value)) {
      applyTheme(value);
      tasteStore.update((profile) => recordThemePreference(profile, value));
    }
  });
  if (isVisualTheme(themeSelect.value)) applyTheme(themeSelect.value);
}

// --- Reduced motion ---
function applyReducedMotion(): void {
  document.body.classList.toggle('reduced-motion', reducedMotion);
  setReduceMotion(reducedMotion);
  reduceMotionButton.textContent = reducedMotion ? 'Reduced motion on' : 'Reduced motion off';
  reduceMotionButton.setAttribute('aria-pressed', String(reducedMotion));
}

reduceMotionButton.addEventListener('click', () => {
  reducedMotion = !reducedMotion;
  applyReducedMotion();
});

// --- Page activity ---
function syncPageActivity(): void {
  setPageActive(document.visibilityState === 'visible');
}

document.addEventListener('visibilitychange', syncPageActivity);
window.addEventListener('pagehide', () => setPageActive(false));
window.addEventListener('pageshow', syncPageActivity);

// --- Boot ---
registerBrainBreakPlugin();
setCameraEvaluation(false);
syncPageActivity();
applyReducedMotion();
window.load(__BRAINBREAK_WASM_PATH__);
