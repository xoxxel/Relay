import { test } from 'node:test';
import assert from 'node:assert/strict';
import { copyText } from './clipboard.js';

test('copy supports secure API, HTTP fallback and reports failure', async () => {
  let written, removed = 0, restored = 0, selected = 0;
  Object.defineProperty(globalThis, 'navigator', { configurable: true, value: { clipboard: { writeText: async text => { written = text; } } } });
  globalThis.isSecureContext = true;
  await copyText('سلام');
  assert.equal(written, 'سلام');
  globalThis.document = {
    activeElement: { focus() { restored++; } },
    body: { appendChild() {} },
    createElement: () => ({style:{}, setAttribute(){}, focus(){}, select(){ selected++; }, setSelectionRange(){}, remove(){ removed++; }}),
    execCommand: () => true,
  };
  globalThis.isSecureContext = false;
  await copyText('LAN');
  assert.equal(selected, 1);
  globalThis.isSecureContext = true;
  navigator.clipboard.writeText = async () => { throw new Error('denied'); };
  await copyText('fallback');
  document.execCommand = () => false;
  await assert.rejects(copyText('blocked'), /blocked copying/);
  assert.equal(removed, 3);
  assert.equal(restored, 3);
});
