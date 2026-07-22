export class CameraStartCancelledError extends Error {
  constructor() {
    super('Camera start cancelled');
    this.name = 'CameraStartCancelledError';
  }
}

function stopStream(stream: MediaStream): void {
  stream.getTracks().forEach((track) => track.stop());
}

export function isCameraStartCancelled(error: unknown): error is CameraStartCancelledError {
  return error instanceof CameraStartCancelledError;
}

export function throwIfCameraStartCancelled(signal?: AbortSignal): void {
  if (signal?.aborted) throw new CameraStartCancelledError();
}

export function waitForCameraStream(
  request: Promise<MediaStream>,
  signal?: AbortSignal,
): Promise<MediaStream> {
  if (!signal) return request;

  return new Promise<MediaStream>((resolve, reject) => {
    let settled = false;

    const discardLateStream = (): void => {
      void request.then(stopStream, () => undefined);
    };
    const cancel = (): void => {
      if (settled) return;
      settled = true;
      signal.removeEventListener('abort', cancel);
      discardLateStream();
      reject(new CameraStartCancelledError());
    };

    if (signal.aborted) {
      cancel();
      return;
    }

    signal.addEventListener('abort', cancel, { once: true });
    void request.then(
      (stream) => {
        if (settled) return;
        settled = true;
        signal.removeEventListener('abort', cancel);
        resolve(stream);
      },
      (error: unknown) => {
        if (settled) return;
        settled = true;
        signal.removeEventListener('abort', cancel);
        reject(error);
      },
    );
  });
}
