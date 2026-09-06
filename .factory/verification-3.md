# Give one agent command a temporary credential — verification 3

**Verdict: PASS.** This independent check found zero product findings and zero
untested public claims.

- Implementation reviewed: `f018b7d8f15a1be374575d642f4e25c158fa74d0`
- Documentation baseline: `97da8082e5dd599f3d6cc76ee5563192b56ab49c`
- Live URL: <https://agent-secret-capsule.sociobot.in/>
- Date: 2026-09-06 UTC

## Job, audience, and first action

Fresh desktop (1440×900) and phone (390×844) browser contexts showed, before
scrolling:

| Check | Evidence |
| --- | --- |
| Job | “Give one agent command a temporary credential.” |
| Audience | “For developers running coding agents…” |
| First action | **Try it with sample data** |
| Desktop bounds | Audience ended at y=657, action at y=736, and facts at y=803. |
| Phone bounds | Audience ended at y=471, action at y=543, and facts at y=673. |

The live Home document, JavaScript, and CSS hashes matched a fresh build from
the reviewed checkout. The checked SHA-256 pairs were identical for
`index.html`, `assets/main-BQwBpWqm.js`, and `assets/style-Bm-cRZAp.css`.

## Demo and browser checks

The Home sample action leads to `/demo/`. The page immediately shows the
`api-gateway` production result, two redacted output locations, the time-limit
result, and a no-value receipt. Its “Demo — sample data, nothing is saved”
label remains visible.

On a fresh phone context, after seeding separate `real:sentinel` values, running
the sample created only `sessionStorage["demo:asc:run-count"]` in addition to
the sentinels. **Reset demo** returned the visible state to `READY` and removed
the demo key. The sample output contains only redaction markers, not the fake
credential. A service-worker-controlled `/demo/` reload also kept that result
visible while offline.

The phone had no horizontal overflow. Tab first focused “Skip to main content”;
Enter moved focus to `#main`. With reduced motion, the sampled transition was
`0.00001s`.

All live header and footer links measured at least 44×44 CSS pixels on each of
Home, Demo, Privacy, Terms, and the designed 404. The repaired narrow targets
measure 44×44: Demo and Terms.

`/opt/fleet/lib/verify-url.sh` passed on Home, Demo, Privacy, Terms, and
`/404.html`. Each page had its expected route title, `lang="en"`, one h1, a
main landmark, no missing image alt text, no unlabeled buttons, and no console
errors. The deliberate unknown-route response was HTTP 404 with the designed
page, which is expected. All site and linked GitHub URLs returned HTTP 200.

Live Axe scans found no serious or critical violations on those five routes at
both desktop and phone viewports.

## Declared claims from a clean checkout

A new clone at `97da8082e5dd599f3d6cc76ee5563192b56ab49c` ran `npm ci`
successfully (60 packages; zero reported vulnerabilities). Every command in
`.factory/claims.json` ran independently and passed.

| Claim | Result |
| --- | --- |
| `demo-isolation` | PASS |
| `offline-reload` | PASS |
| `cli-demo` | PASS |
| `redaction-forms` | PASS |
| `process-tree` | PASS |
| `captured-output-receipt` | PASS |
| `credential-lifecycle` | PASS |
| `receipt-commands` | PASS |
| `receipt-storage-schema` | PASS |
| `cli-interface` | PASS |
| `demo-parity` | PASS |
| `license-package` | PASS |
| `site-privacy` | PASS |
| `build-output` | PASS |

Each claim ID occurs once in the tests. The landing pages, legal pages, README,
and changelog were checked against the manifest. The former false “paid license
restore” release-note claim is absent. No public claim was left without a test.

## CLI and quality checks

From the same clean checkout:

| Check | Result |
| --- | --- |
| `npm test` | PASS: 40 Playwright passes and 6 intentional duplicate-project skips; Rust and Vitest suites also passed. |
| `npm run build` | PASS; produced the release CLI and `dist/site`. |
| `cargo fmt --check` | PASS |
| Strict Clippy | PASS: `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` |
| Package list | PASS: `cargo package -p agent-secret-capsule --allow-dirty --list` listed the expected 11 files. |

The packaged crate was installed into a new consumer prefix. The installed
artifact reports `asc 0.1.0`; its help lists commands and exit codes. `asc --json
demo` created a new temporary sample directory, returned redacted stdout and
stderr, and supplied two no-value receipts. Empty `list --json` and `receipts
--json` returned valid empty states. An invalid short `put --stdin` input
returned the documented operational error without revealing input.

This container has no unlocked native Secret Service session. Successful native
keychain persistence was therefore exercised through the isolated
`test-keyring` claim suite, while the installed artifact exercised the documented
keychain-free demo and recovery paths. This is an environment limitation, not a
product finding.

## Earlier findings

| Earlier finding | Current disposition |
| --- | --- |
| Verification 1 parser panic | Fixed; compiled claims and installed consumer commands no longer panic. |
| Verification 1 cache policy | Fixed; current static-cache test and live deployment headers pass. |
| Review 1 B1–B6, M1–M3, CW01–CW26, U01–U57 | Fixed or deliberately removed. Current live copy, routes, demo, privacy test, and claim suite cover every retained promise. |
| Review 2 F-2-1 through F-2-4 | Fixed: first screen, claim ledger, plain wording, and 404 metadata pass. |
| Review 3 F-3-1 through F-3-3 | Fixed: populated first demo view, core claim coverage, and phone fact placement pass. |
| Review 4 F-4-1 through F-4-5 | Fixed: route focus, build-output claim, literal headings, named external links, and README demo link pass. |
| Review 6 F-6-1 | Fixed: the false paid-license restore changelog claim is absent. |
| Review 6 F-6-2 | Fixed: the Terms warranty text is now two sentences, each within 22 words. |
| Review 6 F-6-3 | Fixed: shared Demo and Terms links now measure at least 44×44. |

No backend applies to this CLI product, so tenant isolation, restart persistence,
health, and HTTP rate-limit checks are not applicable.

## Evidence

Live route reports and screenshots are in
`/work/.evidence/agent-secret-capsule-verify-3/`. The clean-clone command output
was captured during this verification. No product code was changed.
