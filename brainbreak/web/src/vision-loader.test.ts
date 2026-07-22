import { describe, expect, it, vi } from 'vitest';
import { VisionModuleLoader } from './vision-loader';
import type { VisionModulePort } from './vision-loader';

function fakeVisionModule(): VisionModulePort {
  return {
    setOverlayConfig: vi.fn(),
    startVision: vi.fn(async () => {}),
    stopVision: vi.fn(),
  };
}

describe('VisionModuleLoader', () => {
  it('stores preferences without importing until camera intent', () => {
    const importer = vi.fn(async () => fakeVisionModule());
    const loader = new VisionModuleLoader(
      { mode: 'glass_pip', theme: 'cyber-trunk' },
      importer,
    );

    loader.updatePreferences({ mode: 'hologram', theme: 'purple-vaporwave' });

    expect(importer).not.toHaveBeenCalled();
  });

  it('deduplicates concurrent imports and applies the latest preferences', async () => {
    const module = fakeVisionModule();
    let resolveImport: ((value: VisionModulePort) => void) | undefined;
    const importer = vi.fn(() => new Promise<VisionModulePort>((resolve) => {
      resolveImport = resolve;
    }));
    const loader = new VisionModuleLoader(
      { mode: 'glass_pip', theme: 'cyber-trunk' },
      importer,
    );

    const first = loader.load();
    loader.updatePreferences({ mode: 'cutout', theme: 'pink-girl' });
    const second = loader.load();
    resolveImport?.(module);

    expect(await first).toBe(module);
    expect(await second).toBe(module);
    expect(importer).toHaveBeenCalledTimes(1);
    expect(module.setOverlayConfig).toHaveBeenLastCalledWith({
      mode: 'cutout',
      theme: 'pink-girl',
    });
  });

  it('retries after import failure and cleanup never triggers an import', async () => {
    const module = fakeVisionModule();
    const importer = vi.fn()
      .mockRejectedValueOnce(new Error('offline'))
      .mockResolvedValueOnce(module);
    const loader = new VisionModuleLoader(
      { mode: 'skeleton', theme: 'cyber-trunk' },
      importer,
    );

    loader.stopIfLoaded();
    expect(importer).not.toHaveBeenCalled();
    await expect(loader.load()).rejects.toThrow('offline');
    expect(await loader.load()).toBe(module);
    loader.stopIfLoaded();

    expect(importer).toHaveBeenCalledTimes(2);
    expect(module.stopVision).toHaveBeenCalledTimes(1);
  });

  it('does not cache a module whose initial preference application fails', async () => {
    const brokenModule = fakeVisionModule();
    vi.mocked(brokenModule.setOverlayConfig).mockImplementationOnce(() => {
      throw new Error('invalid adapter state');
    });
    const recoveredModule = fakeVisionModule();
    const importer = vi.fn()
      .mockResolvedValueOnce(brokenModule)
      .mockResolvedValueOnce(recoveredModule);
    const loader = new VisionModuleLoader(
      { mode: 'glass_pip', theme: 'cyber-trunk' },
      importer,
    );

    await expect(loader.load()).rejects.toThrow('invalid adapter state');
    expect(await loader.load()).toBe(recoveredModule);
    expect(importer).toHaveBeenCalledTimes(2);
  });
});
