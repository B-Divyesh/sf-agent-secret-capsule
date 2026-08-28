# Agent Secret Capsule — review 4 handoff

## Outcome

Adversarial review 4 is complete at
`cd3d3e13661076dab0b7796a2fba07a2289b1403`. The verdict is **FAIL** with three
blocking and two minor findings. No product code was modified and no deployment was
performed.

The first-read, demo, isolation, listed claims, build, accessibility, link
availability, metadata, routing status, and visual-identity checks pass. The open
findings are recorded in `.factory/review-4.md`:

- F-4-1: Demo → Home leaves focus on `body` and does not announce the route.
- F-4-2: three factual README verification/build statements are unlisted claims.
- F-4-3: the landing figure caption and 404 h1 retain metaphorical copy.
- F-4-4: GitHub links are not identified as external.
- F-4-5: the README web-demo path is not an absolute clickable link.

## How this review was verified

From a fresh clone at `/tmp/asc-review4-clean.hxaJNi/repo`:

```sh
npm ci
# every test command in .factory/claims.json, run independently
npm test
npm run build
cargo fmt --check
cargo clippy --workspace --all-targets --locked -- -D warnings
```

All 13 listed claim commands passed. The full suite passed 10 Rust tests, 2 Vitest
tests, and 32 Playwright tests with 6 intentional cross-project skips. The release
CLI and `dist/site` built successfully.

Live verification covered fresh 390×844 and 1440×900 contexts, one-click and direct
demo entry, reset/exit with seeded real-storage sentinels, offline reload, request
and cookie logs, CLI demo execution in a temporary directory, every route and link,
unknown-route 404 behavior, h1/back focus, metadata, `verify-url.sh`, and Axe. Valid
routes had no console errors; Axe found no serious/critical violations. Home, Demo,
Privacy, and Terms HTML hashes exactly matched the clean build.

## Next steps

Address F-4-1 through F-4-5 without weakening the passing demo or claim coverage.
Add regression coverage for Demo → Home focus and any newly listed build-output
claim. Repeat the complete review checklist before changing the verdict to PASS.
