import { describe, expect, it, vi } from 'vitest';
import {
  CameraStartCancelledError,
  throwIfCameraStartCancelled,
  waitForCameraStream,
} from './camera-start-cancellation';

function deferredStream(): {
  promise: Promise<MediaStream>;
  resolve: (stream: MediaStream) => void;
} {
  let resolve!: (stream: MediaStream) => void;
  const promise = new Promise<MediaStream>((next) => { resolve = next; });
  return { promise, resolve };
}

describe('camera start cancellation', () => {
  it('rejects immediately and stops a stream that resolves after cancellation', async () => {
    const request = deferredStream();
    const stop = vi.fn();
    const stream = { getTracks: () => [{ stop }] } as unknown as MediaStream;
    const controller = new AbortController();
    const result = waitForCameraStream(request.promise, controller.signal);

    controller.abort();
    await expect(result).rejects.toBeInstanceOf(CameraStartCancelledError);
    request.resolve(stream);
    await Promise.resolve();

    expect(stop).toHaveBeenCalledTimes(1);
  });

  it('returns an on-time stream without stopping it', async () => {
    const stop = vi.fn();
    const stream = { getTracks: () => [{ stop }] } as unknown as MediaStream;
    const controller = new AbortController();

    await expect(waitForCameraStream(Promise.resolve(stream), controller.signal)).resolves.toBe(stream);
    expect(stop).not.toHaveBeenCalled();
  });

  it('guards publication after any later asynchronous startup stage', () => {
    const controller = new AbortController();
    controller.abort();

    expect(() => throwIfCameraStartCancelled(controller.signal)).toThrow(CameraStartCancelledError);
  });
});
