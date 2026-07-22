import type {
  CameraOverlayMode,
  OverlayConfig,
  PoseSnapshot,
  VisionStatus,
  VisualTheme,
} from './vision';
import type { CameraFrameGeometry } from './motion-capability';

export interface VisionModulePort {
  setOverlayConfig(config: Partial<OverlayConfig>): void;
  startVision(
    video: HTMLVideoElement,
    overlay: HTMLCanvasElement,
    onPoses: (poses: PoseSnapshot[]) => void,
    onStatus: (status: VisionStatus) => void,
    onFrameGeometry: (geometry: CameraFrameGeometry) => void,
    signal?: AbortSignal,
  ): Promise<void>;
  stopVision(): void;
}

export interface VisionPreferences {
  mode: CameraOverlayMode;
  theme: VisualTheme;
}

type VisionImporter = () => Promise<VisionModulePort>;

export class VisionModuleLoader {
  private module: VisionModulePort | undefined;
  private pending: Promise<VisionModulePort> | undefined;

  constructor(
    private preferences: VisionPreferences,
    private readonly importer: VisionImporter = () => import('./vision'),
  ) {}

  updatePreferences(patch: Partial<VisionPreferences>): void {
    this.preferences = { ...this.preferences, ...patch };
    this.module?.setOverlayConfig(this.preferences);
  }

  load(): Promise<VisionModulePort> {
    if (this.module) {
      this.module.setOverlayConfig(this.preferences);
      return Promise.resolve(this.module);
    }
    if (!this.pending) {
      this.pending = this.importer()
        .then((module) => {
          module.setOverlayConfig(this.preferences);
          this.module = module;
          return module;
        })
        .catch((error: unknown) => {
          this.pending = undefined;
          throw error;
        });
    }
    return this.pending;
  }

  stopIfLoaded(): void {
    this.module?.stopVision();
  }
}
