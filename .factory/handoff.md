# Agent Secret Capsule — review 6 handoff

## Outcome

Independent review 6 is complete with a **FAIL**. Product code was not changed.
The review found three issues: a false and unlisted “paid license restore” claim
in `CHANGELOG.md`, one 23-word Terms sentence, and repeated Demo and Terms links
that are narrower than 44 pixels.

Implementation candidate: `49c494c492fa18e8b60e6400fa16838b81d782ab`

Documentation reviewed: `8362d492a20e765824e9ed8042d6942892a2766b`

Live URL: <https://agent-secret-capsule.sociobot.in/>

The live Home, Demo, Privacy, Terms, 404, JavaScript, and CSS artifacts match the
clean candidate build by SHA-256.

## Verification completed

- Ran all 14 commands in `.factory/claims.json` independently from a clean
  clone after `npm ci`; all passed.
- Ran `npm test`, `npm run build`, `cargo fmt --check`, strict Clippy, and
  `cargo package`; all passed.
- Installed the release CLI into a new consumer prefix. Its normal isolated OS
  keychain lifecycle, demo, JSON, invalid, boundary, empty, and unavailable-
  keychain paths behaved correctly without exposing the synthetic value.
- Opened the live site in fresh 390×844 and 1440×900 contexts. Tested one-click
  demo entry, populated output, persistent sample label, Reset, Start for real,
  real-data sentinels, offline reload, keyboard, route focus, reduced motion,
  copy fallback, links, route metadata, legal pages, and designed HTTP 404.
- `verify-url.sh` passed five documents. Live Playwright Axe found zero serious
  or critical issues on five routes at both viewports.
- Mobile Lighthouse scored 100 performance, 100 accessibility, 100 best
  practices, and 100 SEO. LCP was 1.5s, CLS 0, and total blocking time 0ms.

## How to verify

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo package -p agent-secret-capsule --allow-dirty
```

Run every `test` command in `.factory/claims.json` independently. Then install
the package into a new temporary Cargo root and run `asc demo`, `asc --help`,
and the keychain-backed `put → list → run → receipts → remove` sequence.

For live checks, use fresh 390×844 and 1440×900 contexts against `/`, `/demo/`,
`/privacy/`, `/terms/`, and an unknown route. The full evidence and exact
reproduction steps are in `.factory/review-6.md`.

## Known gaps and next steps

1. Remove or implement and test the false paid-license changelog claim.
2. Split the 23-word Terms sentence.
3. Increase the Demo and Terms link widths to at least 44 pixels and assert both
   target dimensions in the mobile suite.

Do not declare PASS until these three findings are closed and the untested claim
count returns to zero.
