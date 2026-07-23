import type { PlayableGameMode } from './motion-capability';

export type ProductFamily = 'brainbreak' | 'ar';

export const BRAINBREAK_MODES: readonly PlayableGameMode[] = ['mirror', 'strike', 'duo'];
export const AR_MODES: readonly PlayableGameMode[] = ['supernova'];

export function productFamilyForMode(mode: PlayableGameMode): ProductFamily {
  return mode === 'supernova' ? 'ar' : 'brainbreak';
}

export function modesForProductFamily(
  family: ProductFamily,
  available: readonly PlayableGameMode[],
): readonly PlayableGameMode[] {
  const familyModes = family === 'ar' ? AR_MODES : BRAINBREAK_MODES;
  return familyModes.filter((mode) => available.includes(mode));
}

export function isProductFamily(value: string | undefined): value is ProductFamily {
  return value === 'brainbreak' || value === 'ar';
}
