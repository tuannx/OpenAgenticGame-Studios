class ProceduralBeat {
  private context?: AudioContext;
  private timer?: number;
  private nextBeat = 0;
  private beat = 0;
  private muted = false;

  async start(): Promise<void> {
    this.context ??= new AudioContext({ latencyHint: 'interactive' });
    await this.context.resume();
    if (this.timer !== undefined) return;
    this.nextBeat = this.context.currentTime + 0.05;
    this.schedule();
    this.timer = window.setInterval(() => this.schedule(), 25);
  }

  toggle(): boolean {
    this.muted = !this.muted;
    return this.muted;
  }

  private schedule(): void {
    const context = this.context;
    if (!context) return;
    const secondsPerBeat = 60 / 112;
    while (this.nextBeat < context.currentTime + 0.12) {
      if (!this.muted) this.playPulse(context, this.nextBeat, this.beat % 4 === 0);
      this.nextBeat += secondsPerBeat;
      this.beat += 1;
    }
  }

  private playPulse(context: AudioContext, at: number, downbeat: boolean): void {
    const oscillator = context.createOscillator();
    const gain = context.createGain();
    oscillator.type = downbeat ? 'sine' : 'triangle';
    oscillator.frequency.setValueAtTime(downbeat ? 120 : 620, at);
    oscillator.frequency.exponentialRampToValueAtTime(downbeat ? 48 : 420, at + 0.08);
    gain.gain.setValueAtTime(downbeat ? 0.18 : 0.055, at);
    gain.gain.exponentialRampToValueAtTime(0.001, at + 0.1);
    oscillator.connect(gain).connect(context.destination);
    oscillator.start(at);
    oscillator.stop(at + 0.11);
  }
}

export const proceduralBeat = new ProceduralBeat();
