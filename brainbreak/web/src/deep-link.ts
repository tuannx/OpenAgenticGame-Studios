// URL deep-linking: the address bar always mirrors the selected game/mode so
// links are shareable and bookmarkable. Query-param based (?mode= / ?game=),
// which works with the existing single-page-application asset handling.

export type DeepLinkMode = 'random' | 'mirror' | 'strike' | 'duo' | 'supernova';

const MODE_KEYS: readonly DeepLinkMode[] = ['random', 'mirror', 'strike', 'duo', 'supernova'];

export interface DeepLink {
  mode?: DeepLinkMode;
  game?: string;
}

export function isDeepLinkMode(value: string): value is DeepLinkMode {
  return (MODE_KEYS as readonly string[]).includes(value);
}

/** Parse a location.search string (with or without the leading "?"). */
export function parseDeepLink(search: string): DeepLink {
  const params = new URLSearchParams(search.startsWith('?') ? search.slice(1) : search);
  const link: DeepLink = {};
  const mode = params.get('mode');
  if (mode && isDeepLinkMode(mode)) link.mode = mode;
  const game = params.get('game');
  if (game && game.trim() !== '') link.game = game.trim();
  return link;
}

/** Build a query string (no leading "?"); empty when there are no params. */
export function buildDeepLinkQuery(params: DeepLink): string {
  const search = new URLSearchParams();
  if (params.mode) search.set('mode', params.mode);
  if (params.game) search.set('game', params.game);
  return search.toString();
}

/** Read the current deep link from the address bar. */
export function readDeepLink(): DeepLink {
  if (typeof window === 'undefined') return {};
  return parseDeepLink(window.location.search);
}

/** Update the address bar to reflect the given position (no reload). */
export function writeDeepLink(params: DeepLink): void {
  if (typeof window === 'undefined') return;
  const query = buildDeepLinkQuery(params);
  const url = query ? `${window.location.pathname}?${query}` : window.location.pathname;
  window.history.replaceState(null, '', url);
}
