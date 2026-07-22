import type { PoseFramingStatus } from './pose-framing';

export interface PoseFramingPresentation {
  cameraState: PoseFramingStatus['kind'];
  instruction: string;
}

export function presentPoseFraming(status: PoseFramingStatus): PoseFramingPresentation {
  switch (status.kind) {
    case 'searching':
      return { cameraState: status.kind, instruction: 'STEP INTO THE FRAME' };
    case 'move-closer':
      return { cameraState: status.kind, instruction: 'MOVE A LITTLE CLOSER' };
    case 'move-back':
      return { cameraState: status.kind, instruction: 'STEP BACK - SHOW HEAD, SHOULDERS & HIPS' };
    case 'center':
      return { cameraState: status.kind, instruction: 'MOVE TO THE CENTER' };
    case 'show-body':
      return { cameraState: status.kind, instruction: 'SHOW HEAD, SHOULDERS, HIPS & HANDS' };
    case 'steady':
      return { cameraState: status.kind, instruction: 'GOOD POSITION - HOLD STILL' };
    case 'add-player':
      return { cameraState: status.kind, instruction: 'P2: STEP INTO FRAME' };
    case 'ready':
      return { cameraState: status.kind, instruction: 'BODY READY' };
  }
}
