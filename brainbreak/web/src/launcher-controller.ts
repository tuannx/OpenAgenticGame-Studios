import type { PlayableGameMode } from './motion-navigation';
import type { LocalTasteStore, TasteRecommendation } from './taste-profile';
import { recordGuideOnlyStart, recordPlayStart, recordRunOutcome } from './taste-profile';
import type { GameModeValue } from './bridge';
import { clearGameConfig, setRuntimeGameModeHandler, setRunOutcomeHandler, setSelectedGameMode } from './bridge';
import { writeDeepLink, type DeepLinkMode } from './deep-link';

export type LaunchMode = 'random' | PlayableGameMode;

export interface GameModePresentation {
  imageSrc: string;
  modeVal: 0 | 1 | 2 | 3;
  title: string;
}

export const GAME_MODE_PRESENTATION: Record<PlayableGameMode, GameModePresentation> = {
  mirror: { imageSrc: '/assets/game_mode_mirror.webp', modeVal: 0, title: 'Mirror Beat' },
  strike: { imageSrc: '/assets/game_mode_strike.webp', modeVal: 1, title: 'Beat Strike' },
  duo: { imageSrc: '/assets/game_mode_duo.webp', modeVal: 2, title: 'Duo Groove' },
  supernova: { imageSrc: '/assets/game_mode_supernova.webp', modeVal: 3, title: 'Supernova Drop' },
};

export function playableModeFromValue(mode: GameModeValue): PlayableGameMode | null {
  if (mode === 1) return 'strike';
  if (mode === 2) return 'duo';
  if (mode === 3) return 'supernova';
  if (mode === 4) return null; // custom config mode — not a taste-tracked playable mode
  return 'mirror';
}

export function modeLabel(mode: PlayableGameMode): string {
  return GAME_MODE_PRESENTATION[mode].title.toUpperCase();
}

export interface LauncherDeps {
  modeCards: NodeListOf<HTMLElement>;
  primaryCameraButton: HTMLButtonElement;
  guideOnlyButton: HTMLButtonElement;
  motionGate: HTMLElement;
  guideDemoGate: HTMLElement;
  guideDemoImage: HTMLImageElement;
  guideDemoTitle: HTMLElement;
  guideDemoCameraButton: HTMLButtonElement;
  guideDemoBackButton: HTMLButtonElement;
  tasteStore: LocalTasteStore;
  tasteRecommendation: TasteRecommendation;
  availablePlayableModes: readonly PlayableGameMode[];
  onOpenReady: () => Promise<void>;
  onGuideOnly: (mode: PlayableGameMode) => void;
  onGuideOnlyReturn: () => void;
}

export class LauncherController {
  private currentSelectedModeKey: LaunchMode = 'random';
  private deps: LauncherDeps;

  constructor(deps: LauncherDeps) {
    this.deps = deps;
    this.initModeCards();
    this.initBridgeHandlers();
    this.applyTasteRecommendation();
    this.initGuideDemo();
  }

  get selectedModeKey(): LaunchMode {
    return this.currentSelectedModeKey;
  }

  resolvedLaunchMode(): PlayableGameMode {
    const { availablePlayableModes } = this.deps;
    if (this.currentSelectedModeKey === 'random') {
      return availablePlayableModes[Math.floor(Math.random() * availablePlayableModes.length)] ?? 'mirror';
    }
    return this.isPlayableModeAvailable(this.currentSelectedModeKey)
      ? this.currentSelectedModeKey
      : availablePlayableModes[0] ?? 'mirror';
  }

  selectModeByKey(modeKey: string): void {
    const card = [...this.deps.modeCards].find((c) => c.dataset.mode === modeKey);
    if (card) this.selectModeCard(card);
  }

  renderPrimaryCameraCta(): void {
    const { primaryCameraButton, tasteRecommendation } = this.deps;
    const activeRememberedMode = tasteRecommendation.mode === this.currentSelectedModeKey
      ? tasteRecommendation.mode
      : null;
    primaryCameraButton.textContent = activeRememberedMode
      ? `CONTINUE ${modeLabel(activeRememberedMode)}`
      : 'TURN ON CAMERA';

    const tasteNote = document.querySelector<HTMLElement>('#taste-note');
    if (!tasteNote) return;
    if (activeRememberedMode) {
      tasteNote.textContent = `${GAME_MODE_PRESENTATION[activeRememberedMode].title} remembered on this device • no photos or poses saved`;
    } else if (this.currentSelectedModeKey !== 'random') {
      tasteNote.textContent = `${GAME_MODE_PRESENTATION[this.currentSelectedModeKey].title} selected • preferences stay on this device`;
    } else {
      tasteNote.textContent = 'Preferences stay on this device • no photos or poses saved';
    }
  }

  private isPlayableModeAvailable(mode: PlayableGameMode): boolean {
    return this.deps.availablePlayableModes.includes(mode);
  }

  private isLaunchMode(value: string | undefined): value is LaunchMode {
    return value === 'random' || value === 'mirror' || value === 'strike' || value === 'duo' || value === 'supernova';
  }

  private isCardAvailable(card: HTMLElement): boolean {
    const mode = card.dataset.mode;
    return mode === 'random' || (this.isLaunchMode(mode) && mode !== 'random' && this.isPlayableModeAvailable(mode));
  }

  private selectModeCard(card: HTMLElement): void {
    const mode = card.dataset.mode;
    if (!this.isLaunchMode(mode) || !this.isCardAvailable(card)) return;
    clearGameConfig(); // selecting a built-in mode deselects any custom game
    this.deps.modeCards.forEach((candidate) => {
      const selected = candidate === card;
      candidate.classList.toggle('active', selected);
      candidate.setAttribute('aria-checked', String(selected));
      candidate.tabIndex = selected && this.isCardAvailable(candidate) ? 0 : -1;
    });
    // Selecting a built-in mode clears any custom-game card highlight.
    document.querySelectorAll<HTMLElement>('.mode-card[data-mode="custom"]').forEach((custom) => {
      custom.classList.remove('active');
      custom.setAttribute('aria-checked', 'false');
    });
    this.currentSelectedModeKey = mode;
    writeDeepLink({ mode: mode as DeepLinkMode });
    this.renderPrimaryCameraCta();
  }

  private moveModeCardSelection(currentCard: HTMLElement, delta: -1 | 1): void {
    const cards = [...this.deps.modeCards].filter((c) => this.isCardAvailable(c));
    const currentIndex = cards.indexOf(currentCard);
    const nextCard = cards[(currentIndex + delta + cards.length) % cards.length];
    if (!nextCard) return;
    this.selectModeCard(nextCard);
    nextCard.focus();
  }

  private initModeCards(): void {
    const { modeCards, motionGate, primaryCameraButton, guideOnlyButton } = this.deps;
    modeCards.forEach((card) => {
      const available = this.isCardAvailable(card);
      card.dataset.available = String(available);
      card.setAttribute('aria-disabled', String(!available));
      if (!available) card.tabIndex = -1;
      const selectCard = () => this.selectModeCard(card);
      card.addEventListener('click', selectCard);
      card.addEventListener('keydown', (e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          selectCard();
        } else if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') {
          e.preventDefault();
          this.moveModeCardSelection(card, -1);
        } else if (e.key === 'ArrowRight' || e.key === 'ArrowDown') {
          e.preventDefault();
          this.moveModeCardSelection(card, 1);
        }
      });
    });

    motionGate.addEventListener('keydown', (event) => {
      if (event.key !== 'Tab') return;
      const selectedModeCard = [...modeCards].find((card) => card.tabIndex === 0 && this.isCardAvailable(card));
      const first = selectedModeCard ?? primaryCameraButton;
      const last = guideOnlyButton;
      if (!event.shiftKey && document.activeElement === last) {
        event.preventDefault();
        first.focus();
      } else if (event.shiftKey && document.activeElement === first) {
        event.preventDefault();
        last.focus();
      }
    });

    primaryCameraButton.addEventListener('click', async () => {
      primaryCameraButton.disabled = true;
      primaryCameraButton.textContent = 'Starting camera…';
      await this.deps.onOpenReady();
      primaryCameraButton.disabled = false;
      this.renderPrimaryCameraCta();
    });
  }

  private initBridgeHandlers(): void {
    const { tasteStore, modeCards } = this.deps;
    setRuntimeGameModeHandler((mode) => {
      const modeKey = playableModeFromValue(mode);
      if (!modeKey) return; // custom mode has no launcher card / taste entry
      const selectedCard = [...modeCards].find((card) => card.dataset.mode === modeKey);
      if (selectedCard) this.selectModeCard(selectedCard);
      tasteStore.update((profile) => recordPlayStart(profile, modeKey, 'gesture'));
    });

    setRunOutcomeHandler((mode, outcome) => {
      const modeKey = playableModeFromValue(mode);
      if (!modeKey) return;
      tasteStore.update((profile) => recordRunOutcome(profile, modeKey, outcome));
    });
  }

  private applyTasteRecommendation(): void {
    const { tasteRecommendation, modeCards, motionGate } = this.deps;
    const recommendedCard = tasteRecommendation.mode
      && this.isPlayableModeAvailable(tasteRecommendation.mode)
      ? [...modeCards].find((card) => card.dataset.mode === tasteRecommendation.mode)
      : undefined;
    if (recommendedCard) this.selectModeCard(recommendedCard);

    document.body.classList.toggle('compact-setup', tasteRecommendation.compactSetup);
    motionGate.dataset.taste = tasteRecommendation.compactSetup ? 'compact' : 'full';
    this.renderPrimaryCameraCta();
  }

  private initGuideDemo(): void {
    const {
      guideOnlyButton, guideDemoGate, guideDemoImage, guideDemoTitle,
      guideDemoCameraButton, guideDemoBackButton, tasteStore, modeCards,
    } = this.deps;

    guideOnlyButton.addEventListener('click', () => {
      const mode = this.resolvedLaunchMode();
      const selectedCard = [...modeCards].find((card) => card.dataset.mode === mode);
      if (selectedCard) this.selectModeCard(selectedCard);
      const presentation = GAME_MODE_PRESENTATION[mode];
      guideDemoImage.src = presentation.imageSrc;
      guideDemoImage.alt = `${presentation.title} demo`;
      guideDemoTitle.textContent = presentation.title;
      setSelectedGameMode(GAME_MODE_PRESENTATION[mode].modeVal);
      tasteStore.update((profile) => recordGuideOnlyStart(profile, mode));
      this.deps.onGuideOnly(mode);
    });

    guideDemoCameraButton.addEventListener('click', async () => {
      guideDemoCameraButton.disabled = true;
      guideDemoCameraButton.textContent = 'Starting camera…';
      await this.deps.onOpenReady();
      guideDemoCameraButton.disabled = false;
      guideDemoCameraButton.textContent = 'TURN ON CAMERA';
    });

    const returnFromGuideDemo = (): void => {
      this.deps.onGuideOnlyReturn();
      const selectedCard = [...modeCards].find((card) => card.dataset.mode === this.currentSelectedModeKey);
      requestAnimationFrame(() => selectedCard?.focus());
    };

    guideDemoBackButton.addEventListener('click', returnFromGuideDemo);

    guideDemoGate.addEventListener('keydown', (event) => {
      if (event.key === 'Escape') {
        event.preventDefault();
        returnFromGuideDemo();
      } else if (event.key === 'Tab' && !event.shiftKey && document.activeElement === guideDemoBackButton) {
        event.preventDefault();
        guideDemoCameraButton.focus();
      } else if (event.key === 'Tab' && event.shiftKey && document.activeElement === guideDemoCameraButton) {
        event.preventDefault();
        guideDemoBackButton.focus();
      }
    });
  }
}
