import { isCameraStartCancelled } from './camera-start-cancellation';
import {
  hasCustomGameConfig,
  setCameraEvaluation,
  setSelectedGameMode,
  updatePoseBridge,
} from './bridge';
import type { CameraFrameGeometry } from './motion-capability';
import { MotionNavigationController } from './motion-navigation';
import type { PlayableGameMode } from './motion-navigation';
import { PoseFramingCoach } from './pose-framing';
import { presentPoseFraming } from './pose-framing-status';
import { presentReadyNavigation } from './ready-navigation-presentation';
import type { LocalTasteStore } from './taste-profile';
import { recordCameraAttempt, recordPlayStart, recordSetupExit } from './taste-profile';
import type { PoseSnapshot, VisionStatus } from './vision';
import type { VisionModuleLoader } from './vision-loader';
import {
  presentCameraStartFailure,
  presentReadyCameraStatus,
  presentVisionStatus,
} from './vision-status';
import { GAME_MODE_PRESENTATION, modeLabel } from './launcher-controller';
import type { LaunchMode } from './launcher-controller';

export interface ReadyDeps {
  readyGate: HTMLElement;
  readyModeTitle: HTMLElement;
  readyPoseImg: HTMLImageElement;
  startGameNowButton: HTMLButtonElement;
  backToModesButton: HTMLButtonElement;
  readyHoldRing: HTMLElement;
  readyGestureHint: HTMLElement;
  cameraCard: HTMLElement;
  cameraStatus: HTMLElement;
  cameraButton: HTMLButtonElement;
  gateError: HTMLElement;
  video: HTMLVideoElement;
  overlay: HTMLCanvasElement;
  gameCanvas: HTMLCanvasElement;
  visionLoader: VisionModuleLoader;
  tasteStore: LocalTasteStore;
  availablePlayableModes: readonly PlayableGameMode[];
  getSelectedModeKey: () => LaunchMode;
  resolvedLaunchMode: () => PlayableGameMode;
  selectModeByKey: (key: string) => void;
  setShellOwner: (owner: 'launcher' | 'ready' | 'guide' | 'gameplay') => void;
  setGuideOnly: (enabled: boolean) => void;
  unlockAudio: () => Promise<boolean>;
  isCameraRunning: () => boolean;
  setCameraRunning: (running: boolean) => void;
  setCameraEvaluation: (enabled: boolean) => void;
  renderPrimaryCameraCta: () => void;
}

export class ReadyController {
  private deps: ReadyDeps;
  private readyNavigation: MotionNavigationController;
  private framingCoach = new PoseFramingCoach();
  private activeReadyStartedAt: number | null = null;
  private cameraSetupPending: Promise<void> | null = null;
  private activeCameraStartController: AbortController | null = null;
  private readyFallbackEnabled = false;

  constructor(deps: ReadyDeps) {
    this.deps = deps;
    this.readyNavigation = new MotionNavigationController('mirror', {}, deps.availablePlayableModes);
    this.initListeners();
  }

  async openReadyStanceCheck(): Promise<void> {
    if (this.cameraSetupPending) return this.cameraSetupPending;
    const controller = new AbortController();
    this.activeCameraStartController = controller;
    this.cameraSetupPending = this.runReadyStanceCheck(controller.signal).finally(() => {
      if (this.activeCameraStartController === controller) this.activeCameraStartController = null;
      this.cameraSetupPending = null;
    });
    return this.cameraSetupPending;
  }

  async stopCamera(): Promise<void> {
    const { visionLoader, video, overlay, cameraCard, cameraButton, cameraStatus } = this.deps;
    visionLoader.stopIfLoaded();
    this.framingCoach.reset();
    cameraCard.dataset.framing = 'searching';
    cameraCard.style.removeProperty('--camera-aspect-ratio');
    delete cameraCard.dataset.frameOrientation;
    updatePoseBridge([]);
    setCameraEvaluation(false);
    video.srcObject = null;
    overlay.getContext('2d')?.clearRect(0, 0, overlay.width, overlay.height);
    this.deps.setCameraRunning(false);
    cameraButton.textContent = 'Enable motion';
    cameraStatus.textContent = 'Camera off • local only';
  }

  async cancelReadySetup(): Promise<void> {
    const { readyGate, setShellOwner, setGuideOnly, renderPrimaryCameraCta } = this.deps;
    if (readyGate.classList.contains('hidden')) return;
    this.activeCameraStartController?.abort();
    this.recordActiveSetupExit();
    await this.stopCamera();
    setGuideOnly(false);
    setShellOwner('launcher');
    this.deps.cameraButton.disabled = false;
    renderPrimaryCameraCta();
    requestAnimationFrame(() => this.deps.cameraButton.focus());
  }

  get selectedMode(): PlayableGameMode {
    return this.readyNavigation.selectedMode();
  }

  private initListeners(): void {
    const { readyGate, startGameNowButton, backToModesButton, cameraButton } = this.deps;

    readyGate.addEventListener('keydown', (event) => {
      if (event.key === 'Escape') {
        event.preventDefault();
        void this.cancelReadySetup();
        return;
      }
      if (event.key !== 'Tab') return;
      const targets = this.visibleReadyFocusTargets();
      const first = targets[0];
      const last = targets.at(-1);
      if (!first || !last) {
        event.preventDefault();
        readyGate.focus();
      } else if (document.activeElement === readyGate) {
        event.preventDefault();
        (event.shiftKey ? last : first).focus();
      } else if (!event.shiftKey && document.activeElement === last) {
        event.preventDefault();
        first.focus();
      } else if (event.shiftKey && document.activeElement === first) {
        event.preventDefault();
        last.focus();
      }
    });

    cameraButton.addEventListener('click', async () => {
      if (this.deps.isCameraRunning()) {
        this.recordActiveSetupExit();
        await this.stopCamera();
        this.deps.setGuideOnly(true);
        this.deps.setShellOwner('launcher');
      } else {
        await this.openReadyStanceCheck();
      }
    });

    startGameNowButton.addEventListener('click', () => {
      if (this.readyFallbackEnabled) {
        this.completeGestureLaunch(this.readyNavigation.selectedMode(), 'touch-fallback');
      }
    });

    backToModesButton.addEventListener('click', async () => {
      const selectedMode = this.readyNavigation.selectedMode();
      this.deps.selectModeByKey(selectedMode);
      this.recordActiveSetupExit();
      await this.stopCamera();
      this.deps.setShellOwner('launcher');
      requestAnimationFrame(() => this.deps.cameraButton.focus());
    });
  }

  private visibleReadyFocusTargets(): HTMLElement[] {
    return [...this.deps.readyGate.querySelectorAll<HTMLElement>(
      'summary, button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])',
    )].filter((element) => element.getClientRects().length > 0 && !element.closest('[inert]'));
  }

  private setReadyFallbackEnabled(enabled: boolean): void {
    this.readyFallbackEnabled = enabled;
    this.deps.startGameNowButton.disabled = !enabled;
    this.deps.backToModesButton.disabled = !enabled;
  }

  private focusReadyOwner(): void {
    requestAnimationFrame(() => {
      if (!this.deps.readyGate.classList.contains('hidden')) this.deps.readyGate.focus();
    });
  }

  private showPreparingVision(): void {
    const { readyGate, cameraCard, cameraStatus } = this.deps;
    readyGate.dataset.phase = 'preparing';
    readyGate.dataset.framing = 'searching';
    cameraCard.dataset.framing = 'searching';
    cameraStatus.textContent = 'PREPARING MOTION AI';
    readyGate.dataset.action = 'framing';
    this.setReadyFallbackEnabled(false);
  }

  private renderVisionStatus(status: VisionStatus): void {
    const { readyGate, cameraCard, cameraStatus } = this.deps;
    const presentation = presentVisionStatus(status);
    readyGate.dataset.phase = presentation.phase;
    if (presentation.phase !== 'tracking') {
      readyGate.dataset.framing = 'searching';
      readyGate.dataset.action = 'framing';
      cameraCard.dataset.framing = 'searching';
    }
    const readyCameraStatus = presentReadyCameraStatus(presentation);
    if (readyCameraStatus) {
      cameraStatus.textContent = readyCameraStatus;
    } else if (readyGate.classList.contains('hidden')) {
      cameraStatus.textContent = presentation.camera;
    }
    if (presentation.phase !== 'tracking') this.setReadyFallbackEnabled(presentation.phase === 'error');
  }

  private recordActiveSetupExit(): void {
    if (this.activeReadyStartedAt === null) return;
    this.deps.tasteStore.update(recordSetupExit);
    this.activeReadyStartedAt = null;
  }

  private applyCameraFrameGeometry(geometry: CameraFrameGeometry): void {
    const { cameraCard } = this.deps;
    cameraCard.style.setProperty('--camera-aspect-ratio', String(geometry.aspectRatio));
    cameraCard.dataset.frameOrientation = geometry.orientation;
  }

  private renderReadyMode(mode: PlayableGameMode, fromRandom = false): void {
    const { readyModeTitle, readyPoseImg, readyGestureHint } = this.deps;
    const presentation = GAME_MODE_PRESENTATION[mode];
    readyModeTitle.textContent = fromRandom ? `RANDOM - ${modeLabel(mode)}` : modeLabel(mode);
    readyPoseImg.src = presentation.imageSrc;
    readyPoseImg.alt = `${presentation.title} game`;
    readyGestureHint.textContent = mode === 'duo'
      ? 'BOTH PLAYERS: RAISE A HAND'
      : 'RAISE A HAND TO PLAY';
  }

  private completeGestureLaunch(mode: PlayableGameMode, entry: 'gesture' | 'touch-fallback'): void {
    const customActive = hasCustomGameConfig();
    const presentation = GAME_MODE_PRESENTATION[mode];
    const readyMs = this.activeReadyStartedAt === null ? undefined : performance.now() - this.activeReadyStartedAt;
    this.deps.tasteStore.update((profile) => recordPlayStart(profile, mode, entry, readyMs));
    this.activeReadyStartedAt = null;
    if (!customActive) setSelectedGameMode(presentation.modeVal);
    this.deps.setGuideOnly(false);
    this.deps.setShellOwner('gameplay');
    this.deps.cameraButton.textContent = 'Stop camera';
    this.deps.cameraStatus.textContent = customActive
      ? 'Custom game • camera evaluated'
      : `${presentation.title} • camera evaluated`;
    this.deps.gameCanvas.focus();
  }

  private updateReadyNavigation = (poses: PoseSnapshot[]): void => {
    updatePoseBridge(poses);
    const { readyGate, readyHoldRing, cameraCard, cameraStatus } = this.deps;
    if (readyGate.classList.contains('hidden')) return;
    readyGate.dataset.phase = 'tracking';

    const nowMs = performance.now();
    const requiredPlayers = this.readyNavigation.selectedMode() === 'duo' ? 2 : 1;
    const framing = this.framingCoach.update(poses, requiredPlayers, nowMs);
    const framingPresentation = presentPoseFraming(framing.status);
    readyGate.dataset.framing = framingPresentation.cameraState;
    cameraCard.dataset.framing = framingPresentation.cameraState;

    const previousMode = this.readyNavigation.selectedMode();
    const state = this.readyNavigation.update(framing.navigationPoses, nowMs);
    this.setReadyFallbackEnabled(state.trackedPlayers >= state.requiredPlayers);
    if (!hasCustomGameConfig() && state.mode !== previousMode) this.renderReadyMode(state.mode);
    readyHoldRing.style.setProperty('--hold-angle', `${state.confirmProgress * 360}deg`);
    readyHoldRing.setAttribute('aria-valuenow', String(Math.round(state.confirmProgress * 100)));
    const navigationPresentation = presentReadyNavigation(state, framingPresentation);
    readyGate.dataset.action = navigationPresentation.visualState;
    cameraStatus.textContent = navigationPresentation.status;

    if (state.confirmed) this.completeGestureLaunch(state.mode, 'gesture');
  };

  private async runReadyStanceCheck(signal: AbortSignal): Promise<void> {
    const {
      visionLoader, video, overlay, gateError, tasteStore,
      availablePlayableModes, setShellOwner, setGuideOnly, setCameraEvaluation,
    } = this.deps;
    setGuideOnly(false);
    const startedFromRandom = this.deps.getSelectedModeKey() === 'random';
    const initialMode = this.deps.resolvedLaunchMode();
    this.readyNavigation = new MotionNavigationController(initialMode, {}, availablePlayableModes);
    this.framingCoach.reset();
    if (hasCustomGameConfig()) {
      this.deps.readyModeTitle.textContent = 'MY GAME';
      this.deps.readyGestureHint.textContent = 'RAISE A HAND TO PLAY';
    } else {
      this.renderReadyMode(initialMode, startedFromRandom);
    }
    this.deps.readyHoldRing.style.setProperty('--hold-angle', '0deg');
    this.deps.readyHoldRing.setAttribute('aria-valuenow', '0');
    gateError.textContent = '';
    delete gateError.dataset.reason;
    setShellOwner('ready');
    this.focusReadyOwner();
    setCameraEvaluation(false);
    tasteStore.update(recordCameraAttempt);
    this.activeReadyStartedAt = performance.now();
    this.showPreparingVision();

    try {
      await Promise.all([
        this.deps.unlockAudio(),
        (async () => {
          if (this.deps.isCameraRunning()) return;
          const vision = await visionLoader.load();
          await vision.startVision(
            video,
            overlay,
            this.updateReadyNavigation,
            (status) => this.renderVisionStatus(status),
            (geometry) => this.applyCameraFrameGeometry(geometry),
            signal,
          );
          if (signal.aborted) return;
          this.deps.setCameraRunning(true);
        })(),
      ]);
      if (signal.aborted) return;
      this.deps.cameraButton.textContent = 'Stop camera';
    } catch (error) {
      if (isCameraStartCancelled(error)) return;
      await this.stopCamera();
      this.recordActiveSetupExit();
      const failure = presentCameraStartFailure(error);
      this.deps.readyGate.dataset.phase = 'error';
      this.deps.readyGate.dataset.action = 'framing';
      this.setReadyFallbackEnabled(true);
      gateError.textContent = failure.message;
      gateError.dataset.reason = failure.reason;
      setShellOwner('launcher');
      requestAnimationFrame(() => this.deps.cameraButton.focus());
    }
  }
}
