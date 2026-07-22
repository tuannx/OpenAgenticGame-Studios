import { describe, expect, it } from 'vitest';
import { presentShellOwner, type ShellOwner } from './shell-presentation';

describe('presentShellOwner', () => {
  it.each<ShellOwner>(['launcher', 'ready', 'guide'])('%s owns one modal and blocks the background', (owner) => {
    const presentation = presentShellOwner(owner);
    const visibleModalCount = [
      presentation.launcherVisible,
      presentation.readyVisible,
      presentation.guideVisible,
      presentation.orientationVisible,
    ].filter(Boolean).length;

    expect(visibleModalCount).toBe(1);
    expect(presentation.backgroundInert).toBe(true);
  });

  it('gives gameplay an interactive background with no modal owner', () => {
    expect(presentShellOwner('gameplay')).toEqual({
      launcherVisible: false,
      readyVisible: false,
      guideVisible: false,
      orientationVisible: false,
      backgroundInert: false,
    });
  });

  it.each<ShellOwner>(['launcher', 'ready', 'guide', 'gameplay'])('lets orientation own portrait above %s', (owner) => {
    expect(presentShellOwner(owner, true)).toEqual({
      launcherVisible: false,
      readyVisible: false,
      guideVisible: false,
      orientationVisible: true,
      backgroundInert: true,
    });
  });
});
