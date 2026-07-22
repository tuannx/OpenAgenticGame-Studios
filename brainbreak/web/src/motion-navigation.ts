import { ALL_PLAYABLE_MODES } from './motion-capability';
import type { PlayableGameMode } from './motion-capability';
import type { PoseSnapshot } from './vision';

export type { PlayableGameMode } from './motion-capability';

export interface MotionNavigationConfig {
  confirmHoldMs: number;
  keypointConfidence: number;
  leanThreshold: number;
  neutralThreshold: number;
  poseQuality: number;
  raisedHandMargin: number;
}

export interface MotionNavigationState {
  confirmed: boolean;
  confirmProgress: number;
  direction: 'left' | 'neutral' | 'right';
  mode: PlayableGameMode;
  requiredPlayers: 1 | 2;
  trackedPlayers: number;
}

const DEFAULT_CONFIG: MotionNavigationConfig = {
  confirmHoldMs: 1_000,
  keypointConfidence: 0.3,
  leanThreshold: 0.065,
  neutralThreshold: 0.028,
  poseQuality: 0.25,
  raisedHandMargin: 0.035,
};

function average(a: number, b: number): number {
  return (a + b) / 2;
}

function confidentPoint(
  pose: PoseSnapshot,
  index: number,
  minimumConfidence: number,
): { x: number; y: number } | undefined {
  const point = pose.keypoints[index];
  return point && point.score >= minimumConfidence ? point : undefined;
}

function bodyLean(pose: PoseSnapshot, minimumConfidence: number): number | undefined {
  const leftShoulder = confidentPoint(pose, 5, minimumConfidence);
  const rightShoulder = confidentPoint(pose, 6, minimumConfidence);
  const leftHip = confidentPoint(pose, 11, minimumConfidence);
  const rightHip = confidentPoint(pose, 12, minimumConfidence);
  if (!leftShoulder || !rightShoulder || !leftHip || !rightHip) return undefined;
  return average(leftShoulder.x, rightShoulder.x) - average(leftHip.x, rightHip.x);
}

function hasRaisedHand(pose: PoseSnapshot, config: MotionNavigationConfig): boolean {
  const leftShoulder = confidentPoint(pose, 5, config.keypointConfidence);
  const rightShoulder = confidentPoint(pose, 6, config.keypointConfidence);
  const leftWrist = confidentPoint(pose, 9, config.keypointConfidence);
  const rightWrist = confidentPoint(pose, 10, config.keypointConfidence);
  const leftRaised = leftShoulder && leftWrist
    ? leftWrist.y < leftShoulder.y - config.raisedHandMargin
    : false;
  const rightRaised = rightShoulder && rightWrist
    ? rightWrist.y < rightShoulder.y - config.raisedHandMargin
    : false;
  return leftRaised || rightRaised;
}

export class MotionNavigationController {
  private readonly config: MotionNavigationConfig;
  private confirmStartedAt: number | undefined;
  private confirmationLatched = false;
  private leanArmed = true;
  private modeIndex = 0;
  private readonly availableModes: readonly PlayableGameMode[];

  constructor(
    initialMode: PlayableGameMode = 'mirror',
    config: Partial<MotionNavigationConfig> = {},
    availableModes: readonly PlayableGameMode[] = ALL_PLAYABLE_MODES,
  ) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    const supportedModes = [...new Set(availableModes.filter((mode) => ALL_PLAYABLE_MODES.includes(mode)))];
    this.availableModes = supportedModes.length > 0 ? supportedModes : ['mirror'];
    this.setMode(initialMode);
  }

  setMode(mode: PlayableGameMode): void {
    const requestedIndex = this.availableModes.indexOf(mode);
    this.modeIndex = requestedIndex >= 0 ? requestedIndex : 0;
    this.resetGestureState();
  }

  selectedMode(): PlayableGameMode {
    return this.availableModes[this.modeIndex] ?? 'mirror';
  }

  update(poses: readonly PoseSnapshot[], nowMs: number): MotionNavigationState {
    const tracked = poses.filter((pose) => pose.quality >= this.config.poseQuality);
    const primary = tracked[0];
    let direction: MotionNavigationState['direction'] = 'neutral';
    let selectionChanged = false;

    if (primary) {
      const lean = bodyLean(primary, this.config.keypointConfidence);
      if (lean !== undefined && Math.abs(lean) <= this.config.neutralThreshold) {
        this.leanArmed = true;
      } else if (lean !== undefined && this.leanArmed && Math.abs(lean) >= this.config.leanThreshold) {
        const delta = lean < 0 ? -1 : 1;
        this.modeIndex = (this.modeIndex + delta + this.availableModes.length) % this.availableModes.length;
        direction = delta < 0 ? 'left' : 'right';
        this.leanArmed = false;
        selectionChanged = true;
        this.resetConfirmation();
      }
    }

    const requiredPlayers = this.selectedMode() === 'duo' ? 2 : 1;
    const enoughPlayers = tracked.length >= requiredPlayers;
    const allReady = enoughPlayers
      && tracked.slice(0, requiredPlayers).every((pose) => hasRaisedHand(pose, this.config));

    if (selectionChanged || !allReady) {
      this.resetConfirmation();
    } else if (this.confirmStartedAt === undefined) {
      this.confirmStartedAt = nowMs;
    }

    const confirmProgress = this.confirmStartedAt === undefined
      ? 0
      : Math.min(1, Math.max(0, (nowMs - this.confirmStartedAt) / this.config.confirmHoldMs));
    const confirmed = confirmProgress >= 1 && !this.confirmationLatched;
    if (confirmed) this.confirmationLatched = true;

    return {
      confirmed,
      confirmProgress,
      direction,
      mode: this.selectedMode(),
      requiredPlayers,
      trackedPlayers: tracked.length,
    };
  }

  private resetGestureState(): void {
    this.leanArmed = true;
    this.resetConfirmation();
  }

  private resetConfirmation(): void {
    this.confirmStartedAt = undefined;
    this.confirmationLatched = false;
  }
}
