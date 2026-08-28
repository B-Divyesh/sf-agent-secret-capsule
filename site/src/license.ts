export const PRODUCT = 'agent-secret-capsule';
export const LICENSE_KEY = `sb_license:${PRODUCT}`;
export const VERDICT_KEY = `sb_license_verdict:${PRODUCT}`;
export const DAY_MS = 86_400_000;

export type Verdict = { valid: boolean; reason: string; expires_at?: string | null };
export type CachedVerdict = Verdict & { checkedAt: number };

export function apiBase(hostname: string): string {
  return hostname === `${PRODUCT}.sociobot.in`
    ? 'https://api.sociobot.in/api/v1'
    : 'https://pilot-api.sociobot.in/api/v1';
}

export function readFreshVerdict(storage: Pick<Storage, 'getItem'>, now = Date.now()): CachedVerdict | null {
  try {
    const parsed = JSON.parse(storage.getItem(VERDICT_KEY) ?? 'null') as CachedVerdict | null;
    return parsed && now - parsed.checkedAt < DAY_MS ? parsed : null;
  } catch {
    return null;
  }
}

export function consumeQueryLicense(url: URL): string | null {
  const token = url.searchParams.get('license');
  return token?.trim() || null;
}

export async function verifyLicense(base: string, token: string): Promise<Verdict> {
  const response = await fetch(`${base}/products/${PRODUCT}/verify?license=${encodeURIComponent(token)}`, {
    headers: { accept: 'application/json' }
  });
  if (!response.ok) throw new Error(`Verification service returned ${response.status}`);
  return response.json() as Promise<Verdict>;
}
