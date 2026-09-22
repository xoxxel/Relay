// HTTP LAN pages may not expose the secure-context Clipboard API.
export async function copyText(text) {
  if (globalThis.isSecureContext && navigator.clipboard?.writeText) {
    try {
      await navigator.clipboard.writeText(text);
      return;
    } catch (_) { /* Fall through to the user-gesture copy path. */ }
  }
  const active = document.activeElement;
  const field = document.createElement('textarea');
  field.value = text;
  field.setAttribute('readonly', '');
  field.style.cssText = 'position:fixed;top:0;left:0;opacity:0;font-size:16px;';
  document.body.appendChild(field);
  field.focus();
  field.select();
  field.setSelectionRange(0, text.length);
  let copied = false;
  try { copied = document.execCommand('copy'); }
  catch (_) { /* The UI will offer manual selection. */ }
  finally { field.remove(); active?.focus(); }
  if (!copied) throw new Error('Your browser blocked copying. Select the text below and copy it manually.');
}
