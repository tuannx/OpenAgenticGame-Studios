import { DurableObject } from 'cloudflare:workers';

interface Env {
  ASSETS: Fetcher;
  ROOMS: DurableObjectNamespace<RoomCoordinator>;
  TURN_KEY_ID?: string;
  TURN_KEY_API_TOKEN?: string;
  BUILD_SHA?: string;
}

interface RoomState {
  hostToken: string;
  expiresAt: number;
  turnGrants: number;
}

interface SocketAttachment {
  role: 'host' | 'guest';
}

const ROOM_ALPHABET = '23456789ABCDEFGHJKLMNPQRSTUVWXYZ';
const ROOM_TTL_MS = 2 * 60 * 60 * 1000;

function json(body: unknown, init: ResponseInit = {}): Response {
  const headers = new Headers(init.headers);
  headers.set('content-type', 'application/json; charset=utf-8');
  headers.set('cache-control', 'no-store');
  return new Response(JSON.stringify(body), { ...init, headers });
}

function randomCode(length = 8): string {
  const bytes = crypto.getRandomValues(new Uint8Array(length));
  return Array.from(bytes, (byte) => ROOM_ALPHABET[byte % ROOM_ALPHABET.length]).join('');
}

function securityHeaders(response: Response, pathname: string): Response {
  const output = new Response(response.body, response);
  output.headers.set('x-content-type-options', 'nosniff');
  output.headers.set('referrer-policy', 'strict-origin-when-cross-origin');
  output.headers.set('permissions-policy', 'camera=(self), microphone=(), geolocation=()');
  output.headers.set('cross-origin-opener-policy', 'same-origin');
  output.headers.set(
    'content-security-policy',
    "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob:; media-src 'self' blob:; connect-src 'self' https: wss:; worker-src 'self' blob:; frame-ancestors 'none'; base-uri 'self'; form-action 'self'",
  );
  if (pathname.endsWith('.wasm')) {
    output.headers.set('content-type', 'application/wasm');
  }
  const isHtml = output.headers.get('content-type')?.startsWith('text/html') ?? false;
  if (isHtml) {
    output.headers.set('cache-control', 'no-cache');
  } else if (
    pathname.startsWith('/assets/')
    || /\/brainbreak-game-[a-f0-9]{12}\.wasm$/.test(pathname)
    || /\/audio\/[a-z0-9-]+-[a-f0-9]{12}\.mp3$/.test(pathname)
  ) {
    output.headers.set('cache-control', 'public, max-age=31536000, immutable');
  } else if (pathname.endsWith('.wasm') || pathname.endsWith('mq_js_bundle.js')) {
    output.headers.set('cache-control', 'no-cache');
  } else if (pathname === '/' || pathname.endsWith('.html')) {
    output.headers.set('cache-control', 'no-cache');
  }
  return output;
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);
    if (url.pathname === '/health') {
      return json({
        status: 'ok',
        service: 'brainbreak-motion-party',
        game: 'neon-beat-runner',
        schemaVersion: 2,
        build: env.BUILD_SHA ?? 'dev',
      });
    }
    if (url.pathname === '/api/rooms' && request.method === 'POST') {
      for (let attempt = 0; attempt < 5; attempt += 1) {
        const code = randomCode();
        const room = env.ROOMS.getByName(code);
        const response = await room.fetch(new Request('https://room.internal/claim', { method: 'POST' }));
        if (response.status === 201) {
          const state = await response.json<RoomState>();
          return json({ code, hostToken: state.hostToken, expiresAt: state.expiresAt }, { status: 201 });
        }
      }
      return json({ error: 'room_capacity_exhausted' }, { status: 503 });
    }
    if (url.pathname === '/api/turn-credentials' && request.method === 'GET') {
      const roomCode = url.searchParams.get('room')?.toUpperCase() ?? '';
      if (!/^[A-Z2-9]{8}$/.test(roomCode)) {
        return json({ error: 'valid_room_required' }, { status: 400 });
      }
      const access = await env.ROOMS.getByName(roomCode).fetch('https://room.internal/turn-access', { method: 'POST' });
      if (!access.ok) return json({ error: 'room_not_found' }, { status: 404 });
      if (!env.TURN_KEY_ID || !env.TURN_KEY_API_TOKEN) {
        return json({
          iceServers: [{ urls: ['stun:stun.cloudflare.com:3478', 'stun:stun.cloudflare.com:53'] }],
          turnConfigured: false,
        });
      }
      const response = await fetch(
        `https://rtc.live.cloudflare.com/v1/turn/keys/${encodeURIComponent(env.TURN_KEY_ID)}/credentials/generate-ice-servers`,
        {
          method: 'POST',
          headers: { authorization: `Bearer ${env.TURN_KEY_API_TOKEN}`, 'content-type': 'application/json' },
          body: JSON.stringify({ ttl: 7200 }),
        },
      );
      if (!response.ok) {
        console.error('TURN credential generation failed', response.status);
        return json({ error: 'turn_unavailable' }, { status: 502 });
      }
      return json(await response.json());
    }
    const roomMatch = url.pathname.match(/^\/ws\/rooms\/([A-Z2-9]{8})$/i);
    if (roomMatch && request.headers.get('upgrade')?.toLowerCase() === 'websocket') {
      return env.ROOMS.getByName(roomMatch[1].toUpperCase()).fetch(request);
    }
    if (url.pathname.startsWith('/api/') || url.pathname.startsWith('/ws/')) {
      return json({ error: 'not_found' }, { status: 404 });
    }
    return securityHeaders(await env.ASSETS.fetch(request), url.pathname);
  },
} satisfies ExportedHandler<Env>;

export class RoomCoordinator extends DurableObject<Env> {
  private state?: RoomState;

  constructor(ctx: DurableObjectState, env: Env) {
    super(ctx, env);
    ctx.blockConcurrencyWhile(async () => {
      this.state = await ctx.storage.get<RoomState>('room');
    });
  }

  async fetch(request: Request): Promise<Response> {
    const url = new URL(request.url);
    if (url.pathname === '/claim' && request.method === 'POST') {
      if (this.state && this.state.expiresAt > Date.now()) return json({ error: 'claimed' }, { status: 409 });
      this.state = { hostToken: crypto.randomUUID(), expiresAt: Date.now() + ROOM_TTL_MS, turnGrants: 0 };
      await this.ctx.storage.put('room', this.state);
      await this.ctx.storage.setAlarm(this.state.expiresAt);
      return json(this.state, { status: 201 });
    }
    if (url.pathname === '/turn-access' && request.method === 'POST') {
      const grants = this.state?.turnGrants ?? 0;
      if (!this.state || this.state.expiresAt <= Date.now() || grants >= 8) {
        return json({ error: 'turn_access_denied' }, { status: 403 });
      }
      this.state.turnGrants = grants + 1;
      await this.ctx.storage.put('room', this.state);
      return json({ allowed: true });
    }
    if (request.headers.get('upgrade')?.toLowerCase() !== 'websocket') {
      return json({ error: 'upgrade_required' }, { status: 426 });
    }
    if (!this.state || this.state.expiresAt <= Date.now()) {
      return json({ error: 'room_not_found' }, { status: 404 });
    }
    const role = url.searchParams.get('role');
    if (role !== 'host' && role !== 'guest') return json({ error: 'invalid_role' }, { status: 400 });
    if (role === 'host' && url.searchParams.get('token') !== this.state.hostToken) {
      return json({ error: 'invalid_host_token' }, { status: 403 });
    }
    const sockets = this.ctx.getWebSockets();
    if (sockets.some((socket) => (socket.deserializeAttachment() as SocketAttachment | null)?.role === role)) {
      return json({ error: 'role_already_connected' }, { status: 409 });
    }
    if (sockets.length >= 2) return json({ error: 'room_full' }, { status: 409 });

    const pair = new WebSocketPair();
    const [client, server] = Object.values(pair);
    this.ctx.acceptWebSocket(server, [role]);
    server.serializeAttachment({ role } satisfies SocketAttachment);
    server.send(JSON.stringify({ type: 'signal-ready', role, expiresAt: this.state.expiresAt }));
    if (this.ctx.getWebSockets().length === 2) {
      for (const socket of this.ctx.getWebSockets()) socket.send(JSON.stringify({ type: 'peer-ready' }));
    }
    return new Response(null, { status: 101, webSocket: client });
  }

  async webSocketMessage(socket: WebSocket, message: string | ArrayBuffer): Promise<void> {
    const attachment = socket.deserializeAttachment() as SocketAttachment | null;
    if (!attachment) {
      socket.close(1008, 'missing attachment');
      return;
    }
    const size = typeof message === 'string' ? new TextEncoder().encode(message).byteLength : message.byteLength;
    if (size > 16_384) {
      socket.close(1009, 'message too large');
      return;
    }
    for (const peer of this.ctx.getWebSockets()) {
      if (peer !== socket && peer.readyState === WebSocket.OPEN) peer.send(message);
    }
  }

  async webSocketClose(socket: WebSocket, code: number, reason: string): Promise<void> {
    const attachment = socket.deserializeAttachment() as SocketAttachment | null;
    for (const peer of this.ctx.getWebSockets()) {
      if (peer !== socket && peer.readyState === WebSocket.OPEN) {
        peer.send(JSON.stringify({ type: 'peer-left', role: attachment?.role ?? 'unknown' }));
      }
    }
    void code;
    void reason;
  }

  async alarm(): Promise<void> {
    for (const socket of this.ctx.getWebSockets()) socket.close(1001, 'room expired');
    await this.ctx.storage.deleteAll();
    this.state = undefined;
  }
}
