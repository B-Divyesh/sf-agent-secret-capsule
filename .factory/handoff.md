# Agent Secret Capsule — polish 4 handoff

## Outcome

**PASS.** Release candidate `cd3d3e13661076dab0b7796a2fba07a2289b1403` was
repaired, committed as `49c494c492fa18e8b60e6400fa16838b81d782ab`, pushed to
`main`, and deployed to <https://agent-secret-capsule.sociobot.in/> through the
static work-order configuration.

All three blocking and two minor review-four findings are fixed. The repair
adds Home route-change focus/announcement behavior with a skip-link-safe
keyboard path, lists/tests the static build output claim, replaces the last
metaphorical caption/404 wording, identifies GitHub links as external, and
makes the README web demo a clickable absolute URL. The concrete-and-moss
visual system, local-first CLI artifact, real demo sandbox, and no-payment
scope are preserved.

## How to run and verify

```sh
npm ci
# Run every command listed in .factory/claims.json independently.
npm test
npm run build
cargo fmt --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo package -p agent-secret-capsule --allow-dirty --list
```

The one-click web sample is <https://agent-secret-capsule.sociobot.in/demo/>;
`?demo=1` redirects there. The CLI sample is `asc demo` and uses bundled fake
data in a new temporary directory.

## Exact evidence

- Clean clone: `/tmp/asc-round4-clean.u4edhs/repo` at the repair commit;
  `npm ci` passed with zero vulnerabilities.
- All 14 manifest claim commands passed independently, including the new
  `@claim:build-output` artifact check.
- Full suite: 10 Rust, 2 Vitest, and 38 Playwright passes; 6 intentional
  duplicate-project skips. Release CLI and `dist/site` build passed. Formatting,
  strict Clippy, and package-file listing passed.
- Local route reports/screenshots: `.factory/evidence/polish-4-local*/`.
  Lighthouse mobile: 100 performance, 100 accessibility, 100 best practices,
  100 SEO; LCP 1,843 ms, CLS 0, TBT 0.
- Live route reports/screenshots: `.factory/evidence/polish-4-live*/`.
  Live Axe found zero serious/critical findings across five routes at desktop
  and mobile. Valid routes returned 200; unknown route returned HTTP 404; all
  crawled links returned 200.
- `.factory/evidence/polish-4-live/review4-checks.json` records the cold live
  `?demo=1` flow, reset/start isolation, Home h1 focus/announcement, literal
  copy, explicit external links, clickable README demo link, and zero console
  errors.

## Known gaps and next steps

None. The product is ready for the factory’s release workflow; do not publish
the CLI from this worker. The ready-to-package command is shown above.
