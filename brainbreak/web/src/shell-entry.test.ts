import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

const entryHtml = readFileSync(new URL('../index.html', import.meta.url), 'utf8');

describe('production shell entry', () => {
  it('gives first paint to the launcher without a duplicate boot surface', () => {
    expect(entryHtml).toContain('<body class="launcher-open">');
    expect(entryHtml).toContain('id="motion-gate" role="dialog"');
    expect(entryHtml).toContain('id="glcanvas" tabindex="1" aria-label="BrainBreak game canvas" inert');
    expect(entryHtml).not.toContain('id="boot-screen"');
    expect(entryHtml).not.toContain('Charging the rhythm highway');
  });

  it('gives Ready status to the live camera preview without a hidden art owner', () => {
    expect(entryHtml.match(/id="camera-status"/g)).toHaveLength(1);
    expect(entryHtml).toContain('id="camera-status" role="status" aria-live="polite"');
    expect(entryHtml).not.toContain('id="ready-status-badge"');
    expect(entryHtml).not.toContain('class="ready-badge-box"');
  });

  it('makes the Ready dialog a programmatic focus owner', () => {
    expect(entryHtml).toContain('id="ready-gate" class="hidden" data-action="framing" role="dialog" aria-modal="true" aria-labelledby="ready-gate-title" tabindex="-1"');
  });

  it('makes the illustrated English rotate state a no-touch modal owner', () => {
    expect(entryHtml).toContain('<html lang="en">');
    expect(entryHtml).toContain('id="orientation-gate" class="hidden" role="dialog" aria-modal="true"');
    expect(entryHtml).toContain('class="rotate-device-visual" aria-hidden="true"');
    expect(entryHtml).toContain('src="/assets/phone_placement_guide.webp"');
    expect(entryHtml).toContain('ROTATE TO PLAY');
    expect(entryHtml).not.toMatch(/[À-ỹĐđ]/u);
  });
});
