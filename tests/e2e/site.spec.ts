import AxeBuilder from '@axe-core/playwright';
import { expect, test } from '@playwright/test';

for (const path of ['/', '/privacy/', '/terms/']) {
  test(`${path} has semantic structure and no serious accessibility violations`, async ({ page }) => {
    const errors: string[] = [];
    page.on('console', (message) => { if (message.type() === 'error') errors.push(message.text()); });
    await page.goto(path);
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('h1')).toHaveCount(1);
    await expect(page.locator('html')).toHaveAttribute('lang', 'en');
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact ?? ''))).toEqual([]);
    expect(errors).toEqual([]);
  });
}

test('demo visibly scrubs the fake credential', async ({ page }) => {
  await page.goto('/');
  await page.getByRole('button', { name: 'Run fake command' }).click();
  await expect(page.getByText('[REDACTED:ASC]')).toBeVisible();
  await expect(page.locator('#demo-output')).not.toContainText('capsule_fake_token');
});

test('license query token is stored then removed from the URL', async ({ page }) => {
  await page.route('**/verify?license=test-token', async (route) => {
    await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ valid: true, reason: 'ok', expires_at: null }) });
  });
  await page.goto('/?license=test-token#license');
  await expect(page.locator('#license-status')).toContainText('License active');
  await expect(page).toHaveURL(/\/#license$/);
  expect(await page.evaluate(() => localStorage.getItem('sb_license:agent-secret-capsule'))).toBe('test-token');
  await expect(page.getByRole('button', { name: 'Download the team rollout kit' })).toBeVisible();
});

test('keyboard path reaches the primary action', async ({ page }) => {
  await page.goto('/');
  await page.keyboard.press('Tab');
  await expect(page.getByRole('link', { name: 'Skip to main content' })).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(page.locator('#main')).toBeFocused();
});

test('cached shell stays usable offline and the service worker has no pending update', async ({ page, context }) => {
  await page.goto('/');
  const worker = await page.evaluate(async () => {
    const registration = await navigator.serviceWorker.ready;
    await registration.update();
    return { active: Boolean(registration.active), waiting: Boolean(registration.waiting) };
  });
  expect(worker).toEqual({ active: true, waiting: false });

  await context.setOffline(true);
  await page.reload();
  await expect(page.locator('main')).toBeVisible();
  await context.setOffline(false);
});
