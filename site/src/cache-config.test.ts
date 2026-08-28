import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

type StaticWebAppConfig = {
  globalHeaders: Record<string, string>;
  routes: Array<{ route: string; headers?: Record<string, string> }>;
};

const config = JSON.parse(
  readFileSync(new URL('../public/staticwebapp.config.json', import.meta.url), 'utf8')
) as StaticWebAppConfig;

describe('static deployment cache policy', () => {
  it('keeps documents revalidating while immutable hashed assets stay cached', () => {
    expect(config.globalHeaders['Cache-Control']).toBe('public, max-age=0, must-revalidate');
    expect(config.routes).toContainEqual({
      route: '/assets/*',
      headers: { 'Cache-Control': 'public, max-age=31536000, immutable' }
    });
  });

  it('always revalidates the service worker so deployments can update offline clients', () => {
    expect(config.routes).toContainEqual({
      route: '/sw.js',
      headers: { 'Cache-Control': 'no-cache' }
    });
  });
});
