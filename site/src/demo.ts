import './styles.css';

if ('serviceWorker' in navigator) window.addEventListener('load', () => { void navigator.serviceWorker.register('/sw.js'); });

const demoKey = 'demo:asc:run-count';
const output = document.getElementById('demo-output')!;
const state = document.getElementById('demo-state')!;
const announce = document.getElementById('route-announcement')!;

function render() {
  const count = Number(sessionStorage.getItem(demoKey) ?? '0');
  state.textContent = count ? `RUN ${count}` : 'READY';
  output.innerHTML = `<span class="prompt">$</span> asc demo\n\nstdout credential=<span class="redacted">[REDACTED:ASC]</span>\nstderr credential=<span class="redacted">[REDACTED:ASC]</span>\n\n✓ sample command succeeded · 2 redactions\n✓ second sample command reached its 30ms time limit\n✓ receipt asc-demo-001 omits the credential`;
}

document.getElementById('rerun-demo')?.addEventListener('click', () => {
  const count = Number(sessionStorage.getItem(demoKey) ?? '0') + 1;
  sessionStorage.setItem(demoKey, String(count));
  render();
  announce.textContent = `Sample run ${count} complete`;
});
document.getElementById('reset-demo')?.addEventListener('click', () => {
  sessionStorage.removeItem(demoKey);
  render();
  announce.textContent = 'Demo reset. Sample data is back to its starting state.';
});
document.getElementById('leave-demo')?.addEventListener('click', () => {
  sessionStorage.removeItem(demoKey);
});
render();
document.querySelector<HTMLElement>('h1')?.focus();
announce.textContent = 'Demo page loaded';
