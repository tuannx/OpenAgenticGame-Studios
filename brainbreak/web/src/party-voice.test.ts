import { afterEach, describe, expect, it, vi } from 'vitest';
import { musicEngine } from './audio';
import { VOICE_CUES, countdownCueId } from './party-voice-cues';
import { SupernovaEvent, partyDirector } from './party-voice';

describe('party voice cue map', () => {
  it('keeps every cue to one short kid line', () => {
    for (const cue of Object.values(VOICE_CUES)) {
      expect(cue.text.length).toBeLessThanOrEqual(24);
      expect(cue.text.includes('.')).toBe(false);
    }
  });

  it('maps countdown numbers to spoken keys', () => {
    expect(countdownCueId(3)).toBe('count_3');
    expect(countdownCueId(2)).toBe('count_2');
    expect(countdownCueId(1)).toBe('count_1');
    expect(countdownCueId(0)).toBe('go');
    expect(countdownCueId(9)).toBeNull();
  });
});

describe('party director guided voice + music', () => {
  afterEach(() => {
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  it('survives result afterglow without throwing', () => {
    partyDirector.handleEvent(SupernovaEvent.Result, 1);
  });

  it('engages drop tension then clears it on Drop', () => {
    const tension = vi.spyOn(musicEngine, 'setDropTension').mockImplementation(() => undefined);
    const resume = vi.spyOn(musicEngine, 'resume').mockImplementation(() => undefined);

    partyDirector.handleEvent(SupernovaEvent.DropImminent, 0);
    expect(tension).toHaveBeenCalledWith(true);

    partyDirector.handleEvent(SupernovaEvent.Drop, 0);
    expect(tension).toHaveBeenCalledWith(false);
    expect(resume).toHaveBeenCalled();
  });

  it('keeps music through lava warning and pauses only on freeze', () => {
    const resume = vi.spyOn(musicEngine, 'resume').mockImplementation(() => undefined);
    const pause = vi.spyOn(musicEngine, 'pause').mockImplementation(() => undefined);

    partyDirector.handleEvent(SupernovaEvent.LavaWarning, 0);
    expect(resume).toHaveBeenCalled();
    expect(pause).not.toHaveBeenCalled();

    partyDirector.handleEvent(SupernovaEvent.Freeze, 0);
    expect(pause).toHaveBeenCalled();
  });

  it('resumes music on dance without throwing', () => {
    const resume = vi.spyOn(musicEngine, 'resume').mockImplementation(() => undefined);
    partyDirector.handleEvent(SupernovaEvent.Dance, 0);
    expect(resume).toHaveBeenCalled();
  });

  it('handles both result outcomes', () => {
    expect(() => partyDirector.handleEvent(SupernovaEvent.Result, 0)).not.toThrow();
    expect(() => partyDirector.handleEvent(SupernovaEvent.Result, 1)).not.toThrow();
  });

  it('ducks music when speaking a shell guide line', async () => {
    const duck = vi.spyOn(musicEngine, 'duck').mockImplementation(() => undefined);
    const speakFn = vi.fn();
    vi.stubGlobal('fetch', vi.fn().mockRejectedValue(new Error('no clips')));
    vi.stubGlobal('speechSynthesis', {
      cancel: vi.fn(),
      speak: speakFn,
      getVoices: () => [],
      addEventListener: vi.fn(),
    });
    vi.stubGlobal(
      'SpeechSynthesisUtterance',
      function MockUtterance(this: SpeechSynthesisUtterance, text?: string) {
        this.text = text ?? '';
        this.pitch = 1;
        this.rate = 1;
        this.volume = 1;
        this.onend = null;
        this.onerror = null;
      },
    );

    partyDirector.guide('jump', { force: true });
    await vi.waitFor(() => {
      expect(speakFn).toHaveBeenCalled();
      expect(duck).toHaveBeenCalled();
    });
  });

  it('announces hold once when ready progress starts', () => {
    partyDirector.resetReadyGuides();
    const guide = vi.spyOn(partyDirector, 'guide');
    partyDirector.onReadyHoldProgress(0.05);
    expect(guide).not.toHaveBeenCalled();
    partyDirector.onReadyHoldProgress(0.2);
    expect(guide).toHaveBeenCalledWith('hold', { force: true });
    partyDirector.onReadyHoldProgress(0.9);
    expect(guide).toHaveBeenCalledTimes(1);
  });
});
