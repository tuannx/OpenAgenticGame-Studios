import { readdirSync } from 'node:fs';
import { resolve } from 'node:path';
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

const publicDirectory = resolve(import.meta.dirname, 'web/public');
const wasmAsset = readdirSync(publicDirectory).find((name) => /^brainbreak-game-[a-f0-9]{12}\.wasm$/.test(name));
if (!wasmAsset) throw new Error('Run npm run build:wasm before Vite so the fingerprinted WASM asset exists');

export default defineConfig({
  root: 'web',
  plugins: [react()],
  define: {
    __BRAINBREAK_WASM_PATH__: JSON.stringify(`/${wasmAsset}`),
  },
  build: {
    target: 'es2022',
    rollupOptions: {
      input: {
        game: resolve(import.meta.dirname, 'web/index.html'),
        studio: resolve(import.meta.dirname, 'web/studio.html'),
      },
    },
  },
  server: {
    proxy: {
      '/api': 'http://localhost:8787',
      '/ws': { target: 'ws://localhost:8787', ws: true },
    },
  },
});
