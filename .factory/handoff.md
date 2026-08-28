# Agent Secret Capsule v0.1.0 — handoff

## What shipped

- A publishable Rust `asc` binary with helpful subcommand help, documented exit
  codes, and global `--json` output.
- Named secret storage in macOS Keychain, Linux Secret Service, and Windows
  Credential Manager; secret input is hidden interactively or explicit via
  `--stdin`, so values do not enter shell history.
- A single-command lease that adds one selected environment variable, captures
  stdout/stderr before release, redacts exact raw/URL/base64/base64url/hex forms,
  preserves the subprocess exit code, and enforces a maximum 60-minute TTL.
- Lease expiry terminates the isolated Unix process group or Windows process
  tree, covering ordinary descendants that inherited the environment.
- Local JSONL receipts containing only time, alias, environment name,
  executable name, duration, outcome, exit code, TTL, and redaction count.
- Empty states and actionable errors for aliases, receipts, invalid input,
  unavailable keychains, command launch, and lease expiry.
- A Vite landing/docs site in `dist/site` with an original concrete-and-moss
  visual system, interactive safe fake-secret demo, responsive 390px layout,
  keyboard path, reduced-motion treatment, offline shell, privacy and terms.
- $19 one-time Sociobot supporter license flow: hosted checkout, return-token
  capture, localStorage, daily cached verification, offline reconciliation,
  paste-to-restore, revoked/invalid states, and an unlocked team rollout kit.
  The core safety product remains free.

## Verification

- `npm ci`: clean dependency install; `npm audit --audit-level=high`: 0 known
  vulnerabilities.
- `npm test`: passes 7 Rust tests, 3 site unit tests, and 12 Playwright tests
  across desktop Chromium and a 390×844 Chromium viewport.
- The controlled security test executes 100 credential-bearing subprocesses:
  100/100 complete successfully and neither captured stream contains the raw
  configured secret.
- `cargo clippy --workspace --all-targets -- -D warnings`: passes.
- `cargo package -p agent-secret-capsule`: packages and verifies successfully
  (22.7 KiB compressed package). Registry publishing was not
  attempted; the factory owns credentials.
- `npm run build`: passes. It produces `target/release/asc` (3.2 MiB) and
  `dist/site/index.html` plus the legal routes.
- Production site payload: 3.91 KiB main JS, 12.02 KiB CSS, 37 KiB total fonts;
  responsive hero images are 83 KiB and 187 KiB WebP.
- Lighthouse 12.2.1 mobile: Performance 100, Accessibility 100, Best Practices
  96, SEO 92; LCP 1.8 s, total blocking time 0 ms, CLS 0.
- Factory `verify-url.sh`: HTTP 200, 532 ms network-idle load, one `<h1>`,
  `lang=en`, main landmark, no missing image alt, no unlabeled buttons, and no
  console/page errors.
- Axe via Playwright: no serious or critical violations on product, privacy,
  or terms pages in either viewport.

## Run and release

```sh
npm ci
npm test
npm run build
cargo package -p agent-secret-capsule
```

Deploy the contents of `dist/site`. Publish the generated Cargo package or
platform release binaries through factory-owned credentials.

## Known gaps / factory next steps

- This headless Linux worker has no unlocked desktop Secret Service session, so
  keychain calls were compile-checked and failure-path checked; a release smoke
  test should store/run/remove once on each target OS keychain.
- The live paid product is registered later by the factory. Local/staging builds
  deliberately use `pilot-api.sociobot.in`; the production hostname switches
  automatically to `api.sociobot.in`.
- Redaction is deliberately not presented as a sandbox. An authorized process
  can still transmit a secret, write it, create an unknown transformation, or
  escape its process group/session; this limitation is prominent in CLI help,
  README, site, privacy, and terms.

## Original asset provenance

The source image and exact prompt/deployment metadata are retained in
`.factory/assets/capsule-concrete-source.png` and its adjacent JSON sidecar.
It was generated with `/opt/fleet/lib/gen-image.sh` using the factory image
deployment, inspected, and converted locally to the two shipped WebP variants.
