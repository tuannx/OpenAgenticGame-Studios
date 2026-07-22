export type ShellOwner = 'launcher' | 'ready' | 'guide' | 'gameplay';

export interface ShellPresentation {
  launcherVisible: boolean;
  readyVisible: boolean;
  guideVisible: boolean;
  orientationVisible: boolean;
  backgroundInert: boolean;
}

export function presentShellOwner(owner: ShellOwner, portraitBlocked = false): ShellPresentation {
  if (portraitBlocked) {
    return {
      launcherVisible: false,
      readyVisible: false,
      guideVisible: false,
      orientationVisible: true,
      backgroundInert: true,
    };
  }
  return {
    launcherVisible: owner === 'launcher',
    readyVisible: owner === 'ready',
    guideVisible: owner === 'guide',
    orientationVisible: false,
    backgroundInert: owner !== 'gameplay',
  };
}
