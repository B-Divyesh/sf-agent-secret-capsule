# Agent Secret Capsule — polish 1 handoff

## Delivered

- Repaired every B1–B6 and M1–M4 item in `.factory/review-1.md`; the detailed finding map is `.factory/polish-1.md`.
- Added a one-click `/demo/` sandbox, `?demo=1` entry, persistent banner, reset/start controls, separate `demo:asc:` browser storage, and documented reset behavior.
- Added the actual `asc demo` command. It uses fake bundled data, calls the production lease/redaction/receipt code, creates a fresh temporary directory, and does not read `ASC_HOME` or the OS keychain.
- Rewrote first-screen and README copy, removed unavailable billing, and added claim inventory/tests, static route metadata, designed 404, discovery assets, legal skeleton, focus management, mobile targets, and derived social assets.

## Verification

From this checkout:

```text
npm ci                                                        PASS (60 packages, 0 vulnerabilities)
npm test                                                      PASS (10 Rust, 2 Vitest, 21 Playwright; 3 mobile-only skips)
npm run build                                                 PASS (target/release/asc and dist/site)
cargo fmt --check                                             PASS
cargo clippy --workspace --all-targets --locked -- -D warnings PASS
cargo package -p agent-secret-capsule --allow-dirty            PASS (8 files, 87.8 KiB unpacked / 24.4 KiB compressed)
```

Every command in `.factory/claims.json` was run independently and passed:

```text
@claim:demo-isolation              PASS
@claim:offline-reload              PASS
@claim:cli-demo                    PASS
@claim:redaction-forms             PASS
@claim:process-tree                PASS
@claim:captured-output-receipt     PASS
```

`verify-url.sh` passed for local `/` and `/demo/`: each report has a title, `lang=en`, one h1, main landmark, zero missing image alt attributes, zero unlabelled buttons, and zero browser errors. The Playwright Axe integration passes with zero serious/critical violations on `/`, `/demo/`, `/privacy/`, and `/terms/` at desktop and 390px. Evidence and screenshots are under `.factory/evidence/polish-1-local/` and `.factory/evidence/polish-1-demo/`.

## Production deployment and cold verification

- Commit deployed: `0a93e390baa6df87a54567388e08fe6039606441`
- Deployment: `924afb99-5623-4294-a57d-6e8cf0bf7394` via
  `/opt/fleet/lib/deploy-static.sh agent-secret-capsule dist/site`
- Live URL: `https://agent-secret-capsule.sociobot.in/`
- Cold live `verify-url.sh` passed for `/` and `/demo/`; no console errors.
- Live HTTP checks: `/`, `/demo/`, `/privacy/`, `/terms/`, `/robots.txt`,
  `/sitemap.xml`, and `/favicon.svg` return 200 with the expected content
  types. `/not-a-real-route` returns 404.
- A fresh 390px live browser context opened `?demo=1`, landed on `/demo/`,
  showed the banner, wrote only `demo:asc:run-count`, reset it to no storage,
  and made 13 same-origin requests. Screenshot:
  `.factory/evidence/polish-1-live-demo/cold-mobile.png`.
- Live Lighthouse: **100 performance, 100 accessibility, 100 best practices,
  100 SEO**; LCP 1.5 s, CLS 0, transfer 127 KiB. Report:
  `.factory/evidence/polish-1-live/lighthouse.json`.

## Run and publish

```sh
npm ci
npm test
npm run build
cargo package -p agent-secret-capsule --allow-dirty
```

The factory owns registry publishing. The ready-to-publish command is `cargo package -p agent-secret-capsule --allow-dirty`.

## Known gaps

None. The optional paid tier was deliberately removed because its configured checkout returned 404; no unavailable purchase flow is exposed.
