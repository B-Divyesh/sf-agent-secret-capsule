import './styles.css';
import {
  LICENSE_KEY,
  PRODUCT,
  VERDICT_KEY,
  apiBase,
  consumeQueryLicense,
  readFreshVerdict,
  verifyLicense,
  type Verdict
} from './license';

if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => { void navigator.serviceWorker.register('/sw.js'); });
}

const byId = <T extends HTMLElement>(id: string) => document.getElementById(id) as T | null;

const demoButton = byId<HTMLButtonElement>('run-demo');
const demoOutput = byId<HTMLElement>('demo-output');
const demoState = byId<HTMLElement>('demo-state');
demoButton?.addEventListener('click', () => {
  if (!demoOutput || !demoState) return;
  demoButton.disabled = true;
  demoState.textContent = 'CAPTURING';
  demoOutput.textContent = '$ asc run demo --env API_TOKEN -- printenv API_TOKEN\n\nCapturing stdout + stderr before release…';
  window.setTimeout(() => {
    demoOutput.innerHTML = '<span class="prompt">$</span> asc run demo --env API_TOKEN -- printenv API_TOKEN\n\n<span class="redacted">[REDACTED:ASC]</span>\n\n✓ succeeded · 1 redaction · receipt asc-demo-001';
    demoState.textContent = 'LEASE CLOSED';
    demoButton.textContent = 'Run again';
    demoButton.disabled = false;
  }, window.matchMedia('(prefers-reduced-motion: reduce)').matches ? 0 : 360);
});

document.querySelectorAll<HTMLButtonElement>('[data-copy-target]').forEach((button) => {
  button.addEventListener('click', async () => {
    const target = byId<HTMLElement>(button.dataset.copyTarget ?? '');
    if (!target) return;
    try {
      await navigator.clipboard.writeText(target.textContent ?? '');
      button.textContent = 'Copied';
    } catch {
      button.textContent = 'Select command';
      const selection = window.getSelection();
      const range = document.createRange();
      range.selectNodeContents(target);
      selection?.removeAllRanges();
      selection?.addRange(range);
    }
  });
});

const form = byId<HTMLFormElement>('license-form');
const input = byId<HTMLInputElement>('license-token');
const status = byId<HTMLElement>('license-status');
const kitButton = byId<HTMLButtonElement>('download-kit');
const buyLink = byId<HTMLAnchorElement>('buy-link');
const base = apiBase(location.hostname);
if (buyLink) buyLink.href = `${base}/products/${PRODUCT}/checkout`;

function renderVerdict(verdict: Verdict, offline = false) {
  if (!status || !kitButton) return;
  if (verdict.valid) {
    status.dataset.state = 'success';
    status.textContent = offline ? 'License active from the last verified check. You appear offline.' : 'License active. Team rollout kit unlocked.';
    kitButton.hidden = false;
  } else {
    status.dataset.state = 'error';
    status.textContent = verdict.reason === 'revoked' || verdict.reason === 'expired'
      ? 'License no longer active. The free safety tools remain available.'
      : 'That license could not be verified. Check the token or purchase a license.';
    kitButton.hidden = true;
  }
}

async function checkToken(token: string, announce = true) {
  if (announce && status) {
    status.dataset.state = '';
    status.textContent = 'Verifying license…';
  }
  try {
    const verdict = await verifyLicense(base, token);
    localStorage.setItem(VERDICT_KEY, JSON.stringify({ ...verdict, checkedAt: Date.now() }));
    renderVerdict(verdict);
  } catch {
    const cached = readFreshVerdict(localStorage);
    if (cached?.valid) renderVerdict(cached, true);
    else if (status) {
      status.dataset.state = 'error';
      status.textContent = 'Verification is offline. The free CLI still works; retry when connected.';
    }
  }
}

const queryToken = consumeQueryLicense(new URL(location.href));
if (queryToken) {
  localStorage.setItem(LICENSE_KEY, queryToken);
  const cleaned = new URL(location.href);
  cleaned.searchParams.delete('license');
  history.replaceState({}, '', cleaned.pathname + cleaned.search + cleaned.hash);
  void checkToken(queryToken);
} else {
  const token = localStorage.getItem(LICENSE_KEY);
  const cached = readFreshVerdict(localStorage);
  if (token && cached) {
    renderVerdict(cached);
  } else if (token) {
    void checkToken(token, false);
  }
}

form?.addEventListener('submit', (event) => {
  event.preventDefault();
  const token = input?.value.trim();
  if (!token) return;
  localStorage.setItem(LICENSE_KEY, token);
  input.value = '';
  void checkToken(token);
});

kitButton?.addEventListener('click', () => {
  const kit = `# Agent Secret Capsule — team rollout kit\n\n1. Inventory the exact command and network endpoint.\n2. Create a least-privilege credential in the upstream service.\n3. Store it locally with: asc put <alias>\n4. Set a lease TTL no longer than the expected command runtime.\n5. Review no-value receipts with: asc receipts --json\n6. Run hostile code inside a separate process and network sandbox.\n\nReceipt retention starter: retain no-value receipts for 30 days; restrict file access to the local user; never attach raw command output to tickets.\n`;
  const url = URL.createObjectURL(new Blob([kit], { type: 'text/markdown' }));
  const link = document.createElement('a');
  link.href = url;
  link.download = 'agent-secret-capsule-team-rollout.md';
  link.click();
  URL.revokeObjectURL(url);
});
