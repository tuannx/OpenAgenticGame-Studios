import type { PoseSnapshot } from './vision';

export type PoseFramingStatus =
  | { kind: 'searching' }
  | { kind: 'move-closer' }
  | { kind: 'move-back' }
  | { kind: 'center' }
  | { kind: 'show-body' }
  | { kind: 'steady' }
  | { kind: 'add-player'; current: 1; required: 2 }
  | { kind: 'ready'; players: number };

export interface PoseFramingConfig {
  keypointConfidence: number;
  maximumShoulderWidth: number;
  minimumPoseQuality: number;
  minimumShoulderWidth: number;
  stableMs: number;
  horizontalMargin: number;
  topMargin: number;
  bottomMargin: number;
}

export interface PoseFramingState {
  detectedPlayers: number;
  framedPlayers: number;
  navigationPoses: readonly PoseSnapshot[];
  requiredPlayers: 1 | 2;
  status: PoseFramingStatus;
}

type FramingIssue = 'move-closer' | 'move-back' | 'center' | 'show-body';

const DEFAULT_CONFIG: PoseFramingConfig = {
  keypointConfidence: 0.3,
  maximumShoulderWidth: 0.42,
  minimumPoseQuality: 0.25,
  minimumShoulderWidth: 0.075,
  stableMs: 400,
  horizontalMargin: 0.12,
  topMargin: 0.035,
  bottomMargin: 0.92,
};

const NOSE = 0;
const LEFT_SHOULDER = 5;
const RIGHT_SHOULDER = 6;
const LEFT_WRIST = 9;
const RIGHT_WRIST = 10;
const LEFT_HIP = 11;
const RIGHT_HIP = 12;

function confidentPoint(
  pose: PoseSnapshot,
  index: number,
  confidence: number,
): { x: number; y: number } | undefined {
  const point = pose.keypoints[index];
  return point && point.score >= confidence ? point : undefined;
}

function classifyPose(pose: PoseSnapshot, config: PoseFramingConfig): FramingIssue | undefined {
  if (pose.quality < config.minimumPoseQuality) return 'show-body';

  const leftShoulder = confidentPoint(pose, LEFT_SHOULDER, config.keypointConfidence);
  const rightShoulder = confidentPoint(pose, RIGHT_SHOULDER, config.keypointConfidence);
  const leftHip = confidentPoint(pose, LEFT_HIP, config.keypointConfidence);
  const rightHip = confidentPoint(pose, RIGHT_HIP, config.keypointConfidence);
  if (!leftShoulder || !rightShoulder || !leftHip || !rightHip) return 'show-body';

  const shoulderWidth = Math.abs(leftShoulder.x - rightShoulder.x);
  if (shoulderWidth > config.maximumShoulderWidth) return 'move-back';
  if (shoulderWidth < config.minimumShoulderWidth) return 'move-closer';

  const nose = confidentPoint(pose, NOSE, config.keypointConfidence);
  const leftWrist = confidentPoint(pose, LEFT_WRIST, config.keypointConfidence);
  const rightWrist = confidentPoint(pose, RIGHT_WRIST, config.keypointConfidence);
  if (!nose || (!leftWrist && !rightWrist)) return 'show-body';

  const lowestHip = Math.max(leftHip.y, rightHip.y);
  if (nose.y < config.topMargin || lowestHip > config.bottomMargin) return 'move-back';

  const torsoCenter = (leftShoulder.x + rightShoulder.x + leftHip.x + rightHip.x) / 4;
  if (torsoCenter < config.horizontalMargin || torsoCenter > 1 - config.horizontalMargin) {
    return 'center';
  }
  return undefined;
}

export class PoseFramingCoach {
  private readonly config: PoseFramingConfig;
  private stableSignature = '';
  private stableSince: number | undefined;

  constructor(config: Partial<PoseFramingConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  reset(): void {
    this.stableSignature = '';
    this.stableSince = undefined;
  }

  update(poses: readonly PoseSnapshot[], requiredPlayers: 1 | 2, nowMs: number): PoseFramingState {
    const detected = poses.filter((pose) => pose.quality >= this.config.minimumPoseQuality * 0.6);
    if (detected.length === 0) return this.blockedState(requiredPlayers, 0, { kind: 'searching' });

    // Single-pass: classify each pose once, partition into framed vs issues
    const framed: PoseSnapshot[] = [];
    let firstPriorityIssue: FramingIssue | undefined;
    const priority: readonly FramingIssue[] = ['move-back', 'move-closer', 'center', 'show-body'];
    const issueSet = new Set<FramingIssue>();
    for (const pose of detected) {
      const issue = classifyPose(pose, this.config);
      if (issue === undefined) {
        framed.push(pose);
      } else {
        issueSet.add(issue);
      }
    }
    if (framed.length === 0) {
      firstPriorityIssue = priority.find((i) => issueSet.has(i));
      return this.blockedState(requiredPlayers, detected.length, { kind: firstPriorityIssue ?? 'show-body' });
    }
    firstPriorityIssue = priority.find((i) => issueSet.has(i));

    const signature = framed.map((pose) => pose.id).join(':');
    if (signature !== this.stableSignature) {
      this.stableSignature = signature;
      this.stableSince = nowMs;
    }
    const isStable = this.stableSince !== undefined && nowMs - this.stableSince >= this.config.stableMs;
    if (!isStable) {
      return {
        detectedPlayers: detected.length,
        framedPlayers: framed.length,
        navigationPoses: [],
        requiredPlayers,
        status: { kind: 'steady' },
      };
    }

    const status: PoseFramingStatus = framed.length >= requiredPlayers
      ? { kind: 'ready', players: framed.length }
      : detected.length >= requiredPlayers && firstPriorityIssue
        ? { kind: firstPriorityIssue }
        : { kind: 'add-player', current: 1, required: 2 };
    return {
      detectedPlayers: detected.length,
      framedPlayers: framed.length,
      navigationPoses: framed,
      requiredPlayers,
      status,
    };
  }

  private blockedState(
    requiredPlayers: 1 | 2,
    detectedPlayers: number,
    status: PoseFramingStatus,
  ): PoseFramingState {
    this.reset();
    return {
      detectedPlayers,
      framedPlayers: 0,
      navigationPoses: [],
      requiredPlayers,
      status,
    };
  }
}
