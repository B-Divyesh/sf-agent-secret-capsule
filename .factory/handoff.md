# Agent Secret Capsule — review 8 handoff

## Outcome

Strict review 8 **failed** with one major finding and two untested public
claims. No product code was changed.

- Implementation reviewed: `6ce85df2f4a76d674fa95a08bda005918257cfcb`
- Documentation baseline: `80a7442b06e5a5ce5def4ed21a61564e1901aa95`
- Live URL: <https://agent-secret-capsule.sociobot.in/>
- Full report: `.factory/review-8.md`

## Open finding

F-8-1: installed `asc --help` says `doctor` reports capability and storage
paths without reading a secret, while `asc --json doctor` reports
`"telemetry": false`. Neither privacy claim has a `.factory/claims.json` entry
or a tagged test. Add one release-binary claim test that verifies both, or
remove the statements.

## What passed

- Every one of the 14 declared claim commands passed independently in a fresh
  clone pinned to the implementation SHA. Each tag occurs exactly once.
- `npm test`, `npm run build`, `cargo fmt --check`, strict all-feature Clippy,
  package verification, and a clean consumer installation passed.
- The installed CLI's demo, empty states, invalid input, duration and limit
  boundaries, unavailable-store recovery, and release/test-store separation
  behaved safely without input disclosure.
- Fresh live desktop and phone checks covered first-screen comprehension,
  one-click/direct demo entry, realistic output, persistent sample labeling,
  Reset, Start for real, unrelated-data isolation, keyboard use, focus, Back,
  reduced motion, 200% text, touch targets, overflow, Axe, privacy, offline,
  update state, links, titles, legal pages, headers, and the designed 404.
- Live Home, Demo, Privacy, Terms, 404, emitted scripts and stylesheet, and the
  phone hero matched the implementation build byte for byte.
- Mobile Lighthouse scored 100/100/100/100; LCP was 1.37 seconds, CLS 0, and
  total blocking time 0 ms.

The separately named authoritative QA attachment was not present in this
checkout or `/work/.evidence`. The complete repository verification-4 report
was read and all required paths were rechecked independently.

## How to verify

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo package -p agent-secret-capsule --allow-dirty
```

Run every `test` command in `.factory/claims.json` separately. Install the
packaged crate into a new prefix and exercise `asc --help`, `asc --json demo`,
empty JSON states, invalid inputs, time-limit bounds, and unavailable-store
recovery.

## Next step

Repair F-8-1, rerun the full claim inventory and quality gates from a clean
checkout, then repeat the installed-artifact privacy check before declaring
PASS. No backend, payment flow, or runtime AI check applies to this product.
