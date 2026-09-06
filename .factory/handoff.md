# Agent Secret Capsule — repair 2 handoff

## Outcome

Repair 2 closes every finding from review 6.

- Removed the false `paid license restore` release-note claim.
- Split the long Terms warranty sentence into two sentences.
- Made every shared header and footer link at least 44 by 44 CSS pixels.
- Added browser regressions that measure the rendered shared targets on every
  route and count rendered Terms sentences.

Implementation SHA: `f018b7d8f15a1be374575d642f4e25c158fa74d0`

Documentation SHA: recorded after this handoff is committed.

The static implementation was deployed to
<https://agent-secret-capsule.sociobot.in/>. Live Home, CSS, and main JavaScript
SHA-256 values match the local `dist/site` build from the implementation SHA.

## Verification

From a fresh clone at the implementation SHA, after `npm ci`:

- All 14 commands in `.factory/claims.json` passed independently.
- `npm test` passed: 10 Rust tests, 2 site unit tests, 40 Playwright tests, and
  6 intentional duplicate-project skips.
- `npm run build`, `cargo fmt --check`, strict Clippy, and
  `cargo package -p agent-secret-capsule --allow-dirty --list` passed.
- The packaged crate was extracted and installed into a new consumer prefix.
  Its help, fake-data demo, empty JSON states, invalid-alias path, 61-minute
  time-limit rejection, and unavailable-keychain recovery behaved as documented.
  The demo output redacted its fake credential.
- Production `verify-url.sh` passed for Home, Demo, Privacy, Terms, and the
  designed 404. The unknown route returned the expected HTTP 404.
- Fresh 1440 by 900 and 390 by 844 browser contexts confirmed the job, audience,
  and sample action before scrolling. They also confirmed one-click sample entry,
  populated output, persistent sample label, Reset, Start for real, storage
  isolation, keyboard route focus, offline demo reload, reduced motion, no console
  errors, and no horizontal overflow.
- Live Axe scans found zero serious or critical violations across five routes at
  both viewports. Every shared header/footer target measured at least 44 by 44.
- Live mobile Lighthouse: performance 100, accessibility 100, best practices 100,
  SEO 100; LCP 1503 ms, CLS 0, and total blocking time 0 ms.

Evidence is under `/work/.evidence/agent-secret-capsule-repair-2/`. The required
catalog text is copied to `/work/.evidence/catalog-description.txt`.

## How to verify

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo package -p agent-secret-capsule --allow-dirty
```

Run every test command in `.factory/claims.json` independently. For the site,
open `/` and use **Try it with sample data**. The Demo page shows the sample result
immediately and keeps its only browser state under `demo:asc:`.

## Known gaps and next steps

No current product finding remains.

This worker has no unlocked native Secret Service daemon. The installed package
therefore exercised its documented unavailable-keychain recovery path; successful
native `put → list → run → receipts → remove` remains covered by the isolated
test-keyring claim. Repeat that native lifecycle on an unlocked target OS before
distributing binaries.

No paid offer is currently advertised or registered. The product remains a free,
MIT-licensed CLI. A future one-time paid offer requires Sociobot billing
registration and a real entitlement path before it is advertised; no mock billing
flow or billing metadata was added.
