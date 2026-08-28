import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  webServer: {
    command: 'npx vite preview --config site/vite.config.ts --host 127.0.0.1',
    url: 'http://127.0.0.1:4173',
    reuseExistingServer: false
  },
  use: {
    baseURL: 'http://127.0.0.1:4173',
    trace: 'retain-on-failure'
  },
  projects: [
    { name: 'desktop', use: { ...devices['Desktop Chrome'] } },
    { name: 'mobile-390', use: { ...devices['iPhone 13'], browserName: 'chromium', viewport: { width: 390, height: 844 } } }
  ]
});
