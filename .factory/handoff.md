# Agent Secret Capsule — adversarial review 3 handoff

## Outcome

Review 3 is complete at revision
`19b8058765876fc100311b781687eda950da8cf1`. The verdict is **FAIL** with two
blocking findings and one minor finding. The full evidence, exact copy audit,
claim results, route checks, and prior-finding ledger are in
`.factory/review-3.md`.

No product code was changed.

## Findings left for the next repair round

- **F-3-1 / review-1 B2 — BLOCKING:** The one-click demo hides the usable result
  below the initial viewport at both review sizes. On 390×844, output starts at
  y=834 and the receipt at y=1154; on 1440×900, only the terminal command and
  receipt alias begin at the bottom edge. The sample is also a generic
  `demo-api`/`sh` echo rather than a realistic coding-agent task.
- **F-3-2 / review-1 B3, U16, U36, U39–U42 — BLOCKING:** Public claims about
  credential lifecycle, receipt commands/schema, the stated 30-second limit,
  CLI help/non-TTY behavior, and browser/CLI sample parity lack manifest entries
  with successful tagged end-to-end tests.
- **F-3-3 — MINOR:** At 390×844, only **Local CLI** is fully visible from the
  three-fact strip. The second fact is clipped and the third is below the fold.

## Verified

- Cold live first read at 390×844 and 1440×900: the job, audience, and first
  action are clear at both widths.
- All eight `.factory/claims.json` commands passed independently after `npm ci`
  in a fresh non-local clone.
- Browser demo isolation passed manually: only `demo:asc:run-count` was created;
  Reset and Start for real removed it while preserving seeded non-demo storage.
- The live demo reloaded offline after its first online visit. Its request log
  was same-origin only and contained no console errors.
- `asc --json demo` ran from a temporary working directory with an unavailable
  keychain and sentinel `ASC_HOME`; it produced two redacted receipts in a new
  `/tmp/asc-demo-*` directory and did not touch the sentinel.
- Live route/metadata/link/back-focus checks passed. The unknown route returned
  a designed HTTP 404. All crawled links and discovery assets returned their
  expected status/content type.
- Live Axe checks found zero serious/critical findings across `/`, `/demo/`,
  `/privacy/`, `/terms/`, and the 404 at desktop and mobile.
- `/opt/fleet/lib/verify-url.sh` passed the four 200 routes with no console
  errors.
- The distinct concrete-and-moss visual system matches `.factory/design.md`.

## Local quality gates

```text
npm ci                 PASS (60 packages, 0 vulnerabilities)
npm test               PASS (10 Rust, 2 Vitest, 29 Playwright; 5 intentional skips)
npm run build          PASS (target/release/asc and dist/site)
```

## Recommended repair order

1. Recompose `/demo/` so populated output and receipt evidence are fully visible
   in the initial 390×844 and 1440×900 viewports; replace the generic sample with
   a realistic bundled agent-command fixture and add viewport assertions.
2. Complete `.factory/claims.json` and tagged packaged-CLI tests for every item
   in F-3-2, or narrow/remove the corresponding copy.
3. Compact the mobile landing hero so all three tested facts are above the fold.
4. Re-run the entire adversarial checklist, not only these findings.
