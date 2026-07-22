const FINGERPRINTED_WASM = /\/brainbreak-game-[a-f0-9]{12}\.wasm$/;
const FINGERPRINTED_AUDIO = /\/audio\/[a-z0-9-]+-[a-f0-9]{12}\.mp3$/;

export function cacheControlForAsset(
  pathname: string,
  contentType: string | null,
  responseOk: boolean,
): string | null {
  const normalizedType = contentType?.toLowerCase() ?? '';
  const isHtml = normalizedType.startsWith('text/html');
  if (!responseOk || isHtml) return 'no-cache';

  const isBundledAssetType = normalizedType.startsWith('text/css')
    || normalizedType.startsWith('text/javascript')
    || normalizedType.startsWith('application/javascript')
    || normalizedType.startsWith('image/')
    || normalizedType.startsWith('font/')
    || normalizedType.startsWith('application/font');
  if (pathname.startsWith('/assets/') && isBundledAssetType) {
    return 'public, max-age=31536000, immutable';
  }
  if (FINGERPRINTED_WASM.test(pathname) && normalizedType.startsWith('application/wasm')) {
    return 'public, max-age=31536000, immutable';
  }
  if (FINGERPRINTED_AUDIO.test(pathname) && normalizedType.startsWith('audio/')) {
    return 'public, max-age=31536000, immutable';
  }
  if (pathname.endsWith('.wasm') || pathname.endsWith('mq_js_bundle.js')) return 'no-cache';
  if (pathname === '/' || pathname.endsWith('.html')) return 'no-cache';
  if (pathname.startsWith('/assets/')) return 'no-cache';
  return null;
}
