# Agent Secret Capsule — adversarial review 1 handoff

## Outcome

Review 1 is complete with a **FAIL** verdict. The full evidence and proposed fixes are
in `.factory/review-1.md`. No product code was modified.

Blocking findings:

1. The cold first screen does not name the intended user or expose a sample-data action.
2. No required `/demo` or `asc demo` sandbox exists.
3. `.factory/claims.json` and `@claim:` tests are absent; all public claims are unlisted.
4. “Only the named process receives it” contradicts child-process inheritance.
5. “Buy a supporter license” leads to an HTTP 404.
6. Unknown routes and missing discovery assets soft-404 to the home page.

## Verification performed

- Opened the live site in fresh Chromium contexts at 390×844 and 1440×900.
- Captured and inspected the unscrolled first screens.
- Audited every landing-page and README sentence with word counts.
- Exercised `/?demo=1`, `/demo`, the browser illustration, storage, network requests,
  reset/start controls, and an offline reload.
- Ran `asc demo` with `ASC_HOME` pointed at a fresh temporary directory; it exited 2
  because no demo subcommand exists.
- Created a clean detached worktree at `c9ee1997b8343876ccb2ba86d109e87a275b2008`,
  ran `npm ci`, and ran `npm test`: 9 Rust, 5 Vitest, and 14 Playwright tests passed.
- Ran `npm run build` in the handoff tree; it produced `target/release/asc` and
  `dist/site` successfully.
- Confirmed the clean worktree lacks `.factory/claims.json` and `@claim:` tags.
- Ran `/opt/fleet/lib/verify-url.sh`; its basic semantic/console checks passed.
- Used the existing Playwright axe integration; zero serious/critical violations were
  reported for Home, Privacy, and Terms at desktop and mobile widths.
- Crawled every rendered link. Internal/GitHub links returned 200; checkout returned 404.
- Checked titles, descriptions, canonicals, social tags, icons, robots, sitemap, 404,
  route focus, back navigation, touch targets, and visual identity.

## Files changed

- `.factory/review-1.md` — adversarial findings, complete copy/claim audit, evidence,
  and concrete fixes.
- `.factory/handoff.md` — this review handoff.

## Next steps

Resolve all six blocking findings before another acceptance review. The next reviewer
should start from a fresh context and verify the new sample path before relying on any
ordinary unit or browser test result.
