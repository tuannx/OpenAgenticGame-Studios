import { describe, expect, it } from 'vitest';
import {
  modesForProductFamily,
  productFamilyForMode,
} from './product-family';

describe('product family mapping', () => {
  it('maps runner modes to brainbreak and supernova to ar', () => {
    expect(productFamilyForMode('mirror')).toBe('brainbreak');
    expect(productFamilyForMode('strike')).toBe('brainbreak');
    expect(productFamilyForMode('duo')).toBe('brainbreak');
    expect(productFamilyForMode('supernova')).toBe('ar');
  });

  it('filters family modes by device capacity', () => {
    expect(modesForProductFamily('brainbreak', ['mirror', 'strike', 'duo', 'supernova'])).toEqual([
      'mirror',
      'strike',
      'duo',
    ]);
    expect(modesForProductFamily('brainbreak', ['mirror', 'strike', 'supernova'])).toEqual([
      'mirror',
      'strike',
    ]);
    expect(modesForProductFamily('ar', ['mirror', 'strike', 'duo', 'supernova'])).toEqual([
      'supernova',
    ]);
  });
});
