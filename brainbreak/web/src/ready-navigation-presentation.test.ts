import { describe, expect, it } from 'vitest';
import type { MotionNavigationState } from './motion-navigation';
import type { PoseFramingPresentation } from './pose-framing-status';
import { presentReadyNavigation } from './ready-navigation-presentation';

const baseState: MotionNavigationState = {
  confirmed: false,
  confirmProgress: 0,
  direction: 'neutral',
  mode: 'mirror',
  requiredPlayers: 1,
  trackedPlayers: 1,
};

const readyFraming: PoseFramingPresentation = {
  cameraState: 'ready',
  instruction: 'READY · RAISE',
};

describe('shape-first ready navigation presentation', () => {
  it('keeps a physical framing correction above navigation feedback', () => {
    const framing: PoseFramingPresentation = {
      cameraState: 'move-back',
      instruction: 'STEP BACK · FULL BODY',
    };

    expect(presentReadyNavigation({
      ...baseState,
      confirmProgress: 0.8,
      direction: 'right',
    }, framing)).toEqual({
      visualState: 'framing',
      status: framing.instruction,
    });
  });

  it('maps direction, neutral, and holding to closed visual states', () => {
    expect(presentReadyNavigation({ ...baseState, direction: 'left' }, readyFraming).visualState)
      .toBe('left');
    expect(presentReadyNavigation({ ...baseState, direction: 'right' }, readyFraming).visualState)
      .toBe('right');
    expect(presentReadyNavigation(baseState, readyFraming)).toEqual({
      visualState: 'neutral',
      status: 'READY · RAISE',
    });
  });

  it('keeps visible hold copy static while progress remains numeric and separate', () => {
    const early = presentReadyNavigation({ ...baseState, confirmProgress: 0.1 }, readyFraming);
    const late = presentReadyNavigation({ ...baseState, confirmProgress: 0.95 }, readyFraming);

    expect(early).toEqual({ visualState: 'holding', status: 'HOLD · KEEP RISING' });
    expect(late).toEqual(early);
    expect(early.status).not.toMatch(/[0-9%•↔⬅➡🙌👀✨]/u);
  });

  it('surfaces party invite copy when Duo has two tracked bodies', () => {
    expect(presentReadyNavigation({
      ...baseState,
      mode: 'duo',
      requiredPlayers: 2,
      trackedPlayers: 2,
    }, readyFraming)).toEqual({
      visualState: 'neutral',
      status: 'PARTY · BOTH RAISE',
    });
  });

  it('asks for a partner when Duo is selected with one body', () => {
    expect(presentReadyNavigation({
      ...baseState,
      mode: 'duo',
      requiredPlayers: 2,
      trackedPlayers: 1,
    }, readyFraming).status).toBe('NEED P2 IN FRAME');
  });
});
