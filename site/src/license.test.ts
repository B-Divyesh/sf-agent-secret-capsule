import { describe, expect, it } from 'vitest';
import { DAY_MS, VERDICT_KEY, apiBase, consumeQueryLicense, readFreshVerdict } from './license';

describe('license helpers', () => {
  it('uses production billing only on the production hostname', () => {
    expect(apiBase('agent-secret-capsule.sociobot.in')).toContain('https://api.sociobot.in');
    expect(apiBase('localhost')).toContain('https://pilot-api.sociobot.in');
  });

  it('accepts query licenses without retaining blanks', () => {
    expect(consumeQueryLicense(new URL('https://example.test/?license=abc'))).toBe('abc');
    expect(consumeQueryLicense(new URL('https://example.test/?license=%20'))).toBeNull();
  });

  it('uses only verdicts checked within one day', () => {
    const now = 10 * DAY_MS;
    const storage = { getItem: (key: string) => key === VERDICT_KEY ? JSON.stringify({ valid: true, reason: 'ok', checkedAt: now - 1 }) : null };
    expect(readFreshVerdict(storage as Storage, now)?.valid).toBe(true);
    expect(readFreshVerdict(storage as Storage, now + DAY_MS)).toBeNull();
  });
});
