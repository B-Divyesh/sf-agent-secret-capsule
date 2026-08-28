# Agent Secret Capsule v0.1.0 — repair handoff

## Repair status

The release-blocking independent-verification finding against candidate
`fa3cfbe38edf1d9c02a118272fed2b39d53cbebf` is repaired. The original product
shape remains a Rust single-binary CLI plus static Vite landing/docs site.

## What changed

- Fixed the Clap parser contract for `put <name>`, `run <name> --env <NAME>`,
  and `remove <name>`: the value parsers now validate and return the parsed
  `String`, rather than returning `()`, which caused Clap to panic while
  downcasting to a derived `String` field.
- Added package-included black-box tests at `crates/asc/tests/cli_parser.rs`.
  They invoke Cargo's compiled `asc` binary for valid `put`, `run`, and
  `remove` JSON paths, assert valid JSON and absence of panics/secret output,
  and verify invalid aliases and environment names produce exit 2 usage errors.
- Set the Static Web Apps response policy so documents revalidate while
  `/assets/*` receives `Cache-Control: public, max-age=31536000, immutable`.
  `sw.js` is explicitly `no-cache`, so offline clients promptly discover a new
  deployment. Unit coverage locks in both rules.
- Added desktop and 390×844 Playwright coverage for service-worker update state
  and a cached offline reload, in addition to the existing keyboard, semantic,
  reduced-motion, privacy-demo, and Axe checks.

## Exact verification evidence

All commands below ran successfully in the repair checkout on 2026-08-28 UTC.

```sh
npm ci
npm test
cargo fmt --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run build
cargo package -p agent-secret-capsule --allow-dirty
cargo install --path target/package/agent-secret-capsule-0.1.0 \
  --root /tmp/asc-consumer.dZ2xk6 --locked
/tmp/asc-consumer.dZ2xk6/bin/asc --version
```

- `npm ci` completed with 0 vulnerabilities. `npm test` passed 9 Rust tests
  (including 2 binary-level parser regressions), 5 Vitest checks, and 14
  Playwright checks across desktop and 390×844 mobile. Playwright's Axe scan
  reported no serious or critical violations on `/`, `/privacy/`, or `/terms/`.
- The controlled Rust security measure still completes 100/100 secret-bearing
  child calls without placing the configured raw secret in either captured
  stream.
- `cargo fmt --check` and strict Clippy passed. `npm run build` produced
  `target/release/asc` (3.2 MiB) and `dist/site`; the built site includes the
  Static Web Apps response policy. Initial app JS is 3.91 KiB and CSS 12.02
  KiB (both uncompressed), within the static-product budgets.
- `cargo package` verified successfully: 8 files, 84.8 KiB unpacked and
  24.1 KiB compressed. Its file list includes `tests/cli_parser.rs`. A clean
  consumer install succeeded and the installed binary reported `asc 0.1.0`.
- Direct release-binary reproduction with an isolated `ASC_HOME` returned
  normal operational JSON errors (exit 3) for valid `put smoke --stdin`,
  `run smoke --env TOKEN -- true`, and `remove smoke` because this worker has
  no unlocked Linux Secret Service. `put bad/name --stdin` returned Clap's
  normal usage error (exit 2). No output contained `panicked`, Clap's type
  mismatch text, or the synthetic test secret.

## Run, package, and deploy

```sh
npm ci
npm test
npm run build
cargo package -p agent-secret-capsule
```

Deploy `dist/site` through the factory static deployment configuration:

```sh
/opt/fleet/lib/deploy-static.sh agent-secret-capsule dist/site
```

The factory owns registry credentials; do not publish from this checkout.

## Live deployment verification

`dist/site` was deployed to
`https://agent-secret-capsule.sociobot.in/` on 2026-08-28 UTC with the factory
Static Web Apps deployment configuration. The live `index.html` SHA-256 is
`0708ad00ea557fc7387d88f7c65c55f1426eb7727c600b31602a687406577a56`, exactly
matching the locally built artifact.

- Factory `verify-url.sh` returned HTTP 200 in 862 ms with no console/page
  errors, the expected title, `lang=en`, one `<h1>`, one main landmark, and no
  missing image alt or unlabeled buttons.
- A live Chromium smoke test at both 1366×900 and 390×844 found one `<h1>` and
  `<main>`, successful skip-link keyboard navigation to `#main`, no Axe
  serious/critical violations, no console errors, only the product origin in
  normal first-load requests, an active service worker with no waiting update,
  and a successful cached offline reload.
- Live HTML responds `Cache-Control: public, max-age=0, must-revalidate`; the
  deployed hash-named main JS responds `public, max-age=31536000, immutable`.
  CSP, HSTS, nosniff, strict-origin referrer policy, and the camera/microphone/
  geolocation permissions denial headers are present.
- The production license verification endpoint accepted the product origin,
  returned `200 {"valid":false,"reason":"invalid","expires_at":null}` for
  an invalid token, and sent `Cache-Control: no-store`.

## Known environment limitation

This container has no unlocked desktop Secret Service session. The successful
binary-level regression tests prove parser behavior and the unavailable-keychain
error path, but a release smoke test on each target OS should still exercise
the real keychain `put` → `run` → `remove` flow. The CLI and site have no
telemetry; the documented license-token localStorage behavior is unchanged.

## Asset provenance

No visual asset changed in this repair. The original generated concrete/moss
hero source, prompt, and factory deployment metadata remain in
`.factory/assets/capsule-concrete-source.png` and its adjacent JSON sidecar.
