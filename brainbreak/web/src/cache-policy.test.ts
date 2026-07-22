import { describe, expect, it } from 'vitest';
import { cacheControlForAsset } from '../../worker/src/cache-policy';

describe('Cloudflare asset cache policy', () => {
  it('immutably caches only typed fingerprinted runtime assets', () => {
    expect(cacheControlForAsset(
      '/brainbreak-game-123456abcdef.wasm',
      'application/wasm',
      true,
    )).toBe('public, max-age=31536000, immutable');
    expect(cacheControlForAsset(
      '/audio/special-spotlight-123456abcdef.mp3',
      'audio/mpeg',
      true,
    )).toBe('public, max-age=31536000, immutable');
  });

  it('never lets an SPA HTML fallback poison a typed immutable URL', () => {
    expect(cacheControlForAsset(
      '/brainbreak-game-deadbeefdead.wasm',
      'text/html; charset=utf-8',
      true,
    )).toBe('no-cache');
    expect(cacheControlForAsset('/assets/missing.js', 'text/html', true)).toBe('no-cache');
    expect(cacheControlForAsset('/assets/unknown.js', null, true)).toBe('no-cache');
  });

  it('keeps failures and unhashed loader assets revalidating', () => {
    expect(cacheControlForAsset('/brainbreak-game-deadbeefdead.wasm', null, false)).toBe('no-cache');
    expect(cacheControlForAsset('/mq_js_bundle.js', 'text/javascript', true)).toBe('no-cache');
  });
});
