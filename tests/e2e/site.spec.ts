import AxeBuilder from '@axe-core/playwright';
import { expect, test } from '@playwright/test';
import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync, rmSync } from 'node:fs';

const routes = [
  ['/', 'Agent Secret Capsule — give one command a credential'],
  ['/demo/', 'Demo — Agent Secret Capsule'],
  ['/privacy/', 'Privacy — Agent Secret Capsule'],
  ['/terms/', 'Terms — Agent Secret Capsule'],
  ['/404.html', 'Page not found — Agent Secret Capsule']
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
  await page.goto('/');
  await expect(page.getByRole('link', { name: 'Try it with sample data' })).toBeVisible();
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await expect(page).toHaveURL(/\/demo\/$/);
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();

  await page.goto('/?demo=1');
  await expect(page).toHaveURL(/\/demo\/$/);
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  await page.getByRole('button', { name: 'Run sample again' }).click();
  expect(await page.evaluate(() => Object.keys(localStorage))).toEqual([]);
  expect(await page.evaluate(() => Object.keys(sessionStorage))).toEqual(['demo:asc:run-count']);
  await page.reload();
  await expect(page.getByText('RUN 1')).toBeVisible();
  await page.getByRole('button', { name: 'Reset demo' }).click();
  expect(await page.evaluate(() => Object.keys(sessionStorage))).toEqual([]);
  await page.getByRole('button', { name: 'Run sample again' }).click();
  await page.getByRole('link', { name: 'Start for real' }).click();
  await expect(page).toHaveURL(/\/$/);
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

test('@claim:cli-demo creates redacted no-value sample receipts without using the keychain or ASC_HOME', async ({}, testInfo) => {
  test.skip(testInfo.project.name !== 'desktop', 'The claim is exercised once, not per viewport.');
  const sentinel = `/tmp/asc-real-data-${process.pid}`;
  rmSync(sentinel, { recursive: true, force: true });
  const output = execFileSync('cargo', ['run', '--quiet', '-p', 'agent-secret-capsule', '--', '--json', 'demo'], {
    cwd: process.cwd(),
    env: {
      ...process.env,
      ASC_HOME: sentinel,
      DBUS_SESSION_BUS_ADDRESS: 'unix:path=/tmp/asc-no-keychain-session'
    },
    encoding: 'utf8'
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

test('@claim:license-package source and packaged CLI use the MIT License', async ({}, testInfo) => {
  test.skip(testInfo.project.name !== 'desktop', 'The claim is exercised once, not per viewport.');
  const metadata = JSON.parse(execFileSync('cargo', ['metadata', '--no-deps', '--format-version', '1'], {
    cwd: process.cwd(), encoding: 'utf8'
  })) as { packages: Array<{ name: string; license: string | null }> };
  const packageMetadata = metadata.packages.find(({ name }) => name === 'agent-secret-capsule');
  expect(packageMetadata?.license).toBe('MIT');
  const license = readFileSync('LICENSE', 'utf8');
  expect(license).toContain('Permission is hereby granted, free of charge');
  expect(readFileSync('crates/asc/LICENSE', 'utf8')).toBe(license);
  const packageFiles = execFileSync('cargo', ['package', '-p', 'agent-secret-capsule', '--allow-dirty', '--list'], {
    cwd: process.cwd(), encoding: 'utf8'
  }).split('\n');
  expect(packageFiles).toContain('LICENSE');
  expect(packageFiles).toContain('README.md');
});

test('@claim:site-privacy loads without analytics, advertising cookies, or third-party scripts', async ({ page, context }) => {
  const requests: string[] = [];
  page.on('request', (request) => requests.push(request.url()));
  await page.goto('/');
  await page.goto('/demo/');
  await page.getByRole('button', { name: 'Run sample again' }).click();
  await page.goto('/privacy/');
  expect(await context.cookies()).toEqual([]);
  expect(await page.evaluate(() => Object.keys(localStorage))).toEqual([]);
  expect(requests.length).toBeGreaterThan(0);
  expect(requests.every((url) => new URL(url).origin === 'http://127.0.0.1:4173')).toBe(true);
  const scriptOrigins = await page.locator('script[src]').evaluateAll((scripts) => scripts.map((script) => new URL((script as HTMLScriptElement).src).origin));
  expect(scriptOrigins.every((origin) => origin === 'http://127.0.0.1:4173')).toBe(true);
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
  await page.goBack();
  await expect(page).toHaveTitle('Privacy — Agent Secret Capsule');
  await expect(page.locator('h1')).toBeFocused();
  expect((await request.get('/not-a-real-route')).status()).toBe(404);
  expect((await request.get('/robots.txt')).headers()['content-type']).toContain('text/plain');
  expect((await request.get('/sitemap.xml')).headers()['content-type']).toMatch(/xml/);
  expect((await request.get('/favicon.svg')).headers()['content-type']).toContain('image/svg+xml');
});

test('the 404 route has complete route-specific social metadata', async ({ page }) => {
  const response = await page.goto('/404.html');
  expect(response?.status()).toBe(200);
  await expect(page).toHaveTitle('Page not found — Agent Secret Capsule');
  await expect(page.locator('meta[property="og:type"]')).toHaveAttribute('content', 'website');
  await expect(page.locator('meta[property="og:title"]')).toHaveAttribute('content', 'Page not found — Agent Secret Capsule');
  await expect(page.locator('meta[property="og:description"]')).toHaveCount(1);
  await expect(page.locator('meta[property="og:image"]')).toHaveAttribute('content', /capsule-social\.webp$/);
  await expect(page.locator('meta[name="twitter:card"]')).toHaveAttribute('content', 'summary_large_image');
  await expect(page.locator('meta[name="twitter:title"]')).toHaveCount(1);
  await expect(page.locator('meta[name="twitter:description"]')).toHaveCount(1);
  await expect(page.locator('meta[name="twitter:image"]')).toHaveCount(1);
});

test('desktop first screen shows its audience and sample action', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'desktop', 'This is the exact 1440 by 900 review viewport.');
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto('/');
  const audience = await page.locator('.lede').boundingBox();
  const action = await page.getByRole('link', { name: 'Try it with sample data' }).boundingBox();
  expect(audience).not.toBeNull();
  expect(action).not.toBeNull();
  expect(audience!.y + audience!.height).toBeLessThanOrEqual(900);
  expect(action!.y + action!.height).toBeLessThanOrEqual(900);
});

test('mobile controls meet the touch target and never overflow', async ({ page }) => {
  await page.goto('/');
  const audience = await page.locator('.lede').boundingBox();
  const primaryAction = await page.getByRole('link', { name: 'Try it with sample data' }).boundingBox();
  expect(audience!.y + audience!.height).toBeLessThanOrEqual(844);
  expect(primaryAction!.y + primaryAction!.height).toBeLessThanOrEqual(844);
  await page.goto('/demo/');
  for (const name of ['Reset demo', 'Start for real']) {
    const box = await page.getByRole(name === 'Reset demo' ? 'button' : 'link', { name }).boundingBox();
    expect(box?.height).toBeGreaterThanOrEqual(44);
  }
  for (const link of await page.locator('footer a').all()) {
    const box = await link.boundingBox();
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
