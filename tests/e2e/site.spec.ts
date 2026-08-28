import AxeBuilder from '@axe-core/playwright';
import { expect, test } from '@playwright/test';
import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync, rmSync } from 'node:fs';

const routes = [
  ['/', 'Agent Secret Capsule — give one command a credential'],
  ['/demo/', 'Demo — Agent Secret Capsule'],
  ['/privacy/', 'Privacy — Agent Secret Capsule'],
  ['/terms/', 'Terms — Agent Secret Capsule']
] as const;

for (const [path, title] of routes) {
  test(`${path} has metadata, semantic structure, and no serious accessibility violations`, async ({ page }) => {
    const errors: string[] = [];
    page.on('console', (message) => { if (message.type() === 'error') errors.push(message.text()); });
    await page.goto(path);
    await expect(page).toHaveTitle(title);
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('h1')).toHaveCount(1);
    await expect(page.locator('html')).toHaveAttribute('lang', 'en');
    await expect(page.locator('link[rel="canonical"]')).toHaveCount(1);
    await expect(page.locator('meta[property="og:image"]')).toHaveCount(1);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact ?? ''))).toEqual([]);
    expect(errors).toEqual([]);
  });
}

test('@claim:demo-isolation opens the isolated sample, stores no real data, and resets', async ({ page }) => {
  const requests: string[] = [];
  page.on('request', (request) => requests.push(request.url()));
  await page.goto('/?demo=1');
  await expect(page).toHaveURL(/\/demo\/$/);
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  await page.getByRole('button', { name: 'Run sample again' }).click();
  expect(await page.evaluate(() => Object.keys(localStorage))).toEqual([]);
  expect(await page.evaluate(() => Object.keys(sessionStorage))).toEqual(['demo:asc:run-count']);
  await page.getByRole('button', { name: 'Reset demo' }).click();
  expect(await page.evaluate(() => Object.keys(sessionStorage))).toEqual([]);
  expect(requests.every((url) => new URL(url).origin === 'http://127.0.0.1:4173')).toBe(true);
});

test('@claim:offline-reload works offline after the first visit', async ({ page, context }) => {
  await page.goto('/demo/');
  await page.evaluate(async () => { await navigator.serviceWorker.ready; });
  await context.setOffline(true);
  await page.reload();
  await expect(page.getByRole('heading', { level: 1 })).toBeVisible();
  await context.setOffline(false);
});

test('@claim:cli-demo creates redacted no-value sample receipts without using ASC_HOME', async ({}, testInfo) => {
  test.skip(testInfo.project.name !== 'desktop', 'The claim is exercised once, not per viewport.');
  const sentinel = `/tmp/asc-real-data-${process.pid}`;
  const output = execFileSync('cargo', ['run', '--quiet', '-p', 'agent-secret-capsule', '--', '--json', 'demo'], {
    cwd: process.cwd(), env: { ...process.env, ASC_HOME: sentinel }, encoding: 'utf8'
  });
  const demo = JSON.parse(output) as { directory: string; stdout: string; stderr: string; receipts: Array<Record<string, unknown>> };
  expect(demo.stdout).toContain('[REDACTED:ASC]');
  expect(demo.stderr).toContain('[REDACTED:ASC]');
  expect(demo.stdout).not.toContain('demo_credential_7Kp9mQ2x');
  expect(demo.stderr).not.toContain('demo_credential_7Kp9mQ2x');
  expect(existsSync(sentinel)).toBe(false);
  const receiptText = readFileSync(`${demo.directory}/receipts.jsonl`, 'utf8');
  expect(receiptText).not.toContain('demo_credential_7Kp9mQ2x');
  expect(demo.receipts).toHaveLength(2);
  rmSync(demo.directory, { recursive: true, force: true });
});

test('@claim:process-tree gives the credential to the selected process children until the time limit', async ({}, testInfo) => {
  test.skip(testInfo.project.name !== 'desktop', 'The claim is exercised once, not per viewport.');
  execFileSync('cargo', ['test', '--locked', 'selected_process_children_inherit_the_credential_and_are_redacted'], {
    cwd: process.cwd(), stdio: 'pipe'
  });
  execFileSync('cargo', ['test', '--locked', 'expired_lease_stops_command'], { cwd: process.cwd(), stdio: 'pipe' });
});

test('@claim:captured-output-receipt redacts both captured streams and omits the credential from the receipt', async ({}, testInfo) => {
  test.skip(testInfo.project.name !== 'desktop', 'The claim is exercised once, not per viewport.');
  execFileSync('cargo', ['test', '--locked', 'documented_run_scrubs_output_and_records_no_value'], {
    cwd: process.cwd(), stdio: 'pipe'
  });
});

test('real routes load, unknown routes return 404, and discovery assets exist', async ({ page, request }) => {
  await page.goto('/privacy/');
  await expect(page.locator('h1')).toBeFocused();
  await page.getByRole('link', { name: 'Terms' }).click();
  await expect(page.locator('h1')).toBeFocused();
  expect((await request.get('/not-a-real-route')).status()).toBe(404);
  expect((await request.get('/robots.txt')).headers()['content-type']).toContain('text/plain');
  expect((await request.get('/sitemap.xml')).headers()['content-type']).toMatch(/xml/);
  expect((await request.get('/favicon.svg')).headers()['content-type']).toContain('image/svg+xml');
});

test('mobile controls meet the touch target and never overflow', async ({ page }) => {
  await page.goto('/demo/');
  for (const name of ['Reset demo', 'Start for real']) {
    const box = await page.getByRole(name === 'Reset demo' ? 'button' : 'link', { name }).boundingBox();
    expect(box?.height).toBeGreaterThanOrEqual(44);
  }
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
});

test('keyboard skip link reaches main content', async ({ page }) => {
  await page.goto('/');
  await page.keyboard.press('Tab');
  await expect(page.getByRole('link', { name: 'Skip to main content' })).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(page.locator('#main')).toBeFocused();
});
