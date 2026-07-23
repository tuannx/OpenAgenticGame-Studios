import type { PoseFramingStatus } from './pose-framing';

export interface PoseFramingPresentation {
  cameraState: PoseFramingStatus['kind'];
  instruction: string;
}

export function presentPoseFraming(status: PoseFramingStatus): PoseFramingPresentation {
  switch (status.kind) {
    case 'searching':
      return { cameraState: status.kind, instruction: 'STEP INTO FRAME' };
    case 'move-closer':
      return { cameraState: status.kind, instruction: 'STEP CLOSER' };
    case 'move-back':
      return { cameraState: status.kind, instruction: 'STEP BACK · FULL BODY' };
    case 'center':
      return { cameraState: status.kind, instruction: 'CENTER YOUR BODY' };
    case 'show-body':
      return { cameraState: status.kind, instruction: 'SHOW HEAD TO HIPS' };
    case 'steady':
      return { cameraState: status.kind, instruction: 'HOLD STILL' };
    case 'add-player':
      return { cameraState: status.kind, instruction: 'P2 · STEP INTO FRAME' };
    case 'ready':
      return { cameraState: status.kind, instruction: 'READY · RAISE' };
  }
}
