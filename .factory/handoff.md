# Agent Secret Capsule — repair 3 handoff

## Outcome

Repair 3 closes review 7 finding F-7-1.

- Removed the unproved native OS-keychain success promise from release notes,
  installed root help, `put` help, and successful human-readable output.
- Kept the platform credential-store implementation and the tested generic
  alias lifecycle intact.
- Added a default-feature regression that forces the Linux platform store
  unavailable. It proves the release-configured binary exits safely, omits the
  credential, writes no alias metadata, and cannot activate the test file store.
- Re-audited the changed words in `.factory/copy-audit.md`.

Implementation SHA: `6ce85df2f4a76d674fa95a08bda005918257cfcb`

Verification documentation baseline: `1d53583897b464d69aa496307d7de73a614253e4`

The implementation was pushed to `origin/main` and the static artifact was
deployed to <https://agent-secret-capsule.sociobot.in/>. Azure Static Web Apps
reported deployment `eb68eeb6-c281-41dd-b268-e9287a3d6eb0` as successful.

## Verification

From a fresh remote clone at the implementation SHA, `npm ci` installed 60
packages with zero reported vulnerabilities. All 14 commands in
`.factory/claims.json` then passed independently.

The implementation checkout also passed:

- `npm test`: 11 Rust tests, 2 site unit tests, and 40 Playwright tests; 6
  duplicate-project checks were intentionally skipped.
- `npm run build`: produced `target/release/asc` and `dist/site`.
- `cargo fmt --check`.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`.
- `cargo package -p agent-secret-capsule --allow-dirty`: verified an 11-file
  package.

The packaged crate was installed into a new consumer prefix. Its version, root
help, `put` help, fake-data demo, two no-value receipts, and unavailable-store
recovery behaved as documented. The fake credential was absent from the error
output. The failed store attempt wrote no alias metadata.

Production checks covered Home, Demo, Privacy, Terms, the designed 404, and an
unknown route in fresh desktop and phone browser contexts. They confirmed:

- the job, developer audience, and sample action appear before scrolling;
- one-click populated sample output and the persistent sample-data label;
- Reset returns `READY` and removes only `demo:asc:` state;
- Start for real and Reset preserve separate real-data sentinels;
- offline demo reload, keyboard skip-link focus, route focus, reduced motion,
  44 by 44 CSS-pixel shared targets, and no horizontal overflow;
- no cookies, third-party requests, console errors, or serious/critical Axe
  violations;
- HTTP 200 for real pages and the expected HTTP 404 for an unknown route.

`verify-url.sh` passed all five documents. Live Home, JavaScript, stylesheet,
and mobile hero hashes match the local deployment build. Hashed assets return
one-year immutable caching. Live mobile Lighthouse scored 99 performance, 100
accessibility, 100 best practices, and 100 SEO; LCP was 1.63 seconds, CLS was
0.038, and total blocking time was 0 milliseconds.

Evidence is under `/work/.evidence/agent-secret-capsule-repair-3/`. The required
catalog description was copied to `/work/.evidence/catalog-description.txt`.

## How to verify

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo package -p agent-secret-capsule --allow-dirty
```

Run every `test` command in `.factory/claims.json` independently. For the web
sample, open `/demo/`; its only browser state uses the `demo:asc:` prefix.

## Earlier findings

The complete review, polish, and verification history was checked. Earlier
parser, cache, first-screen, demo, claim-ledger, route, copy, legal, focus,
touch-target, privacy, and packaging findings remain fixed. Review 7's native
storage claim is now removed rather than represented by the test-only store.

## Known gaps and next steps

This worker has no unlocked native Secret Service daemon. Successful native
`put → list → run → receipts → remove` is therefore not claimed. Repeat that
lifecycle on each supported target before advertising native-keychain success.

No paid offer is advertised or registered. The product remains a free,
MIT-licensed CLI, so no billing-offer evidence applies. There is no backend;
tenant isolation, server persistence, health, and HTTP rate limits do not apply.
