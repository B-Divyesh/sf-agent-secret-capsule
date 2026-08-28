# Agent Secret Capsule — polish 3 handoff

## Outcome

All findings from adversarial reviews 1–3 are closed. The deployed implementation
is `d9737dd9b8bfaf20ccae35ab8fcbe9cc6d90de00`; the evidence-and-handoff commit
is `1244469ad5de1fe96edb3950350e22324ea991bf`. The static deployment
`09283e34-d89d-42b4-91f0-414ed0641b9d` is live at
<https://agent-secret-capsule.sociobot.in/>.

The repaired demo is a one-click, isolated read-only deployment-status sample.
It now exposes its redacted result, expiry result, receipt alias, and receipt
outcome in the initial desktop and phone viewports. The CLI ships the same
bundled fake fixture with `asc demo`.

## What changed

- Added the feature-gated, isolated test keychain used only by compiled CLI
  claim tests. Normal release builds still use the OS keychain path.
- Added real black-box coverage for credential lifecycle, redaction forms,
  process-tree expiry, output/receipt omission, receipt commands/schema, and
  help/non-TTY behavior.
- Added browser/CLI sample parity coverage and a realistic deployment-status
  fixture in both the source and packaged crate.
- Reworked `/demo/` and compacted the mobile landing hero. The three fact lines
  are now tested above the 390×844 fold.
- Updated the manifest, README, demo documentation, copy audit, catalog line,
  offline cache version, and final cumulative ledger.

## How to run and verify

```sh
npm ci
npm test
npm run build
cargo package -p agent-secret-capsule --allow-dirty
```

Run every command in `.factory/claims.json` independently. The clean-clone
verification used `/tmp/asc-clean-WHfeRN` and passed all 13 claim commands,
then `npm test`, `npm run build`, and package listing.

The fresh-clone full suite passed 10 Rust tests, 2 Vitest tests, and 32
Playwright tests (6 intentional duplicate-project skips). Local Lighthouse
scored 100 performance, 100 accessibility, 100 best practices, and 100 SEO;
LCP was 1,825ms, CLS 0, and TBT 0.

## Live evidence

- `verify-url.sh` passed `/`, `/demo/`, `/privacy/`, and `/terms/` with no
  console errors, one h1, `lang=en`, main landmarks, and complete image alt
  coverage. Evidence is under `.factory/evidence/polish-3-live*/`.
- Cold live 1440×900 and 390×844 screenshots are in
  `.factory/evidence/polish-3-live/`. The desktop demo evidence ends at 616px;
  phone output/expiry end at 600px and receipt alias/outcome at 715/749px.
- Live Axe scans found zero serious or critical findings for landing and demo
  at both viewports. Privacy → Terms → Back restored h1 focus. All crawled
  links returned 200; `/not-a-real-route` returned a designed 404 with complete
  social metadata.
- A cold live demo reload worked offline after its first visit. Its request log
  had 19 same-origin requests, no cookies, and no localStorage writes.

## Known gaps and next steps

None. Do not publish the crate from this worker; the ready-to-publish command
is `cargo package -p agent-secret-capsule` and registry credentials remain with
the factory.
