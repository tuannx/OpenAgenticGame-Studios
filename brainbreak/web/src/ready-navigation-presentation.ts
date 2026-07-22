import type { MotionNavigationState } from './motion-navigation';
import type { PoseFramingPresentation } from './pose-framing-status';

export type ReadyNavigationVisualState = 'framing' | 'neutral' | 'left' | 'right' | 'holding';

export interface ReadyNavigationPresentation {
  visualState: ReadyNavigationVisualState;
  status: string;
}

const MODE_CHANGED_STATUS = 'GAME CHANGED • STAND TALL';
const HOLDING_STATUS = 'KEEP YOUR HAND UP';
const READY_STATUS = 'BODY READY';

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
    return { visualState: 'holding', status: HOLDING_STATUS };
  }
  return { visualState: 'neutral', status: READY_STATUS };
}
