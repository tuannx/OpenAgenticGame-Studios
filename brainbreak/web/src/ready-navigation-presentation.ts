import type { MotionNavigationState } from './motion-navigation';
import type { PoseFramingPresentation } from './pose-framing-status';

export type ReadyNavigationVisualState = 'framing' | 'neutral' | 'left' | 'right' | 'holding';

export interface ReadyNavigationPresentation {
  visualState: ReadyNavigationVisualState;
  status: string;
}

const MODE_CHANGED_STATUS = 'MODE CHANGED · STAND TALL';
const HOLDING_STATUS = 'HOLD · KEEP RISING';
const HOLDING_DUO_STATUS = 'BOTH HOLD · KEEP RISING';
const READY_STATUS = 'READY · RAISE';
const PARTY_INVITE_STATUS = 'PARTY · BOTH RAISE';
const NEED_PARTNER_STATUS = 'NEED P2 IN FRAME';

export function presentReadyNavigation(
  state: MotionNavigationState,
  framing: PoseFramingPresentation,
): ReadyNavigationPresentation {
  if (framing.cameraState !== 'ready') {
    return { visualState: 'framing', status: framing.instruction };
  }
  if (state.direction === 'left' || state.direction === 'right') {
    return { visualState: state.direction, status: MODE_CHANGED_STATUS };
  }
  if (state.confirmProgress > 0) {
    return {
      visualState: 'holding',
      status: state.requiredPlayers >= 2 ? HOLDING_DUO_STATUS : HOLDING_STATUS,
    };
  }
  if (state.mode === 'duo' && state.trackedPlayers >= 2) {
    return { visualState: 'neutral', status: PARTY_INVITE_STATUS };
  }
  if (state.mode === 'duo' && state.trackedPlayers < 2) {
    return { visualState: 'neutral', status: NEED_PARTNER_STATUS };
  }
  return { visualState: 'neutral', status: READY_STATUS };
}
