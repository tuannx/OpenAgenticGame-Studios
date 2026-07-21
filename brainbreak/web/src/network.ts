type SignalMessage =
  | { type: 'peer-ready' }
  | { type: 'offer'; sdp: RTCSessionDescriptionInit }
  | { type: 'answer'; sdp: RTCSessionDescriptionInit }
  | { type: 'candidate'; candidate: RTCIceCandidateInit };

interface RoomCredentials {
  code: string;
  hostToken: string;
  expiresAt: number;
}

class RoomTransport {
  statusCode = 0;
  private websocket?: WebSocket;
  private peer?: RTCPeerConnection;
  private channel?: RTCDataChannel;
  private host = false;
  private remoteActions = [0, 0];
  private pendingCandidates: RTCIceCandidateInit[] = [];
  private iceServers: RTCIceServer[] = [{ urls: 'stun:stun.cloudflare.com:3478' }];
  onStatus: (message: string) => void = () => undefined;

  async createRoom(): Promise<string> {
    const response = await fetch('/api/rooms', { method: 'POST' });
    if (!response.ok) throw new Error(`Room creation failed (${response.status})`);
    const room = await response.json() as RoomCredentials;
    this.host = true;
    await this.connectSignal(room.code, room.hostToken);
    return room.code;
  }

  async joinRoom(code: string): Promise<void> {
    this.host = false;
    await this.connectSignal(code.toUpperCase(), '');
  }

  sendAction(player: number, mask: number): void {
    if (this.channel?.readyState !== 'open') return;
    this.channel.send(JSON.stringify({ type: 'action', player, mask, at: Date.now() }));
  }

  takeRemoteAction(player: number): number {
    const value = this.remoteActions[player] ?? 0;
    this.remoteActions[player] = 0;
    return value;
  }

  private async loadIceServers(code: string): Promise<void> {
    try {
      const response = await fetch(`/api/turn-credentials?room=${encodeURIComponent(code)}`);
      if (response.ok) {
        const payload = await response.json() as { iceServers?: RTCIceServer[] };
        if (payload.iceServers?.length) this.iceServers = payload.iceServers;
      }
    } catch {
      // STUN-only remains a valid direct-connect fallback.
    }
  }

  private async connectSignal(code: string, token: string): Promise<void> {
    this.close();
    this.statusCode = 1;
    this.onStatus(`Connecting to ${code}…`);
    await this.loadIceServers(code);
    const protocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
    const role = this.host ? 'host' : 'guest';
    const url = `${protocol}//${location.host}/ws/rooms/${encodeURIComponent(code)}?role=${role}&token=${encodeURIComponent(token)}`;
    this.websocket = new WebSocket(url);
    this.websocket.addEventListener('message', (event) => {
      try {
        void this.handleSignal(JSON.parse(String(event.data)) as SignalMessage);
      } catch {
        this.onStatus('Ignored malformed signaling message');
      }
    });
    this.websocket.addEventListener('close', () => {
      if (this.statusCode < 2) this.onStatus('Room signaling closed');
    });
    this.websocket.addEventListener('error', () => this.onStatus('Room connection failed'));
  }

  private setupPeer(): RTCPeerConnection {
    this.peer?.close();
    const peer = new RTCPeerConnection({ iceServers: this.iceServers });
    peer.addEventListener('icecandidate', (event) => {
      if (event.candidate) this.signal({ type: 'candidate', candidate: event.candidate.toJSON() });
    });
    peer.addEventListener('connectionstatechange', () => {
      if (peer.connectionState === 'connected') {
        this.statusCode = this.host ? 3 : 2;
        this.onStatus(this.host ? 'Host • P2P connected' : 'P2P connected');
      } else if (['failed', 'disconnected'].includes(peer.connectionState)) {
        this.onStatus(`P2P ${peer.connectionState}`);
      }
    });
    peer.addEventListener('datachannel', (event) => this.attachChannel(event.channel));
    this.peer = peer;
    this.pendingCandidates = [];
    return peer;
  }

  private attachChannel(channel: RTCDataChannel): void {
    this.channel = channel;
    channel.addEventListener('open', () => {
      this.statusCode = this.host ? 3 : 2;
      this.onStatus(this.host ? 'Host • P2P connected' : 'P2P connected');
    });
    channel.addEventListener('message', (event) => {
      const message = JSON.parse(String(event.data)) as { type: string; player?: number; mask?: number };
      if (message.type === 'action' && Number.isInteger(message.player) && Number.isInteger(message.mask)) {
        const player = Math.max(0, Math.min(1, message.player!));
        this.remoteActions[player] |= message.mask! >>> 0;
      }
    });
  }

  private async handleSignal(message: SignalMessage): Promise<void> {
    if (message.type === 'peer-ready' && this.host) {
      const peer = this.setupPeer();
      this.attachChannel(peer.createDataChannel('brainbreak-actions', { ordered: true }));
      await peer.setLocalDescription(await peer.createOffer());
      this.signal({ type: 'offer', sdp: peer.localDescription! });
    } else if (message.type === 'offer' && !this.host) {
      const peer = this.setupPeer();
      await peer.setRemoteDescription(message.sdp);
      await this.flushCandidates(peer);
      await peer.setLocalDescription(await peer.createAnswer());
      this.signal({ type: 'answer', sdp: peer.localDescription! });
    } else if (message.type === 'answer' && this.host && this.peer) {
      await this.peer.setRemoteDescription(message.sdp);
      await this.flushCandidates(this.peer);
    } else if (message.type === 'candidate' && this.peer) {
      if (this.peer.remoteDescription) await this.peer.addIceCandidate(message.candidate);
      else this.pendingCandidates.push(message.candidate);
    }
  }

  private async flushCandidates(peer: RTCPeerConnection): Promise<void> {
    for (const candidate of this.pendingCandidates.splice(0)) await peer.addIceCandidate(candidate);
  }

  private signal(message: SignalMessage): void {
    if (this.websocket?.readyState === WebSocket.OPEN) this.websocket.send(JSON.stringify(message));
  }

  private close(): void {
    this.channel?.close();
    this.peer?.close();
    this.websocket?.close();
    this.statusCode = 0;
  }
}

export const roomTransport = new RoomTransport();
