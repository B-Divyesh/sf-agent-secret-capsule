import './styles.css';
import { focusAndAnnounceRoute, keepSkipLinkFirstAfterRouteFocus } from './route-focus';

if (new URL(location.href).searchParams.get('demo') === '1') {
  location.replace('/demo/');
}

if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => { void navigator.serviceWorker.register('/sw.js'); });
}

keepSkipLinkFirstAfterRouteFocus();
window.addEventListener('load', focusAndAnnounceRoute);

document.querySelectorAll<HTMLButtonElement>('[data-copy-target]').forEach((button) => {
  button.addEventListener('click', async () => {
    const target = document.getElementById(button.dataset.copyTarget ?? '');
    if (!target) return;
    try {
      await navigator.clipboard.writeText(target.textContent ?? '');
      button.textContent = 'Copied install command';
    } catch {
      button.textContent = 'Select install command';
      const selection = window.getSelection();
      const range = document.createRange();
      range.selectNodeContents(target);
      selection?.removeAllRanges();
      selection?.addRange(range);
    }
  });
});

window.addEventListener('pageshow', (event) => {
  if (event.persisted) {
    focusAndAnnounceRoute();
  }
});
