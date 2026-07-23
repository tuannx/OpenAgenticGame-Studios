import type { PlayableGameMode } from './motion-navigation';
import type { LocalTasteStore, TasteRecommendation } from './taste-profile';
import { recordGuideOnlyStart, recordPlayStart, recordRunOutcome } from './taste-profile';
import type { GameModeValue } from './bridge';
import { clearGameConfig, setRuntimeGameModeHandler, setRunOutcomeHandler, setSelectedGameMode } from './bridge';
import { writeDeepLink, type DeepLinkMode } from './deep-link';
import {
  isProductFamily,
  modesForProductFamily,
  productFamilyForMode,
  type ProductFamily,
} from './product-family';
import { partyDirector } from './party-voice';

export type LaunchMode = ProductFamily;

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

export const PRODUCT_FAMILY_PRESENTATION: Record<ProductFamily, {
  imageSrc: string;
  title: string;
  defaultMode: PlayableGameMode;
}> = {
  brainbreak: {
    imageSrc: '/assets/game_mode_mirror.webp',
    title: 'BrainBreak Game',
    defaultMode: 'mirror',
  },
  ar: {
    imageSrc: '/assets/game_mode_supernova.webp',
    title: 'Supernova',
    defaultMode: 'supernova',
  },
};

export function playableModeFromValue(mode: GameModeValue): PlayableGameMode | null {
  if (mode === 1) return 'strike';
  if (mode === 2) return 'duo';
  if (mode === 3) return 'supernova';
  if (mode === 4) return null;
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
  private currentSelectedModeKey: LaunchMode = 'brainbreak';
  private preferredPlayableMode: PlayableGameMode = 'mirror';
  private randomizeOnLaunch = false;
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

  get isRandomLaunch(): boolean {
    return this.randomizeOnLaunch;
  }

  readyModes(): readonly PlayableGameMode[] {
    return modesForProductFamily(this.currentSelectedModeKey, this.deps.availablePlayableModes);
  }

  resolvedLaunchMode(): PlayableGameMode {
    const familyModes = this.readyModes();
    if (familyModes.length === 0) {
      return this.currentSelectedModeKey === 'ar' ? 'supernova' : 'mirror';
    }
    if (this.randomizeOnLaunch) {
      return familyModes[Math.floor(Math.random() * familyModes.length)] ?? familyModes[0];
    }
    if (familyModes.includes(this.preferredPlayableMode)) {
      return this.preferredPlayableMode;
    }
    return familyModes[0];
  }

  selectModeByKey(modeKey: string): void {
    if (isProductFamily(modeKey)) {
      const card = [...this.deps.modeCards].find((c) => c.dataset.mode === modeKey);
      if (card) this.selectModeCard(card);
      return;
    }
    if (modeKey === 'random') {
      this.randomizeOnLaunch = true;
      this.preferredPlayableMode = 'mirror';
      const card = [...this.deps.modeCards].find((c) => c.dataset.mode === 'brainbreak');
      if (card) this.selectModeCard(card, { preservePreferred: true, deepLink: 'random' });
      return;
    }
    if (modeKey === 'mirror' || modeKey === 'strike' || modeKey === 'duo' || modeKey === 'supernova') {
      this.randomizeOnLaunch = false;
      this.preferredPlayableMode = modeKey;
      const family = productFamilyForMode(modeKey);
      const card = [...this.deps.modeCards].find((c) => c.dataset.mode === family);
      if (card) this.selectModeCard(card, { preservePreferred: true, deepLink: modeKey });
    }
  }

  renderPrimaryCameraCta(): void {
    const { primaryCameraButton, tasteRecommendation } = this.deps;
    const family = this.currentSelectedModeKey;
    const rememberedInFamily = tasteRecommendation.mode
      && productFamilyForMode(tasteRecommendation.mode) === family
      && this.isPlayableModeAvailable(tasteRecommendation.mode)
      ? tasteRecommendation.mode
      : null;
    primaryCameraButton.textContent = rememberedInFamily
      ? 'CONTINUE'
      : 'CAMERA';

    const tasteNote = document.querySelector<HTMLElement>('#taste-note');
    if (!tasteNote) return;
    if (rememberedInFamily) {
      tasteNote.textContent = 'Remembered · local only';
    } else {
      tasteNote.textContent = 'Local only';
    }
  }

  private isPlayableModeAvailable(mode: PlayableGameMode): boolean {
    return this.deps.availablePlayableModes.includes(mode);
  }

  private isCardAvailable(card: HTMLElement): boolean {
    const mode = card.dataset.mode;
    if (!isProductFamily(mode)) return false;
    return this.readyModesFor(mode).length > 0;
  }

  private readyModesFor(family: ProductFamily): readonly PlayableGameMode[] {
    return modesForProductFamily(family, this.deps.availablePlayableModes);
  }

  private selectModeCard(
    card: HTMLElement,
    options: { preservePreferred?: boolean; deepLink?: DeepLinkMode; announce?: boolean } = {},
  ): void {
    const mode = card.dataset.mode;
    if (!isProductFamily(mode) || !this.isCardAvailable(card)) return;
    clearGameConfig();
    this.deps.modeCards.forEach((candidate) => {
      const selected = candidate === card;
      candidate.classList.toggle('active', selected);
      candidate.setAttribute('aria-checked', String(selected));
      candidate.tabIndex = selected && this.isCardAvailable(candidate) ? 0 : -1;
    });
    document.querySelectorAll<HTMLElement>('.mode-card[data-mode="custom"]').forEach((custom) => {
      custom.classList.remove('active');
      custom.setAttribute('aria-checked', 'false');
    });
    this.currentSelectedModeKey = mode;
    if (!options.preservePreferred) {
      this.randomizeOnLaunch = false;
      const familyModes = this.readyModesFor(mode);
      const recommended = this.deps.tasteRecommendation.mode;
      this.preferredPlayableMode = recommended && familyModes.includes(recommended)
        ? recommended
        : PRODUCT_FAMILY_PRESENTATION[mode].defaultMode;
      if (!familyModes.includes(this.preferredPlayableMode)) {
        this.preferredPlayableMode = familyModes[0] ?? PRODUCT_FAMILY_PRESENTATION[mode].defaultMode;
      }
    }
    writeDeepLink({ mode: options.deepLink ?? mode });
    this.renderPrimaryCameraCta();
    if (options.announce) partyDirector.onRitualSelected(mode);
  }

  private moveModeCardSelection(currentCard: HTMLElement, delta: -1 | 1): void {
    const cards = [...this.deps.modeCards].filter((c) => this.isCardAvailable(c));
    const currentIndex = cards.indexOf(currentCard);
    const nextCard = cards[(currentIndex + delta + cards.length) % cards.length];
    if (!nextCard) return;
    this.selectModeCard(nextCard, { announce: true });
    nextCard.focus();
  }

  private initModeCards(): void {
    const { modeCards, motionGate, primaryCameraButton, guideOnlyButton } = this.deps;
    modeCards.forEach((card) => {
      const available = this.isCardAvailable(card);
      card.dataset.available = String(available);
      card.setAttribute('aria-disabled', String(!available));
      if (!available) card.tabIndex = -1;
      const selectCard = () => this.selectModeCard(card, { announce: true });
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
      primaryCameraButton.textContent = '…';
      await this.deps.onOpenReady();
      primaryCameraButton.disabled = false;
      this.renderPrimaryCameraCta();
    });
  }

  private initBridgeHandlers(): void {
    const { tasteStore, modeCards } = this.deps;
    setRuntimeGameModeHandler((mode) => {
      const modeKey = playableModeFromValue(mode);
      if (!modeKey) return;
      this.preferredPlayableMode = modeKey;
      this.randomizeOnLaunch = false;
      const family = productFamilyForMode(modeKey);
      const selectedCard = [...modeCards].find((card) => card.dataset.mode === family);
      if (selectedCard) this.selectModeCard(selectedCard, { preservePreferred: true, deepLink: modeKey });
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
    if (tasteRecommendation.mode && this.isPlayableModeAvailable(tasteRecommendation.mode)) {
      this.preferredPlayableMode = tasteRecommendation.mode;
      this.randomizeOnLaunch = false;
      const family = productFamilyForMode(tasteRecommendation.mode);
      const recommendedCard = [...modeCards].find((card) => card.dataset.mode === family);
      if (recommendedCard) {
        this.selectModeCard(recommendedCard, {
          preservePreferred: true,
          deepLink: tasteRecommendation.mode,
        });
      }
    } else {
      const defaultCard = [...modeCards].find((card) => card.dataset.mode === 'brainbreak');
      if (defaultCard) this.selectModeCard(defaultCard);
    }

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
      const family = productFamilyForMode(mode);
      const selectedCard = [...modeCards].find((card) => card.dataset.mode === family);
      if (selectedCard) {
        this.preferredPlayableMode = mode;
        this.selectModeCard(selectedCard, { preservePreferred: true, deepLink: mode });
      }
      const familyPresentation = PRODUCT_FAMILY_PRESENTATION[family];
      guideDemoImage.src = familyPresentation.imageSrc;
      guideDemoImage.alt = `${familyPresentation.title} demo`;
      guideDemoTitle.textContent = familyPresentation.title;
      setSelectedGameMode(GAME_MODE_PRESENTATION[mode].modeVal);
      tasteStore.update((profile) => recordGuideOnlyStart(profile, mode));
      this.deps.onGuideOnly(mode);
    });

    guideDemoCameraButton.addEventListener('click', async () => {
      guideDemoCameraButton.disabled = true;
      guideDemoCameraButton.textContent = '…';
      await this.deps.onOpenReady();
      guideDemoCameraButton.disabled = false;
      guideDemoCameraButton.textContent = 'CAMERA';
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
