import { describe, expect, it } from 'vitest';
import {
  buildDeepLinkQuery,
  isDeepLinkMode,
  parseDeepLink,
} from './deep-link';

describe('deep-link parsing', () => {
  it('parses a valid mode', () => {
    expect(parseDeepLink('?mode=mirror')).toEqual({ mode: 'mirror' });
    expect(parseDeepLink('mode=supernova')).toEqual({ mode: 'supernova' });
  });

  it('parses a game id and trims whitespace', () => {
    expect(parseDeepLink('?game=star-catch')).toEqual({ game: 'star-catch' });
    expect(parseDeepLink('?game=  star-catch  ')).toEqual({ game: 'star-catch' });
  });

  it('ignores unknown modes and blank games', () => {
    expect(parseDeepLink('?mode=bogus')).toEqual({});
    expect(parseDeepLink('?game=')).toEqual({});
    expect(parseDeepLink('')).toEqual({});
  });

  it('prefers game over mode when both are present', () => {
    expect(parseDeepLink('?mode=mirror&game=star-catch')).toEqual({ mode: 'mirror', game: 'star-catch' });
  });

  it('recognizes exactly the five mode keys', () => {
    expect(isDeepLinkMode('random')).toBe(true);
    expect(isDeepLinkMode('mirror')).toBe(true);
    expect(isDeepLinkMode('strike')).toBe(true);
    expect(isDeepLinkMode('duo')).toBe(true);
    expect(isDeepLinkMode('supernova')).toBe(true);
    expect(isDeepLinkMode('custom')).toBe(false);
  });
});

describe('deep-link query building', () => {
  it('builds a query without a leading "?"', () => {
    expect(buildDeepLinkQuery({ mode: 'duo' })).toBe('mode=duo');
    expect(buildDeepLinkQuery({ game: 'star-catch' })).toBe('game=star-catch');
  });

  it('returns an empty string when there are no params', () => {
    expect(buildDeepLinkQuery({})).toBe('');
  });

  it('round-trips through parse', () => {
    const cases = [{ mode: 'strike' as const }, { game: 'star-catch' }, { mode: 'mirror' as const, game: 'a-b' }];
    for (const params of cases) {
      expect(parseDeepLink(buildDeepLinkQuery(params))).toEqual(params);
    }
  });
});
